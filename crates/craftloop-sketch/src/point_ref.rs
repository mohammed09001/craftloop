//! Named point references into stored primitive geometry.
//!
//! Execution 01, Phase 12, Tasks 084-090. Authority: Engine Contract 10.
//!
//! A [`GeometricConstraint`](craftloop_constraint::GeometricConstraint)
//! only understands opaque solver variables; this module is the bridge
//! that lets a caller instead say "the start of this line" or "the center
//! of that circle" and have it resolved against real
//! [`BeautifiedPrimitive`] geometry (Phase 06). Deliberately scoped to
//! `Line` and `Circle` (plus `Rectangle` corners, which are already stored
//! as raw points -- see `RelationalRectangle`'s own doc comment on being
//! "just four corner points" so later constraint code never special-cases
//! rectangles): `Arc2`'s start/end points are *derived* from
//! `center`/`radius`/`start_angle`/`sweep_angle`, not stored directly, so
//! writing a solved point back would require inverse-angle math this
//! phase's tasks (084-090) do not ask for. No Version 1 task in this phase
//! references arc points; adding that support speculatively would be the
//! scope creep the Loop Engineering Contract forbids. Recorded here as an
//! explicit, safe-to-defer gap rather than a silent omission.

use craftloop_errors::{DomainError, DomainResult, SketchErrorKind};
use craftloop_geometry::Point2;
use craftloop_ids::PrimitiveId;
use craftloop_recognition::{Beautified, BeautifiedPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A stored primitive, keyed the same way [`crate::sketch::Sketch`] stores
/// them. Re-declared as a type alias only to keep call sites short.
pub type PrimitiveMap = BTreeMap<PrimitiveId, Beautified>;

/// A named point on a stored primitive.
///
/// `CirclePointOnCircle` is not a real, independently-meaningful point --
/// it is the same solver trick `craftloop-constraint`'s own
/// [`GeometricConstraint::Radius`](craftloop_constraint::GeometricConstraint::Radius)
/// already uses (Phase 11): a circle's radius is represented as the
/// distance from its center to *some* point on its boundary, because
/// `Circle2` itself stores a radius scalar, not a boundary point. The
/// solver is free to move this synthetic point anywhere on the solved
/// circle; only the resulting *distance* to the center is meaningful, and
/// only [`crate::sketch::Sketch::solve`] (not this module) is allowed to
/// read it back, by recomputing `radius` and discarding the point's angle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PointRef {
    LineStart(PrimitiveId),
    LineEnd(PrimitiveId),
    CircleCenter(PrimitiveId),
    CirclePointOnCircle(PrimitiveId),
    RectangleCorner(PrimitiveId, u8),
}

impl PointRef {
    pub fn primitive_id(&self) -> PrimitiveId {
        match self {
            PointRef::LineStart(id)
            | PointRef::LineEnd(id)
            | PointRef::CircleCenter(id)
            | PointRef::CirclePointOnCircle(id)
            | PointRef::RectangleCorner(id, _) => *id,
        }
    }

    /// The current value of this point, read from `primitives`.
    pub fn resolve(&self, primitives: &PrimitiveMap) -> DomainResult<Point2> {
        let primitive =
            primitives
                .get(&self.primitive_id())
                .ok_or_else(|| DomainError::Sketch {
                    kind: SketchErrorKind::UnknownPrimitive,
                    detail: format!("no primitive with id {:?}", self.primitive_id()),
                })?;
        match (self, &primitive.primitive) {
            (PointRef::LineStart(_), BeautifiedPrimitive::Line(segment)) => Ok(segment.a),
            (PointRef::LineEnd(_), BeautifiedPrimitive::Line(segment)) => Ok(segment.b),
            (PointRef::CircleCenter(_), BeautifiedPrimitive::Circle(circle)) => Ok(circle.center),
            (PointRef::CirclePointOnCircle(_), BeautifiedPrimitive::Circle(circle)) => Ok(
                Point2::new(circle.center.x + circle.radius, circle.center.y),
            ),
            (PointRef::RectangleCorner(_, index), BeautifiedPrimitive::Rectangle(rectangle)) => {
                rectangle
                    .corners
                    .get(*index as usize)
                    .copied()
                    .ok_or_else(|| DomainError::Sketch {
                        kind: SketchErrorKind::WrongPrimitiveKind,
                        detail: format!("rectangle corner index {index} out of range"),
                    })
            }
            _ => Err(DomainError::Sketch {
                kind: SketchErrorKind::WrongPrimitiveKind,
                detail: format!(
                    "point reference {self:?} does not match the stored primitive kind"
                ),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::{Circle2, Segment2};
    use craftloop_ids::CraftLoopId;
    use craftloop_recognition::Beautified;

    fn line(id: PrimitiveId, a: Point2, b: Point2) -> (PrimitiveId, Beautified) {
        (
            id,
            Beautified {
                primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
                displacement: 0.0,
            },
        )
    }

    fn circle(id: PrimitiveId, center: Point2, radius: f64) -> (PrimitiveId, Beautified) {
        (
            id,
            Beautified {
                primitive: BeautifiedPrimitive::Circle(Circle2::new(center, radius).unwrap()),
                displacement: 0.0,
            },
        )
    }

    #[test]
    fn line_start_and_end_resolve_to_the_segments_endpoints() {
        let id = PrimitiveId::new();
        let mut primitives = PrimitiveMap::new();
        let (id, beautified) = line(id, Point2::new(0.0, 0.0), Point2::new(3.0, 4.0));
        primitives.insert(id, beautified);

        assert_eq!(
            PointRef::LineStart(id).resolve(&primitives).unwrap(),
            Point2::new(0.0, 0.0)
        );
        assert_eq!(
            PointRef::LineEnd(id).resolve(&primitives).unwrap(),
            Point2::new(3.0, 4.0)
        );
    }

    #[test]
    fn circle_point_on_circle_is_radius_away_from_center() {
        let id = PrimitiveId::new();
        let mut primitives = PrimitiveMap::new();
        let (id, beautified) = circle(id, Point2::new(1.0, 1.0), 5.0);
        primitives.insert(id, beautified);

        let point = PointRef::CirclePointOnCircle(id)
            .resolve(&primitives)
            .unwrap();
        let center = PointRef::CircleCenter(id).resolve(&primitives).unwrap();
        assert!((center.distance_to(point) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn resolving_against_an_unknown_primitive_is_a_structured_error() {
        let primitives = PrimitiveMap::new();
        let err = PointRef::LineStart(PrimitiveId::new())
            .resolve(&primitives)
            .unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::UnknownPrimitive,
                ..
            }
        ));
    }

    #[test]
    fn resolving_the_wrong_point_kind_for_a_primitive_is_a_structured_error() {
        let id = PrimitiveId::new();
        let mut primitives = PrimitiveMap::new();
        let (id, beautified) = circle(id, Point2::ORIGIN, 3.0);
        primitives.insert(id, beautified);

        // A circle has no "line start" -- must fail structurally, not
        // panic or silently return a default point.
        let err = PointRef::LineStart(id).resolve(&primitives).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Sketch {
                kind: SketchErrorKind::WrongPrimitiveKind,
                ..
            }
        ));
    }
}
