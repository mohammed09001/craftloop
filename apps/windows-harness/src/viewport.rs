//! Canvas viewport: pan, zoom, fit, and screen/world coordinate mapping.
//!
//! Execution 01, Phase 04, Task 028. Authority: Engine Contract 27 (Windows
//! Harness: "test adapter, never production authority"); MCP Article 246
//! "Detailed Specification of Zoom and Pan".
//!
//! Pure math, no `egui` dependency, so it is unit-testable without a
//! windowing system and so the harness's own module-boundary rule (Task
//! 033) has something concrete to point at: this is exactly the kind of
//! logic that must not silently grow a dependency on `egui` types.

use craftloop_geometry::{Bounds2, Point2, Vector2};

/// Maps between world (document/engineering) coordinates and screen
/// (pixel) coordinates. `screen = (world + pan) * zoom`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pan: Vector2,
    zoom: f64,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            pan: Vector2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn pan(&self) -> Vector2 {
        self.pan
    }

    pub fn world_to_screen(&self, world: Point2) -> Point2 {
        Point2::new(
            (world.x + self.pan.x) * self.zoom,
            (world.y + self.pan.y) * self.zoom,
        )
    }

    pub fn screen_to_world(&self, screen: Point2) -> Point2 {
        Point2::new(
            screen.x / self.zoom - self.pan.x,
            screen.y / self.zoom - self.pan.y,
        )
    }

    /// Pan by a screen-space pixel delta (e.g. from a mouse drag).
    pub fn pan_by_screen_delta(&mut self, screen_delta: Vector2) {
        self.pan = self.pan + screen_delta.scaled(1.0 / self.zoom);
    }

    /// Multiply zoom by `factor`, keeping the world point currently under
    /// `screen_anchor` fixed on screen (the standard "zoom toward cursor"
    /// behavior).
    pub fn zoom_at(&mut self, screen_anchor: Point2, factor: f64) {
        let world_anchor = self.screen_to_world(screen_anchor);
        self.zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
        // Recompute pan so world_anchor still maps to screen_anchor at the
        // new zoom level.
        self.pan = Vector2::new(
            screen_anchor.x / self.zoom - world_anchor.x,
            screen_anchor.y / self.zoom - world_anchor.y,
        );
    }

    /// Set pan/zoom so `bounds` is fully visible within a
    /// `viewport_size`-pixel screen, with `margin_fraction` of the smaller
    /// screen dimension left as empty border on each side.
    pub fn fit_bounds(&mut self, bounds: Bounds2, viewport_size: (f64, f64), margin_fraction: f64) {
        let (screen_w, screen_h) = viewport_size;
        let usable_w = screen_w * (1.0 - 2.0 * margin_fraction);
        let usable_h = screen_h * (1.0 - 2.0 * margin_fraction);

        let world_w = bounds.width().max(1e-9);
        let world_h = bounds.height().max(1e-9);

        self.zoom = (usable_w / world_w)
            .min(usable_h / world_h)
            .clamp(MIN_ZOOM, MAX_ZOOM);

        let center = bounds.center();
        let screen_center = Point2::new(screen_w / 2.0, screen_h / 2.0);
        self.pan = Vector2::new(
            screen_center.x / self.zoom - center.x,
            screen_center.y / self.zoom - center.y,
        );
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

const MIN_ZOOM: f64 = 0.01;
const MAX_ZOOM: f64 = 100.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_viewport_maps_world_to_screen_identically() {
        let v = Viewport::new();
        let p = Point2::new(3.0, 4.0);
        assert_eq!(v.world_to_screen(p), p);
    }

    #[test]
    fn screen_to_world_inverts_world_to_screen() {
        let mut v = Viewport::new();
        v.pan_by_screen_delta(Vector2::new(50.0, -20.0));
        v.zoom_at(Point2::new(100.0, 100.0), 2.5);

        let world = Point2::new(17.0, -42.0);
        let screen = v.world_to_screen(world);
        let back = v.screen_to_world(screen);
        assert!((back.x - world.x).abs() < 1e-9);
        assert!((back.y - world.y).abs() < 1e-9);
    }

    #[test]
    fn zoom_at_keeps_the_anchor_point_fixed_on_screen() {
        let mut v = Viewport::new();
        let anchor = Point2::new(200.0, 150.0);
        let world_before = v.screen_to_world(anchor);
        v.zoom_at(anchor, 3.0);
        let screen_after = v.world_to_screen(world_before);
        assert!((screen_after.x - anchor.x).abs() < 1e-9);
        assert!((screen_after.y - anchor.y).abs() < 1e-9);
    }

    #[test]
    fn zoom_is_clamped_to_a_sane_range() {
        let mut v = Viewport::new();
        v.zoom_at(Point2::ORIGIN, 1e9);
        assert!(v.zoom() <= MAX_ZOOM);
        v.zoom_at(Point2::ORIGIN, 1e-9);
        assert!(v.zoom() >= MIN_ZOOM);
    }

    #[test]
    fn fit_bounds_centers_the_bounds_on_screen() {
        let mut v = Viewport::new();
        let bounds = Bounds2 {
            min: Point2::new(-5.0, -5.0),
            max: Point2::new(5.0, 5.0),
        };
        v.fit_bounds(bounds, (800.0, 600.0), 0.1);
        let screen_center = v.world_to_screen(bounds.center());
        assert!((screen_center.x - 400.0).abs() < 1e-6);
        assert!((screen_center.y - 300.0).abs() < 1e-6);
    }

    #[test]
    fn fit_bounds_makes_the_whole_bounds_visible_within_the_usable_area() {
        let mut v = Viewport::new();
        let bounds = Bounds2 {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(100.0, 20.0),
        };
        v.fit_bounds(bounds, (800.0, 600.0), 0.05);
        let min_screen = v.world_to_screen(bounds.min);
        let max_screen = v.world_to_screen(bounds.max);
        assert!(min_screen.x >= 0.0 && max_screen.x <= 800.0);
        assert!(min_screen.y >= 0.0 && max_screen.y <= 600.0);
    }

    #[test]
    fn reset_returns_to_identity_transform() {
        let mut v = Viewport::new();
        v.pan_by_screen_delta(Vector2::new(100.0, 100.0));
        v.zoom_at(Point2::ORIGIN, 5.0);
        v.reset();
        assert_eq!(v, Viewport::new());
    }
}
