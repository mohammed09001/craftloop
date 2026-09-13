//! Page-space vs. engineering-space separation.
//!
//! Execution 01, Phase 07, Task 049. Authority: MCP Article 130
//! "Separation of Geometry and Presentation"; the specific requirement
//! this task states is "moving a view block on the page cannot change its
//! internal engineering dimensions."
//!
//! View blocks with their own local coordinate frame do not exist until
//! Phase 20 (Engine Contract 18). What this phase can and does establish
//! now is the *type-level* boundary Phase 20 must build inside: a page
//! placement is a wholly separate value from anything stored on a
//! `SemanticEntity`. `PageLayoutTransform` never appears as a field of
//! `Stroke`, `Note`, or `Beautified` -- so there is no code path by which
//! changing where something sits on the page could reach in and mutate its
//! own coordinates. That is the invariant, enforced by construction rather
//! than by a runtime check; [`PageLayoutTransform::apply`] demonstrates it
//! is a pure, external mapping.

use craftloop_geometry::{Point2, Vector2};
use serde::{Deserialize, Serialize};

/// Where a block of content sits on the page, independent of the
/// engineering coordinates of anything inside it. For Phase 07 (single
/// implicit page-space per page, no distinct view blocks yet) this exists
/// primarily to prove the separation exists in the type system; Phase 20
/// gives each view block one of these for real.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PageLayoutTransform {
    pub offset: Vector2,
}

impl PageLayoutTransform {
    pub const IDENTITY: PageLayoutTransform = PageLayoutTransform {
        offset: Vector2::ZERO,
    };

    pub fn translated(offset: Vector2) -> Self {
        Self { offset }
    }

    /// Map an engineering-space point to page space for rendering. This is
    /// a read-only projection: it takes `point` by value and returns a new
    /// one, and nothing about calling it can reach back and mutate the
    /// entity `point` came from.
    pub fn apply(&self, point: Point2) -> Point2 {
        point.translated(self.offset)
    }
}

impl Default for PageLayoutTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::{CraftLoopId, StrokeId};
    use craftloop_ink::Stroke;
    use craftloop_input::{InputCapabilities, PointerButtons, PointerSample, PointerSource};

    fn sample(x: f64) -> PointerSample {
        PointerSample::new(
            Point2::new(x, 0.0),
            0.0,
            None,
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    #[test]
    fn moving_the_page_layout_transform_never_touches_stored_entity_coordinates() {
        let stroke = Stroke::new(StrokeId::new(), vec![sample(1.0), sample(2.0)]).unwrap();
        let original_positions: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();

        // "Move the view block on the page" repeatedly: each reassignment
        // is a fresh, independent value with no path back to `stroke`.
        for offset in [
            craftloop_geometry::Vector2::new(500.0, -200.0),
            craftloop_geometry::Vector2::new(-30.0, 30.0),
        ] {
            let transform = PageLayoutTransform::translated(offset);
            // Exercise the transform (e.g. as rendering would) without it
            // ever touching `stroke`.
            let _ = transform.apply(Point2::ORIGIN);
        }

        // The stroke itself was never passed to anything that could mutate
        // it; its samples are exactly what they were before any of the
        // above.
        let unchanged_positions: Vec<Point2> =
            stroke.samples().iter().map(|s| s.position).collect();
        assert_eq!(original_positions, unchanged_positions);
    }

    #[test]
    fn apply_maps_engineering_points_to_page_space_without_mutating_the_input() {
        let transform =
            PageLayoutTransform::translated(craftloop_geometry::Vector2::new(10.0, 5.0));
        let engineering_point = Point2::new(1.0, 1.0);
        let page_point = transform.apply(engineering_point);
        assert_eq!(page_point, Point2::new(11.0, 6.0));
        // The original point is unaffected (Point2 is Copy; this asserts
        // the API shape is "produce a new point," not "mutate in place").
        assert_eq!(engineering_point, Point2::new(1.0, 1.0));
    }

    #[test]
    fn identity_transform_is_a_no_op() {
        let p = Point2::new(3.0, -4.0);
        assert_eq!(PageLayoutTransform::IDENTITY.apply(p), p);
        assert_eq!(
            PageLayoutTransform::default(),
            PageLayoutTransform::IDENTITY
        );
    }
}
