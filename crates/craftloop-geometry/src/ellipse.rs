//! Ellipse representation.
//!
//! Execution 01, Phase 02, Task 017. Authority: Engine Contract 03.
//!
//! Deliberately minimal per the task's own instruction: "implement ellipse
//! data and only the operations required by recognition/rendering; do not
//! overbuild conic algebra without a Version 1 use." No ellipse-ellipse or
//! ellipse-line intersection, no focus/directrix/eccentricity machinery --
//! none of that has a named Version 1 consumer yet. Add it when a later
//! phase has a real failing test that needs it, per the Loop Engineering
//! Contract, not speculatively here.

use craftloop_errors::{DomainError, DomainResult, GeometryErrorKind};
use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;

use crate::bounds::Bounds2;
use crate::point::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ellipse2 {
    pub center: Point2,
    pub semi_major: f64,
    pub semi_minor: f64,
    /// Rotation of the major axis from the positive x-axis, radians.
    pub rotation: f64,
}

impl Ellipse2 {
    pub fn new(
        center: Point2,
        semi_major: f64,
        semi_minor: f64,
        rotation: f64,
    ) -> DomainResult<Self> {
        if !(semi_major > 0.0 && semi_major.is_finite()) {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::ToleranceViolation,
                detail: format!("ellipse semi_major must be positive and finite, got {semi_major}"),
            });
        }
        if !(semi_minor > 0.0 && semi_minor.is_finite()) {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::ToleranceViolation,
                detail: format!("ellipse semi_minor must be positive and finite, got {semi_minor}"),
            });
        }
        if !rotation.is_finite() {
            return Err(DomainError::Geometry {
                kind: GeometryErrorKind::DegenerateInput,
                detail: "ellipse rotation must be finite".to_string(),
            });
        }
        Ok(Self {
            center,
            semi_major,
            semi_minor,
            rotation,
        })
    }

    /// Evaluate the ellipse boundary at parameter `t` in `[0, 1]`, one full
    /// revolution per unit of `t`. Used for rendering a tessellated outline.
    pub fn evaluate(&self, t: f64) -> Point2 {
        let angle = t * TAU;
        // Point on the unrotated ellipse, then rotate into place.
        let local_x = self.semi_major * angle.cos();
        let local_y = self.semi_minor * angle.sin();
        let (sin_r, cos_r) = self.rotation.sin_cos();
        Point2::new(
            self.center.x + local_x * cos_r - local_y * sin_r,
            self.center.y + local_x * sin_r + local_y * cos_r,
        )
    }

    /// Exact axis-aligned bounding box of the (possibly rotated) ellipse,
    /// via the closed-form half-extent formula, not tessellation.
    pub fn bounds(&self) -> Bounds2 {
        let (sin_r, cos_r) = self.rotation.sin_cos();
        let half_width =
            ((self.semi_major * cos_r).powi(2) + (self.semi_minor * sin_r).powi(2)).sqrt();
        let half_height =
            ((self.semi_major * sin_r).powi(2) + (self.semi_minor * cos_r).powi(2)).sqrt();
        Bounds2 {
            min: Point2::new(self.center.x - half_width, self.center.y - half_height),
            max: Point2::new(self.center.x + half_width, self.center.y + half_height),
        }
    }

    pub fn is_circle(&self, length_equality_tolerance: f64) -> bool {
        (self.semi_major - self.semi_minor).abs() <= length_equality_tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tolerance::Tolerances;
    use std::f64::consts::PI;

    #[test]
    fn non_positive_axis_is_rejected() {
        assert!(Ellipse2::new(Point2::ORIGIN, 0.0, 1.0, 0.0).is_err());
        assert!(Ellipse2::new(Point2::ORIGIN, 1.0, -1.0, 0.0).is_err());
    }

    #[test]
    fn unrotated_bounds_match_center_plus_minus_axes() {
        let e = Ellipse2::new(Point2::new(1.0, 2.0), 5.0, 3.0, 0.0).unwrap();
        let b = e.bounds();
        assert!((b.min.x - (-4.0)).abs() < 1e-9);
        assert!((b.max.x - 6.0).abs() < 1e-9);
        assert!((b.min.y - (-1.0)).abs() < 1e-9);
        assert!((b.max.y - 5.0).abs() < 1e-9);
    }

    #[test]
    fn quarter_turn_rotation_swaps_the_effective_extents() {
        let e = Ellipse2::new(Point2::ORIGIN, 5.0, 3.0, PI / 2.0).unwrap();
        let b = e.bounds();
        assert!((b.width() - 6.0).abs() < 1e-9); // was semi_minor*2
        assert!((b.height() - 10.0).abs() < 1e-9); // was semi_major*2
    }

    #[test]
    fn evaluate_at_t_zero_lands_on_the_major_axis_vertex() {
        let e = Ellipse2::new(Point2::ORIGIN, 5.0, 3.0, 0.0).unwrap();
        let p = e.evaluate(0.0);
        assert!((p.x - 5.0).abs() < 1e-9);
        assert!(p.y.abs() < 1e-9);
    }

    #[test]
    fn evaluate_is_periodic_in_t() {
        let e = Ellipse2::new(Point2::new(2.0, -1.0), 5.0, 3.0, 0.4).unwrap();
        let p0 = e.evaluate(0.25);
        let p1 = e.evaluate(1.25);
        assert!(p0.distance_to(p1) < 1e-9);
    }

    #[test]
    fn is_circle_detects_equal_axes_within_tolerance() {
        let e = Ellipse2::new(Point2::ORIGIN, 5.0, 5.0, 0.0).unwrap();
        assert!(e.is_circle(Tolerances::committed().length_equality));
        let not_circle = Ellipse2::new(Point2::ORIGIN, 5.0, 3.0, 0.0).unwrap();
        assert!(!not_circle.is_circle(Tolerances::committed().length_equality));
    }
}
