//! Degree-of-freedom state per primitive.
//!
//! Execution 01, Phase 13, Tasks 093-095, 099. Authority: MCP Article 28
//! "Constraint State Engine" (states: Unknown, Free, Partially
//! Constrained, Bounded, Derived, Shared, Confirmed, Conflicting) and
//! Article 29 "Under-Constrained Does Not Mean Wrong".
//!
//! `Bounded`/`Derived`/`Shared` are dimension-*role* states (MCP Article
//! 24), already owned by `craftloop-dimension::DimensionRole` (Phase 10)
//! -- this module does not duplicate them. `DegreesOfFreedomState` covers
//! the remaining, purely geometric states: whether a primitive's points
//! still have freedom to move, are fully pinned, or are part of a broken
//! (unsatisfied) constraint.

use std::collections::{BTreeMap, BTreeSet};

use craftloop_constraint::{ConstraintRequest, ConstraintSolver, PointVariables, SolveStatus};
use craftloop_ids::PrimitiveId;

use crate::sketch::Sketch;

/// Task 093: the DOF state of one primitive, translated from raw solver
/// freedom/satisfaction data into Article 28's vocabulary -- never
/// exposed as solver jargon ("N degrees of freedom remain") by default;
/// see [`Self::explain`] for the human-facing text (Task 099).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegreesOfFreedomState {
    /// Not referenced by any stored constraint -- never entered the
    /// system, so nothing has been analyzed yet.
    Unknown,
    /// Every point of this primitive is locally free to move without
    /// breaking anything already confirmed (Task 094: a legitimate,
    /// healthy exploratory state, not an error).
    Free,
    /// Some but not all of this primitive's points are free.
    PartiallyConstrained,
    /// No remaining freedom: every point is pinned by the current
    /// constraint set (Task 095; Article 28's "Confirmed").
    FullyDetermined,
    /// At least one constraint touching this primitive is unsatisfied at
    /// the current solution -- distinct from `Unknown`/`Free` (Engine
    /// Contract 12: "Unknown is distinct from invalid").
    Conflicting,
}

impl DegreesOfFreedomState {
    /// Task 094: `Free`/`PartiallyConstrained`/`FullyDetermined` are all
    /// legitimate states for an in-progress sketch -- only `Conflicting`
    /// represents something actually wrong. Callers deciding whether to
    /// surface a warning treatment should consult this, not merely
    /// whether the state is `FullyDetermined`.
    pub fn is_healthy_for_exploration(&self) -> bool {
        !matches!(self, DegreesOfFreedomState::Conflicting)
    }

    /// Task 099: UI-ready explanation, deliberately free of solver jargon
    /// (Article 99's "prefer 'Depth is still unknown' over 'One degree of
    /// freedom remains'"). Presentation layers (a future phase) may
    /// further tailor this with the entity's own name/label; this text
    /// stands on its own without one.
    pub fn explain(&self) -> &'static str {
        match self {
            DegreesOfFreedomState::Unknown => "Not yet part of any relationship.",
            DegreesOfFreedomState::Free => {
                "Still free to move -- no relationship fixes its position yet."
            }
            DegreesOfFreedomState::PartiallyConstrained => {
                "Partly fixed by existing relationships, but still has room to move."
            }
            DegreesOfFreedomState::FullyDetermined => "Fully fixed by its current relationships.",
            DegreesOfFreedomState::Conflicting => "Its relationships cannot all be true at once.",
        }
    }
}

impl Sketch {
    /// Tasks 093-095: compute the DOF state of every stored primitive.
    /// Solves once (for satisfaction) and asks the backend once more for
    /// freedom analysis; both are legitimate, already-demonstrated solver
    /// capabilities (Phase 11's `SolveResult`, Phase 13's
    /// `underconstrained_variables`) rather than a hand-rolled DOF-count
    /// heuristic.
    pub fn degrees_of_freedom(
        &self,
        solver: &mut dyn ConstraintSolver,
    ) -> BTreeMap<PrimitiveId, DegreesOfFreedomState> {
        let mut states: BTreeMap<PrimitiveId, DegreesOfFreedomState> = self
            .primitives
            .keys()
            .map(|id| (*id, DegreesOfFreedomState::Unknown))
            .collect();

        let Ok((var_map, variables)) = self.variable_map() else {
            return states;
        };
        if variables.is_empty() {
            return states;
        }

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

        let mut primitive_points: BTreeMap<PrimitiveId, Vec<PointVariables>> = BTreeMap::new();
        for (point_ref, vars) in &var_map {
            primitive_points
                .entry(point_ref.primitive_id())
                .or_default()
                .push(*vars);
        }

        let result = solver.solve(&variables, &requests);
        if result.status == SolveStatus::Failed {
            // A totally failed solve gives no usable information -- treat
            // every referenced primitive as conflicting rather than
            // silently reporting a misleadingly specific DOF breakdown.
            for primitive_id in primitive_points.keys() {
                states.insert(*primitive_id, DegreesOfFreedomState::Conflicting);
            }
            return states;
        }

        let unsatisfied_constraint_ids: BTreeSet<_> =
            result.unsatisfied.iter().map(|d| d.constraint_id).collect();
        let mut conflicting: BTreeSet<PrimitiveId> = BTreeSet::new();
        for (id, entry) in &self.constraints {
            if unsatisfied_constraint_ids.contains(id) {
                for point_ref in entry.kind.point_refs() {
                    conflicting.insert(point_ref.primitive_id());
                }
            }
        }

        let free_vars: BTreeSet<_> = solver
            .underconstrained_variables(&variables, &requests)
            .into_iter()
            .collect();

        for (primitive_id, points) in primitive_points {
            let state = if conflicting.contains(&primitive_id) {
                DegreesOfFreedomState::Conflicting
            } else {
                let total = points.len() * 2;
                let free_count = points
                    .iter()
                    .flat_map(|p| [p.x, p.y])
                    .filter(|v| free_vars.contains(v))
                    .count();
                if free_count == 0 {
                    DegreesOfFreedomState::FullyDetermined
                } else if free_count == total {
                    DegreesOfFreedomState::Free
                } else {
                    DegreesOfFreedomState::PartiallyConstrained
                }
            };
            states.insert(primitive_id, state);
        }

        states
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraint_kind::SketchConstraintKind;
    use crate::ezpz_adapter::EzpzSolver;
    use crate::point_ref::PointRef;
    use crate::provenance::ConstraintProvenance;
    use craftloop_geometry::{Point2, Segment2};
    use craftloop_ids::{ConstraintId, CraftLoopId};
    use craftloop_recognition::{Beautified, BeautifiedPrimitive};

    fn line(a: Point2, b: Point2) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
            displacement: 0.0,
        }
    }

    #[test]
    fn a_primitive_with_no_constraints_is_unknown() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::ORIGIN, Point2::new(1.0, 0.0)));

        let mut solver = EzpzSolver::new();
        let states = sketch.degrees_of_freedom(&mut solver);
        assert_eq!(states[&id], DegreesOfFreedomState::Unknown);
        assert!(states[&id].is_healthy_for_exploration());
    }

    #[test]
    fn a_fully_anchored_primitive_is_fully_determined() {
        let mut sketch = Sketch::new();
        let id = PrimitiveId::new();
        sketch.insert_primitive(id, line(Point2::new(0.3, 0.1), Point2::new(4.2, -0.2)));
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
                SketchConstraintKind::Fixed(PointRef::LineEnd(id), Point2::new(4.0, 0.0)),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let states = sketch.degrees_of_freedom(&mut solver);
        assert_eq!(states[&id], DegreesOfFreedomState::FullyDetermined);
        assert!(states[&id].is_healthy_for_exploration());
    }

    #[test]
    fn a_primitive_touched_only_by_a_relative_constraint_is_free_or_partially_constrained() {
        let mut sketch = Sketch::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        sketch.insert_primitive(a, line(Point2::new(0.0, 0.0), Point2::new(4.0, 0.3)));
        sketch.insert_primitive(b, line(Point2::new(0.0, 2.0), Point2::new(3.8, 2.3)));
        sketch
            .add_constraint(
                ConstraintId::new(),
                SketchConstraintKind::Parallel(a, b),
                ConstraintProvenance::UserCreated,
            )
            .unwrap();

        let mut solver = EzpzSolver::new();
        let states = sketch.degrees_of_freedom(&mut solver);
        // Nothing anchors either line, so at least some freedom must
        // remain on each -- never `FullyDetermined`, never `Conflicting`.
        for id in [a, b] {
            assert!(
                matches!(
                    states[&id],
                    DegreesOfFreedomState::Free | DegreesOfFreedomState::PartiallyConstrained
                ),
                "expected Free or PartiallyConstrained for {id:?}, got {:?}",
                states[&id]
            );
            assert!(states[&id].is_healthy_for_exploration());
        }
    }

    #[test]
    fn a_primitive_in_a_contradictory_system_is_conflicting_and_unhealthy() {
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
        let states = sketch.degrees_of_freedom(&mut solver);
        assert_eq!(states[&id], DegreesOfFreedomState::Conflicting);
        assert!(!states[&id].is_healthy_for_exploration());
    }

    #[test]
    fn every_state_has_a_jargon_free_explanation() {
        for state in [
            DegreesOfFreedomState::Unknown,
            DegreesOfFreedomState::Free,
            DegreesOfFreedomState::PartiallyConstrained,
            DegreesOfFreedomState::FullyDetermined,
            DegreesOfFreedomState::Conflicting,
        ] {
            let text = state.explain();
            assert!(!text.is_empty());
            assert!(
                !text.to_lowercase().contains("degree of freedom")
                    && !text.to_lowercase().contains("solver"),
                "explanation for {state:?} leaked solver jargon: {text:?}"
            );
        }
    }
}
