//! Dimension/constraint consistency.
//!
//! Execution 01, Phase 14, Task 102. Authority: MCP Article 27; Article 23
//! ("The dimension engine must therefore communicate with the constraint
//! solver").
//!
//! Coordinates three things Article 23 explicitly names as separate
//! layers that must not drift apart: the parser (a raw value became a
//! canonical millimeter/radian number, Phase 09), the dimension model
//! (that number's role -- only a `Driving` dimension *asserts* a value
//! the rest of the system must honor, Phase 10), and the solver's own
//! verdict on whether the constraint that dimension drives is actually
//! satisfiable (Phase 11-13). Only `Driving` dimensions can conflict here:
//! `Reference`/`Derived`/`Shared`/`Bounded` dimensions report geometry
//! rather than asserting a value, so an unsatisfied constraint under one
//! of those is `craftloop_sketch::DegreesOfFreedomState::Conflicting`
//! (Phase 13) -- a geometry-level fact, not a *new* dimension-specific
//! conflict this module would be fabricating (Task 105).

use craftloop_constraint::SolveResult;
use craftloop_dimension::{DimensionRole, SemanticDimension};
use craftloop_errors::Severity;
use craftloop_ids::{ConflictId, ConstraintId, CraftLoopId};

use crate::conflict::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};

/// `constraint_id` is the solver constraint this dimension drives --
/// established by whatever wires a `SemanticDimension` into a
/// `craftloop_sketch::Sketch` (outside this crate's scope; this function
/// takes the association as a given, the same way
/// `craftloop-constraint`'s own interface takes a `ConstraintId` per
/// request rather than inferring one).
pub fn validate_dimension_against_solve(
    dimension: &SemanticDimension,
    constraint_id: ConstraintId,
    solve_result: &SolveResult,
) -> Option<Conflict> {
    if dimension.role != DimensionRole::Driving {
        return None;
    }
    let diagnostic = solve_result
        .unsatisfied
        .iter()
        .find(|d| d.constraint_id == constraint_id)?;
    Some(Conflict {
        id: ConflictId::new(),
        kind: ConflictKind::DimensionConstraintMismatch,
        severity: Severity::Error,
        affected_entities: dimension
            .target
            .primitive_ids()
            .iter()
            .map(|primitive_id| format!("{primitive_id:?}"))
            .collect(),
        existing_truth: "geometry as currently solved".to_string(),
        proposed_truth: format!("driving value {}", dimension.value()),
        evidence: format!("solver residual = {}", diagnostic.residual),
        resolution_choices: vec![
            ResolutionChoice::KeepExisting,
            ResolutionChoice::ReplaceAndPropagate,
            ResolutionChoice::RemoveConstraint,
            ResolutionChoice::Cancel,
        ],
        status: ConflictStatus::Unresolved,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_constraint::{ConstraintDiagnostic, SolveStatus};
    use craftloop_dimension::{DimensionKind, DimensionTarget};
    use craftloop_ids::{DimensionId, PrimitiveId};
    use std::collections::BTreeMap;

    fn driving_dimension() -> SemanticDimension {
        SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            10.0,
        )
        .unwrap()
    }

    #[test]
    fn a_driving_dimension_whose_constraint_is_unsatisfied_is_a_conflict() {
        let dimension = driving_dimension();
        let constraint_id = ConstraintId::new();
        let result = SolveResult {
            status: SolveStatus::Unsatisfied,
            values: BTreeMap::new(),
            unsatisfied: vec![ConstraintDiagnostic {
                constraint_id,
                residual: 3.5,
            }],
        };
        let conflict =
            validate_dimension_against_solve(&dimension, constraint_id, &result).unwrap();
        assert_eq!(conflict.kind, ConflictKind::DimensionConstraintMismatch);
        assert!(conflict.evidence.contains("3.5"));
    }

    #[test]
    fn a_driving_dimension_whose_constraint_is_satisfied_has_no_conflict() {
        let dimension = driving_dimension();
        let constraint_id = ConstraintId::new();
        let result = SolveResult {
            status: SolveStatus::Solved,
            values: BTreeMap::new(),
            unsatisfied: Vec::new(),
        };
        assert!(validate_dimension_against_solve(&dimension, constraint_id, &result).is_none());
    }

    #[test]
    fn an_unsatisfied_constraint_for_a_different_dimension_does_not_leak_into_this_one() {
        let dimension = driving_dimension();
        let this_constraint = ConstraintId::new();
        let other_constraint = ConstraintId::new();
        let result = SolveResult {
            status: SolveStatus::Unsatisfied,
            values: BTreeMap::new(),
            unsatisfied: vec![ConstraintDiagnostic {
                constraint_id: other_constraint,
                residual: 3.5,
            }],
        };
        assert!(validate_dimension_against_solve(&dimension, this_constraint, &result).is_none());
    }

    #[test]
    fn non_driving_roles_never_produce_a_conflict_here_task_105() {
        // Task 105: a Reference/Derived/Shared/Bounded dimension does not
        // *assert* a value, so it cannot conflict with the solver at this
        // layer -- an unsatisfied underlying constraint is a geometry-level
        // fact (`DegreesOfFreedomState::Conflicting`, Phase 13), not a
        // fabricated dimension-specific conflict.
        let constraint_id = ConstraintId::new();
        let result = SolveResult {
            status: SolveStatus::Unsatisfied,
            values: BTreeMap::new(),
            unsatisfied: vec![ConstraintDiagnostic {
                constraint_id,
                residual: 99.0,
            }],
        };
        for role in [
            DimensionRole::Reference,
            DimensionRole::Derived,
            DimensionRole::Shared,
            DimensionRole::Bounded,
        ] {
            let dimension = SemanticDimension::new(
                DimensionId::new(),
                DimensionKind::Linear,
                role,
                DimensionTarget::Single(PrimitiveId::new()),
                10.0,
            )
            .unwrap();
            assert!(
                validate_dimension_against_solve(&dimension, constraint_id, &result).is_none(),
                "role {role:?} must not produce a conflict"
            );
        }
    }
}
