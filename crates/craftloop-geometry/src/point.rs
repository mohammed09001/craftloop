//! Two-dimensional point and vector primitives.
//!
//! Execution 01, Phase 02, Task 013. Authority: Engine Contract 03
//! (Geometry Kernel: "deterministic and UI-independent"); MCP Article 95
//! "Vector Geometry Engine".
//!
//! `Point2` (a location) and `Vector2` (a displacement/direction) are kept
//! as distinct types even though both are pairs of `f64`, because they
//! answer different questions and support different operations: two points
//! can be subtracted to get a vector, but adding two points is not a
//! meaningful geometric operation the way adding two vectors is.

use serde::{Deserialize, Serialize};

use crate::tolerance::Tolerances;

/// A location in 2D document space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

/// A 2D displacement/direction, distinct from [`Point2`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const ORIGIN: Point2 = Point2::new(0.0, 0.0);

    /// The displacement from `self` to `other`.
    pub fn vector_to(&self, other: Point2) -> Vector2 {
        Vector2::new(other.x - self.x, other.y - self.y)
    }

    pub fn translated(&self, offset: Vector2) -> Point2 {
        Point2::new(self.x + offset.x, self.y + offset.y)
    }

    pub fn distance_to(&self, other: Point2) -> f64 {
        self.vector_to(other).length()
    }

    /// True if the two points are within `tolerance.point_coincidence` of
    /// each other. Use [`Tolerances::committed`] for exact-geometry checks
    /// and [`Tolerances::recognition`] for interpreting hand-drawn ink.
    pub fn is_coincident_with(&self, other: Point2, tolerance: &Tolerances) -> bool {
        self.distance_to(other) <= tolerance.point_coincidence
    }

    /// Linear interpolation between `self` (t=0) and `other` (t=1). `t` is
    /// not clamped: values outside `[0, 1]` extrapolate, which callers such
    /// as arc/segment evaluation rely on intentionally.
    pub fn lerp(&self, other: Point2, t: f64) -> Point2 {
        Point2::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }
}

impl Vector2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(&self, other: Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// 2D "cross product": the scalar z-component of the 3D cross product of
    /// `(x, y, 0)` vectors. Positive when `other` is counter-clockwise from
    /// `self`.
    pub fn cross(&self, other: Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// A vector rotated 90 degrees counter-clockwise from `self`, i.e. a
    /// perpendicular direction (not normalized).
    pub fn perpendicular(&self) -> Vector2 {
        Vector2::new(-self.y, self.x)
    }

    /// Returns a unit-length vector in the same direction, or `None` if
    /// `self` is degenerate (zero or near-zero length under `tolerance`).
    pub fn normalized(&self, tolerance: &Tolerances) -> Option<Vector2> {
        let len = self.length();
        if len <= tolerance.point_coincidence {
            None
        } else {
            Some(Vector2::new(self.x / len, self.y / len))
        }
    }

    pub fn angle_radians(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn negated(&self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }

    pub fn scaled(&self, factor: f64) -> Vector2 {
        Vector2::new(self.x * factor, self.y * factor)
    }
}

impl std::ops::Add<Vector2> for Point2 {
    type Output = Point2;
    fn add(self, rhs: Vector2) -> Point2 {
        self.translated(rhs)
    }
}

impl std::ops::Sub<Point2> for Point2 {
    type Output = Vector2;
    fn sub(self, rhs: Point2) -> Vector2 {
        rhs.vector_to(self)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        self.negated()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_is_symmetric_and_nonnegative() {
        let a = Point2::new(1.0, 2.0);
        let b = Point2::new(4.0, 6.0);
        assert_eq!(a.distance_to(b), b.distance_to(a));
        assert!(a.distance_to(b) >= 0.0);
        assert_eq!(a.distance_to(b), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn coincidence_respects_the_supplied_tolerance_profile() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(0.01, 0.0);
        assert!(!a.is_coincident_with(b, &Tolerances::committed()));
        assert!(a.is_coincident_with(b, &Tolerances::recognition()));
    }

    #[test]
    fn normalized_rejects_a_degenerate_vector() {
        assert!(Vector2::ZERO.normalized(&Tolerances::committed()).is_none());
        let unit = Vector2::new(3.0, 4.0)
            .normalized(&Tolerances::committed())
            .unwrap();
        assert!((unit.length() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn perpendicular_vectors_have_zero_dot_product() {
        let v = Vector2::new(3.0, 7.0);
        let p = v.perpendicular();
        assert!(v.dot(p).abs() < 1e-12);
    }

    #[test]
    fn cross_product_sign_encodes_rotation_direction() {
        let x_axis = Vector2::new(1.0, 0.0);
        let y_axis = Vector2::new(0.0, 1.0);
        assert!(x_axis.cross(y_axis) > 0.0); // CCW
        assert!(y_axis.cross(x_axis) < 0.0); // CW
    }

    #[test]
    fn lerp_at_t_zero_and_one_returns_the_endpoints() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(10.0, 20.0);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 0.5), Point2::new(5.0, 10.0));
    }

    #[test]
    fn translate_and_inverse_translate_round_trips() {
        let a = Point2::new(2.0, 3.0);
        let offset = Vector2::new(5.0, -1.5);
        let moved = a.translated(offset);
        let back = moved.translated(offset.negated());
        assert!((back.x - a.x).abs() < 1e-12);
        assert!((back.y - a.y).abs() < 1e-12);
    }
}
