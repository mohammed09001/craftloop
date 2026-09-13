//! Rejection memory at document scope.
//!
//! Execution 01, Phase 06, Task 046. Authority: Engine Contract 04
//! ("Recognition proposes; it does not silently commit engineering
//! truth"); MCP Article 97 "Human Correction as Signal".
//!
//! Records a *quantized* signature of each rejected interpretation, not the
//! exact fitted parameters: if the user redraws the stroke and the fit
//! changes meaningfully (new evidence), the same rectangle/circle must be
//! offered again rather than staying suppressed forever. Quantization
//! granularity is therefore the whole design: too coarse and a genuinely
//! different shape gets wrongly suppressed; too fine and jitter from
//! re-fitting the same ink re-triggers the same rejected suggestion.

use std::collections::{HashMap, HashSet};

use craftloop_ids::StrokeId;

use crate::candidate::RecognitionCandidate;

/// How finely to round a parameter before comparing signatures. Chosen to
/// be small relative to `Tolerances::recognition().point_coincidence`
/// scale but coarse enough to absorb ordinary re-fit jitter (a redrawn
/// stroke that is "the same shape, roughly" should stay suppressed; a
/// stroke extended or reshaped enough to move the fit by more than this
/// should not).
const QUANTIZATION_STEP: f64 = 0.5;

fn quantize(value: f64) -> i64 {
    (value / QUANTIZATION_STEP).round() as i64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CandidateSignature {
    Line {
        start: (i64, i64),
        end: (i64, i64),
    },
    Circle {
        center: (i64, i64),
        radius: i64,
    },
    Arc {
        center: (i64, i64),
        radius: i64,
        start_angle: i64,
        sweep_angle: i64,
    },
    Rectangle {
        min: (i64, i64),
        max: (i64, i64),
    },
}

impl CandidateSignature {
    fn of(candidate: &RecognitionCandidate) -> Option<Self> {
        match candidate {
            RecognitionCandidate::Line(c) => Some(Self::Line {
                start: (quantize(c.start.x), quantize(c.start.y)),
                end: (quantize(c.end.x), quantize(c.end.y)),
            }),
            RecognitionCandidate::Circle(c) => Some(Self::Circle {
                center: (quantize(c.center.x), quantize(c.center.y)),
                radius: quantize(c.radius),
            }),
            RecognitionCandidate::Arc(c) => Some(Self::Arc {
                center: (quantize(c.center.x), quantize(c.center.y)),
                radius: quantize(c.radius),
                start_angle: quantize(c.start_angle),
                sweep_angle: quantize(c.sweep_angle),
            }),
            RecognitionCandidate::Rectangle(c) => {
                let bounds = c.rectangle.bounds();
                Some(Self::Rectangle {
                    min: (quantize(bounds.min.x), quantize(bounds.min.y)),
                    max: (quantize(bounds.max.x), quantize(bounds.max.y)),
                })
            }
            RecognitionCandidate::KeepAsInk => None, // nothing to ever suppress
        }
    }
}

/// Per-document memory of rejected recognition candidates.
#[derive(Debug, Default)]
pub struct RejectionMemory {
    rejected: HashMap<StrokeId, HashSet<CandidateSignature>>,
}

impl RejectionMemory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that the user rejected `candidate` for `stroke_id`.
    /// `KeepAsInk` cannot be rejected (there is nothing more conservative
    /// to fall back to), so this is a no-op for it.
    pub fn reject(&mut self, stroke_id: StrokeId, candidate: &RecognitionCandidate) {
        if let Some(signature) = CandidateSignature::of(candidate) {
            self.rejected
                .entry(stroke_id)
                .or_default()
                .insert(signature);
        }
    }

    pub fn is_rejected(&self, stroke_id: StrokeId, candidate: &RecognitionCandidate) -> bool {
        let Some(signature) = CandidateSignature::of(candidate) else {
            return false;
        };
        self.rejected
            .get(&stroke_id)
            .is_some_and(|set| set.contains(&signature))
    }

    /// Remove every previously-rejected candidate for `stroke_id` from
    /// `candidates`, keeping their relative order.
    pub fn filter_candidates(
        &self,
        stroke_id: StrokeId,
        candidates: Vec<RecognitionCandidate>,
    ) -> Vec<RecognitionCandidate> {
        candidates
            .into_iter()
            .filter(|c| !self.is_rejected(stroke_id, c))
            .collect()
    }

    pub fn clear_stroke(&mut self, stroke_id: StrokeId) {
        self.rejected.remove(&stroke_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fit::fit_line;
    use craftloop_geometry::Point2;
    use craftloop_ids::CraftLoopId;

    fn line_candidate(dy_per_x: f64) -> RecognitionCandidate {
        let points: Vec<Point2> = (0..10)
            .map(|i| Point2::new(i as f64, dy_per_x * i as f64))
            .collect();
        RecognitionCandidate::Line(fit_line(&points).unwrap())
    }

    #[test]
    fn a_rejected_candidate_is_filtered_out_of_future_recognition_results() {
        let stroke_id = StrokeId::new();
        let mut memory = RejectionMemory::new();
        let candidate = line_candidate(2.0);
        memory.reject(stroke_id, &candidate);

        let candidates = vec![RecognitionCandidate::KeepAsInk, candidate];
        let filtered = memory.filter_candidates(stroke_id, candidates);
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], RecognitionCandidate::KeepAsInk));
    }

    #[test]
    fn keep_as_ink_can_never_be_suppressed() {
        let stroke_id = StrokeId::new();
        let mut memory = RejectionMemory::new();
        memory.reject(stroke_id, &RecognitionCandidate::KeepAsInk);
        assert!(!memory.is_rejected(stroke_id, &RecognitionCandidate::KeepAsInk));
    }

    #[test]
    fn rejection_is_scoped_per_stroke() {
        let a = StrokeId::new();
        let b = StrokeId::new();
        let mut memory = RejectionMemory::new();
        let candidate = line_candidate(2.0);
        memory.reject(a, &candidate);
        assert!(memory.is_rejected(a, &candidate));
        assert!(!memory.is_rejected(b, &candidate));
    }

    #[test]
    fn a_materially_different_refit_after_new_evidence_is_not_suppressed() {
        let stroke_id = StrokeId::new();
        let mut memory = RejectionMemory::new();
        memory.reject(stroke_id, &line_candidate(2.0));

        // A very different slope is new evidence -- not the same
        // interpretation the user rejected.
        assert!(!memory.is_rejected(stroke_id, &line_candidate(0.1)));
    }

    #[test]
    fn tiny_refit_jitter_of_the_same_shape_stays_suppressed() {
        let stroke_id = StrokeId::new();
        let mut memory = RejectionMemory::new();
        memory.reject(stroke_id, &line_candidate(2.0));

        // Recompute the identical fit -- must still match (this is the
        // realistic "same stroke re-recognized" case).
        assert!(memory.is_rejected(stroke_id, &line_candidate(2.0)));
    }

    #[test]
    fn clear_stroke_forgets_all_rejections_for_that_stroke() {
        let stroke_id = StrokeId::new();
        let mut memory = RejectionMemory::new();
        let candidate = line_candidate(2.0);
        memory.reject(stroke_id, &candidate);
        memory.clear_stroke(stroke_id);
        assert!(!memory.is_rejected(stroke_id, &candidate));
    }
}
