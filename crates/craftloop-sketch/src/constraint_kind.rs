//! Domain-level constraint vocabulary, bound to real primitives.
//!
//! Execution 01, Phase 12, Tasks 084-090. Authority: Engine Contract 10.
//!
//! `craftloop-constraint`'s `GeometricConstraint` (Phase 11) speaks only in
//! terms of opaque solver variables. This module is one layer up: each
//! variant here names a real relationship between stored primitives
//! (`Horizontal(line)`, `Concentric(circle, circle)`, ...) and knows how to
//! lower itself into the variable-level vocabulary once a
//! [`PointRef`]-to-[`PointVariables`] mapping exists (built by
//! [`crate::sketch::Sketch`]).
//!
//! `Concentric` is deliberately not its own `GeometricConstraint` variant:
//! numerically it is exactly "these two centers coincide," so it lowers to
//! `GeometricConstraint::Coincident`. Keeping the distinction only at this
//! domain layer (rather than adding a duplicate-but-identical variant to
//! the backend-neutral interface) avoids Task 084's "do not put shared
//! engineering state" duplicated-truth shortcut while still letting a
//! caller (and this constraint's own provenance/undo entry) say
//! "Concentric," which is what the user actually asked for and what
//! Task 088 explicitly names.
//!
//! `EqualRadius` similarly lowers to `GeometricConstraint::EqualLength`
//! over each circle's center and synthetic `CirclePointOnCircle` (see
//! `point_ref.rs`), for the same reason `Radius` already represents a
//! radius as a center-to-boundary-point distance (Phase 11).

use craftloop_constraint::GeometricConstraint;
use craftloop_errors::{DomainError, DomainResult, SketchErrorKind};
use craftloop_geometry::Point2;
use craftloop_ids::PrimitiveId;
use craftloop_recognition::BeautifiedPrimitive;
use serde::{Deserialize, Serialize};

use crate::point_ref::{PointRef, PrimitiveMap};

/// One domain-level constraint, referencing primitives by
/// [`PrimitiveId`]/[`PointRef`] rather than raw solver variables.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SketchConstraintKind {
    /// Anchors a point to an exact location. Not one of Tasks 084-090's
    /// named relationships, but necessary infrastructure both of them
    /// depend on to be testable/usable at all: Task 084's "maintain the
    /// relation through edits" and Task 085's "stable drag behavior" both
    /// presuppose that *some* point is authoritative while a solve
    /// happens (the point currently being dragged, or a sketch's origin)
    /// -- without an anchor, every relationship here is only ever
    /// relative, and a system with zero anchors has no unique solution
    /// (Article 307's "redundant but consistent" is about redundant
    /// *relative* constraints, not the complete absence of any absolute
    /// one). Lowers directly to the `FixedValue` variant
    /// `craftloop-constraint` already defined in Phase 11.
    Fixed(PointRef, Point2),
    /// Task 084: two named points occupy the same location.
    Coincident(PointRef, PointRef),
    /// Task 085: a line's two endpoints share the same Y.
    Horizontal(PrimitiveId),
    /// Task 085: a line's two endpoints share the same X.
    Vertical(PrimitiveId),
    /// Task 086: two lines are parallel.
    Parallel(PrimitiveId, PrimitiveId),
    /// Task 086: two lines are perpendicular.
    Perpendicular(PrimitiveId, PrimitiveId),
    /// Task 087: two lines have equal length.
    EqualLength(PrimitiveId, PrimitiveId),
    /// Task 087: two circles have equal radius (their centers stay
    /// independent, unlike `Concentric`).
    EqualRadius(PrimitiveId, PrimitiveId),
    /// Task 088: two circles share a center; radii stay independent unless
    /// an `EqualRadius` constraint is also present.
    Concentric(PrimitiveId, PrimitiveId),
    /// Task 089: a line is tangent to a circle.
    LineTangentToCircle(PrimitiveId, PrimitiveId),
    /// Task 089: two circles are externally tangent.
    CircleTangentToCircle(PrimitiveId, PrimitiveId),
    /// Task 090: two named points mirror each other across the axis line.
    Symmetric {
        axis: PrimitiveId,
        a: PointRef,
        b: PointRef,
    },
}

fn expect_line(primitives: &PrimitiveMap, id: PrimitiveId) -> DomainResult<()> {
    match primitives.get(&id).map(|b| &b.primitive) {
        Some(BeautifiedPrimitive::Line(_)) => Ok(()),
        Some(_) => Err(DomainError::Sketch {
            kind: SketchErrorKind::WrongPrimitiveKind,
            detail: format!("primitive {id:?} must be a Line"),
        }),
        None => Err(DomainError::Sketch {
            kind: SketchErrorKind::UnknownPrimitive,
            detail: format!("no primitive with id {id:?}"),
        }),
    }
}

fn expect_circle(primitives: &PrimitiveMap, id: PrimitiveId) -> DomainResult<()> {
    match primitives.get(&id).map(|b| &b.primitive) {
        Some(BeautifiedPrimitive::Circle(_)) => Ok(()),
        Some(_) => Err(DomainError::Sketch {
            kind: SketchErrorKind::WrongPrimitiveKind,
            detail: format!("primitive {id:?} must be a Circle"),
        }),
        None => Err(DomainError::Sketch {
            kind: SketchErrorKind::UnknownPrimitive,
            detail: format!("no primitive with id {id:?}"),
        }),
    }
}

impl SketchConstraintKind {
    /// Every point this constraint reads/constrains, so a caller can
    /// allocate solver variables for exactly the points involved.
    pub fn point_refs(&self) -> Vec<PointRef> {
        match self {
            SketchConstraintKind::Fixed(point_ref, _) => vec![*point_ref],
            SketchConstraintKind::Coincident(a, b) => vec![*a, *b],
            SketchConstraintKind::Horizontal(line) | SketchConstraintKind::Vertical(line) => {
                vec![PointRef::LineStart(*line), PointRef::LineEnd(*line)]
            }
            SketchConstraintKind::Parallel(a, b)
            | SketchConstraintKind::Perpendicular(a, b)
            | SketchConstraintKind::EqualLength(a, b) => vec![
                PointRef::LineStart(*a),
                PointRef::LineEnd(*a),
                PointRef::LineStart(*b),
                PointRef::LineEnd(*b),
            ],
            SketchConstraintKind::EqualRadius(a, b) => vec![
                PointRef::CircleCenter(*a),
                PointRef::CirclePointOnCircle(*a),
                PointRef::CircleCenter(*b),
                PointRef::CirclePointOnCircle(*b),
            ],
            SketchConstraintKind::Concentric(a, b) => {
                vec![PointRef::CircleCenter(*a), PointRef::CircleCenter(*b)]
            }
            SketchConstraintKind::LineTangentToCircle(line, circle) => vec![
                PointRef::LineStart(*line),
                PointRef::LineEnd(*line),
                PointRef::CircleCenter(*circle),
                PointRef::CirclePointOnCircle(*circle),
            ],
            SketchConstraintKind::CircleTangentToCircle(a, b) => vec![
                PointRef::CircleCenter(*a),
                PointRef::CirclePointOnCircle(*a),
                PointRef::CircleCenter(*b),
                PointRef::CirclePointOnCircle(*b),
            ],
            SketchConstraintKind::Symmetric { axis, a, b } => {
                vec![PointRef::LineStart(*axis), PointRef::LineEnd(*axis), *a, *b]
            }
        }
    }

    /// Validate that every primitive this constraint names exists and is
    /// the right kind, *before* it is ever handed to a solver. Task 084's
    /// forbidden shortcut: "do not catch and ignore domain errors merely
    /// to keep the UI running" -- reject up front instead of discovering
    /// a type mismatch mid-solve.
    pub fn validate(&self, primitives: &PrimitiveMap) -> DomainResult<()> {
        match self {
            SketchConstraintKind::Fixed(point_ref, _) => {
                point_ref.resolve(primitives)?;
                Ok(())
            }
            SketchConstraintKind::Coincident(a, b) => {
                a.resolve(primitives)?;
                b.resolve(primitives)?;
                Ok(())
            }
            SketchConstraintKind::Horizontal(line) | SketchConstraintKind::Vertical(line) => {
                expect_line(primitives, *line)
            }
            SketchConstraintKind::Parallel(a, b)
            | SketchConstraintKind::Perpendicular(a, b)
            | SketchConstraintKind::EqualLength(a, b) => {
                expect_line(primitives, *a)?;
                expect_line(primitives, *b)
            }
            SketchConstraintKind::EqualRadius(a, b) | SketchConstraintKind::Concentric(a, b) => {
                expect_circle(primitives, *a)?;
                expect_circle(primitives, *b)
            }
            SketchConstraintKind::LineTangentToCircle(line, circle) => {
                expect_line(primitives, *line)?;
                expect_circle(primitives, *circle)
            }
            SketchConstraintKind::CircleTangentToCircle(a, b) => {
                expect_circle(primitives, *a)?;
                expect_circle(primitives, *b)
            }
            SketchConstraintKind::Symmetric { axis, a, b } => {
                expect_line(primitives, *axis)?;
                a.resolve(primitives)?;
                b.resolve(primitives)?;
                Ok(())
            }
        }
    }

    /// Lower to the backend-neutral vocabulary, given a resolved
    /// `PointRef -> PointVariables` mapping covering every point returned
    /// by [`Self::point_refs`]. Returns more than one `GeometricConstraint`
    /// only for `Fixed` (one `FixedValue` per axis) -- every other variant
    /// corresponds to exactly one relationship in
    /// `craftloop-constraint`'s vocabulary.
    pub fn to_geometric_constraint(
        &self,
        vars: &impl Fn(PointRef) -> craftloop_constraint::PointVariables,
    ) -> Vec<GeometricConstraint> {
        match self {
            SketchConstraintKind::Fixed(point_ref, value) => {
                let variables = vars(*point_ref);
                vec![
                    GeometricConstraint::FixedValue {
                        variable: variables.x,
                        value: value.x,
                    },
                    GeometricConstraint::FixedValue {
                        variable: variables.y,
                        value: value.y,
                    },
                ]
            }
            SketchConstraintKind::Coincident(a, b) => vec![GeometricConstraint::Coincident {
                a: vars(*a),
                b: vars(*b),
            }],
            SketchConstraintKind::Horizontal(line) => vec![GeometricConstraint::Horizontal {
                a: vars(PointRef::LineStart(*line)),
                b: vars(PointRef::LineEnd(*line)),
            }],
            SketchConstraintKind::Vertical(line) => vec![GeometricConstraint::Vertical {
                a: vars(PointRef::LineStart(*line)),
                b: vars(PointRef::LineEnd(*line)),
            }],
            SketchConstraintKind::Parallel(a, b) => vec![GeometricConstraint::Parallel {
                a0: vars(PointRef::LineStart(*a)),
                a1: vars(PointRef::LineEnd(*a)),
                b0: vars(PointRef::LineStart(*b)),
                b1: vars(PointRef::LineEnd(*b)),
            }],
            SketchConstraintKind::Perpendicular(a, b) => vec![GeometricConstraint::Perpendicular {
                a0: vars(PointRef::LineStart(*a)),
                a1: vars(PointRef::LineEnd(*a)),
                b0: vars(PointRef::LineStart(*b)),
                b1: vars(PointRef::LineEnd(*b)),
            }],
            SketchConstraintKind::EqualLength(a, b) => vec![GeometricConstraint::EqualLength {
                a0: vars(PointRef::LineStart(*a)),
                a1: vars(PointRef::LineEnd(*a)),
                b0: vars(PointRef::LineStart(*b)),
                b1: vars(PointRef::LineEnd(*b)),
            }],
            SketchConstraintKind::EqualRadius(a, b) => vec![GeometricConstraint::EqualLength {
                a0: vars(PointRef::CircleCenter(*a)),
                a1: vars(PointRef::CirclePointOnCircle(*a)),
                b0: vars(PointRef::CircleCenter(*b)),
                b1: vars(PointRef::CirclePointOnCircle(*b)),
            }],
            SketchConstraintKind::Concentric(a, b) => vec![GeometricConstraint::Coincident {
                a: vars(PointRef::CircleCenter(*a)),
                b: vars(PointRef::CircleCenter(*b)),
            }],
            SketchConstraintKind::LineTangentToCircle(line, circle) => {
                vec![GeometricConstraint::LineTangentToCircle {
                    line_a: vars(PointRef::LineStart(*line)),
                    line_b: vars(PointRef::LineEnd(*line)),
                    center: vars(PointRef::CircleCenter(*circle)),
                    point_on_circle: vars(PointRef::CirclePointOnCircle(*circle)),
                }]
            }
            SketchConstraintKind::CircleTangentToCircle(a, b) => {
                vec![GeometricConstraint::CircleTangentToCircle {
                    a_center: vars(PointRef::CircleCenter(*a)),
                    a_point_on_circle: vars(PointRef::CirclePointOnCircle(*a)),
                    b_center: vars(PointRef::CircleCenter(*b)),
                    b_point_on_circle: vars(PointRef::CirclePointOnCircle(*b)),
                }]
            }
            SketchConstraintKind::Symmetric { axis, a, b } => {
                vec![GeometricConstraint::Symmetric {
                    axis_a: vars(PointRef::LineStart(*axis)),
                    axis_b: vars(PointRef::LineEnd(*axis)),
                    a: vars(*a),
                    b: vars(*b),
                }]
            }
        }
    }

    /// Task 096: does `other` express exactly the same relationship as
    /// `self`, up to argument order for symmetric relationships (e.g.
    /// `Parallel(a, b)` and `Parallel(b, a)` are the same relationship)?
    /// Used to detect and reject a literal duplicate before it enters the
    /// constraint graph (Article 312's "the internal graph should avoid
    /// unnecessary duplicate constraints") -- deliberately narrower than
    /// general symbolic redundancy (e.g. noticing two lines are *already*
    /// forced parallel by an unrelated chain of constraints, which needs
    /// graph/symbolic reasoning no task in this phase asks for), whereas
    /// literal duplicate detection is exactly what Task 096 asks for.
    pub fn is_equivalent(&self, other: &SketchConstraintKind) -> bool {
        fn unordered_primitives(a: PrimitiveId, b: PrimitiveId) -> (PrimitiveId, PrimitiveId) {
            if a <= b {
                (a, b)
            } else {
                (b, a)
            }
        }
        fn unordered_points(a: PointRef, b: PointRef) -> (PointRef, PointRef) {
            if a <= b {
                (a, b)
            } else {
                (b, a)
            }
        }
        const EPSILON: f64 = 1e-9;
        match (self, other) {
            (SketchConstraintKind::Fixed(p1, v1), SketchConstraintKind::Fixed(p2, v2)) => {
                p1 == p2 && (v1.x - v2.x).abs() < EPSILON && (v1.y - v2.y).abs() < EPSILON
            }
            (
                SketchConstraintKind::Coincident(a1, b1),
                SketchConstraintKind::Coincident(a2, b2),
            ) => unordered_points(*a1, *b1) == unordered_points(*a2, *b2),
            (SketchConstraintKind::Horizontal(a), SketchConstraintKind::Horizontal(b)) => a == b,
            (SketchConstraintKind::Vertical(a), SketchConstraintKind::Vertical(b)) => a == b,
            (SketchConstraintKind::Parallel(a1, b1), SketchConstraintKind::Parallel(a2, b2))
            | (
                SketchConstraintKind::Perpendicular(a1, b1),
                SketchConstraintKind::Perpendicular(a2, b2),
            )
            | (
                SketchConstraintKind::EqualLength(a1, b1),
                SketchConstraintKind::EqualLength(a2, b2),
            )
            | (
                SketchConstraintKind::EqualRadius(a1, b1),
                SketchConstraintKind::EqualRadius(a2, b2),
            )
            | (
                SketchConstraintKind::Concentric(a1, b1),
                SketchConstraintKind::Concentric(a2, b2),
            )
            | (
                SketchConstraintKind::CircleTangentToCircle(a1, b1),
                SketchConstraintKind::CircleTangentToCircle(a2, b2),
            ) => unordered_primitives(*a1, *b1) == unordered_primitives(*a2, *b2),
            (
                SketchConstraintKind::LineTangentToCircle(line1, circle1),
                SketchConstraintKind::LineTangentToCircle(line2, circle2),
            ) => line1 == line2 && circle1 == circle2,
            (
                SketchConstraintKind::Symmetric {
                    axis: ax1,
                    a: a1,
                    b: b1,
                },
                SketchConstraintKind::Symmetric {
                    axis: ax2,
                    a: a2,
                    b: b2,
                },
            ) => ax1 == ax2 && unordered_points(*a1, *b1) == unordered_points(*a2, *b2),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_constraint::{PointVariables, VariableId};
    use craftloop_geometry::{Circle2, Point2, Segment2};
    use craftloop_ids::CraftLoopId;
    use craftloop_recognition::Beautified;

    fn line_primitive(a: Point2, b: Point2) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
            displacement: 0.0,
        }
    }

    fn circle_primitive(center: Point2, radius: f64) -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Circle(Circle2::new(center, radius).unwrap()),
            displacement: 0.0,
        }
    }

    fn dummy_vars(index: &std::cell::Cell<u64>) -> impl Fn(PointRef) -> PointVariables + '_ {
        move |_point_ref| {
            let current = index.get();
            index.set(current + 2);
            PointVariables::new(VariableId(current), VariableId(current + 1))
        }
    }

    #[test]
    fn horizontal_requires_a_line_primitive() {
        let mut primitives = PrimitiveMap::new();
        let circle_id = PrimitiveId::new();
        primitives.insert(circle_id, circle_primitive(Point2::ORIGIN, 1.0));

        let kind = SketchConstraintKind::Horizontal(circle_id);
        let err = kind.validate(&primitives).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::WrongPrimitiveKind,
                ..
            }
        ));
    }

    #[test]
    fn concentric_requires_two_circles() {
        let mut primitives = PrimitiveMap::new();
        let line_id = PrimitiveId::new();
        let circle_id = PrimitiveId::new();
        primitives.insert(
            line_id,
            line_primitive(Point2::ORIGIN, Point2::new(1.0, 1.0)),
        );
        primitives.insert(circle_id, circle_primitive(Point2::ORIGIN, 1.0));

        let kind = SketchConstraintKind::Concentric(line_id, circle_id);
        let err = kind.validate(&primitives).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::WrongPrimitiveKind,
                ..
            }
        ));
    }

    #[test]
    fn a_well_formed_constraint_validates_and_lowers_without_error() {
        let mut primitives = PrimitiveMap::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        primitives.insert(a, circle_primitive(Point2::ORIGIN, 2.0));
        primitives.insert(b, circle_primitive(Point2::new(5.0, 0.0), 1.0));

        let kind = SketchConstraintKind::Concentric(a, b);
        kind.validate(&primitives).unwrap();

        let index = std::cell::Cell::new(0u64);
        let lowered = kind.to_geometric_constraint(&dummy_vars(&index));
        assert_eq!(lowered.len(), 1);
        assert!(matches!(lowered[0], GeometricConstraint::Coincident { .. }));
    }

    #[test]
    fn concentric_lowers_to_coincident_on_the_centers_not_a_duplicated_variant() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let kind = SketchConstraintKind::Concentric(a, b);
        let index = std::cell::Cell::new(0u64);
        let lowered = kind.to_geometric_constraint(&dummy_vars(&index));
        let [GeometricConstraint::Coincident { .. }] = lowered.as_slice() else {
            panic!("expected Concentric to lower to exactly one Coincident, got {lowered:?}");
        };
    }

    #[test]
    fn equal_radius_lowers_to_equal_length_over_center_and_point_on_circle() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let kind = SketchConstraintKind::EqualRadius(a, b);
        let index = std::cell::Cell::new(0u64);
        let lowered = kind.to_geometric_constraint(&dummy_vars(&index));
        assert_eq!(lowered.len(), 1);
        assert!(matches!(
            lowered[0],
            GeometricConstraint::EqualLength { .. }
        ));
    }

    #[test]
    fn fixed_lowers_to_two_fixed_value_constraints_one_per_axis() {
        let point_ref = PointRef::LineStart(PrimitiveId::new());
        let kind = SketchConstraintKind::Fixed(point_ref, Point2::new(3.0, 4.0));
        let index = std::cell::Cell::new(0u64);
        let lowered = kind.to_geometric_constraint(&dummy_vars(&index));
        assert_eq!(lowered.len(), 2);
        assert!(lowered
            .iter()
            .all(|c| matches!(c, GeometricConstraint::FixedValue { .. })));
    }

    #[test]
    fn parallel_is_equivalent_to_itself_with_arguments_swapped() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        assert!(SketchConstraintKind::Parallel(a, b)
            .is_equivalent(&SketchConstraintKind::Parallel(b, a)));
    }

    #[test]
    fn parallel_between_different_primitives_is_not_equivalent() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let c = PrimitiveId::new();
        assert!(!SketchConstraintKind::Parallel(a, b)
            .is_equivalent(&SketchConstraintKind::Parallel(a, c)));
    }

    #[test]
    fn different_relationship_kinds_over_the_same_primitives_are_not_equivalent() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        assert!(!SketchConstraintKind::Parallel(a, b)
            .is_equivalent(&SketchConstraintKind::Perpendicular(a, b)));
    }

    #[test]
    fn line_tangent_to_circle_is_not_symmetric_line_and_circle_are_distinct_roles() {
        let line = PrimitiveId::new();
        let circle = PrimitiveId::new();
        // Swapping the roles would name a different (nonsensical, since
        // neither is actually the other's kind) relationship -- must not
        // be treated as equivalent, unlike the genuinely symmetric kinds
        // above.
        assert!(!SketchConstraintKind::LineTangentToCircle(line, circle)
            .is_equivalent(&SketchConstraintKind::LineTangentToCircle(circle, line)));
    }

    #[test]
    fn symmetric_constraints_on_the_same_axis_with_swapped_points_are_equivalent() {
        let axis = PrimitiveId::new();
        let a = PointRef::LineStart(PrimitiveId::new());
        let b = PointRef::LineStart(PrimitiveId::new());
        assert!(SketchConstraintKind::Symmetric { axis, a, b }
            .is_equivalent(&SketchConstraintKind::Symmetric { axis, a: b, b: a }));
    }

    #[test]
    fn fixed_constraints_with_different_target_values_are_not_equivalent() {
        let point_ref = PointRef::LineStart(PrimitiveId::new());
        let a = SketchConstraintKind::Fixed(point_ref, Point2::new(0.0, 0.0));
        let b = SketchConstraintKind::Fixed(point_ref, Point2::new(5.0, 5.0));
        assert!(
            !a.is_equivalent(&b),
            "different target values must not be treated as duplicates"
        );
    }
}
