//! Design intent: querying and curating the relationships a user expects
//! to remain true.
//!
//! Execution 01, Phase 15, Tasks 107-112. Authority: MCP Article 26
//! "Design Intent" ("the set of relationships the user expects to remain
//! true when the drawing changes... the system may infer likely intent,
//! but inference must remain distinguishable from explicit user
//! decisions").
//!
//! Deliberately not a second, parallel model: a `Sketch`'s stored
//! constraints (`SketchConstraintKind` + `ConstraintProvenance`, Phase 12)
//! *already are* the design-intent graph -- a confirmed relationship the
//! solver keeps re-enforcing on every `solve()` call, tagged with who
//! asserted it. Duplicating that as a second "intent relation" type would
//! be exactly the duplicated-truth shortcut this workspace's conventions
//! forbid. This module is the query/curation layer Tasks 107-112 actually
//! ask for on top of that existing storage: "which relations control this
//! entity" (Task 111), accepting/rejecting suggestions without silently
//! promoting mere numeric coincidence into enforced truth (Tasks 108-110),
//! and the durability proof Task 107/112 ask for (nothing in `solve()`
//! ever touches `self.constraints`, so a stored relationship is never
//! silently dropped by an edit).

use std::collections::BTreeSet;

use craftloop_errors::DomainResult;
use craftloop_geometry::Tolerances;
use craftloop_ids::{ConstraintId, PrimitiveId};
use craftloop_recognition::BeautifiedPrimitive;

use crate::constraint_kind::SketchConstraintKind;
use crate::provenance::ConstraintProvenance;
use crate::sketch::{ConstraintEntry, ConstraintOutcome, Sketch};

impl Sketch {
    /// Task 111: every stored constraint that references `primitive` --
    /// "which relations control this entity."
    pub fn constraints_touching(&self, primitive: PrimitiveId) -> Vec<ConstraintId> {
        self.constraints
            .iter()
            .filter(|(_, entry): &(&ConstraintId, &ConstraintEntry)| {
                entry
                    .kind
                    .point_refs()
                    .iter()
                    .any(|point_ref| point_ref.primitive_id() == primitive)
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Task 111: every primitive transitively reachable from `primitive`
    /// through shared constraints -- "what would changing this entity
    /// affect." Does not include `primitive` itself.
    pub fn affected_primitives(&self, primitive: PrimitiveId) -> BTreeSet<PrimitiveId> {
        let mut visited: BTreeSet<PrimitiveId> = BTreeSet::new();
        let mut frontier = vec![primitive];
        while let Some(current) = frontier.pop() {
            if !visited.insert(current) {
                continue;
            }
            for entry in self.constraints.values() {
                let touched: Vec<PrimitiveId> = entry
                    .kind
                    .point_refs()
                    .iter()
                    .map(|p| p.primitive_id())
                    .collect();
                if touched.contains(&current) {
                    for other in touched {
                        if !visited.contains(&other) {
                            frontier.push(other);
                        }
                    }
                }
            }
        }
        visited.remove(&primitive);
        visited
    }

    /// Task 109: accept a machine-suggested relationship as a real,
    /// enforced constraint -- identical in solving to one the user drew
    /// directly (Phase 12's solver never branches on provenance), only
    /// distinguishable afterward via its stored
    /// [`ConstraintProvenance::AcceptedSuggestion`]. Clears any prior
    /// rejection of the same relationship first, since accepting it now
    /// supersedes having declined it before.
    pub fn accept_suggestion(
        &mut self,
        id: ConstraintId,
        kind: SketchConstraintKind,
    ) -> DomainResult<ConstraintOutcome> {
        self.rejected_suggestions
            .retain(|rejected| !rejected.is_equivalent(&kind));
        self.add_constraint(id, kind, ConstraintProvenance::AcceptedSuggestion)
    }

    /// Task 110: record that a suggested relationship was declined,
    /// without ever storing it as an enforced constraint -- lightweight
    /// enough (`is_equivalent`, Task 096) that the same proposal can later
    /// be checked and skipped rather than nagging the user again.
    pub fn reject_suggestion(&mut self, kind: SketchConstraintKind) {
        if !self.is_suggestion_rejected(&kind) {
            self.rejected_suggestions.push(kind);
        }
    }

    /// Has this exact relationship already been declined?
    pub fn is_suggestion_rejected(&self, kind: &SketchConstraintKind) -> bool {
        self.rejected_suggestions
            .iter()
            .any(|rejected| rejected.is_equivalent(kind))
    }

    /// Task 108: lines whose current lengths merely happen to be close,
    /// with **no** stored `EqualLength` constraint already linking them.
    /// Purely observational/advisory -- feeding a future suggestion
    /// engine is the only legitimate use; nothing in this crate ever
    /// reads this list to auto-create a constraint (see this module's
    /// tests for the proof that near-equal geometry never silently
    /// becomes `Equal`).
    pub fn observed_length_coincidences(&self) -> Vec<(PrimitiveId, PrimitiveId)> {
        let tolerance = Tolerances::recognition().length_equality;
        let lines: Vec<(PrimitiveId, f64)> = self
            .primitives
            .iter()
            .filter_map(|(id, beautified)| match &beautified.primitive {
                BeautifiedPrimitive::Line(segment) => Some((*id, segment.length())),
                _ => None,
            })
            .collect();

        let mut pairs = Vec::new();
        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                let (id_a, len_a) = lines[i];
                let (id_b, len_b) = lines[j];
                if (len_a - len_b).abs() < tolerance
                    && !self.has_equal_length_constraint(id_a, id_b)
                {
                    pairs.push((id_a, id_b));
                }
            }
        }
        pairs
    }

    fn has_equal_length_constraint(&self, a: PrimitiveId, b: PrimitiveId) -> bool {
        self.constraints.values().any(|entry| {
            matches!(
                entry.kind,
                SketchConstraintKind::EqualLength(x, y)
                    if (x == a && y == b) || (x == b && y == a)
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::{Point2, Segment2};
    use craftloop_ids::CraftLoopId;
    use craftloop_recognition::Beautified;

    fn line(a: Point2, b: Point2) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
            displacement: 0.0,
        }
    }

    // --- Task 111: query intent dependencies -------------------------------

    #[test]
    fn constraints_touching_finds_every_relation_that_references_a_primitive() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let c = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(1.0, 2.0)));
        sketch.insert_primitive(c, line(Point2::new(0.0, 4.0), Point2::new(1.0, 4.0)));

        let parallel_ab = ConstraintId::new();
        sketch
            .add_constraint(
                parallel_ab,
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        // A relation not touching `a` at all.
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(b, c),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let touching_a = sketch.constraints_touching(a);
        assert_eq!(touching_a, vec![parallel_ab]);
    }

    #[test]
    fn affected_primitives_follows_transitive_chains_of_shared_constraints() {
        // a--b--c, a chain: changing `a` should transitively reach `c`
        // through `b`.
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let c = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(1.0, 2.0)));
        sketch.insert_primitive(c, line(Point2::new(0.0, 4.0), Point2::new(1.0, 4.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(b, c),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let affected = sketch.affected_primitives(a);
        assert_eq!(affected, BTreeSet::from([b, c]));
    }

    #[test]
    fn an_unconstrained_primitive_affects_nothing() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        assert!(sketch.affected_primitives(a).is_empty());
    }

    // --- Task 108: observed coincidence vs. intended relation --------------

    #[test]
    fn near_equal_length_lines_with_no_constraint_are_reported_as_observed_only() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(5.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(5.001, 2.0)));

        // `observed_length_coincidences` iterates primitives in
        // `PrimitiveId`'s own (UUID) order, which is random per run --
        // the pair can legitimately come back as `(a, b)` or `(b, a)`.
        // Assert the unordered pair, not a fixed position.
        let observed = sketch.observed_length_coincidences();
        assert_eq!(observed.len(), 1);
        let (first, second) = observed[0];
        assert_eq!(
            std::collections::BTreeSet::from([first, second]),
            std::collections::BTreeSet::from([a, b])
        );

        // Task 108's core assertion: merely observing the coincidence must
        // never itself create a constraint. The lines stay independent.
        assert!(sketch.constraints_touching(a).is_empty());
        assert!(sketch.constraints_touching(b).is_empty());
    }

    #[test]
    fn a_pair_already_linked_by_equal_length_is_not_reported_as_merely_observed() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(5.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(5.0, 2.0)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::EqualLength(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        assert!(sketch.observed_length_coincidences().is_empty());
    }

    #[test]
    fn lines_with_clearly_different_lengths_are_never_reported() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(5.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(9.0, 2.0)));
        assert!(sketch.observed_length_coincidences().is_empty());
    }

    // --- Task 109: accepted suggestions become deterministic intent --------

    #[test]
    fn an_accepted_suggestion_is_enforced_by_solve_exactly_like_a_user_created_constraint() {
        use crate::ezpz_adapter::EzpzSolver;

        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.3)));
        sketch
            .accept_suggestion(ConstraintId::new(), SketchConstraintKind::Horizontal(id))
            .unwrap();

        let mut solver = EzpzSolver::new();
        let result = sketch.solve(&mut solver);
        assert!(result.is_solved());
        let segment = match &sketch.primitive(id).unwrap().primitive {
            BeautifiedPrimitive::Line(s) => *s,
            other => panic!("expected a line, got {other:?}"),
        };
        assert!((segment.a.y - segment.b.y).abs() < 1e-6);
    }

    #[test]
    fn accepted_suggestion_provenance_is_recorded_as_accepted_not_user_created() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        let constraint_id = ConstraintId::new();
        sketch
            .accept_suggestion(constraint_id, SketchConstraintKind::Horizontal(id))
            .unwrap();
        assert_eq!(
            sketch.constraint(constraint_id).unwrap().1,
            &ConstraintProvenance::AcceptedSuggestion
        );
    }

    #[test]
    fn accepting_a_suggestion_clears_any_prior_rejection_of_the_same_relationship() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        sketch.reject_suggestion(SketchConstraintKind::Horizontal(id));
        assert!(sketch.is_suggestion_rejected(&SketchConstraintKind::Horizontal(id)));

        sketch
            .accept_suggestion(ConstraintId::new(), SketchConstraintKind::Horizontal(id))
            .unwrap();
        assert!(!sketch.is_suggestion_rejected(&SketchConstraintKind::Horizontal(id)));
    }

    // --- Task 110: preserve rejected suggestions ----------------------------

    #[test]
    fn a_rejected_suggestion_is_remembered_without_becoming_a_constraint() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.3)));
        let kind = SketchConstraintKind::Horizontal(id);

        assert!(!sketch.is_suggestion_rejected(&kind));
        sketch.reject_suggestion(kind);
        assert!(sketch.is_suggestion_rejected(&SketchConstraintKind::Horizontal(id)));
        // Never stored as an enforced relation.
        assert!(sketch.constraints_touching(id).is_empty());
    }

    #[test]
    fn rejecting_the_same_suggestion_twice_does_not_duplicate_it() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.3)));
        sketch.reject_suggestion(SketchConstraintKind::Horizontal(id));
        sketch.reject_suggestion(SketchConstraintKind::Horizontal(id));
        assert_eq!(sketch.rejected_suggestions.len(), 1);
    }

    // --- Task 107/112: durability across edits ------------------------------

    #[test]
    fn a_confirmed_relation_survives_repeated_solves_and_is_never_silently_dropped() {
        use crate::ezpz_adapter::EzpzSolver;

        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.3)));
        let constraint_id = ConstraintId::new();
        sketch
            .add_constraint(
                constraint_id,
                SketchConstraintKind::Horizontal(id),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        for _ in 0..5 {
            sketch.solve(&mut solver);
            // The stored relation itself -- not just its numeric effect --
            // must still be there after every edit/re-solve cycle.
            assert!(sketch.constraint(constraint_id).is_some());
            assert_eq!(sketch.constraints_touching(id), vec![constraint_id]);
        }
    }

    #[test]
    fn inserting_an_unrelated_primitive_does_not_disturb_an_existing_relation() {
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

        // An unrelated edit: adding a new, disconnected primitive.
        sketch.insert_primitive(
            PrimitiveId::new(),
            line(Point2::new(9.0, 9.0), Point2::new(9.0, 20.0)),
        );

        assert!(sketch.constraint(constraint_id).is_some());
        assert_eq!(
            sketch.constraint(constraint_id).unwrap().1,
            &ConstraintProvenance::UserCreated
        );
    }

    #[test]
    fn removing_one_constraint_never_removes_a_different_ones_intent() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(1.0, 2.3)));
        let keep = ConstraintId::new();
        let remove = ConstraintId::new();
        sketch
            .add_constraint(
                keep,
                SketchConstraintKind::Horizontal(a),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();
        sketch
            .add_constraint(
                remove,
                SketchConstraintKind::Horizontal(b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        sketch.remove_constraint(remove).unwrap();

        assert!(sketch.constraint(keep).is_some());
        assert!(sketch.constraint(remove).is_none());
    }
}
