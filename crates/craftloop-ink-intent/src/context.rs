//! Deterministic context features.
//!
//! Execution 01, Phase 16, Task 114. Authority: Engine Contract 04
//! ("Use current tool/context, location, nearby geometry, timing,
//! selection, and active guides before introducing machine learning").
//!
//! Every field here is computed from data this workspace already has --
//! no model, no fuzzy matching, nothing that could silently change
//! behavior between runs on the same input.

use craftloop_geometry::Bounds2;
use craftloop_ink::Stroke;
use serde::{Deserialize, Serialize};

/// Which tool was active while the stroke was drawn. Deliberately small
/// and adapter-agnostic: a platform harness (e.g. the Windows test
/// harness) maps its own richer tool palette down to this before calling
/// into this crate, so this crate stays free of any platform UI
/// dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolContext {
    /// Ordinary ink/pen input -- the default assumption.
    Pen,
    Eraser,
    Selector,
    /// The active tool is not known to this layer (e.g. a platform that
    /// has not wired tool state through yet). Never treated as
    /// equivalent to `Pen` -- see `ContextFeatures::compute`'s doc
    /// comment on why guessing here would be a No-Hallucination Contract
    /// violation.
    Unknown,
}

/// The deterministic signals Task 114 names, computed once per stroke
/// and handed to the classifier (Task 115) -- never recomputed
/// differently by different call sites.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ContextFeatures {
    pub tool: ToolContext,
    pub bounds: Bounds2,
    /// How many existing entities' bounds this stroke's bounds intersect
    /// or come within a caller-supplied proximity of. A plain count, not
    /// identities -- this crate does not need to know entity identity to
    /// classify intent, only that geometry is nearby.
    pub nearby_entity_count: usize,
    pub duration_seconds: f64,
    pub selection_active: bool,
    /// Is this stroke positioned where a view-labeling guide would be
    /// (e.g. beneath a view block, Article 30)? Supplied by the caller,
    /// which owns document layout -- this crate has no concept of pages
    /// or view blocks itself.
    pub near_labeling_guide: bool,
    /// Path length divided by the bounding-box diagonal. A closed,
    /// deliberate shape (a drawn circle, a rectangle) stays close to a
    /// small multiple of this ratio; a scribbled-over erase gesture is
    /// usually far higher (many back-and-forth passes inside a small
    /// area). Purely geometric, computed once here so every classifier
    /// (deterministic today, a future ML one, Task 118) reads the same
    /// number.
    pub path_length_to_diagonal_ratio: f64,
}

impl ContextFeatures {
    /// Compute every feature from real, already-available data. Never
    /// infers a tool: an `Unknown` `tool` stays `Unknown` in the output,
    /// exactly the raw input -- guessing "probably Pen" here would be
    /// exactly the fabricated certainty the No-Hallucination Contract
    /// forbids, and would make an erase gesture on an unknown-tool
    /// platform silently misclassify as geometry.
    pub fn compute(
        stroke: &Stroke,
        tool: ToolContext,
        nearby_entity_count: usize,
        selection_active: bool,
        near_labeling_guide: bool,
    ) -> Self {
        let bounds = stroke.bounds();
        let duration_seconds = stroke.end_timestamp_seconds() - stroke.start_timestamp_seconds();
        let diagonal = (bounds.width().powi(2) + bounds.height().powi(2)).sqrt();
        let path_length: f64 = stroke
            .samples()
            .windows(2)
            .map(|pair| pair[0].position.distance_to(pair[1].position))
            .sum();
        let path_length_to_diagonal_ratio = if diagonal > 0.0 {
            path_length / diagonal
        } else {
            0.0
        };

        Self {
            tool,
            bounds,
            nearby_entity_count,
            duration_seconds,
            selection_active,
            near_labeling_guide,
            path_length_to_diagonal_ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, StrokeId};
    use craftloop_input::MouseSimulator;

    fn stroke_from(points: &[(f64, f64, f64)]) -> Stroke {
        let samples = points
            .iter()
            .map(|(x, y, t)| MouseSimulator::sample(Point2::new(*x, *y), *t, Default::default()))
            .collect();
        Stroke::new(StrokeId::new(), samples).unwrap()
    }

    #[test]
    fn a_straight_short_stroke_has_a_low_path_to_diagonal_ratio() {
        let stroke = stroke_from(&[(0.0, 0.0, 0.0), (10.0, 0.0, 0.1)]);
        let features = ContextFeatures::compute(&stroke, ToolContext::Pen, 0, false, false);
        assert!((features.path_length_to_diagonal_ratio - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_scribble_confined_to_a_small_area_has_a_high_ratio() {
        // Back and forth many times within a tiny bounding box.
        let mut points = Vec::new();
        for i in 0..20 {
            let x = if i % 2 == 0 { 0.0 } else { 1.0 };
            points.push((x, 0.0, i as f64 * 0.01));
        }
        let stroke = stroke_from(&points);
        let features = ContextFeatures::compute(&stroke, ToolContext::Eraser, 1, false, false);
        assert!(features.path_length_to_diagonal_ratio > 10.0);
    }

    #[test]
    fn duration_reflects_the_strokes_own_timestamps() {
        let stroke = stroke_from(&[(0.0, 0.0, 1.0), (1.0, 0.0, 1.5)]);
        let features = ContextFeatures::compute(&stroke, ToolContext::Pen, 0, false, false);
        assert!((features.duration_seconds - 0.5).abs() < 1e-9);
    }

    #[test]
    fn unknown_tool_context_is_never_silently_upgraded_to_pen() {
        let stroke = stroke_from(&[(0.0, 0.0, 0.0), (1.0, 0.0, 0.1)]);
        let features = ContextFeatures::compute(&stroke, ToolContext::Unknown, 0, false, false);
        assert_eq!(features.tool, ToolContext::Unknown);
    }
}
