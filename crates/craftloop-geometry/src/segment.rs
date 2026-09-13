//! Line segment geometry.
//!
//! Execution 01, Phase 02, Task 014. Authority: Engine Contract 03; MCP
//! Article 587 "Requirement Group: Structured Lines".

use serde::{Deserialize, Serialize};

use crate::bounds::Bounds2;
use crate::point::{Point2, Vector2};
use crate::tolerance::Tolerances;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Segment2 {
    pub a: Point2,
    pub b: Point2,
}

/// Result of intersecting two segments. Deliberately does not attempt to
/// describe the overlap *extent* of two collinear-overlapping segments --
/// that is more than Task 014 asks for and is left for whichever later
/// engine (e.g. consistency, Phase 14) first needs it, so it can be added
/// with a real test driving the exact semantics it needs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentIntersection {
    /// The segments do not intersect within their bounds.
    None,
    /// The segments cross (or touch) at exactly one point.
    Point(Point2),
    /// The segments are parallel and not collinear.
    Parallel,
    /// The segments lie on the same infinite line.
    Collinear,
}

impl Segment2 {
    pub fn new(a: Point2, b: Point2) -> Self {
        Self { a, b }
    }

    pub fn length(&self) -> f64 {
        self.a.distance_to(self.b)
    }

    /// The raw (non-normalized) displacement from `a` to `b`.
    pub fn vector(&self) -> Vector2 {
        self.a.vector_to(self.b)
    }

    /// The unit direction from `a` to `b`, or `None` if the segment is
    /// degenerate (zero length under `tolerance`).
    pub fn direction(&self, tolerance: &Tolerances) -> Option<Vector2> {
        self.vector().normalized(tolerance)
    }

    pub fn midpoint(&self) -> Point2 {
        self.a.lerp(self.b, 0.5)
    }

    pub fn bounds(&self) -> Bounds2 {
        Bounds2::from_points(&[self.a, self.b]).expect("a segment always has two points")
    }

    /// The endpoint-reversed segment. `Segment2::new(b, a)`. Used by
    /// property tests to check operations are consistent under reversal.
    pub fn reversed(&self) -> Segment2 {
        Segment2::new(self.b, self.a)
    }

    /// The parametric value `t` (not clamped to `[0, 1]`) of the closest
    /// point on the *infinite* line through `a`/`b` to `point`. Returns
    /// `None` for a degenerate (zero-length) segment.
    pub fn closest_t(&self, point: Point2, tolerance: &Tolerances) -> Option<f64> {
        let d = self.vector();
        let len_sq = d.length_squared();
        if len_sq <= tolerance.point_coincidence * tolerance.point_coincidence {
            return None;
        }
        let to_point = self.a.vector_to(point);
        Some(to_point.dot(d) / len_sq)
    }

    /// The closest point *on the segment* (clamped to the endpoints) to
    /// `point`. Falls back to `a` for a degenerate segment.
    pub fn closest_point(&self, point: Point2, tolerance: &Tolerances) -> Point2 {
        match self.closest_t(point, tolerance) {
            Some(t) => self.a.lerp(self.b, t.clamp(0.0, 1.0)),
            None => self.a,
        }
    }

    pub fn distance_to_point(&self, point: Point2, tolerance: &Tolerances) -> f64 {
        self.closest_point(point, tolerance).distance_to(point)
    }

    /// True if `point` is within `max_distance` of the segment. Distinct
    /// from a `Tolerances` field because hit-testing is an interactive
    /// concept (what the harness/selection code offers) rather than a fixed
    /// geometric constant -- callers typically pass
    /// `Tolerances::recognition().snap_distance` or a UI-scaled value.
    pub fn hit_test(&self, point: Point2, max_distance: f64, tolerance: &Tolerances) -> bool {
        self.distance_to_point(point, tolerance) <= max_distance
    }

    /// Intersect this segment with `other`. See [`SegmentIntersection`] for
    /// the cases distinguished.
    pub fn intersect(&self, other: &Segment2, tolerance: &Tolerances) -> SegmentIntersection {
        let d1 = self.vector();
        let d2 = other.vector();
        let denom = d1.cross(d2);

        let a1_to_a2 = self.a.vector_to(other.a);

        if denom.abs() <= tolerance.parallel {
            // Parallel or collinear. Collinear iff a1_to_a2 is also
            // parallel to d1 (cross product with d1 is ~0).
            if a1_to_a2.cross(d1).abs() <= tolerance.parallel {
                return SegmentIntersection::Collinear;
            }
            return SegmentIntersection::Parallel;
        }

        let t = a1_to_a2.cross(d2) / denom;
        let u = a1_to_a2.cross(d1) / denom;

        let in_unit_range = |v: f64| {
            (-tolerance.solver_convergence..=1.0 + tolerance.solver_convergence).contains(&v)
        };

        if in_unit_range(t) && in_unit_range(u) {
            SegmentIntersection::Point(self.a.lerp(self.b, t))
        } else {
            SegmentIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tol() -> Tolerances {
        Tolerances::committed()
    }

    #[test]
    fn length_is_nonnegative_and_symmetric_under_reversal() {
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(3.0, 4.0));
        assert_eq!(s.length(), 5.0);
        assert_eq!(s.length(), s.reversed().length());
        assert!(s.length() >= 0.0);
    }

    #[test]
    fn direction_is_unit_length_and_none_for_degenerate_segment() {
        let s = Segment2::new(Point2::new(1.0, 1.0), Point2::new(1.0, 5.0));
        let dir = s.direction(&tol()).unwrap();
        assert!((dir.length() - 1.0).abs() < 1e-12);

        let degenerate = Segment2::new(Point2::new(2.0, 2.0), Point2::new(2.0, 2.0));
        assert!(degenerate.direction(&tol()).is_none());
    }

    #[test]
    fn closest_point_on_segment_clamps_to_endpoints() {
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        // Perpendicular from the middle lands on the segment.
        assert_eq!(
            s.closest_point(Point2::new(5.0, 3.0), &tol()),
            Point2::new(5.0, 0.0)
        );
        // Beyond `b` clamps to `b`.
        assert_eq!(s.closest_point(Point2::new(20.0, 3.0), &tol()), s.b);
        // Before `a` clamps to `a`.
        assert_eq!(s.closest_point(Point2::new(-5.0, 3.0), &tol()), s.a);
    }

    #[test]
    fn distance_to_point_is_nonnegative() {
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        assert!(s.distance_to_point(Point2::new(4.0, -7.0), &tol()) >= 0.0);
    }

    #[test]
    fn hit_test_respects_max_distance() {
        let s = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        assert!(s.hit_test(Point2::new(5.0, 0.5), 1.0, &tol()));
        assert!(!s.hit_test(Point2::new(5.0, 5.0), 1.0, &tol()));
    }

    #[test]
    fn crossing_segments_intersect_at_one_point() {
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let s2 = Segment2::new(Point2::new(0.0, 10.0), Point2::new(10.0, 0.0));
        match s1.intersect(&s2, &tol()) {
            SegmentIntersection::Point(p) => {
                assert!((p.x - 5.0).abs() < 1e-9);
                assert!((p.y - 5.0).abs() < 1e-9);
            }
            other => panic!("expected a point intersection, got {other:?}"),
        }
    }

    #[test]
    fn intersection_is_symmetric_regardless_of_argument_order() {
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0));
        let s2 = Segment2::new(Point2::new(0.0, 10.0), Point2::new(10.0, 0.0));
        let forward = s1.intersect(&s2, &tol());
        let backward = s2.intersect(&s1, &tol());
        match (forward, backward) {
            (SegmentIntersection::Point(p1), SegmentIntersection::Point(p2)) => {
                assert!(p1.distance_to(p2) < 1e-9);
            }
            (a, b) => assert_eq!(std::mem::discriminant(&a), std::mem::discriminant(&b)),
        }
    }

    #[test]
    fn non_crossing_segments_that_would_cross_if_extended_do_not_intersect() {
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
        let s2 = Segment2::new(Point2::new(5.0, 0.0), Point2::new(5.0, -1.0));
        assert_eq!(s1.intersect(&s2, &tol()), SegmentIntersection::None);
    }

    #[test]
    fn parallel_non_collinear_segments_report_parallel() {
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        let s2 = Segment2::new(Point2::new(0.0, 1.0), Point2::new(10.0, 1.0));
        assert_eq!(s1.intersect(&s2, &tol()), SegmentIntersection::Parallel);
    }

    #[test]
    fn collinear_segments_are_detected() {
        let s1 = Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        let s2 = Segment2::new(Point2::new(5.0, 0.0), Point2::new(15.0, 0.0));
        assert_eq!(s1.intersect(&s2, &tol()), SegmentIntersection::Collinear);
    }

    #[test]
    fn bounds_matches_the_endpoint_extremes() {
        let s = Segment2::new(Point2::new(3.0, -2.0), Point2::new(-1.0, 5.0));
        let b = s.bounds();
        assert_eq!(b.min, Point2::new(-1.0, -2.0));
        assert_eq!(b.max, Point2::new(3.0, 5.0));
    }
}
