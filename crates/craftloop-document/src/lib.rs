//! Native document root, semantic paper, and persistence for the Craft
//! Loop shared engineering core.
//!
//! Execution 01, Phase 07. Authority: Engine Contract 15 (Document).
//!
//! No platform UI dependency; depends only on `craftloop-errors`,
//! `craftloop-ids`, `craftloop-geometry`, `craftloop-serialization`,
//! `craftloop-ink`, `craftloop-recognition`.

pub mod autosave;
pub mod document;
pub mod entity;
pub mod history;
pub mod metadata;
pub mod migration;
pub mod note;
pub mod page;
pub mod page_layout;
pub mod persistence;
pub mod provenance;
pub mod stale_result;
pub mod units;
pub mod view;

pub use autosave::AutosaveJournal;
pub use document::Document;
pub use entity::{EntityId, SemanticEntity};
pub use history::{DocumentChange, DocumentHistory};
pub use metadata::DocumentMetadata;
pub use migration::{migrate_to_current, Migration};
pub use note::Note;
pub use page::Page;
pub use page_layout::PageLayoutTransform;
pub use persistence::{load_document, save_document_atomically};
pub use provenance::ProvenanceState;
pub use stale_result::AsyncResult;
pub use units::DocumentUnits;
pub use view::{OrthographicSet, PrincipalViewIdentity, ViewBlock};
