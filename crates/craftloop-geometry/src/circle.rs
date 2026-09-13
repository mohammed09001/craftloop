//! Circle geometry.
//!
//! Execution 01, Phase 02, Task 015. Authority: Engine Contract 03; MCP
//! Article 588 "Requirement Group: Structured Circles".

use craftloop_errors::{DomainError, DomainResult, GeometryErrorKind};
use serde::{Deserialize, Serialize};

use crate::bounds::Bounds2;
use crate::point::Point2;
use crate::segment::Segment2;
use crate::tolerance::Tolerances;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Circle2 {
    pub center: Point2,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircleIntersection {
    /// The circles do not touch.
    None,
    /// The circles touch at exactly one point (internally or externally
    /// tangent).
    Tangent(Point2),
    /// The circles cross at two points.
    TwoPoints(Point2, Point2),
    /// The circles have the same center and radius.
    Coincident,
}

impl Circle2 {
    /// Construct a circle, rejecting a non-positive radius rather than
    /// silently accepting a degenerate/inverted circle. Forbidden shortcut
    /// this closes: earlier reference code (`windows-simulator`) built
    /// circles directly from unchecked drag distances (see
    /// `sim_core.SimulatorDocument.add_circle`); the production kernel must
    /// not repeat that.
    pub fn new(center: Point2, radius: f64) -> DomainResult<Self> {
        if radius > 0.0 && radius.is_finite() {
            Ok(Self { center, radius })
        } else {
            Err(DomainError::Geometry {
                kind: GeometryErrorKind::ToleranceViolation,
                detail: format!("circle radius must be positive and finite, got {radius}"),
            })
        }
    }

    pub fn bounds(&self) -> Bounds2 {
        Bounds2 {
            min: Point2::new(self.center.x - self.radius, self.center.y - self.radius),
            max: Point2::new(self.center.x + self.radius, self.center.y + self.radius),
        }
    }

    /// Signed distance from `point` to the circle boundary: negative inside,
    /// positive outside, zero on the boundary.
    pub fn signed_distance_to_boundary(&self, point: Point2) -> f64 {
        self.center.distance_to(point) - self.radius
    }

    pub fn distance_to_boundary(&self, point: Point2) -> f64 {
        self.signed_distance_to_boundary(point).abs()
    }

    pub fn contains_point(&self, point: Point2) -> bool {
        self.center.distance_to(point) <= self.radius
    }

    pub fn point_on_boundary(&self, point: Point2, tolerance: &Tolerances) -> bool {
        self.distance_to_boundary(point) <= tolerance.point_coincidence
    }

    /// The point on the boundary at `angle_radians` (measured from the
    /// positive x-axis, counter-clockwise).
    pub fn point_at_angle(&self, angle_radians: f64) -> Point2 {
        Point2::new(
            self.center.x + self.radius * angle_radians.cos(),
            self.center.y + self.radius * angle_radians.sin(),
        )
    }

    /// Intersect this circle with `other`.
    pub fn intersect_circle(&self, other: &Circle2, tolerance: &Tolerances) -> CircleIntersection {
        let d = self.center.distance_to(other.center);

        if d <= tolerance.point_coincidence
            && (self.radius - other.radius).abs() <= tolerance.length_equality
        {
            return CircleIntersection::Coincident;
        }

        let radius_sum = self.radius + other.radius;
        let radius_diff = (self.radius - other.radius).abs();

        if d > radius_sum + tolerance.point_coincidence
            || d < radius_diff - tolerance.point_coincidence
        {
            return CircleIntersection::None;
        }

        if (d - radius_sum).abs() <= tolerance.point_coincidence
            || (d - radius_diff).abs() <= tolerance.point_coincidence
        {
            // Tangent: the touching point lies on the line between centers,
            // at distance `self.radius` from `self.center`.
            let dir = self.center.vector_to(other.center);
            let unit = dir
                .normalized(tolerance)
                .expect("d > 0 was checked by the tangency distance comparisons above");
            return CircleIntersection::Tangent(self.center.translated(unit.scaled(self.radius)));
        }

        // Standard two-circle intersection via the radical line.
        let a = (self.radius * self.radius - other.radius * other.radius + d * d) / (2.0 * d);
        let h_sq = self.radius * self.radius - a * a;
        let h = h_sq.max(0.0).sqrt();

        let dir = self.center.vector_to(other.center);
        let unit = dir.normalized(tolerance).expect("d > 0 in this branch");
        let perp = unit.perpendicular();

        let mid = self.center.translated(unit.scaled(a));
        let p1 = mid.translated(perp.scaled(h));
        let p2 = mid.translated(perp.scaled(-h));

        CircleIntersection::TwoPoints(p1, p2)
    }

    /// Intersect this circle with a finite `segment`. Returns only
    /// intersection points that lie within the segment's endpoints (not the
    /// infinite line through it).
    pub fn intersect_segment(&self, segment: &Segment2, tolerance: &Tolerances) -> Vec<Point2> {
        let d = segment.vector();
        let f = self.center.vector_to(segment.a); // a - center

        let a_coeff = d.dot(d);
        if a_coeff <= tolerance.point_coincidence * tolerance.point_coincidence {
            return Vec::new();
        }
        let b_coeff = 2.0 * f.dot(d);
        let c_coeff = f.dot(f) - self.radius * self.radius;

        let discriminant = b_coeff * b_coeff - 4.0 * a_coeff * c_coeff;
        if discriminant < 0.0 {
            return Vec::new();
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b_coeff - sqrt_disc) / (2.0 * a_coeff);
        let t2 = (-b_coeff + sqrt_disc) / (2.0 * a_coeff);

        let mut points = Vec::new();
        for t in [t1, t2] {
            if (0.0..=1.0).contains(&t) {
                points.push(segment.a.lerp(segment.b, t));
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tol() -> Tolerances {
        Tolerances::committed()
    }

    #[test]
    fn non_positive_radius_is_rejected() {
        assert!(Circle2::new(Point2::ORIGIN, 0.0).is_err());
        assert!(Circle2::new(Point2::ORIGIN, -1.0).is_err());
        assert!(Circle2::new(Point2::ORIGIN, f64::NAN).is_err());
    }

    #[test]
    fn positive_radius_is_accepted() {
        assert!(Circle2::new(Point2::ORIGIN, 5.0).is_ok());
    }

    #[test]
    fn contains_point_is_true_on_and_inside_the_boundary() {
        let c = Circle2::new(Point2::ORIGIN, 5.0).unwrap();
        assert!(c.contains_point(Point2::new(0.0, 0.0)));
        assert!(c.contains_point(Point2::new(5.0, 0.0)));
        assert!(!c.contains_point(Point2::new(5.1, 0.0)));
    }

    #[test]
    fn distance_to_boundary_is_nonnegative_and_zero_on_boundary() {
        let c = Circle2::new(Point2::ORIGIN, 5.0).unwrap();
        let boundary_point = c.point_at_angle(0.7);
        assert!(c.distance_to_boundary(boundary_point) < 1e-9);
        assert!(c.distance_to_boundary(Point2::new(100.0, 100.0)) >= 0.0);
    }

    #[test]
    fn separate_circles_do_not_intersect() {
        let c1 = Circle2::new(Point2::new(0.0, 0.0), 1.0).unwrap();
        let c2 = Circle2::new(Point2::new(10.0, 0.0), 1.0).unwrap();
        assert_eq!(c1.intersect_circle(&c2, &tol()), CircleIntersection::None);
    }

    #[test]
    fn externally_tangent_circles_touch_at_one_point() {
        let c1 = Circle2::new(Point2::new(0.0, 0.0), 3.0).unwrap();
        let c2 = Circle2::new(Point2::new(8.0, 0.0), 5.0).unwrap();
        match c1.intersect_circle(&c2, &tol()) {
            CircleIntersection::Tangent(p) => {
                assert!((p.x - 3.0).abs() < 1e-9);
                assert!(p.y.abs() < 1e-9);
            }
            other => panic!("expected tangent, got {other:?}"),
        }
    }

    #[test]
    fn overlapping_circles_intersect_at_two_points_equidistant_from_both_centers() {
        let c1 = Circle2::new(Point2::new(0.0, 0.0), 5.0).unwrap();
        let c2 = Circle2::new(Point2::new(6.0, 0.0), 5.0).unwrap();
        match c1.intersect_circle(&c2, &tol()) {
            CircleIntersection::TwoPoints(p1, p2) => {
                for p in [p1, p2] {
                    assert!((c1.distance_to_boundary(p)) < 1e-9);
                    assert!((c2.distance_to_boundary(p)) < 1e-9);
                }
                assert!(p1.distance_to(p2) > 1e-6);
            }
            other => panic!("expected two points, got {other:?}"),
        }
    }

    #[test]
    fn identical_circles_are_coincident() {
        let c1 = Circle2::new(Point2::new(1.0, 1.0), 2.0).unwrap();
        let c2 = Circle2::new(Point2::new(1.0, 1.0), 2.0).unwrap();
        assert_eq!(
            c1.intersect_circle(&c2, &tol()),
            CircleIntersection::Coincident
        );
    }

    #[test]
    fn intersect_circle_is_symmetric() {
        let c1 = Circle2::new(Point2::new(0.0, 0.0), 5.0).unwrap();
        let c2 = Circle2::new(Point2::new(6.0, 0.0), 5.0).unwrap();
        let forward = c1.intersect_circle(&c2, &tol());
        let backward = c2.intersect_circle(&c1, &tol());
        assert_eq!(
            std::mem::discriminant(&forward),
            std::mem::discriminant(&backward)
        );
    }

    #[test]
    fn segment_through_circle_center_intersects_at_two_diametrically_opposed_points() {
        let c = Circle2::new(Point2::ORIGIN, 5.0).unwrap();
        let s = Segment2::new(Point2::new(-10.0, 0.0), Point2::new(10.0, 0.0));
        let points = c.intersect_segment(&s, &tol());
        assert_eq!(points.len(), 2);
        assert!(points.iter().any(|p| (p.x - (-5.0)).abs() < 1e-9));
        assert!(points.iter().any(|p| (p.x - 5.0).abs() < 1e-9));
    }

    #[test]
    fn segment_entirely_outside_circle_does_not_intersect() {
        let c = Circle2::new(Point2::ORIGIN, 1.0).unwrap();
        let s = Segment2::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        assert!(c.intersect_segment(&s, &tol()).is_empty());
    }

    #[test]
    fn segment_that_would_intersect_the_infinite_line_but_stops_short_does_not_intersect() {
        let c = Circle2::new(Point2::ORIGIN, 1.0).unwrap();
        // The infinite line y=0 crosses the circle, but this segment is
        // entirely to the right of it.
        let s = Segment2::new(Point2::new(5.0, 0.0), Point2::new(10.0, 0.0));
        assert!(c.intersect_segment(&s, &tol()).is_empty());
    }
}
