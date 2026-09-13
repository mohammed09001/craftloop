//! Native document root, semantic paper, and persistence for the Craft
//! Loop shared engineering core.
//!
//! Execution 01, Phase 07. Authority: Engine Contract 15 (Document).
//!
//! No platform UI dependency; depends only on `craftloop-errors`,
//! `craftloop-ids`, `craftloop-geometry`, `craftloop-serialization`,
//! `craftloop-ink`, `craftloop-recognition`.

pub mod autosave;
pub mod correspondence;
pub mod document;
pub mod entity;
pub mod history;
pub mod metadata;
pub mod migration;
pub mod multiview;
pub mod note;
pub mod orthographic;
pub mod page;
pub mod page_layout;
pub mod persistence;
pub mod propagation;
pub mod provenance;
pub mod stale_result;
pub mod units;
pub mod view;

pub use autosave::AutosaveJournal;
pub use correspondence::{
    evaluate_correspondence, smallest_missing_fact, CorrespondenceCandidate, CorrespondenceRanker,
    CorrespondenceStore, DeterministicRanker, Evidence, EvidenceKind, MissingFact,
};
pub use document::Document;
pub use entity::{EntityId, SemanticEntity};
pub use history::{DocumentChange, DocumentHistory};
pub use metadata::DocumentMetadata;
pub use migration::{migrate_to_current, Migration};
pub use multiview::{axes_for_identity, MultiviewGraph, SharedAxis};
pub use note::Note;
pub use orthographic::{
    default_layout_offset, default_orthographic_layout, evaluate_readiness,
    transition_to_orthographic, OrthographicReadiness, ProjectionConvention, ReadinessIssue,
};
pub use page::Page;
pub use page_layout::PageLayoutTransform;
pub use persistence::{load_document, save_document_atomically};
pub use propagation::{
    projection_guides, propagate_confirmed_value, propose_shared_value,
    share_back_extent_from_front, ProjectionGuide,
};
pub use provenance::ProvenanceState;
pub use stale_result::AsyncResult;
pub use units::DocumentUnits;
pub use view::{OrthographicSet, PrincipalViewIdentity, ViewBlock};
