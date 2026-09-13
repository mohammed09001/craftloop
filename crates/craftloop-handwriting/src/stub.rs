//! Windows test stub.
//!
//! Execution 01, Phase 17, Task 120. Authority: Engine Contract 05.
//!
//! No real handwriting recognizer exists on this Windows development
//! workstation (there is no production Apple/Android recognizer to call
//! into here, and building one from scratch is explicitly out of this
//! execution's scope -- see the No-Hallucination Contract). This stub
//! lets the dimension-routing (Task 121) and future command-routing
//! engines be tested end-to-end on Windows using **pre-registered,
//! deterministic** results, never a claim of real recognition accuracy.
//! Every test built on this stub must be read as "given this recognized
//! text, does the rest of the pipeline behave correctly," never as
//! "handwriting recognition works."

use std::collections::BTreeMap;

use craftloop_ids::StrokeId;
use craftloop_ink::Stroke;

use crate::recognizer::{HandwritingRecognizer, TextCandidate};

/// Keyed by the exact ordered sequence of stroke IDs a fixture was
/// registered for -- deliberately exact-match, not fuzzy, so a test's
/// expected input is unambiguous and the stub can never "helpfully"
/// guess at an unregistered stroke sequence.
#[derive(Debug, Default)]
pub struct FixtureHandwritingRecognizer {
    fixtures: BTreeMap<Vec<StrokeId>, Vec<TextCandidate>>,
}

impl FixtureHandwritingRecognizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register the candidates this stub should return for exactly this
    /// stroke sequence.
    pub fn with_fixture(mut self, strokes: &[Stroke], candidates: Vec<TextCandidate>) -> Self {
        let key = strokes.iter().map(|s| s.id).collect();
        self.fixtures.insert(key, candidates);
        self
    }
}

impl HandwritingRecognizer for FixtureHandwritingRecognizer {
    fn recognize(&mut self, strokes: &[Stroke]) -> Vec<TextCandidate> {
        let key: Vec<StrokeId> = strokes.iter().map(|s| s.id).collect();
        // No fixture registered: an honest empty result, never a
        // fabricated guess.
        self.fixtures.get(&key).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_ids::CraftLoopId;
    use craftloop_input::MouseSimulator;
    use craftloop_recognition::Confidence;

    fn stroke() -> Stroke {
        let samples = vec![
            MouseSimulator::sample(Point2::new(0.0, 0.0), 0.0, Default::default()),
            MouseSimulator::sample(Point2::new(1.0, 1.0), 0.1, Default::default()),
        ];
        Stroke::new(StrokeId::new(), samples).unwrap()
    }

    #[test]
    fn a_registered_fixture_is_returned_exactly() {
        let s = stroke();
        let candidates = vec![TextCandidate::new("R5", Confidence::new(0.9))];
        let mut recognizer = FixtureHandwritingRecognizer::new()
            .with_fixture(std::slice::from_ref(&s), candidates.clone());
        assert_eq!(recognizer.recognize(&[s]), candidates);
    }

    #[test]
    fn an_unregistered_stroke_sequence_returns_an_empty_result_not_a_guess() {
        let mut recognizer = FixtureHandwritingRecognizer::new();
        assert!(recognizer.recognize(&[stroke()]).is_empty());
    }

    #[test]
    fn different_stroke_sequences_never_share_a_fixture() {
        let a = stroke();
        let b = stroke();
        let mut recognizer = FixtureHandwritingRecognizer::new().with_fixture(
            std::slice::from_ref(&a),
            vec![TextCandidate::new("R5", Confidence::new(0.9))],
        );
        assert!(recognizer.recognize(&[b]).is_empty());
        assert!(!recognizer.recognize(&[a]).is_empty());
    }
}
