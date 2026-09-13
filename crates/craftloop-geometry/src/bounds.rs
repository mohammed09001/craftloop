//! Axis-aligned bounding box, shared by every primitive that reports bounds
//! (Task 014/015/016/017's "bounds" requirement).

use serde::{Deserialize, Serialize};

use crate::point::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bounds2 {
    pub min: Point2,
    pub max: Point2,
}

impl Bounds2 {
    /// Build bounds directly containing exactly `point`.
    pub fn from_point(point: Point2) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    /// Build bounds spanning a nonempty slice of points.
    pub fn from_points(points: &[Point2]) -> Option<Self> {
        let mut iter = points.iter();
        let first = *iter.next()?;
        let mut bounds = Self::from_point(first);
        for &p in iter {
            bounds = bounds.union_point(p);
        }
        Some(bounds)
    }

    pub fn union_point(&self, point: Point2) -> Self {
        Self {
            min: Point2::new(self.min.x.min(point.x), self.min.y.min(point.y)),
            max: Point2::new(self.max.x.max(point.x), self.max.y.max(point.y)),
        }
    }

    pub fn union(&self, other: Bounds2) -> Self {
        self.union_point(other.min).union_point(other.max)
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn contains_point(&self, point: Point2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn center(&self) -> Point2 {
        Point2::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_of_points_is_the_smallest_enclosing_box() {
        let points = [
            Point2::new(1.0, 5.0),
            Point2::new(-2.0, 3.0),
            Point2::new(4.0, -1.0),
        ];
        let bounds = Bounds2::from_points(&points).unwrap();
        assert_eq!(bounds.min, Point2::new(-2.0, -1.0));
        assert_eq!(bounds.max, Point2::new(4.0, 5.0));
    }

    #[test]
    fn empty_slice_has_no_bounds() {
        assert!(Bounds2::from_points(&[]).is_none());
    }

    #[test]
    fn width_height_and_contains_are_consistent() {
        let bounds = Bounds2 {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(10.0, 4.0),
        };
        assert_eq!(bounds.width(), 10.0);
        assert_eq!(bounds.height(), 4.0);
        assert!(bounds.contains_point(Point2::new(5.0, 2.0)));
        assert!(!bounds.contains_point(Point2::new(11.0, 2.0)));
    }
}
