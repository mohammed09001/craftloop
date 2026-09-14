//! The constraint engine's primitive store and solve loop.
//!
//! Execution 01, Phase 12, Tasks 084-092. Authority: Engine Contract 10.
//!
//! `Sketch` owns a set of [`BeautifiedPrimitive`]s (Phase 06) keyed by
//! [`PrimitiveId`], plus the constraints (Tasks 084-090) that relate them,
//! each carrying its [`ConstraintProvenance`] (Task 091). [`Sketch::solve`]
//! is the whole point of "Constraint Engine Integration" (this phase's
//! title): translate every stored constraint into
//! `craftloop-constraint`'s backend-neutral vocabulary, hand it to a real
//! [`ConstraintSolver`] (Phase 12's `EzpzSolver`, or any other backend a
//! caller supplies -- including tests, which mostly use the reference
//! `ResidualChecker` where a real numeric solve is not what is being
//! tested), and write the solved geometry back into the stored primitives
//! -- "maintain the relation through edits" (Task 084's objective) and
//! "stable drag behavior" (Task 085's objective) both fall out of this:
//! every `solve()` seeds its initial guesses from whatever the primitives
//! currently hold, so a caller who mutates a primitive directly and calls
//! `solve()` again gets a warm start "for free," without this crate having
//! to duplicate `ConstraintSolver::resolve_incremental`'s bookkeeping at
//! this layer as well.
//!
//! Fresh solver variable IDs are allocated on every `solve()` call rather
//! than kept stable across calls: they are purely a translation detail
//! between this crate's `PointRef`s and the backend, never observed by a
//! caller, so there is no correctness reason to persist them, and *not*
//! persisting them means adding or removing a constraint between two
//! `solve()` calls never has to reconcile a stale ID table.

use std::collections::{BTreeMap, BTreeSet};

use craftloop_constraint::{
    ConstraintRequest, ConstraintSolver, PointVariables, SolveResult, SolveStatus, Variable,
    VariableId,
};
use craftloop_errors::{DomainError, DomainResult, SketchErrorKind};
use craftloop_geometry::Point2;
use craftloop_ids::{ConstraintId, PrimitiveId};
use craftloop_recognition::{Beautified, BeautifiedPrimitive};
use serde::{Deserialize, Serialize};

use crate::constraint_kind::SketchConstraintKind;
use crate::point_ref::{PointRef, PrimitiveMap};
use crate::provenance::ConstraintProvenance;

/// Execution 02, Phase 04: `Debug`/`Clone`/`PartialEq`/`Serialize`/
/// `Deserialize` were added so a whole `Sketch` can be a
/// `craftloop-document::Document` field and survive save/reopen
/// (Article 35's Golden Alpha Journey) -- every field type here
/// (`SketchConstraintKind`, `ConstraintProvenance`) already derived
/// `Serialize`/`Deserialize` for its own persistence needs (Phase 12),
/// so this is additive, not a new capability those types lacked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConstraintEntry {
    pub(crate) kind: SketchConstraintKind,
    pub(crate) provenance: ConstraintProvenance,
}

/// What [`Sketch::add_constraint`] actually did. Task 096: recognizing a
/// duplicate is deliberately not an error (see `add_constraint`'s doc
/// comment) -- this type carries the plain-language-ready distinction
/// instead of forcing every caller to special-case a specific error kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintOutcome {
    /// A new constraint was stored.
    Added,
    /// An equivalent constraint already existed; nothing new was stored.
    Redundant { existing: ConstraintId },
}

impl ConstraintOutcome {
    /// Task 096/099: plain-language diagnostic, e.g. for a toast/inline
    /// message when a user re-applies a relationship that already holds.
    pub fn explain(&self) -> String {
        match self {
            ConstraintOutcome::Added => "Relationship added.".to_string(),
            ConstraintOutcome::Redundant { .. } => {
                "This relationship is already guaranteed by an existing one.".to_string()
            }
        }
    }
}

/// A set of primitives plus the constraints relating them. See the module
/// doc comment for `solve()`'s role.
///
/// Execution 02, Phase 04: `Debug`/`Clone`/`PartialEq`/`Serialize`/
/// `Deserialize` were added (see `ConstraintEntry`'s doc comment) so a
/// `Document` can hold and persist a `Sketch` directly rather than every
/// caller (`craftloop-mobile-ffi`'s `CraftLoopSession`) needing to
/// reconstruct one from scratch after every reopen.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sketch {
    pub(crate) primitives: PrimitiveMap,
    pub(crate) constraints: BTreeMap<ConstraintId, ConstraintEntry>,
    /// Task 110: declined suggestions, kept only well enough to recognize
    /// "already asked and declined" -- never enforced like a real
    /// constraint (see `intent.rs`).
    pub(crate) rejected_suggestions: Vec<SketchConstraintKind>,
}

impl Sketch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_primitive(&mut self, id: PrimitiveId, primitive: Beautified) {
        self.primitives.insert(id, primitive);
    }

    pub fn primitive(&self, id: PrimitiveId) -> Option<&Beautified> {
        self.primitives.get(&id)
    }

    /// Execution 02, Phase 04: every primitive id this sketch currently
    /// stores. `craftloop-mobile-ffi::CraftLoopSession::solve_constraints`
    /// needs this to know which primitives to diff after a solve (before
    /// this method existed, a caller had no way to enumerate what a
    /// `Sketch` holds without already knowing every id in advance).
    pub fn primitive_ids(&self) -> impl Iterator<Item = PrimitiveId> + '_ {
        self.primitives.keys().copied()
    }

    /// Execution 02, Phase 04: remove one primitive (and, implicitly,
    /// every constraint that referenced it becomes unresolvable at the
    /// next `solve()` -- callers deleting geometry a live constraint
    /// depends on are expected to remove that constraint too, the same
    /// ordering `craftloop-mobile-ffi::CraftLoopSession` already commits
    /// as one atomic transaction). Returns the removed value, mirroring
    /// `BTreeMap::remove`'s own convention (`Page::remove`,
    /// `DimensionStore::remove_dimension`) elsewhere in this workspace.
    pub fn remove_primitive(&mut self, id: PrimitiveId) -> Option<Beautified> {
        self.primitives.remove(&id)
    }

    /// Task 084/091: add one constraint with its provenance. Validated
    /// against the current primitives before being stored (Task 084's
    /// forbidden shortcut: do not discover a type mismatch mid-solve).
    ///
    /// Task 096: a constraint expressing exactly the same relationship as
    /// one already stored (see [`SketchConstraintKind::is_equivalent`]) is
    /// **not** an error (Article 312: "should not necessarily create an
    /// error") -- it is recognized as redundant and not inserted a second
    /// time, and the caller is told which existing constraint already
    /// covers it.
    pub fn add_constraint(
        &mut self,
        id: ConstraintId,
        kind: SketchConstraintKind,
        provenance: ConstraintProvenance,
    ) -> DomainResult<ConstraintOutcome> {
        if self.constraints.contains_key(&id) {
            return Err(DomainError::Sketch {
                kind: SketchErrorKind::DuplicateConstraintId,
                detail: format!("constraint {id:?} already exists"),
            });
        }
        kind.validate(&self.primitives)?;
        if let Some((&existing, _)) = self
            .constraints
            .iter()
            .find(|(_, entry)| entry.kind.is_equivalent(&kind))
        {
            return Ok(ConstraintOutcome::Redundant { existing });
        }
        self.constraints
            .insert(id, ConstraintEntry { kind, provenance });
        Ok(ConstraintOutcome::Added)
    }

    pub fn remove_constraint(&mut self, id: ConstraintId) -> DomainResult<()> {
        self.constraints
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| DomainError::Sketch {
                kind: SketchErrorKind::UnknownConstraint,
                detail: format!("no constraint with id {id:?}"),
            })
    }

    /// Execution 02, Phase 04: insert a constraint entry without
    /// `add_constraint`'s validation/duplicate-id/redundancy checks. Used
    /// only by `craftloop-document::history::DocumentChange::apply` to
    /// replay a constraint that was already accepted once, through
    /// `add_constraint` itself, when `CraftLoopSession` first applied it
    /// -- re-running redundancy detection on redo could wrongly collapse
    /// a *distinct* constraint into `Redundant` if some other equivalent
    /// constraint happens to exist at that point in history, which would
    /// silently fail to restore the exact state being redone. Same
    /// "already validated once, this only replays it" reasoning as
    /// `MultiviewGraph::insert_binding_unchecked`/
    /// `DimensionStore::set_dimension`.
    pub fn set_constraint_unchecked(
        &mut self,
        id: ConstraintId,
        kind: SketchConstraintKind,
        provenance: ConstraintProvenance,
    ) {
        self.constraints
            .insert(id, ConstraintEntry { kind, provenance });
    }

    /// The removal counterpart to [`Sketch::set_constraint_unchecked`]:
    /// no "unknown constraint" error, since a replayed removal is exactly
    /// undoing/redoing a state this type already held.
    pub fn remove_constraint_unchecked(&mut self, id: ConstraintId) {
        self.constraints.remove(&id);
    }

    pub fn constraint(
        &self,
        id: ConstraintId,
    ) -> Option<(&SketchConstraintKind, &ConstraintProvenance)> {
        self.constraints
            .get(&id)
            .map(|entry| (&entry.kind, &entry.provenance))
    }

    /// Every point referenced by at least one stored constraint, mapped to
    /// a fresh dense `VariableId` pair, plus the `Variable`s (with initial
    /// guesses read from current primitive geometry) a `ConstraintSolver`
    /// needs. Iterates a `BTreeSet<PointRef>` (not a `HashSet`) so variable
    /// allocation order -- and therefore every `VariableId` assigned -- is
    /// deterministic given the same constraint set, matching this
    /// workspace's canonical-serialization philosophy (Phase 01).
    pub(crate) fn variable_map(
        &self,
    ) -> DomainResult<(BTreeMap<PointRef, PointVariables>, Vec<Variable>)> {
        let mut refs: BTreeSet<PointRef> = BTreeSet::new();
        for entry in self.constraints.values() {
            refs.extend(entry.kind.point_refs());
        }
        let mut map = BTreeMap::new();
        let mut variables = Vec::new();
        let mut next = 0u64;
        for point_ref in refs {
            let point = point_ref.resolve(&self.primitives)?;
            let x = VariableId(next);
            let y = VariableId(next + 1);
            next += 2;
            map.insert(point_ref, PointVariables::new(x, y));
            variables.push(Variable::new(x, point.x));
            variables.push(Variable::new(y, point.y));
        }
        Ok((map, variables))
    }

    /// Task 084-090's actual "engine integration": solve every stored
    /// constraint against a real backend and, unless the solve failed
    /// outright, write the result back into the stored primitives.
    ///
    /// Returns the raw `SolveResult` (`Solved`/`Unsatisfied`/`Failed`, see
    /// `craftloop-constraint`) rather than a `DomainResult`: an
    /// unsatisfiable or non-converging constraint set is a legitimate,
    /// already-typed domain outcome (Engine Contract 12: "Unknown is
    /// distinct from invalid"), not a structural error -- every
    /// *structural* problem (unknown primitive, wrong primitive kind) was
    /// already rejected at `add_constraint` time and cannot resurface
    /// here.
    pub fn solve(&mut self, solver: &mut dyn ConstraintSolver) -> SolveResult {
        let (var_map, variables) = match self.variable_map() {
            Ok(built) => built,
            Err(_) => {
                // A primitive referenced by a stored, already-validated
                // constraint has vanished since `add_constraint` (e.g. a
                // caller removed it directly through `insert_primitive`'s
                // absence rather than through this crate). Report as a
                // failed solve rather than panicking -- there is no
                // numeric result to give back.
                return SolveResult {
                    status: SolveStatus::Failed,
                    values: BTreeMap::new(),
                    unsatisfied: Vec::new(),
                };
            }
        };
        let requests: Vec<ConstraintRequest> = self
            .constraints
            .iter()
            .flat_map(|(id, entry)| {
                entry
                    .kind
                    .to_geometric_constraint(&|point_ref| var_map[&point_ref])
                    .into_iter()
                    .map(|constraint| ConstraintRequest {
                        id: *id,
                        constraint,
                    })
            })
            .collect();
        let result = solver.solve(&variables, &requests);
        if result.status != SolveStatus::Failed {
            self.apply_solution(&var_map, &result.values);
        }
        result
    }

    fn apply_solution(
        &mut self,
        var_map: &BTreeMap<PointRef, PointVariables>,
        values: &BTreeMap<VariableId, f64>,
    ) {
        // Circle center/point-on-circle updates are collected first and
        // applied together, since a circle's radius is only meaningful
        // once both are known (see `point_ref.rs`'s doc comment on why
        // `CirclePointOnCircle` is synthetic).
        let mut circle_updates: BTreeMap<PrimitiveId, (Option<Point2>, Option<Point2>)> =
            BTreeMap::new();
        for (point_ref, vars) in var_map {
            let (Some(&x), Some(&y)) = (values.get(&vars.x), values.get(&vars.y)) else {
                continue;
            };
            let solved = Point2::new(x, y);
            match point_ref {
                PointRef::LineStart(id) => {
                    if let Some(Beautified {
                        primitive: BeautifiedPrimitive::Line(segment),
                        ..
                    }) = self.primitives.get_mut(id)
                    {
                        segment.a = solved;
                    }
                }
                PointRef::LineEnd(id) => {
                    if let Some(Beautified {
                        primitive: BeautifiedPrimitive::Line(segment),
                        ..
                    }) = self.primitives.get_mut(id)
                    {
                        segment.b = solved;
                    }
                }
                PointRef::RectangleCorner(id, index) => {
                    if let Some(Beautified {
                        primitive: BeautifiedPrimitive::Rectangle(rectangle),
                        ..
                    }) = self.primitives.get_mut(id)
                    {
                        if let Some(corner) = rectangle.corners.get_mut(*index as usize) {
                            *corner = solved;
                        }
                    }
                }
                PointRef::CircleCenter(id) => {
                    circle_updates.entry(*id).or_default().0 = Some(solved);
                }
                PointRef::CirclePointOnCircle(id) => {
                    circle_updates.entry(*id).or_default().1 = Some(solved);
                }
            }
        }
        for (id, (center, point_on_circle)) in circle_updates {
            if let Some(Beautified {
                primitive: BeautifiedPrimitive::Circle(circle),
                ..
            }) = self.primitives.get_mut(&id)
            {
                if let Some(new_center) = center {
                    circle.center = new_center;
                }
                if let Some(boundary_point) = point_on_circle {
                    let new_radius = circle.center.distance_to(boundary_point);
                    // Defensive, not expected in practice: never let a
                    // degenerate solve corrupt a previously-valid circle
                    // with a non-positive radius (`Circle2::new`'s own
                    // invariant, Phase 02).
                    if new_radius > 0.0 && new_radius.is_finite() {
                        circle.radius = new_radius;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ezpz_adapter::EzpzSolver;
    use craftloop_geometry::{Circle2, Segment2};
    use craftloop_ids::CraftLoopId;

    fn line(a: Point2, b: Point2) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
            displacement: 0.0,
        }
    }

    fn circle(center: Point2, radius: f64) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Circle(Circle2::new(center, radius).unwrap()),
            displacement: 0.0,
        }
    }

    fn line_of(sketch: &Sketch, id: PrimitiveId) -> Segment2 {
        match &sketch.primitive(id).unwrap().primitive {
            BeautifiedPrimitive::Line(segment) => *segment,
            other => panic!("expected a Line, got {other:?}"),
        }
    }

    fn circle_of(sketch: &Sketch, id: PrimitiveId) -> Circle2 {
        match &sketch.primitive(id).unwrap().primitive {
            BeautifiedPrimitive::Circle(circle) => *circle,
            other => panic!("expected a Circle, got {other:?}"),
        }
    }

    // --- Task 084: coincident -------------------------------------------

    #[test]
    fn coincident_pulls_two_line_endpoints_to_the_same_point() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(1.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(1.05, 0.02), Point2::new(2.0, 1.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Coincident(PointRef::LineEnd(a), PointRef::LineStart(b)),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let end_a = line_of(&sketch, a).b;
        let start_b = line_of(&sketch, b).a;
        assert!(end_a.distance_to(start_b) < 1e-6);
    }

    // --- Task 085: horizontal / vertical ---------------------------------

    #[test]
    fn horizontal_levels_a_tilted_line() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.3)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let segment = line_of(&sketch, id);
        assert!((segment.a.y - segment.b.y).abs() < 1e-6);
    }

    #[test]
    fn vertical_straightens_a_tilted_line() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(0.3, 4.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Vertical(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let segment = line_of(&sketch, id);
        assert!((segment.a.x - segment.b.x).abs() < 1e-6);
    }

    // --- Task 086: parallel / perpendicular ------------------------------

    #[test]
    fn parallel_aligns_a_second_line_to_the_first() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(3.8, 2.3)));
        // Pin line b's start so the system has a unique solution.
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(
            !matches!(result.status, SolveStatus::Failed),
            "solve failed: {result:?}"
        );
        let line_a = line_of(&sketch, a);
        let line_b = line_of(&sketch, b);
        let dir_a = (line_a.b.x - line_a.a.x, line_a.b.y - line_a.a.y);
        let dir_b = (line_b.b.x - line_b.a.x, line_b.b.y - line_b.a.y);
        let cross = dir_a.0 * dir_b.1 - dir_a.1 * dir_b.0;
        assert!(cross.abs() < 1e-4, "lines are not parallel: cross={cross}");
    }

    #[test]
    fn perpendicular_rotates_a_second_line_to_a_right_angle() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(1.0, 0.0), Point2::new(1.3, 3.8)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Perpendicular(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(
            !matches!(result.status, SolveStatus::Failed),
            "solve failed: {result:?}"
        );
        let line_a = line_of(&sketch, a);
        let line_b = line_of(&sketch, b);
        let dir_a = (line_a.b.x - line_a.a.x, line_a.b.y - line_a.a.y);
        let dir_b = (line_b.b.x - line_b.a.x, line_b.b.y - line_b.a.y);
        let dot = dir_a.0 * dir_b.0 + dir_a.1 * dir_b.1;
        assert!(dot.abs() < 1e-4, "lines are not perpendicular: dot={dot}");
    }

    // --- Task 087: equal length / equal radius ---------------------------

    #[test]
    fn equal_length_matches_a_second_lines_length_to_the_first() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(5.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(2.7, 2.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::EqualLength(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(
            !matches!(result.status, SolveStatus::Failed),
            "solve failed: {result:?}"
        );
        let len_a = line_of(&sketch, a).length();
        let len_b = line_of(&sketch, b).length();
        assert!(
            (len_a - len_b).abs() < 1e-4,
            "lengths differ: {len_a} vs {len_b}"
        );
    }

    #[test]
    fn equal_radius_matches_a_second_circles_radius_without_moving_its_center() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, circle(Point2::new(0.0, 0.0), 5.0));
        sketch.insert_primitive(b, circle(Point2::new(20.0, 0.0), 2.0));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::EqualRadius(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(
            !matches!(result.status, SolveStatus::Failed),
            "solve failed: {result:?}"
        );
        let circle_a = circle_of(&sketch, a);
        let circle_b = circle_of(&sketch, b);
        assert!((circle_a.radius - circle_b.radius).abs() < 1e-3);
    }

    // --- Task 088: concentric --------------------------------------------

    #[test]
    fn concentric_moves_the_second_circles_center_onto_the_first_but_leaves_radii_independent() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, circle(Point2::new(3.0, 4.0), 5.0));
        sketch.insert_primitive(b, circle(Point2::new(0.0, 0.0), 2.0));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Concentric(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let circle_a = circle_of(&sketch, a);
        let circle_b = circle_of(&sketch, b);
        assert!(circle_a.center.distance_to(circle_b.center) < 1e-6);
        assert!(
            (circle_b.radius - 2.0).abs() < 1e-6,
            "radius should stay independent"
        );
    }

    // --- Task 089: tangency -----------------------------------------------

    #[test]
    fn line_tangent_to_circle_moves_the_line_to_touch_the_circle() {
        let mut sketch = Sketch::new();
        let line_id = PrimitiveId::new();
        let circle_id = PrimitiveId::new();
        sketch.insert_primitive(line_id, line(Point2::new(2.5, -5.0), Point2::new(2.5, 5.0)));
        sketch.insert_primitive(circle_id, circle(Point2::new(0.0, 0.0), 3.0));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::LineTangentToCircle(line_id, circle_id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        // Keep the line vertical and anchor the whole circle so the only
        // remaining freedom is the line's shared X -- otherwise "tangent"
        // has infinitely many solutions (the line could rotate to any
        // tangent angle, or the circle could shrink/move instead) and
        // there would be nothing specific left to assert.
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Vertical(line_id),
                ConstraintProvenance::Derived,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(
                    PointRef::CircleCenter(circle_id),
                    Point2::new(0.0, 0.0),
                ),
                ConstraintProvenance::Derived,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(
                    PointRef::CirclePointOnCircle(circle_id),
                    Point2::new(3.0, 0.0),
                ),
                ConstraintProvenance::Derived,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let segment = line_of(&sketch, line_id);
        assert!((segment.a.x.abs() - 3.0).abs() < 1e-4);
        assert!((segment.b.x.abs() - 3.0).abs() < 1e-4);
    }

    #[test]
    fn circle_tangent_to_circle_moves_the_second_circle_to_touch_the_first() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, circle(Point2::new(0.0, 0.0), 2.0));
        sketch.insert_primitive(b, circle(Point2::new(2.7, 0.0), 1.0));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::CircleTangentToCircle(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        // Anchor circle A fully and pin circle B's radius/Y so only B's
        // center X remains free (mirrors the Phase 11 spike extension's
        // `circle_tangent_to_circle` scenario).
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(PointRef::CircleCenter(a), Point2::new(0.0, 0.0)),
                ConstraintProvenance::Derived,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(
                    PointRef::CirclePointOnCircle(a),
                    Point2::new(2.0, 0.0),
                ),
                ConstraintProvenance::Derived,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(
                    PointRef::CirclePointOnCircle(b),
                    Point2::new(3.7, 0.0),
                ),
                ConstraintProvenance::Derived,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let circle_a = circle_of(&sketch, a);
        let circle_b = circle_of(&sketch, b);
        let center_distance = circle_a.center.distance_to(circle_b.center);
        assert!((center_distance - (circle_a.radius + circle_b.radius)).abs() < 1e-4);
    }

    // --- Task 090: symmetry -------------------------------------------

    #[test]
    fn symmetric_reflects_one_lines_endpoint_across_an_axis() {
        let mut sketch = Sketch::new();
        let axis = PrimitiveId::new();
        let a_line = PrimitiveId::new();
        let b_line = PrimitiveId::new();
        sketch.insert_primitive(axis, line(Point2::new(0.0, -5.0), Point2::new(0.0, 5.0)));
        sketch.insert_primitive(a_line, line(Point2::new(3.0, 2.0), Point2::new(3.0, 2.0)));
        sketch.insert_primitive(b_line, line(Point2::new(-2.8, 2.1), Point2::new(-2.8, 2.1)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Symmetric {
                    axis,
                    a: PointRef::LineStart(a_line),
                    b: PointRef::LineStart(b_line),
                },
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        // Anchor the axis and point A so the system has a unique solution
        // instead of sliding freely along the axis's own DOF (mirrors the
        // Phase 11 spike extension's `symmetric_points` scenario, which
        // fixed the axis and point A the same way).
        for (point_ref, value) in [
            (PointRef::LineStart(axis), Point2::new(0.0, -5.0)),
            (PointRef::LineEnd(axis), Point2::new(0.0, 5.0)),
            (PointRef::LineStart(a_line), Point2::new(3.0, 2.0)),
        ] {
            sketch
                .add_constraint(
                    ConstraintId::new(),
                    SketchConstraintKind::Fixed(point_ref, value),
                    ConstraintProvenance::Derived,
                )
                .unwrap();
        }

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let b_start = line_of(&sketch, b_line).a;
        assert!(
            (b_start.x - -3.0).abs() < 1e-4,
            "expected x≈-3.0, got {}",
            b_start.x
        );
        assert!(
            (b_start.y - 2.0).abs() < 1e-4,
            "expected y≈2.0, got {}",
            b_start.y
        );
    }

    // --- Task 096: redundant constraint detection -------------------------

    #[test]
    fn adding_the_same_relationship_twice_is_recognized_as_redundant_not_an_error() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(4.0, 2.3)));

        let first = ConstraintId::new();
        let outcome = sketch
            .add_constraint(
                first,
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        assert_eq!(outcome, ConstraintOutcome::Added);

        // Article 312: re-adding the same relationship (here with
        // arguments swapped, still the same relationship) is not an
        // error.
        let second = ConstraintId::new();
        let outcome = sketch
            .add_constraint(
                second,
                SketchConstraintKind::Parallel(b, a),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        assert_eq!(outcome, ConstraintOutcome::Redundant { existing: first });
        assert!(outcome.explain().contains("already"));

        // The graph must not carry a duplicate: only the first constraint
        // is actually stored.
        assert!(sketch.constraint(first).is_some());
        assert!(sketch.constraint(second).is_none());
    }

    #[test]
    fn a_different_relationship_between_the_same_primitives_is_not_redundant() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(0.3, 6.0)));

        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        let outcome = sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Perpendicular(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        assert_eq!(
            outcome,
            ConstraintOutcome::Added,
            "a different relationship must not be treated as redundant"
        );
    }

    // --- Task 091: provenance ----------------------------------------

    #[test]
    fn provenance_is_stored_and_retrievable_per_constraint() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        let user_id = ConstraintId::new();
        let derived_id = ConstraintId::new();
        sketch
            .add_constraint(
                user_id,
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        sketch
            .add_constraint(
                derived_id,
                SketchConstraintKind::Coincident(PointRef::LineStart(id), PointRef::LineStart(id)),
                ConstraintProvenance::Derived,
            )
            .unwrap();

        assert_eq!(
            sketch.constraint(user_id).unwrap().1,
            &ConstraintProvenance::UserCreated
        );
        assert_eq!(
            sketch.constraint(derived_id).unwrap().1,
            &ConstraintProvenance::Derived
        );
    }

    // --- Task 092: RED-GREEN invalid/conflict cases -----------------------

    #[test]
    fn adding_a_constraint_that_references_an_unknown_primitive_is_rejected() {
        let mut sketch = Sketch::new();
        let err = sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Horizontal(PrimitiveId::new()),
                ConstraintProvenance::UserCreated,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::UnknownPrimitive,
                ..
            }
        ));
        // Rejected constraints must not be stored (no partial state).
        assert!(sketch.constraint(ConstraintId::new()).is_none());
    }

    #[test]
    fn adding_a_constraint_under_a_duplicate_id_is_rejected() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        let constraint_id = ConstraintId::new();
        sketch
            .add_constraint(
                constraint_id,
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        let err = sketch
            .add_constraint(
                constraint_id,
                SketchConstraintKind::Vertical(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::DuplicateConstraintId,
                ..
            }
        ));
        // The original constraint must be untouched.
        assert!(matches!(
            sketch.constraint(constraint_id).unwrap().0,
            SketchConstraintKind::Horizontal(_)
        ));
    }

    #[test]
    fn removing_an_unknown_constraint_is_rejected() {
        let mut sketch = Sketch::new();
        let err = sketch.remove_constraint(ConstraintId::new()).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::UnknownConstraint,
                ..
            }
        ));
    }

    #[test]
    fn horizontal_and_vertical_together_are_not_a_contradiction_they_collapse_the_line_to_a_point()
    {
        // RED-GREEN discovery while writing this suite (Task 092): the
        // first version of this test assumed Horizontal + Vertical on the
        // same line would be unsatisfiable, and asserted `!= Solved`. It
        // failed -- correctly. Horizontal (a.y == b.y) and Vertical
        // (a.x == b.x) are simultaneously satisfiable by collapsing the
        // line to a single point (a == b), which is not a contradiction,
        // just a degenerate-but-consistent system (the same category
        // Phase 11's spike named via Article 307). The solver finding
        // that degenerate solution is *correct* behavior, not a bug, so
        // this test now asserts the true, verified property instead of
        // the mistaken one.
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(4.0, 3.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Vertical(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved(), "expected solved, got {result:?}");
        let segment = line_of(&sketch, id);
        assert!(
            segment.a.distance_to(segment.b) < 1e-4,
            "expected the line to collapse to a point"
        );
    }

    #[test]
    fn a_genuinely_contradictory_sketch_reports_unsatisfied_not_a_crash() {
        // Two different Fixed constraints on the same point can never
        // both hold -- a real, absolute contradiction (unlike
        // Horizontal+Vertical above).
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(2.0, 2.0), Point2::new(4.0, 3.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(PointRef::LineStart(id), Point2::new(0.0, 0.0)),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Fixed(PointRef::LineStart(id), Point2::new(5.0, 5.0)),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        // Never a panic, never silently "Solved" for a real contradiction.
        assert_ne!(result.status, SolveStatus::Solved);
    }

    #[test]
    fn removing_a_constraint_and_re_solving_releases_the_relation() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.7)));
        let constraint_id = ConstraintId::new();
        sketch
            .add_constraint(
                constraint_id,
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        sketch.solve(&mut solver);
        assert!((line_of(&sketch, id).a.y - line_of(&sketch, id).b.y).abs() < 1e-6);

        sketch.remove_constraint(constraint_id).unwrap();
        // With no constraints left, a solve is a no-op (nothing to
        // satisfy) and must not panic on an empty constraint set.
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved());
    }

    // --- Execution 02, Phase 04: persistence -----------------------------

    #[test]
    fn a_sketch_with_primitives_and_constraints_round_trips_through_json() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0)));
        sketch.insert_primitive(b, circle(Point2::new(1.0, 1.0), 2.0));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Horizontal(a),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let json = serde_json::to_string(&sketch).unwrap();
        let reloaded: Sketch = serde_json::from_str(&json).unwrap();
        assert_eq!(reloaded, sketch);
        assert_eq!(reloaded.primitive(a), sketch.primitive(a));
        assert_eq!(reloaded.constraints.len(), 1);
    }

    #[test]
    fn primitive_ids_lists_every_stored_primitive() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(1.0, 0.0)));
        sketch.insert_primitive(b, circle(Point2::new(0.0, 0.0), 1.0));
        let mut ids: Vec<PrimitiveId> = sketch.primitive_ids().collect();
        ids.sort();
        let mut expected = vec![a, b];
        expected.sort();
        assert_eq!(ids, expected);
    }

    #[test]
    fn an_empty_sketch_round_trips_too() {
        let sketch = Sketch::new();
        let json = serde_json::to_string(&sketch).unwrap();
        let reloaded: Sketch = serde_json::from_str(&json).unwrap();
        assert_eq!(reloaded, sketch);
    }
}
