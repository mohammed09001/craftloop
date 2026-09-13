//! Handwriting recognizer interface.
//!
//! Execution 01, Phase 17, Task 119. Authority: Engine Contract 05
//! (Handwriting/Text Recognition Adapter).
//!
//! Accepts ordered strokes -- exactly one candidate handwritten unit
//! (one word, one number, one symbol run) at a time, in drawing order --
//! and returns ranked text candidates with confidence. Nothing about the
//! signature references Apple's PencilKit/Vision APIs, Android's ML Kit,
//! or any other platform recognizer: an adapter for any of those
//! implements this trait and nothing upstream of it changes (Task 124's
//! platform plan).

use craftloop_ink::Stroke;
use craftloop_recognition::Confidence;
use serde::{Deserialize, Serialize};

/// One candidate reading of a stroke group, ranked by `confidence`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextCandidate {
    pub text: String,
    pub confidence: Confidence,
}

impl TextCandidate {
    pub fn new(text: impl Into<String>, confidence: Confidence) -> Self {
        Self {
            text: text.into(),
            confidence,
        }
    }
}

/// The platform-neutral seam. Every real implementation (a Windows test
/// stub, Task 120; a real Apple/Android adapter, Task 124) speaks only
/// this. Ranking convention matches
/// `craftloop_recognition::rank_candidates` (Phase 06): highest
/// confidence first; an empty result is a legitimate "no candidate at
/// all," not an error, mirroring that engine's own `KeepAsInk`-is-always-
/// an-option precedent (here, the caller falling back to `RawInk`,
/// Phase 16).
pub trait HandwritingRecognizer {
    fn recognize(&mut self, strokes: &[Stroke]) -> Vec<TextCandidate>;
}
