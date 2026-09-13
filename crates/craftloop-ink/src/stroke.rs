//! Raw stroke storage.
//!
//! Execution 01, Phase 05, Task 034. Authority: Engine Contract 02 (Raw
//! Ink: "Ordered samples, stroke identity, grouping"); MCP Article 584
//! "Requirement Group: Ink Preservation".
//!
//! A [`Stroke`] is the ordered pointer-sample record of one physical
//! contact-to-release gesture. It preserves the samples exactly as
//! normalized by `craftloop-input` -- Article 15's "Raw Ink and Structured
//! Geometry Must Coexist" means this type must never be mutated into
//! structured geometry in place; recognition (Phase 06) produces a
//! *separate* candidate, and the raw stroke survives regardless of whether
//! that candidate is accepted.

use craftloop_errors::{DomainError, DomainResult, InkErrorKind};
use craftloop_ids::StrokeId;
use craftloop_input::{PointerEvent, PointerSample, PointerSource};
use serde::{Deserialize, Serialize};

use craftloop_geometry::Bounds2;

/// One raw ink stroke: a stable identity plus its ordered pointer samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stroke {
    pub id: StrokeId,
    samples: Vec<PointerSample>,
}

impl Stroke {
    /// Construct a stroke directly from samples. Rejects an empty stroke
    /// (Task 034 requires "ordered pointer samples" -- zero is not an
    /// order) and samples that disagree about their `PointerSource` (a
    /// stroke is one physical gesture from one input device; a mix would
    /// mean upstream normalization already lost information).
    pub fn new(id: StrokeId, samples: Vec<PointerSample>) -> DomainResult<Self> {
        let Some(first) = samples.first() else {
            return Err(DomainError::Ink {
                kind: InkErrorKind::EmptyStroke,
                detail: "a stroke must have at least one pointer sample".to_string(),
            });
        };
        let expected_source = first.source;
        if samples.iter().any(|s| s.source != expected_source) {
            return Err(DomainError::Ink {
                kind: InkErrorKind::MixedSource,
                detail: "all samples in a stroke must share the same PointerSource".to_string(),
            });
        }
        Ok(Self { id, samples })
    }

    /// Build a stroke from one completed lifecycle's events (as produced by
    /// `craftloop_input::StrokeLifecycleValidator`): every event carrying a
    /// sample contributes it, in order; `CaptureLost` (which carries no
    /// sample) is skipped rather than terminating construction, since by
    /// the time this is called the caller already knows the stroke ended.
    pub fn from_events(id: StrokeId, events: &[PointerEvent]) -> DomainResult<Self> {
        let samples: Vec<PointerSample> =
            events.iter().filter_map(|e| e.sample()).copied().collect();
        Self::new(id, samples)
    }

    pub fn samples(&self) -> &[PointerSample] {
        &self.samples
    }

    pub fn source(&self) -> PointerSource {
        self.samples[0].source
    }

    pub fn start_timestamp_seconds(&self) -> f64 {
        self.samples[0].timestamp_seconds
    }

    pub fn end_timestamp_seconds(&self) -> f64 {
        self.samples[self.samples.len() - 1].timestamp_seconds
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Always `false`: `Stroke::new`/`from_events` reject an empty sample
    /// list, so a constructed `Stroke` never has zero samples. Provided
    /// only to satisfy the `len`/`is_empty` convention clippy expects.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn is_single_point(&self) -> bool {
        self.samples.len() == 1
    }

    pub fn bounds(&self) -> Bounds2 {
        let points: Vec<_> = self.samples.iter().map(|s| s.position).collect();
        Bounds2::from_points(&points).expect("Stroke::new guarantees at least one sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_ids::CraftLoopId;
    use craftloop_input::{InputCapabilities, PointerButtons};

    fn sample(x: f64, t: f64, source: PointerSource) -> PointerSample {
        PointerSample::new(
            Point2::new(x, 0.0),
            t,
            None,
            None,
            None,
            source,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    #[test]
    fn empty_samples_are_rejected() {
        let result = Stroke::new(StrokeId::new(), vec![]);
        assert!(matches!(
            result,
            Err(DomainError::Ink {
                kind: InkErrorKind::EmptyStroke,
                ..
            })
        ));
    }

    #[test]
    fn mixed_source_samples_are_rejected() {
        let result = Stroke::new(
            StrokeId::new(),
            vec![
                sample(0.0, 0.0, PointerSource::SimulatedMouse),
                sample(1.0, 0.1, PointerSource::Stylus),
            ],
        );
        assert!(matches!(
            result,
            Err(DomainError::Ink {
                kind: InkErrorKind::MixedSource,
                ..
            })
        ));
    }

    #[test]
    fn a_valid_stroke_reports_source_and_timestamps() {
        let stroke = Stroke::new(
            StrokeId::new(),
            vec![
                sample(0.0, 1.0, PointerSource::SimulatedMouse),
                sample(1.0, 1.5, PointerSource::SimulatedMouse),
                sample(2.0, 2.0, PointerSource::SimulatedMouse),
            ],
        )
        .unwrap();
        assert_eq!(stroke.source(), PointerSource::SimulatedMouse);
        assert_eq!(stroke.start_timestamp_seconds(), 1.0);
        assert_eq!(stroke.end_timestamp_seconds(), 2.0);
        assert_eq!(stroke.len(), 3);
        assert!(!stroke.is_single_point());
    }

    #[test]
    fn a_single_sample_stroke_is_valid_and_flagged_as_single_point() {
        let stroke = Stroke::new(
            StrokeId::new(),
            vec![sample(0.0, 0.0, PointerSource::Stylus)],
        )
        .unwrap();
        assert!(stroke.is_single_point());
    }

    #[test]
    fn from_events_extracts_samples_in_order_and_skips_capture_lost() {
        let id = StrokeId::new();
        let events = vec![
            PointerEvent::Down(sample(0.0, 0.0, PointerSource::SimulatedMouse)),
            PointerEvent::Move(sample(1.0, 0.1, PointerSource::SimulatedMouse)),
            PointerEvent::CaptureLost,
        ];
        // CaptureLost carries no sample, but the two real samples still
        // form a valid (if abruptly ended) stroke.
        let stroke = Stroke::from_events(id, &events).unwrap();
        assert_eq!(stroke.len(), 2);
        assert_eq!(stroke.id, id);
    }

    #[test]
    fn bounds_spans_all_sample_positions() {
        let stroke = Stroke::new(
            StrokeId::new(),
            vec![
                sample(-3.0, 0.0, PointerSource::SimulatedMouse),
                sample(7.0, 0.1, PointerSource::SimulatedMouse),
            ],
        )
        .unwrap();
        let bounds = stroke.bounds();
        assert_eq!(bounds.min.x, -3.0);
        assert_eq!(bounds.max.x, 7.0);
    }

    #[test]
    fn serialization_round_trips() {
        let stroke = Stroke::new(
            StrokeId::new(),
            vec![sample(0.0, 0.0, PointerSource::Stylus)],
        )
        .unwrap();
        let json = serde_json::to_string(&stroke).unwrap();
        let back: Stroke = serde_json::from_str(&json).unwrap();
        assert_eq!(stroke, back);
    }
}
