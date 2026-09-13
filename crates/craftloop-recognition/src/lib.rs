//! Primitive recognition (candidate fitting/ranking) and beautification for
//! the Craft Loop shared engineering core.
//!
//! Execution 01, Phase 06. Authority: Engine Contracts 04 (Recognition) and
//! 05 (Beautification).
//!
//! No platform UI dependency; depends only on `craftloop-geometry`,
//! `craftloop-ink`, `craftloop-ids`.

pub mod beautify;
pub mod candidate;
pub mod confidence;
pub mod fit;
pub mod rejection_memory;

pub use beautify::{beautify, Beautified, BeautifiedPrimitive};
pub use candidate::{rank_candidates, recognize, CandidateKind, RecognitionCandidate};
pub use confidence::Confidence;
pub use rejection_memory::RejectionMemory;
