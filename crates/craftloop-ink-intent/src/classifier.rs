//! Confidence/result contract, conservative thresholds, and the
//! deterministic baseline classifier.
//!
//! Execution 01, Phase 16, Tasks 115, 116, 118. Authority: Engine
//! Contract 04; MCP Article 22 ("For high confidence, the system may
//! apply the value... For medium confidence... a minimal confirmation.
//! For low confidence, the ink should remain ordinary handwriting.").
//!
//! `IntentClassifier` (Task 118) is the seam a future learned classifier
//! plugs into -- mirroring `craftloop_constraint::ConstraintSolver`'s own
//! "define the interface, ship a real non-ML reference implementation,
//! choose a smarter backend later" pattern (Phase 11).
//! `DeterministicBaselineClassifier` is that reference implementation:
//! real, tested logic using only the features `context.rs` computes, not
//! a placeholder. It deliberately never claims `Text`/`Command`/
//! `NumericDimensionCandidate`/`Annotation` at commit-worthy confidence,
//! because this workspace has no actual handwriting-content reader yet
//! (that is Phase 17's "Engineering Handwriting Adapter Boundary," the
//! very next phase) -- claiming to read words this baseline cannot
//! actually read would be exactly the fabricated-certainty the
//! No-Hallucination Contract forbids.

use serde::{Deserialize, Serialize};

use craftloop_ink::Stroke;
use craftloop_recognition::Confidence;

use crate::category::IntentCategory;
use crate::context::{ContextFeatures, ToolContext};

/// Task 116: matches `craftloop-recognition`'s own established convention
/// (`false_positive_benchmark.rs`'s `COMMIT_THRESHOLD = 0.6`, Phase 06) --
/// the same confidence scale means the same thing everywhere in this
/// workspace, per Article 96.
pub const COMMIT_THRESHOLD: f64 = 0.6;
/// Below `COMMIT_THRESHOLD` but at or above this: worth a minimal
/// confirmation (Article 22's middle tier), not an unprompted apply.
pub const CONFIRM_THRESHOLD: f64 = 0.3;

/// A high, unusual scribble ratio (path length vs. bounding-box diagonal)
/// that, on its own (no explicit Eraser tool), is still suggestive enough
/// of an erase gesture to raise a confirm-tier candidate. Chosen
/// generously above the ratio an ordinary drawn loop reaches (a circle's
/// own ratio is `pi`, about 3.14) so a single clean shape is never
/// mistaken for a scribble.
const SCRIBBLE_RATIO_THRESHOLD: f64 = 8.0;

/// Task 115: one ranked candidate. `RawInk` is always a legitimate
/// member of the returned list, exactly like
/// `craftloop_recognition::RecognitionCandidate::KeepAsInk` (Phase 06).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IntentCandidate {
    pub category: IntentCategory,
    pub confidence: Confidence,
}

impl IntentCandidate {
    pub fn is_committable(&self) -> bool {
        self.confidence.get() >= COMMIT_THRESHOLD
    }

    pub fn needs_confirmation(&self) -> bool {
        let value = self.confidence.get();
        (CONFIRM_THRESHOLD..COMMIT_THRESHOLD).contains(&value)
    }
}

/// Task 118: the seam a future learned classifier implements. Current
/// domain semantics (`IntentCategory`, `ContextFeatures`,
/// `IntentCandidate`) do not depend on any particular implementation of
/// this trait, deterministic or learned.
pub trait IntentClassifier {
    /// `geometry_confidence` is `craftloop-recognition`'s own best
    /// confidence for this stroke (Phase 06), already computed by
    /// whatever recognition pass ran -- this trait does not re-run
    /// geometric fitting itself, only decides what the fit result means
    /// in context.
    fn classify(
        &self,
        stroke: &Stroke,
        features: &ContextFeatures,
        geometry_confidence: Option<Confidence>,
    ) -> Vec<IntentCandidate>;
}

/// The real, non-learned reference implementation (Tasks 113-117).
#[derive(Debug, Default)]
pub struct DeterministicBaselineClassifier;

impl IntentClassifier for DeterministicBaselineClassifier {
    fn classify(
        &self,
        _stroke: &Stroke,
        features: &ContextFeatures,
        geometry_confidence: Option<Confidence>,
    ) -> Vec<IntentCandidate> {
        // Task 116: RawInk is the conservative floor every stroke starts
        // from. Its confidence sits below the commit threshold on
        // purpose -- real, stronger evidence for another category can
        // outrank it, but nothing here defaults to committing anything.
        let mut candidates = vec![IntentCandidate {
            category: IntentCategory::RawInk,
            confidence: Confidence::new(0.5),
        }];

        // GeometryCandidate: trust Phase 06's own fit confidence
        // directly, but only when the active tool doesn't already say
        // otherwise (an Eraser/Selector stroke is not a geometry
        // proposal no matter how clean its shape is -- this is exactly
        // how "letter O vs. circle" and "circle used for selection" are
        // resolved: by tool context, Task 114's first-listed feature,
        // not by shape analysis alone).
        if let Some(geometry_confidence) = geometry_confidence {
            if matches!(features.tool, ToolContext::Pen | ToolContext::Unknown) {
                candidates.push(IntentCandidate {
                    category: IntentCategory::GeometryCandidate,
                    confidence: geometry_confidence,
                });
            }
        }

        if matches!(features.tool, ToolContext::Selector) {
            candidates.push(IntentCandidate {
                category: IntentCategory::SelectionGesture,
                confidence: Confidence::new(0.9),
            });
        }

        if matches!(features.tool, ToolContext::Eraser) {
            candidates.push(IntentCandidate {
                category: IntentCategory::EraseGesture,
                confidence: Confidence::new(0.9),
            });
        } else if features.nearby_entity_count > 0
            && features.path_length_to_diagonal_ratio > SCRIBBLE_RATIO_THRESHOLD
        {
            // No explicit Eraser tool, but the shape itself (many
            // back-and-forth passes over existing geometry) is
            // suggestive enough for a confirm-tier candidate, not a
            // silent commit.
            candidates.push(IntentCandidate {
                category: IntentCategory::EraseGesture,
                confidence: Confidence::new(0.45),
            });
        }

        // ViewLabel/NumericDimensionCandidate/Command/Text/Annotation:
        // this baseline has no way to read the stroke's actual written
        // content (Phase 17's job). Position alone (a stroke sitting
        // where a labeling guide is expected) is real, deterministic
        // evidence worth surfacing -- but only ever below
        // CONFIRM_THRESHOLD, honestly reflecting "plausible location,
        // content unverified" rather than claiming to have read a word
        // this baseline cannot read.
        if features.near_labeling_guide {
            candidates.push(IntentCandidate {
                category: IntentCategory::ViewLabel,
                confidence: Confidence::new(0.2),
            });
        }

        candidates.sort_by(|a, b| {
            b.confidence
                .get()
                .partial_cmp(&a.confidence.get())
                .expect("Confidence is always a finite, clamped value")
        });
        candidates
    }
}
