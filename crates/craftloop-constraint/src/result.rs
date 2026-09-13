//! Solve results and diagnostics.
//!
//! Execution 01, Phase 11, Task 078 (interface) and Task 082 (conflict
//! diagnostics requirement this shape was designed to satisfy). Authority:
//! Engine Contract 10 ("Never silently drop confirmed constraints");
//! Engine Contract 12 ("Unknown is distinct from invalid").

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::variable::VariableId;
use craftloop_ids::ConstraintId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolveStatus {
    /// Every requested constraint is satisfied.
    Solved,
    /// The solver converged, but at least one requested constraint is not
    /// satisfied at the solution found (over-constrained or contradictory
    /// input). Distinct from `Failed`: there *is* a numeric result, it
    /// just does not honor everything asked of it.
    Unsatisfied,
    /// The solver did not converge to any result at all (numerical
    /// failure, not a constraint-satisfaction judgment).
    Failed,
}

/// Per-constraint diagnostic detail, so a caller can point at exactly
/// which confirmed relationship is in trouble rather than reporting "the
/// sketch is broken" (Article 4: never fabricate certainty; Article 99:
/// solver feedback should be humanized -- this is the structured data a
/// later humanization layer, Phase 14, needs to do that).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintDiagnostic {
    pub constraint_id: ConstraintId,
    /// How far from satisfied this constraint is at the returned solution
    /// (0.0 for a satisfied constraint). Units match the constraint's own
    /// (millimeters for a distance, radians for an angle).
    pub residual: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolveResult {
    pub status: SolveStatus,
    /// Every variable's value at the returned solution. Present even for
    /// `Unsatisfied` (a best-effort solution still exists) but empty for
    /// `Failed` (no numeric result at all).
    pub values: BTreeMap<VariableId, f64>,
    /// Only the constraints that are *not* satisfied at this solution.
    /// Empty when `status == Solved`.
    pub unsatisfied: Vec<ConstraintDiagnostic>,
}

impl SolveResult {
    pub fn value_of(&self, variable: VariableId) -> Option<f64> {
        self.values.get(&variable).copied()
    }

    pub fn is_solved(&self) -> bool {
        self.status == SolveStatus::Solved
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    #[test]
    fn value_of_reads_back_a_stored_variable() {
        let mut values = BTreeMap::new();
        values.insert(VariableId(0), 5.0);
        let result = SolveResult {
            status: SolveStatus::Solved,
            values,
            unsatisfied: Vec::new(),
        };
        assert_eq!(result.value_of(VariableId(0)), Some(5.0));
        assert_eq!(result.value_of(VariableId(1)), None);
    }

    #[test]
    fn is_solved_is_true_only_for_the_solved_status() {
        let solved = SolveResult {
            status: SolveStatus::Solved,
            values: BTreeMap::new(),
            unsatisfied: Vec::new(),
        };
        let unsatisfied = SolveResult {
            status: SolveStatus::Unsatisfied,
            values: BTreeMap::new(),
            unsatisfied: Vec::new(),
        };
        assert!(solved.is_solved());
        assert!(!unsatisfied.is_solved());
    }

    #[test]
    fn serialization_round_trips_including_diagnostics() {
        let result = SolveResult {
            status: SolveStatus::Unsatisfied,
            values: BTreeMap::new(),
            unsatisfied: vec![ConstraintDiagnostic {
                constraint_id: ConstraintId::new(),
                residual: 2.5,
            }],
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: SolveResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, back);
    }
}
