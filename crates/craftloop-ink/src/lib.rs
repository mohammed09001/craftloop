//! Raw ink model and stroke processing for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 05. Authority: Engine Contract 02 ("Raw ink is
//! preserved until a semantic transaction commits").
//!
//! No platform UI dependency; depends only on `craftloop-geometry`,
//! `craftloop-input`, `craftloop-ids`, `craftloop-errors`,
//! `craftloop-serialization`.

pub mod grouping;
pub mod provenance;
pub mod smoothing;
pub mod spatial;
pub mod stroke;

pub use grouping::{group_by_proximity, GroupingTolerance};
pub use provenance::{ProvenanceEvent, StrokeProvenance};
pub use smoothing::smoothed_positions;
pub use spatial::StrokeSpatialIndex;
pub use stroke::Stroke;
