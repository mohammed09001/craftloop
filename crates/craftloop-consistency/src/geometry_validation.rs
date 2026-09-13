//! Local geometry consistency.
//!
//! Execution 01, Phase 14, Task 101. Authority: MCP Article 27
//! ("Degenerate geometry" is named explicitly among the things the
//! consistency engine should identify).
//!
//! "Local" means this only looks at one primitive's own geometry, never
//! its relationships to others (Task 102 covers relationships). Most
//! degenerate states are already impossible to construct at all --
//! `Circle2::new`/`Arc2::new` (Phase 02) already reject a non-positive
//! radius or a zero sweep, so nothing here needs to re-check those.
//! `Segment2` and `RelationalRectangle::from_corners`, however, do *not*
//! reject a near-zero-length/near-zero-area result (`Segment2::new` takes
//! no `DomainResult` at all; `from_corners` only rejects a *coincident*
//! adjacent corner pair, not a merely tiny quadrilateral) -- those are the
//! real, remaining gaps this module closes.

use craftloop_errors::Severity;
use craftloop_geometry::Tolerances;
use craftloop_ids::{ConflictId, CraftLoopId, PrimitiveId};
use craftloop_recognition::BeautifiedPrimitive;

use crate::conflict::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};

/// Checks one primitive's own geometry for degeneracy. Returns `None` for
/// anything not degenerate -- including every `Circle`/`Arc`, which can
/// never reach this function in a degenerate state at all (Task 105:
/// never fabricate a conflict where none exists).
pub fn validate_primitive_geometry(
    id: PrimitiveId,
    primitive: &BeautifiedPrimitive,
) -> Option<Conflict> {
    let tolerance = Tolerances::committed();
    match primitive {
        BeautifiedPrimitive::Line(segment) => {
            let length = segment.length();
            if length < tolerance.point_coincidence {
                Some(Conflict {
                    id: ConflictId::new(),
                    kind: ConflictKind::DegenerateGeometry,
                    severity: Severity::Error,
                    affected_entities: vec![format!("{id:?}")],
                    existing_truth: "line has two distinct endpoints".to_string(),
                    proposed_truth: "line's endpoints coincide (zero length)".to_string(),
                    evidence: format!("length = {length}"),
                    resolution_choices: vec![
                        ResolutionChoice::KeepExisting,
                        ResolutionChoice::Cancel,
                    ],
                    status: ConflictStatus::Unresolved,
                })
            } else {
                None
            }
        }
        BeautifiedPrimitive::Rectangle(rectangle) => {
            let corners = rectangle.corners;
            // Shoelace formula.
            let area: f64 = (0..4)
                .map(|i| {
                    let a = corners[i];
                    let b = corners[(i + 1) % 4];
                    a.x * b.y - b.x * a.y
                })
                .sum::<f64>()
                .abs()
                / 2.0;
            if area < tolerance.point_coincidence {
                Some(Conflict {
                    id: ConflictId::new(),
                    kind: ConflictKind::DegenerateGeometry,
                    severity: Severity::Error,
                    affected_entities: vec![format!("{id:?}")],
                    existing_truth: "rectangle encloses a nonzero area".to_string(),
                    proposed_truth: "rectangle has collapsed to (near) zero area".to_string(),
                    evidence: format!("area = {area}"),
                    resolution_choices: vec![
                        ResolutionChoice::KeepExisting,
                        ResolutionChoice::Cancel,
                    ],
                    status: ConflictStatus::Unresolved,
                })
            } else {
                None
            }
        }
        // Circle2/Arc2 already reject a non-positive radius or zero sweep
        // at construction (Phase 02) -- nothing degenerate can reach here.
        BeautifiedPrimitive::Circle(_) | BeautifiedPrimitive::Arc(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::{Circle2, Point2, RelationalRectangle, Segment2};

    #[test]
    fn a_normal_length_line_has_no_conflict() {
        let segment = Segment2::new(Point2::new(0.0, 0.0), Point2::new(4.0, 0.0));
        let conflict =
            validate_primitive_geometry(PrimitiveId::new(), &BeautifiedPrimitive::Line(segment));
        assert!(conflict.is_none());
    }

    #[test]
    fn a_zero_length_line_is_flagged_degenerate() {
        let point = Point2::new(1.0, 1.0);
        let segment = Segment2::new(point, point);
        let conflict =
            validate_primitive_geometry(PrimitiveId::new(), &BeautifiedPrimitive::Line(segment))
                .unwrap();
        assert_eq!(conflict.kind, ConflictKind::DegenerateGeometry);
        assert!(conflict.is_unresolved());
    }

    #[test]
    fn a_line_barely_below_committed_tolerance_is_still_flagged() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(Tolerances::committed().point_coincidence / 2.0, 0.0);
        let conflict = validate_primitive_geometry(
            PrimitiveId::new(),
            &BeautifiedPrimitive::Line(Segment2::new(a, b)),
        );
        assert!(conflict.is_some());
    }

    #[test]
    fn a_normal_rectangle_has_no_conflict() {
        let rectangle = RelationalRectangle::from_corners([
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, 3.0),
            Point2::new(0.0, 3.0),
        ])
        .unwrap();
        let conflict = validate_primitive_geometry(
            PrimitiveId::new(),
            &BeautifiedPrimitive::Rectangle(rectangle),
        );
        assert!(conflict.is_none());
    }

    #[test]
    fn a_collapsed_flat_rectangle_is_flagged_degenerate() {
        // Four corners that are not pairwise coincident (so
        // `from_corners` accepts them) but lie on a single line, giving
        // zero enclosed area.
        let rectangle = RelationalRectangle::from_corners([
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(2.0, 1e-15),
        ])
        .unwrap();
        let conflict = validate_primitive_geometry(
            PrimitiveId::new(),
            &BeautifiedPrimitive::Rectangle(rectangle),
        )
        .unwrap();
        assert_eq!(conflict.kind, ConflictKind::DegenerateGeometry);
    }

    #[test]
    fn a_valid_circle_can_never_be_flagged_it_cannot_be_constructed_degenerate() {
        let circle = Circle2::new(Point2::ORIGIN, 5.0).unwrap();
        let conflict =
            validate_primitive_geometry(PrimitiveId::new(), &BeautifiedPrimitive::Circle(circle));
        assert!(conflict.is_none());
    }
}
