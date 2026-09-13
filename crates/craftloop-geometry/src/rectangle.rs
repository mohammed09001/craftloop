//! Rectangle as relational geometry.
//!
//! Execution 01, Phase 02, Task 018. Authority: Engine Contract 03; MCP
//! Article 130 "Separation of Geometry and Presentation".
//!
//! Forbidden shortcut this module exists to avoid: a rigid `Rectangle { x,
//! y, width, height }` object model. That representation hides the fact
//! that a rectangle is four related edges/corners, and would force a later
//! constraint solver (Phase 12, which operates on segments/points) to special
//! -case rectangles instead of expressing "opposite sides parallel and
//! equal, adjacent sides perpendicular" as ordinary constraints over the
//! same segment primitives it already understands.
//!
//! [`RelationalRectangle`] is therefore just four corner points in order.
//! [`RelationalRectangle::from_corners`] does **not** verify the four points
//! actually form a rectangle (right angles, equal opposite sides) -- that
//! verification is exactly what the constraint/consistency engines (Phases
//! 12-14) exist to do, deterministically, once those constraints are
//! confirmed. This kernel type only refuses truly degenerate input
//! (coincident adjacent corners), the same floor every other primitive in
//! this crate enforces.

use craftloop_errors::{DomainError, DomainResult, GeometryErrorKind};
use serde::{Deserialize, Serialize};

use crate::bounds::Bounds2;
use crate::point::Point2;
use crate::segment::Segment2;
use crate::tolerance::Tolerances;

/// Four corners, in order, describing the boundary of a (not necessarily
/// verified) rectangle: `corners[i]` connects to `corners[(i + 1) % 4]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RelationalRectangle {
    pub corners: [Point2; 4],
}

impl RelationalRectangle {
    /// Build a general relational rectangle from four corners in boundary
    /// order. Rejects only degenerate input (an edge with coincident
    /// endpoints under committed tolerance) -- it does not check
    /// right angles or equal sides.
    pub fn from_corners(corners: [Point2; 4]) -> DomainResult<Self> {
        let tolerance = Tolerances::committed();
        for i in 0..4 {
            let a = corners[i];
            let b = corners[(i + 1) % 4];
            if a.is_coincident_with(b, &tolerance) {
                return Err(DomainError::Geometry {
                    kind: GeometryErrorKind::DegenerateInput,
                    detail: format!("rectangle edge {i} has coincident endpoints"),
                });
            }
        }
        Ok(Self { corners })
    }

    /// Convenience constructor for the common axis-aligned case: build a
    /// relational rectangle from two opposite corners.
    pub fn from_axis_aligned(corner_a: Point2, corner_b: Point2) -> DomainResult<Self> {
        if (corner_a.x - corner_b.x).abs() <= Tolerances::committed().point_coincidence
            || (corner_a.y - corner_b.y).abs() <= Tolerances::committed().point_coincidence
        {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::DegenerateInput,
                detail: "axis-aligned rectangle corners must differ in both x and y".to_string(),
            });
        }
        let (min_x, max_x) = (corner_a.x.min(corner_b.x), corner_a.x.max(corner_b.x));
        let (min_y, max_y) = (corner_a.y.min(corner_b.y), corner_a.y.max(corner_b.y));
        Self::from_corners([
            Point2::new(min_x, min_y),
            Point2::new(max_x, min_y),
            Point2::new(max_x, max_y),
            Point2::new(min_x, max_y),
        ])
    }

    /// The four boundary edges, in the same order as `corners`.
    pub fn edges(&self) -> [Segment2; 4] {
        [
            Segment2::new(self.corners[0], self.corners[1]),
            Segment2::new(self.corners[1], self.corners[2]),
            Segment2::new(self.corners[2], self.corners[3]),
            Segment2::new(self.corners[3], self.corners[0]),
        ]
    }

    pub fn bounds(&self) -> Bounds2 {
        Bounds2::from_points(&self.corners).expect("a rectangle always has four points")
    }

    /// True if this really is a rectangle within `tolerance`: opposite
    /// edges parallel and equal length, adjacent edges perpendicular. This
    /// is a read-only geometric check for tests/diagnostics; it is
    /// deliberately not run at construction time (see module docs).
    pub fn is_axis_respecting_rectangle(&self, tolerance: &Tolerances) -> bool {
        let edges = self.edges();
        for i in 0..4 {
            let current = edges[i];
            let next = edges[(i + 1) % 4];
            let opposite = edges[(i + 2) % 4];

            let (Some(current_dir), Some(next_dir), Some(opposite_dir)) = (
                current.direction(tolerance),
                next.direction(tolerance),
                opposite.direction(tolerance),
            ) else {
                return false;
            };

            let perpendicular = current_dir.dot(next_dir).abs() <= tolerance.perpendicular;
            if !perpendicular {
                return false;
            }

            let opposite_parallel = current_dir.cross(opposite_dir).abs() <= tolerance.parallel;
            if !opposite_parallel {
                return false;
            }

            let equal_length =
                (current.length() - opposite.length()).abs() <= tolerance.length_equality;
            if !equal_length {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tol() -> Tolerances {
        Tolerances::committed()
    }

    #[test]
    fn axis_aligned_rectangle_has_four_edges_of_the_expected_lengths() {
        let rect =
            RelationalRectangle::from_axis_aligned(Point2::new(0.0, 0.0), Point2::new(10.0, 4.0))
                .unwrap();
        let edges = rect.edges();
        let lengths: Vec<f64> = edges.iter().map(Segment2::length).collect();
        assert_eq!(lengths, vec![10.0, 4.0, 10.0, 4.0]);
    }

    #[test]
    fn degenerate_axis_aligned_corners_are_rejected() {
        // Same x: zero width.
        assert!(RelationalRectangle::from_axis_aligned(
            Point2::new(1.0, 1.0),
            Point2::new(1.0, 5.0)
        )
        .is_err());
    }

    #[test]
    fn from_corners_rejects_a_coincident_edge_but_not_a_non_rectangular_quadrilateral() {
        assert!(RelationalRectangle::from_corners([
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 0.0), // coincident with previous corner
            Point2::new(5.0, 5.0),
            Point2::new(0.0, 5.0),
        ])
        .is_err());

        // A valid (non-degenerate) but non-rectangular quadrilateral is
        // still constructible: rectangularity is a constraint-engine
        // question, not a kernel-construction question.
        let skewed = RelationalRectangle::from_corners([
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            Point2::new(12.0, 5.0),
            Point2::new(0.0, 5.0),
        ])
        .unwrap();
        assert!(!skewed.is_axis_respecting_rectangle(&tol()));
    }

    #[test]
    fn axis_aligned_constructor_always_produces_a_verified_rectangle() {
        let rect =
            RelationalRectangle::from_axis_aligned(Point2::new(-3.0, 2.0), Point2::new(7.0, -6.0))
                .unwrap();
        assert!(rect.is_axis_respecting_rectangle(&tol()));
    }

    #[test]
    fn bounds_matches_the_corner_extremes() {
        let rect =
            RelationalRectangle::from_axis_aligned(Point2::new(0.0, 0.0), Point2::new(6.0, 3.0))
                .unwrap();
        let bounds = rect.bounds();
        assert_eq!(bounds.min, Point2::new(0.0, 0.0));
        assert_eq!(bounds.max, Point2::new(6.0, 3.0));
    }

    #[test]
    fn edges_form_a_closed_loop() {
        let rect =
            RelationalRectangle::from_axis_aligned(Point2::new(0.0, 0.0), Point2::new(6.0, 3.0))
                .unwrap();
        let edges = rect.edges();
        for i in 0..4 {
            assert_eq!(edges[i].b, edges[(i + 1) % 4].a);
        }
    }
}
