//! Stroke bounds and spatial index hooks.
//!
//! Execution 01, Phase 05, Task 037. Authority: Engine Contract 02; used by
//! selection and, later, ink intent classification (Phase 16).
//!
//! `StrokeSpatialIndex` is a deliberately simple O(n) bounding-box scan,
//! not a spatial tree. Task 037 asks for "efficient query interfaces" as a
//! *hook* -- the interface strokes/selection code should call through, so
//! the backing implementation can be swapped for an R-tree or grid once a
//! real performance budget (Article 91) and profiling data justify it.
//! Building that structure now, with no measured need, would be exactly
//! the kind of speculative scope the Agent Operating Directive forbids.

use craftloop_geometry::{Bounds2, Point2};
use craftloop_ids::StrokeId;

use crate::stroke::Stroke;

#[derive(Debug, Clone, Copy)]
struct Entry {
    id: StrokeId,
    bounds: Bounds2,
}

/// A queryable index over a set of strokes' bounding boxes.
#[derive(Debug, Default)]
pub struct StrokeSpatialIndex {
    entries: Vec<Entry>,
}

impl StrokeSpatialIndex {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn build(strokes: &[Stroke]) -> Self {
        let mut index = Self::new();
        for stroke in strokes {
            index.insert(stroke.id, stroke.bounds());
        }
        index
    }

    pub fn insert(&mut self, id: StrokeId, bounds: Bounds2) {
        self.entries.push(Entry { id, bounds });
    }

    pub fn remove(&mut self, id: StrokeId) {
        self.entries.retain(|e| e.id != id);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Strokes whose bounding box contains `point`, expanded by `radius` in
    /// each direction (a simple proxy for "close enough to hit-test", ahead
    /// of a real per-stroke distance check that recognition/selection can
    /// layer on top).
    pub fn query_point(&self, point: Point2, radius: f64) -> Vec<StrokeId> {
        self.entries
            .iter()
            .filter(|e| {
                let expanded = Bounds2 {
                    min: Point2::new(e.bounds.min.x - radius, e.bounds.min.y - radius),
                    max: Point2::new(e.bounds.max.x + radius, e.bounds.max.y + radius),
                };
                expanded.contains_point(point)
            })
            .map(|e| e.id)
            .collect()
    }

    /// Strokes whose bounding box intersects `query_bounds` (touching at an
    /// edge counts as intersecting).
    pub fn query_bounds(&self, query_bounds: Bounds2) -> Vec<StrokeId> {
        self.entries
            .iter()
            .filter(|e| bounds_intersect(e.bounds, query_bounds))
            .map(|e| e.id)
            .collect()
    }
}

fn bounds_intersect(a: Bounds2, b: Bounds2) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x && a.min.y <= b.max.y && a.max.y >= b.min.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;
    use craftloop_input::{InputCapabilities, PointerButtons, PointerSample, PointerSource};

    fn stroke_at(x: f64, y: f64) -> Stroke {
        let sample = PointerSample::new(
            Point2::new(x, y),
            0.0,
            None,
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap();
        Stroke::new(StrokeId::new(), vec![sample]).unwrap()
    }

    #[test]
    fn query_point_finds_a_stroke_under_the_point() {
        let stroke = stroke_at(5.0, 5.0);
        let id = stroke.id;
        let index = StrokeSpatialIndex::build(&[stroke]);
        let hits = index.query_point(Point2::new(5.0, 5.0), 0.0);
        assert_eq!(hits, vec![id]);
    }

    #[test]
    fn query_point_respects_radius() {
        let stroke = stroke_at(0.0, 0.0);
        let index = StrokeSpatialIndex::build(&[stroke]);
        assert!(index.query_point(Point2::new(10.0, 0.0), 1.0).is_empty());
        assert!(!index.query_point(Point2::new(10.0, 0.0), 20.0).is_empty());
    }

    #[test]
    fn query_bounds_finds_intersecting_strokes_only() {
        let inside = stroke_at(1.0, 1.0);
        let outside = stroke_at(100.0, 100.0);
        let inside_id = inside.id;
        let index = StrokeSpatialIndex::build(&[inside, outside]);

        let query = Bounds2 {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(2.0, 2.0),
        };
        assert_eq!(index.query_bounds(query), vec![inside_id]);
    }

    #[test]
    fn remove_takes_a_stroke_out_of_future_queries() {
        let stroke = stroke_at(0.0, 0.0);
        let id = stroke.id;
        let mut index = StrokeSpatialIndex::build(&[stroke]);
        assert_eq!(index.len(), 1);
        index.remove(id);
        assert!(index.is_empty());
        assert!(index.query_point(Point2::new(0.0, 0.0), 1.0).is_empty());
    }

    #[test]
    fn empty_index_returns_no_hits() {
        let index = StrokeSpatialIndex::new();
        assert!(index.query_point(Point2::ORIGIN, 1000.0).is_empty());
    }
}
