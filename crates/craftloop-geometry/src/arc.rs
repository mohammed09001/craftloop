//! Arc geometry.
//!
//! Execution 01, Phase 02, Task 016. Authority: Engine Contract 03; MCP
//! Article 589 "Requirement Group: Arcs".
//!
//! Sweep semantics: an arc is `center`/`radius` plus a `start_angle` and a
//! `sweep_angle`, both in radians. The end angle is always
//! `start_angle + sweep_angle`; a positive `sweep_angle` sweeps
//! counter-clockwise, negative sweeps clockwise. This representation was
//! chosen over "start angle + end angle" because start/end alone is
//! ambiguous about which of the two possible arcs (short way or long way
//! around) is meant, and about direction; start+sweep has neither ambiguity
//! and is standard in CAD kernels for that reason.

use craftloop_errors::{DomainError, DomainResult, GeometryErrorKind};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::bounds::Bounds2;
use crate::point::Point2;
use crate::tolerance::Tolerances;

const TWO_PI: f64 = 2.0 * PI;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Arc2 {
    pub center: Point2,
    pub radius: f64,
    pub start_angle: f64,
    pub sweep_angle: f64,
}

impl Arc2 {
    /// Construct an arc. Rejects a non-positive/non-finite radius (same
    /// policy as [`crate::circle::Circle2`]) and a zero or non-finite sweep
    /// angle (a zero-sweep "arc" is a degenerate point, not an arc).
    pub fn new(
        center: Point2,
        radius: f64,
        start_angle: f64,
        sweep_angle: f64,
    ) -> DomainResult<Self> {
        if !(radius > 0.0 && radius.is_finite()) {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::ToleranceViolation,
                detail: format!("arc radius must be positive and finite, got {radius}"),
            });
        }
        if !sweep_angle.is_finite() || sweep_angle == 0.0 {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::DegenerateInput,
                detail: format!("arc sweep_angle must be finite and nonzero, got {sweep_angle}"),
            });
        }
        if !start_angle.is_finite() {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::DegenerateInput,
                detail: "arc start_angle must be finite".to_string(),
            });
        }
        Ok(Self {
            center,
            radius,
            start_angle,
            sweep_angle,
        })
    }

    pub fn end_angle(&self) -> f64 {
        self.start_angle + self.sweep_angle
    }

    pub fn start_point(&self) -> Point2 {
        self.point_at_angle(self.start_angle)
    }

    pub fn end_point(&self) -> Point2 {
        self.point_at_angle(self.end_angle())
    }

    pub fn point_at_angle(&self, angle_radians: f64) -> Point2 {
        Point2::new(
            self.center.x + self.radius * angle_radians.cos(),
            self.center.y + self.radius * angle_radians.sin(),
        )
    }

    /// Evaluate the arc at `t` in `[0, 1]`, where `t = 0` is `start_point()`
    /// and `t = 1` is `end_point()`. Not clamped, so callers extrapolating
    /// past the arc get a well-defined (if out-of-range) result rather than
    /// a silent clamp.
    pub fn evaluate(&self, t: f64) -> Point2 {
        self.point_at_angle(self.start_angle + t * self.sweep_angle)
    }

    pub fn arc_length(&self) -> f64 {
        self.radius * self.sweep_angle.abs()
    }

    pub fn is_full_circle(&self, tolerance: &Tolerances) -> bool {
        self.sweep_angle.abs() >= TWO_PI - tolerance.length_equality
    }

    /// True if `angle_radians` lies within the swept range of this arc
    /// (inclusive of the endpoints), regardless of winding direction.
    pub fn contains_angle(&self, angle_radians: f64) -> bool {
        if self.is_full_circle(&Tolerances::committed()) {
            return true;
        }
        // Offset of `angle` from `start_angle`, going in the arc's own
        // winding direction, normalized to [0, 2*PI).
        let direction = self.sweep_angle.signum();
        let raw_offset = (angle_radians - self.start_angle) * direction;
        let normalized_offset = raw_offset.rem_euclid(TWO_PI);
        normalized_offset <= self.sweep_angle.abs()
    }

    pub fn bounds(&self) -> Bounds2 {
        let mut points = vec![self.start_point(), self.end_point()];
        // The arc's extreme x/y points occur where the tangent is
        // axis-aligned, i.e. at angles 0, PI/2, PI, 3*PI/2. Include any that
        // actually fall within the swept range.
        for axis_angle in [0.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
            if self.contains_angle(axis_angle) {
                points.push(self.point_at_angle(axis_angle));
            }
        }
        Bounds2::from_points(&points).expect("start/end points always present")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tol() -> Tolerances {
        Tolerances::committed()
    }

    #[test]
    fn zero_sweep_is_rejected() {
        assert!(Arc2::new(Point2::ORIGIN, 5.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn non_positive_radius_is_rejected() {
        assert!(Arc2::new(Point2::ORIGIN, 0.0, 0.0, PI).is_err());
        assert!(Arc2::new(Point2::ORIGIN, -1.0, 0.0, PI).is_err());
    }

    #[test]
    fn evaluate_at_zero_and_one_matches_start_and_end_points() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI / 2.0).unwrap();
        assert!(arc.evaluate(0.0).distance_to(arc.start_point()) < 1e-12);
        assert!(arc.evaluate(1.0).distance_to(arc.end_point()) < 1e-12);
    }

    #[test]
    fn quarter_circle_from_zero_sweeps_to_the_expected_point() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI / 2.0).unwrap();
        let end = arc.end_point();
        assert!(end.x.abs() < 1e-9);
        assert!((end.y - 5.0).abs() < 1e-9);
    }

    #[test]
    fn negative_sweep_goes_clockwise() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, -PI / 2.0).unwrap();
        let end = arc.end_point();
        assert!(end.x.abs() < 1e-9);
        assert!((end.y - (-5.0)).abs() < 1e-9);
    }

    #[test]
    fn arc_length_is_nonnegative_and_matches_radius_times_sweep_magnitude() {
        let arc = Arc2::new(Point2::ORIGIN, 2.0, 0.0, PI).unwrap();
        assert!((arc.arc_length() - 2.0 * PI).abs() < 1e-12);
        assert!(arc.arc_length() >= 0.0);
    }

    #[test]
    fn contains_angle_is_true_for_points_strictly_inside_the_sweep() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI).unwrap(); // upper half
        assert!(arc.contains_angle(PI / 2.0));
        assert!(!arc.contains_angle(-PI / 2.0));
    }

    #[test]
    fn contains_angle_handles_wraparound_past_two_pi() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 3.0 * PI / 2.0, PI).unwrap(); // wraps past 2*PI
        assert!(arc.contains_angle(0.0));
        assert!(arc.contains_angle(-PI / 2.0)); // same as 3*PI/2 normalized
    }

    #[test]
    fn bounds_of_a_quarter_circle_touching_an_axis_extreme_includes_that_extreme() {
        // 0 to PI/2 sweep touches the top-most point of the full circle at
        // angle PI/2 exactly at its end, and its right-most point at angle 0
        // exactly at its start.
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI / 2.0).unwrap();
        let bounds = arc.bounds();
        assert!((bounds.max.x - 5.0).abs() < 1e-9);
        assert!((bounds.max.y - 5.0).abs() < 1e-9);
    }

    #[test]
    fn bounds_of_a_half_circle_spans_the_full_diameter_on_the_swept_axis() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI).unwrap();
        let bounds = arc.bounds();
        assert!((bounds.min.x - (-5.0)).abs() < 1e-9);
        assert!((bounds.max.x - 5.0).abs() < 1e-9);
        assert!(bounds.max.y >= 0.0);
    }

    #[test]
    fn is_full_circle_detects_a_two_pi_sweep() {
        let arc = Arc2::new(Point2::ORIGIN, 5.0, 0.0, TWO_PI).unwrap();
        assert!(arc.is_full_circle(&tol()));
        let not_full = Arc2::new(Point2::ORIGIN, 5.0, 0.0, PI).unwrap();
        assert!(!not_full.is_full_circle(&tol()));
    }
}
