//! Real numeric solver backend: `ezpz` behind `ConstraintSolver`.
//!
//! Execution 01, Phase 12. Authority: Engine Contract 10; Task 083's
//! decision record (`execution-evidence/solver-evaluations/`) authorized
//! this integration, pinned at exactly `ezpz = "=0.2.29"` (see this
//! crate's `Cargo.toml`).
//!
//! # Translating `GeometricConstraint` into `ezpz::Constraint`
//!
//! Every `PointVariables` maps directly to an `ezpz::DatumPoint` once a
//! dense `VariableId -> ezpz::Id` table exists (`ezpz::Id` is `u32` and
//! must be a contiguous `0..N` range matching the variable count -- see
//! `ezpz::id::IdGenerator`'s doc comment -- so this adapter builds that
//! table fresh on every solve rather than reusing craftloop's own
//! `VariableId`s directly).
//!
//! One case does not translate one-to-one: `GeometricConstraint::Radius`
//! (Phase 11) and this phase's `LineTangentToCircle`/
//! `CircleTangentToCircle` represent a circle's radius as a
//! center-to-boundary-point *distance*, deliberately avoiding a dedicated
//! scalar-radius variable in the backend-neutral interface. `ezpz`'s
//! `DatumCircle`, however, requires exactly that: a `DatumDistance` scalar
//! variable. `LineTangentToCircle`/`CircleTangentToCircle` therefore lower
//! to *two* `ezpz::Constraint`s each: a fresh synthetic radius variable
//! (never exposed outside this function, given an initial guess computed
//! from the current center/point-on-circle distance) tied to the real
//! points via `ezpz::Constraint::DistanceVar`, plus the tangency
//! constraint itself referencing that synthetic `DatumDistance`.
//!
//! # Diagnostics
//!
//! Phase 11's spike (Task 082) found `ezpz::SolveOutcome` does not expose
//! a per-constraint residual *magnitude*, only which requested-constraint
//! indices are unsatisfied -- and because tangency constraints expand into
//! two `ezpz` requests per one craftloop request, those indices would not
//! even line up with `constraints` here. Both problems are solved the same
//! way the spike already demonstrated: ignore `ezpz`'s own
//! satisfaction/index bookkeeping entirely and re-evaluate
//! `craftloop_constraint::residual` (Phase 11, independent of `ezpz`) at
//! `ezpz`'s solved values, once per original `ConstraintRequest`.

use std::collections::BTreeMap;

use craftloop_constraint::{
    residual as craftloop_residual, ConstraintDiagnostic, ConstraintRequest, ConstraintSolver,
    GeometricConstraint, PointVariables, SolveResult, SolveStatus, Variable, VariableId,
};
use ezpz::datatypes::inputs::{DatumCircle, DatumDistance, DatumLineSegment, DatumPoint};
use ezpz::datatypes::{Angle, AngleKind};
use ezpz::{
    CircleSide, Config, Constraint as EzpzConstraint, ConstraintRequest as EzpzConstraintRequest,
    Id as EzpzId, LineSide,
};

/// How far from satisfied (in the constraint's own unit) a re-evaluated
/// residual may be and still count as solved. Matches `ezpz`'s own
/// internal `EPSILON` (not public, confirmed by reading `ezpz`'s source
/// during the Phase 11 spike) so this adapter does not silently report a
/// constraint as unsatisfied when `ezpz` itself already considered it
/// solved, or vice versa.
const TOLERANCE: f64 = 1e-4;

/// The real, chosen constraint solver backend (Task 083). Stateless: all
/// per-solve bookkeeping (variable id mapping, synthetic radius
/// variables) is local to each `solve`/`resolve_incremental` call, so nothing
/// here needs `&mut self` beyond satisfying `ConstraintSolver`'s signature.
#[derive(Debug, Default)]
pub struct EzpzSolver;

impl EzpzSolver {
    pub fn new() -> Self {
        Self
    }
}

fn point_of(values: &BTreeMap<VariableId, f64>, p: PointVariables) -> (f64, f64) {
    (
        values.get(&p.x).copied().unwrap_or(0.0),
        values.get(&p.y).copied().unwrap_or(0.0),
    )
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// Per-solve translation state: the dense id mapping, a source of fresh
/// synthetic ids for circle radii, and the initial-guess list those
/// synthetic ids get appended to.
struct Lowering<'a> {
    mapping: &'a BTreeMap<VariableId, EzpzId>,
    current_values: &'a BTreeMap<VariableId, f64>,
    next_synthetic_id: EzpzId,
    guesses: &'a mut Vec<(EzpzId, f64)>,
}

impl Lowering<'_> {
    fn point(&self, p: PointVariables) -> DatumPoint {
        DatumPoint {
            x_id: self.mapping[&p.x],
            y_id: self.mapping[&p.y],
        }
    }

    fn line(&self, a: PointVariables, b: PointVariables) -> DatumLineSegment {
        DatumLineSegment {
            p0: self.point(a),
            p1: self.point(b),
        }
    }

    /// Allocate a fresh scalar variable (never a real `VariableId`), seeded
    /// with `initial_guess` so the solver starts from the current
    /// center-to-boundary-point distance rather than an arbitrary default.
    fn fresh_distance(&mut self, initial_guess: f64) -> DatumDistance {
        let id = self.next_synthetic_id;
        self.next_synthetic_id += 1;
        self.guesses.push((id, initial_guess));
        DatumDistance { id }
    }

    /// Lower one `GeometricConstraint` into one or more `ezpz::Constraint`s
    /// (more than one only for the two tangency variants, which need an
    /// auxiliary synthetic radius variable -- see the module doc comment).
    fn lower(&mut self, constraint: &GeometricConstraint) -> Vec<EzpzConstraint> {
        match constraint {
            GeometricConstraint::FixedValue { variable, value } => {
                vec![EzpzConstraint::Fixed(self.mapping[variable], *value)]
            }
            GeometricConstraint::EqualValues { a, b } => {
                vec![EzpzConstraint::ScalarEqual(
                    self.mapping[a],
                    self.mapping[b],
                )]
            }
            GeometricConstraint::Distance { a, b, value } => {
                vec![EzpzConstraint::Distance(
                    self.point(*a),
                    self.point(*b),
                    *value,
                )]
            }
            GeometricConstraint::Coincident { a, b } => {
                vec![EzpzConstraint::PointsCoincident(
                    self.point(*a),
                    self.point(*b),
                )]
            }
            GeometricConstraint::Horizontal { a, b } => {
                vec![EzpzConstraint::Horizontal(self.line(*a, *b))]
            }
            GeometricConstraint::Vertical { a, b } => {
                vec![EzpzConstraint::Vertical(self.line(*a, *b))]
            }
            GeometricConstraint::Parallel { a0, a1, b0, b1 } => {
                vec![EzpzConstraint::lines_parallel([
                    self.line(*a0, *a1),
                    self.line(*b0, *b1),
                ])]
            }
            GeometricConstraint::Perpendicular { a0, a1, b0, b1 } => {
                vec![EzpzConstraint::lines_perpendicular([
                    self.line(*a0, *a1),
                    self.line(*b0, *b1),
                ])]
            }
            GeometricConstraint::Radius {
                center,
                point_on_circle,
                value,
            } => vec![EzpzConstraint::Distance(
                self.point(*center),
                self.point(*point_on_circle),
                *value,
            )],
            GeometricConstraint::Angle {
                vertex,
                a,
                b,
                value_radians,
            } => vec![EzpzConstraint::PointsAtAngle(
                self.point(*vertex),
                self.point(*a),
                self.point(*b),
                AngleKind::Other(Angle::from_radians(*value_radians)),
            )],
            GeometricConstraint::EqualLength { a0, a1, b0, b1 } => {
                vec![EzpzConstraint::LinesEqualLength(
                    self.line(*a0, *a1),
                    self.line(*b0, *b1),
                )]
            }
            GeometricConstraint::LineTangentToCircle {
                line_a,
                line_b,
                center,
                point_on_circle,
            } => {
                let guess = distance(
                    point_of(self.current_values, *center),
                    point_of(self.current_values, *point_on_circle),
                );
                let radius = self.fresh_distance(guess);
                let center_point = self.point(*center);
                let point_on_circle_point = self.point(*point_on_circle);
                let line = self.line(*line_a, *line_b);
                vec![
                    EzpzConstraint::DistanceVar(center_point, point_on_circle_point, radius),
                    EzpzConstraint::LineTangentToCircle(
                        line,
                        DatumCircle {
                            center: center_point,
                            radius,
                        },
                        LineSide::Undefined,
                    ),
                ]
            }
            GeometricConstraint::CircleTangentToCircle {
                a_center,
                a_point_on_circle,
                b_center,
                b_point_on_circle,
            } => {
                let guess_a = distance(
                    point_of(self.current_values, *a_center),
                    point_of(self.current_values, *a_point_on_circle),
                );
                let guess_b = distance(
                    point_of(self.current_values, *b_center),
                    point_of(self.current_values, *b_point_on_circle),
                );
                let radius_a = self.fresh_distance(guess_a);
                let radius_b = self.fresh_distance(guess_b);
                let center_a = self.point(*a_center);
                let point_a = self.point(*a_point_on_circle);
                let center_b = self.point(*b_center);
                let point_b = self.point(*b_point_on_circle);
                vec![
                    EzpzConstraint::DistanceVar(center_a, point_a, radius_a),
                    EzpzConstraint::DistanceVar(center_b, point_b, radius_b),
                    EzpzConstraint::CircleTangentToCircle(
                        DatumCircle {
                            center: center_a,
                            radius: radius_a,
                        },
                        DatumCircle {
                            center: center_b,
                            radius: radius_b,
                        },
                        CircleSide::Exterior,
                    ),
                ]
            }
            GeometricConstraint::Symmetric {
                axis_a,
                axis_b,
                a,
                b,
            } => {
                vec![EzpzConstraint::Symmetric(
                    self.line(*axis_a, *axis_b),
                    self.point(*a),
                    self.point(*b),
                )]
            }
        }
    }
}

/// The dense `VariableId -> ezpz::Id` mapping, the initial-guess list
/// (real variables plus any synthetic radius variables), and the lowered
/// `ezpz` constraint requests -- what [`prepare`] builds, shared by both a
/// real solve and a freedom analysis.
type Prepared = (
    BTreeMap<VariableId, EzpzId>,
    Vec<(EzpzId, f64)>,
    Vec<EzpzConstraintRequest>,
);

/// Shared translation step for both a real solve and a freedom analysis:
/// build the dense `VariableId -> ezpz::Id` mapping, the initial-guess
/// list (real variables plus any synthetic radius variables tangency
/// constraints need -- see the module doc comment), and the lowered
/// `ezpz` constraint requests.
fn prepare(variables: &[Variable], constraints: &[ConstraintRequest]) -> Prepared {
    let mut mapping: BTreeMap<VariableId, EzpzId> = BTreeMap::new();
    let mut guesses: Vec<(EzpzId, f64)> = Vec::with_capacity(variables.len());
    let mut current_values: BTreeMap<VariableId, f64> = BTreeMap::new();
    for (index, variable) in variables.iter().enumerate() {
        let id = index as EzpzId;
        mapping.insert(variable.id, id);
        guesses.push((id, variable.initial_value));
        current_values.insert(variable.id, variable.initial_value);
    }

    let mut lowering = Lowering {
        mapping: &mapping,
        current_values: &current_values,
        next_synthetic_id: variables.len() as EzpzId,
        guesses: &mut guesses,
    };
    let mut requests: Vec<EzpzConstraintRequest> = Vec::new();
    for request in constraints {
        for ezpz_constraint in lowering.lower(&request.constraint) {
            requests.push(EzpzConstraintRequest::highest_priority(ezpz_constraint));
        }
    }
    (mapping, guesses, requests)
}

fn solve_from(variables: &[Variable], constraints: &[ConstraintRequest]) -> SolveResult {
    let (mapping, guesses, requests) = prepare(variables, constraints);

    match ezpz::solve(&requests, guesses, Config::default()) {
        Ok(outcome) => {
            let final_values = outcome.final_values();
            let values: BTreeMap<VariableId, f64> = mapping
                .iter()
                .map(|(variable_id, ezpz_id)| (*variable_id, final_values[*ezpz_id as usize]))
                .collect();
            let mut unsatisfied = Vec::new();
            for request in constraints {
                let residual = craftloop_residual(&request.constraint, &values);
                if residual > TOLERANCE {
                    unsatisfied.push(ConstraintDiagnostic {
                        constraint_id: request.id,
                        residual,
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
        Err(_failure) => SolveResult {
            status: SolveStatus::Failed,
            values: BTreeMap::new(),
            unsatisfied: Vec::new(),
        },
    }
}

/// Task 093: `ezpz` already computes exactly this (`solve_analysis`'s
/// `FreedomAnalysis::underconstrained`) -- using it directly, rather than
/// re-deriving DOF counts by hand from the constraint graph, follows the
/// same "prefer a demonstrated solver capability over a hand-built
/// heuristic" judgment Phase 11's evaluation made about the solver choice
/// itself.
fn underconstrained_from(
    variables: &[Variable],
    constraints: &[ConstraintRequest],
) -> Vec<VariableId> {
    let (mapping, guesses, requests) = prepare(variables, constraints);
    let Ok(analysis) = ezpz::solve_analysis(&requests, guesses, Config::default()) else {
        return Vec::new();
    };
    let underconstrained: std::collections::BTreeSet<EzpzId> = analysis
        .analysis
        .underconstrained()
        .iter()
        .copied()
        .collect();
    mapping
        .iter()
        .filter_map(|(variable_id, ezpz_id)| {
            underconstrained.contains(ezpz_id).then_some(*variable_id)
        })
        .collect()
}

impl ConstraintSolver for EzpzSolver {
    fn solve(&mut self, variables: &[Variable], constraints: &[ConstraintRequest]) -> SolveResult {
        solve_from(variables, constraints)
    }

    fn resolve_incremental(
        &mut self,
        variables: &[Variable],
        constraints: &[ConstraintRequest],
        previous: &SolveResult,
    ) -> SolveResult {
        // Warm start: seed each variable's initial guess from the previous
        // solution when available (Engine Contract 10 "incremental solve"
        // -- see this trait's own doc comment on what a legitimate,
        // non-optimized implementation looks like).
        let warm_started: Vec<Variable> = variables
            .iter()
            .map(|v| Variable::new(v.id, previous.value_of(v.id).unwrap_or(v.initial_value)))
            .collect();
        solve_from(&warm_started, constraints)
    }

    fn underconstrained_variables(
        &mut self,
        variables: &[Variable],
        constraints: &[ConstraintRequest],
    ) -> Vec<VariableId> {
        underconstrained_from(variables, constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_constraint::PointVariables;
    use craftloop_ids::{ConstraintId, CraftLoopId};

    fn req(id: ConstraintId, constraint: GeometricConstraint) -> ConstraintRequest {
        ConstraintRequest { id, constraint }
    }

    #[test]
    fn a_simple_two_point_distance_system_solves() {
        let px = VariableId(0);
        let py = VariableId(1);
        let qx = VariableId(2);
        let qy = VariableId(3);
        let p = PointVariables::new(px, py);
        let q = PointVariables::new(qx, qy);
        let variables = vec![
            Variable::new(px, 0.0),
            Variable::new(py, -0.02),
            Variable::new(qx, 4.39),
            Variable::new(qy, 4.38),
        ];
        let constraints = vec![
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: px,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: py,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Distance {
                    a: p,
                    b: q,
                    value: 4.0,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let result = solver.solve(&variables, &constraints);
        assert!(result.is_solved(), "expected solved, got {result:?}");
    }

    #[test]
    fn a_genuinely_contradictory_system_is_reported_unsatisfied_not_solved() {
        let px = VariableId(0);
        let py = VariableId(1);
        let variables = vec![Variable::new(px, 5.0), Variable::new(py, 0.0)];
        let id_low = ConstraintId::new();
        let id_high = ConstraintId::new();
        let constraints = vec![
            req(
                id_low,
                GeometricConstraint::FixedValue {
                    variable: px,
                    value: 0.0,
                },
            ),
            req(
                id_high,
                GeometricConstraint::FixedValue {
                    variable: px,
                    value: 10.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: py,
                    value: 0.0,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let result = solver.solve(&variables, &constraints);
        assert_eq!(result.status, SolveStatus::Unsatisfied);
        assert_eq!(result.unsatisfied.len(), 2);
        let ids: Vec<_> = result.unsatisfied.iter().map(|d| d.constraint_id).collect();
        assert!(ids.contains(&id_low));
        assert!(ids.contains(&id_high));
    }

    #[test]
    fn line_tangent_to_circle_solves_through_the_synthetic_radius_translation() {
        let line_x = VariableId(0);
        let p0_y = VariableId(1);
        let p1_y = VariableId(2);
        let center_x = VariableId(3);
        let center_y = VariableId(4);
        let point_x = VariableId(5);
        let point_y = VariableId(6);
        // p0 and p1 share the same x variable (line_x), enforcing
        // verticality by construction (same trick used in the Phase 11
        // spike extension).
        let p0 = PointVariables::new(line_x, p0_y);
        let p1 = PointVariables::new(line_x, p1_y);
        let center = PointVariables::new(center_x, center_y);
        let point_on_circle = PointVariables::new(point_x, point_y);

        let variables = vec![
            Variable::new(line_x, 2.5),
            Variable::new(p0_y, -5.0),
            Variable::new(p1_y, 5.0),
            Variable::new(center_x, 0.0),
            Variable::new(center_y, 0.0),
            Variable::new(point_x, 3.0),
            Variable::new(point_y, 0.0),
        ];
        let constraints = vec![
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: p0_y,
                    value: -5.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: p1_y,
                    value: 5.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: center_x,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: center_y,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Radius {
                    center,
                    point_on_circle,
                    value: 3.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::LineTangentToCircle {
                    line_a: p0,
                    line_b: p1,
                    center,
                    point_on_circle,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let result = solver.solve(&variables, &constraints);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let solved_x = result.value_of(line_x).unwrap();
        assert!(
            (solved_x.abs() - 3.0).abs() < 1e-6,
            "expected +/-3.0, got {solved_x}"
        );
    }

    #[test]
    fn circle_tangent_to_circle_solves_through_the_synthetic_radius_translation() {
        let a_cx = VariableId(0);
        let a_cy = VariableId(1);
        let a_px = VariableId(2);
        let a_py = VariableId(3);
        let b_cx = VariableId(4);
        let b_cy = VariableId(5);
        let b_px = VariableId(6);
        let b_py = VariableId(7);
        let a_center = PointVariables::new(a_cx, a_cy);
        let a_point = PointVariables::new(a_px, a_py);
        let b_center = PointVariables::new(b_cx, b_cy);
        let b_point = PointVariables::new(b_px, b_py);

        let variables = vec![
            Variable::new(a_cx, 0.0),
            Variable::new(a_cy, 0.0),
            Variable::new(a_px, 2.0),
            Variable::new(a_py, 0.0),
            Variable::new(b_cx, 2.7),
            Variable::new(b_cy, 0.0),
            Variable::new(b_px, 3.7),
            Variable::new(b_py, 0.0),
        ];
        let constraints = vec![
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: a_cx,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: a_cy,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Radius {
                    center: a_center,
                    point_on_circle: a_point,
                    value: 2.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: b_cy,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Radius {
                    center: b_center,
                    point_on_circle: b_point,
                    value: 1.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::CircleTangentToCircle {
                    a_center,
                    a_point_on_circle: a_point,
                    b_center,
                    b_point_on_circle: b_point,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let result = solver.solve(&variables, &constraints);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let solved_bx = result.value_of(b_cx).unwrap();
        assert!(
            (solved_bx.abs() - 3.0).abs() < 1e-6,
            "expected +/-3.0, got {solved_bx}"
        );
    }

    #[test]
    fn symmetric_points_solve_through_the_direct_translation() {
        let axis_p0x = VariableId(0);
        let axis_p0y = VariableId(1);
        let axis_p1x = VariableId(2);
        let axis_p1y = VariableId(3);
        let ax = VariableId(4);
        let ay = VariableId(5);
        let bx = VariableId(6);
        let by = VariableId(7);
        let axis_a = PointVariables::new(axis_p0x, axis_p0y);
        let axis_b = PointVariables::new(axis_p1x, axis_p1y);
        let a = PointVariables::new(ax, ay);
        let b = PointVariables::new(bx, by);

        let variables = vec![
            Variable::new(axis_p0x, 0.0),
            Variable::new(axis_p0y, -5.0),
            Variable::new(axis_p1x, 0.0),
            Variable::new(axis_p1y, 5.0),
            Variable::new(ax, 3.0),
            Variable::new(ay, 2.0),
            Variable::new(bx, -2.8),
            Variable::new(by, 2.1),
        ];
        let constraints = vec![
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: axis_p0x,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: axis_p0y,
                    value: -5.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: axis_p1x,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: axis_p1y,
                    value: 5.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: ax,
                    value: 3.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: ay,
                    value: 2.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Symmetric {
                    axis_a,
                    axis_b,
                    a,
                    b,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let result = solver.solve(&variables, &constraints);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        assert!((result.value_of(bx).unwrap() - -3.0).abs() < 1e-6);
        assert!((result.value_of(by).unwrap() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn underconstrained_variables_reports_a_point_only_bound_by_distance_as_free() {
        // Mirrors ezpz's own `solve_analysis` doc example exactly,
        // including its initial guess -- deliberately, not incidentally.
        // Freedom analysis is a *local* (linearized) judgment: an earlier
        // version of this test placed Q at (4.0, 0.0), exactly on the
        // x-axis from P, and only qy came back underconstrained. That is
        // correct, not a bug -- at that specific point on the constraint
        // circle, the only *locally* free direction (the tangent to the
        // circle) is purely vertical; moving in x there immediately
        // changes the Distance residual, so x is not locally free even
        // though the point as a whole has one genuine degree of freedom
        // along the circle. Off that axis-aligned special case (as here),
        // both components of Q's tangential freedom show up. This is
        // recorded in Task 093's higher-level DOF state as a caveat: a
        // per-variable "not reported free" is not proof of full
        // constraint at a degenerate initial guess.
        let px = VariableId(0);
        let py = VariableId(1);
        let qx = VariableId(2);
        let qy = VariableId(3);
        let p = PointVariables::new(px, py);
        let q = PointVariables::new(qx, qy);
        let variables = vec![
            Variable::new(px, 0.0),
            Variable::new(py, -0.02),
            Variable::new(qx, 4.39),
            Variable::new(qy, 4.38),
        ];
        let constraints = vec![
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: px,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::FixedValue {
                    variable: py,
                    value: 0.0,
                },
            ),
            req(
                ConstraintId::new(),
                GeometricConstraint::Distance {
                    a: p,
                    b: q,
                    value: 4.0,
                },
            ),
        ];
        let mut solver = EzpzSolver::new();
        let free = solver.underconstrained_variables(&variables, &constraints);
        assert!(free.contains(&qx), "expected qx free, got {free:?}");
        assert!(free.contains(&qy), "expected qy free, got {free:?}");
        assert!(
            !free.contains(&px),
            "px is fixed, must not be reported free"
        );
        assert!(
            !free.contains(&py),
            "py is fixed, must not be reported free"
        );
    }

    #[test]
    fn resolve_incremental_warm_starts_from_the_previous_solution() {
        let px = VariableId(0);
        let py = VariableId(1);
        let variables = vec![Variable::new(px, 0.0), Variable::new(py, 0.0)];
        let constraint = req(
            ConstraintId::new(),
            GeometricConstraint::FixedValue {
                variable: px,
                value: 7.0,
            },
        );
        let mut solver = EzpzSolver::new();
        let first = solver.solve(&variables, std::slice::from_ref(&constraint));
        assert!(first.is_solved());
        assert!((first.value_of(px).unwrap() - 7.0).abs() < 1e-6);

        // Re-solve with stale initial guesses; the warm start should still
        // recover the same solution.
        let stale = vec![Variable::new(px, -100.0), Variable::new(py, 50.0)];
        let second = solver.resolve_incremental(&stale, std::slice::from_ref(&constraint), &first);
        assert!(second.is_solved());
        assert!((second.value_of(px).unwrap() - 7.0).abs() < 1e-6);
    }
}
