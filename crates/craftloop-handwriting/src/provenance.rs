//! Preserve raw handwriting provenance.
//!
//! Execution 01, Phase 17, Task 122. Authority: MCP Article 15 ("Raw Ink
//! and Structured Geometry Must Coexist"), applied here to handwriting the
//! same way Phase 06 already applies it to geometric recognition: the
//! original strokes must stay available until whatever semantic
//! transaction their recognized meaning feeds into is actually confirmed.
//!
//! `HandwritingRecognitionResult` structurally ties every returned
//! candidate back to the exact stroke IDs it came from -- a caller cannot
//! route recognized text without also being handed the provenance, and
//! nothing in this crate ever deletes, mutates, or consumes a `Stroke`
//! (every function in `recognizer.rs`/`stub.rs` takes strokes by shared
//! reference, never by value).

use craftloop_ids::StrokeId;
use serde::{Deserialize, Serialize};

use crate::recognizer::TextCandidate;

/// One recognition attempt's full, traceable result: which strokes fed
/// into it, and what came out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandwritingRecognitionResult {
    pub source_strokes: Vec<StrokeId>,
    pub candidates: Vec<TextCandidate>,
}

impl HandwritingRecognitionResult {
    pub fn new(source_strokes: Vec<StrokeId>, candidates: Vec<TextCandidate>) -> Self {
        Self {
            source_strokes,
            candidates,
        }
    }

    /// Task 122's core question, made directly checkable: does this
    /// result still name the exact strokes it came from? A caller that
    /// only ever constructs results through `HandwritingRecognizer` +
    /// this type cannot lose that association, but this makes the
    /// invariant explicit and testable rather than merely structural.
    pub fn is_traceable_to(&self, strokes: &[StrokeId]) -> bool {
        self.source_strokes == strokes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;
    use craftloop_recognition::Confidence;

    #[test]
    fn a_result_remains_traceable_to_its_exact_source_strokes() {
        let stroke_ids = vec![StrokeId::new(), StrokeId::new()];
        let result = HandwritingRecognitionResult::new(
            stroke_ids.clone(),
            vec![TextCandidate::new("R5", Confidence::new(0.9))],
        );
        assert!(result.is_traceable_to(&stroke_ids));
    }

    #[test]
    fn a_result_is_not_traceable_to_a_different_stroke_set() {
        let result = HandwritingRecognitionResult::new(
            vec![StrokeId::new()],
            vec![TextCandidate::new("R5", Confidence::new(0.9))],
        );
        assert!(!result.is_traceable_to(&[StrokeId::new()]));
    }

    #[test]
    fn serialization_round_trips_including_source_strokes() {
        let result = HandwritingRecognitionResult::new(
            vec![StrokeId::new()],
            vec![TextCandidate::new("45°", Confidence::new(0.7))],
        );
        let json = serde_json::to_string(&result).unwrap();
        let back: HandwritingRecognitionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, back);
    }
}
