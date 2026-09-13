//! The backend-neutral solver trait.
//!
//! Execution 01, Phase 11, Task 078. Authority: Engine Contract 10.
//!
//! Any backend (a real numeric solver chosen via Task 083's decision
//! record, or the [`ResidualChecker`] reference implementation below)
//! implements [`ConstraintSolver`]. Nothing outside this crate needs to
//! know which backend is behind the trait object -- Phase 12 wires a real
//! implementation in without any call site elsewhere in the engine
//! changing.

use std::collections::BTreeMap;

use craftloop_ids::ConstraintId;
use serde::{Deserialize, Serialize};

use crate::residual::residual;
use crate::result::{ConstraintDiagnostic, SolveResult, SolveStatus};
use crate::variable::Variable;
use crate::GeometricConstraint;

/// One constraint plus the stable domain ID it corresponds to, so a
/// [`SolveResult`]'s diagnostics can name exactly which confirmed
/// relationship (traceable back to `craftloop-dimension`/a future
/// constraint-graph entity) is unsatisfied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintRequest {
    pub id: ConstraintId,
    pub constraint: GeometricConstraint,
}

pub trait ConstraintSolver {
    /// Solve `constraints` over `variables` from scratch.
    fn solve(&mut self, variables: &[Variable], constraints: &[ConstraintRequest]) -> SolveResult;

    /// Re-solve after a change, given the previous solution as a warm
    /// start. A backend that has no real incremental algorithm may
    /// implement this by calling `solve` again with `variables`' initial
    /// guesses replaced by `previous`'s values -- still a legitimate
    /// implementation of this interface, just not an optimized one; the
    /// interface does not mandate *how* incrementality is achieved, only
    /// that a caller never has to special-case "first solve" vs.
    /// "re-solve" at the call site (Engine Contract 10: "incremental
    /// solve").
    fn resolve_incremental(
        &mut self,
        variables: &[Variable],
        constraints: &[ConstraintRequest],
        previous: &SolveResult,
    ) -> SolveResult;

    /// Which variables are underconstrained (free to vary without
    /// violating any requested constraint) at the current solution.
    /// Execution 01, Phase 13, Task 093: the raw, solver-reported input a
    /// higher domain layer (`craftloop-sketch`'s degree-of-freedom state)
    /// translates into user-facing state, per Engine Contract 10's
    /// "Variables, constraints, incremental solve, diagnostics".
    ///
    /// Default: report nothing as underconstrained. A backend that cannot
    /// determine this (like [`ResidualChecker`], which does not iterate
    /// and so has no basis for a freedom judgment) legitimately implements
    /// this interface by accepting the default -- callers must treat an
    /// empty result as "this backend did not report," not as "nothing is
    /// free."
    fn underconstrained_variables(
        &mut self,
        variables: &[Variable],
        constraints: &[ConstraintRequest],
    ) -> Vec<crate::variable::VariableId> {
        let _ = (variables, constraints);
        Vec::new()
    }
}

/// Reference implementation: does **not** search for a solution. It
/// evaluates every constraint's residual at the variables' current
/// (`initial_value`) values and reports satisfaction, nothing more. Exists
/// to prove the trait/types in this crate are actually usable end-to-end
/// without depending on a real numeric solver -- useful on its own for
/// "did applying this change already satisfy everything" checks, but not
/// a substitute for a real solver (Phase 12's job).
pub struct ResidualChecker {
    pub tolerance: f64,
}

impl ResidualChecker {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    fn evaluate(&self, variables: &[Variable], constraints: &[ConstraintRequest]) -> SolveResult {
        let values: BTreeMap<_, _> = variables.iter().map(|v| (v.id, v.initial_value)).collect();
        let mut unsatisfied = Vec::new();
        for request in constraints {
            let r = residual(&request.constraint, &values);
            if r > self.tolerance {
                unsatisfied.push(ConstraintDiagnostic {
                    constraint_id: request.id,
                    residual: r,
                });
            }
        }
        let status = if unsatisfied.is_empty() {
            SolveStatus::Solved
        } else {
            SolveStatus::Unsatisfied
        };
        SolveResult {
            status,
            values,
            unsatisfied,
        }
    }
}

impl ConstraintSolver for ResidualChecker {
    fn solve(&mut self, variables: &[Variable], constraints: &[ConstraintRequest]) -> SolveResult {
        self.evaluate(variables, constraints)
    }

    fn resolve_incremental(
        &mut self,
        variables: &[Variable],
        constraints: &[ConstraintRequest],
        _previous: &SolveResult,
    ) -> SolveResult {
        // Nothing to warm-start: this backend does not iterate at all.
        self.evaluate(variables, constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variable::{PointVariables, VariableId};
    use craftloop_ids::CraftLoopId;

    #[test]
    fn a_satisfied_system_reports_solved_with_no_diagnostics() {
        let variables = vec![Variable::new(VariableId(0), 5.0)];
        let request = ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::FixedValue {
                variable: VariableId(0),
                value: 5.0,
            },
        };
        let mut solver = ResidualChecker::new(1e-6);
        let result = solver.solve(&variables, &[request]);
        assert!(result.is_solved());
        assert!(result.unsatisfied.is_empty());
    }

    #[test]
    fn an_unsatisfied_constraint_is_named_in_diagnostics_with_its_residual() {
        let variables = vec![Variable::new(VariableId(0), 8.0)];
        let id = ConstraintId::new();
        let request = ConstraintRequest {
            id,
            constraint: GeometricConstraint::FixedValue {
                variable: VariableId(0),
                value: 5.0,
            },
        };
        let mut solver = ResidualChecker::new(1e-6);
        let result = solver.solve(&variables, &[request]);
        assert_eq!(result.status, SolveStatus::Unsatisfied);
        assert_eq!(result.unsatisfied.len(), 1);
        assert_eq!(result.unsatisfied[0].constraint_id, id);
        assert!((result.unsatisfied[0].residual - 3.0).abs() < 1e-9);
    }

    #[test]
    fn tolerance_controls_what_counts_as_satisfied() {
        let variables = vec![Variable::new(VariableId(0), 5.001)];
        let request = ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::FixedValue {
                variable: VariableId(0),
                value: 5.0,
            },
        };
        let mut loose = ResidualChecker::new(0.01);
        assert!(loose
            .solve(&variables, std::slice::from_ref(&request))
            .is_solved());

        let mut strict = ResidualChecker::new(1e-9);
        assert!(!strict.solve(&variables, &[request]).is_solved());
    }

    #[test]
    fn resolve_incremental_reflects_updated_variable_values() {
        let request = ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::FixedValue {
                variable: VariableId(0),
                value: 5.0,
            },
        };
        let mut solver = ResidualChecker::new(1e-6);
        let first = solver.solve(
            &[Variable::new(VariableId(0), 8.0)],
            std::slice::from_ref(&request),
        );
        assert!(!first.is_solved());

        let second =
            solver.resolve_incremental(&[Variable::new(VariableId(0), 5.0)], &[request], &first);
        assert!(second.is_solved());
    }

    #[test]
    fn the_default_underconstrained_variables_implementation_reports_nothing() {
        // ResidualChecker does not iterate, so it has no basis to claim any
        // variable is free -- an empty result here means "not reported,"
        // never "nothing is free" (see the trait's own doc comment).
        let variables = vec![Variable::new(VariableId(0), 5.0)];
        let mut solver = ResidualChecker::new(1e-6);
        assert!(solver
            .underconstrained_variables(&variables, &[])
            .is_empty());
    }

    #[test]
    fn distance_and_coincident_together_reflect_a_realistic_two_point_system() {
        let a = PointVariables::new(VariableId(0), VariableId(1));
        let b = PointVariables::new(VariableId(2), VariableId(3));
        let variables = vec![
            Variable::new(VariableId(0), 0.0),
            Variable::new(VariableId(1), 0.0),
            Variable::new(VariableId(2), 3.0),
            Variable::new(VariableId(3), 4.0),
        ];
        let request = ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::Distance { a, b, value: 5.0 },
        };
        let mut solver = ResidualChecker::new(1e-6);
        assert!(solver.solve(&variables, &[request]).is_solved());
    }
}
