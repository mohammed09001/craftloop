//! Engineering Handwriting Adapter Boundary for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 17, Tasks 119-124. Authority: Engine Contract 05.
//!
//! Defines the platform-neutral handwriting recognizer interface
//! (`HandwritingRecognizer`), a deterministic Windows test stub
//! (`FixtureHandwritingRecognizer`) since no real on-device recognizer is
//! available in this development environment, routing of recognized text
//! into `craftloop-units`' existing numeric parsers -- never into
//! geometry directly -- and provenance tracking tying every recognition
//! result back to its exact source strokes. See
//! `execution-evidence/handwriting-adapter-platform-plan.md` (Task 124)
//! for how Android/iPad adapters are expected to implement this same
//! interface.

pub mod provenance;
pub mod recognizer;
pub mod routing;
pub mod stub;

pub use provenance::HandwritingRecognitionResult;
pub use recognizer::{HandwritingRecognizer, TextCandidate};
pub use routing::{route_as_angle, route_as_length, route_as_radial, RoutedValue};
pub use stub::FixtureHandwritingRecognizer;
