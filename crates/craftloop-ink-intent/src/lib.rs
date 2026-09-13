//! Ink Intent Classification baseline for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 16 ("Ink Intent Classification Baseline"), Tasks
//! 113-118. Authority: Engine Contract 04.
//!
//! Deliberately precedes any real handwriting-content reading (Phase 17
//! is "Engineering Handwriting Adapter Boundary," the next phase): this
//! crate defines the category vocabulary, the deterministic (non-ML)
//! feature/confidence contract, a real conservative baseline classifier
//! built only from signals already available in this workspace (tool
//! context, geometry, timing, position), and the seam a future learned
//! classifier plugs into without changing any of that vocabulary.

pub mod category;
pub mod classifier;
pub mod context;

pub use category::IntentCategory;
pub use classifier::{
    DeterministicBaselineClassifier, IntentCandidate, IntentClassifier, COMMIT_THRESHOLD,
    CONFIRM_THRESHOLD,
};
pub use context::{ContextFeatures, ToolContext};
