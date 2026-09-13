//! The native document root.
//!
//! Execution 01, Phase 07, Task 048. Authority: Engine Contract 15
//! (Document: "Schema, storage, migration, integrity, autosave"); MCP
//! Article 128 "Document Model".
//!
//! Holds exactly what Tasks 048/050 ask for and this execution has real
//! types for: schema version, metadata, a unit setting, and pages of
//! semantic entities. "Orthographic sets" and "asset references" (also
//! named in Task 048) are deliberately absent -- orthographic sets are
//! Phase 20's `OrthographicSet` (Engine Contract 19), and reference-image
//! asset storage has no owning phase reached yet. Both get a field here
//! when the phase that defines their type is done, not before.

use std::collections::BTreeMap;

use craftloop_errors::{DocumentErrorKind, DomainError, DomainResult};
use craftloop_ids::{CraftLoopId, PageId};
use craftloop_serialization::SchemaVersion;
use serde::{Deserialize, Serialize};

use crate::entity::EntityId;
use crate::metadata::DocumentMetadata;
use crate::page::Page;
use crate::provenance::ProvenanceState;
use crate::units::DocumentUnits;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub schema_version: SchemaVersion,
    pub metadata: DocumentMetadata,
    pub units: DocumentUnits,
    pages: BTreeMap<PageId, Page>,
    /// The page a newly created document starts on, and the default target
    /// for operations that need "the current page" without the caller
    /// naming one. `None` only for a document with zero pages (unusual,
    /// but not invalid -- an empty notebook).
    active_page: Option<PageId>,
    /// Monotonically increasing count of committed transactions (Phase 08,
    /// Task 060: "associate async results with source revision IDs and
    /// discard them when the document changes"). Bumped by
    /// `history::DocumentHistory::commit`, never by direct mutation, so it
    /// is a trustworthy "has anything changed since I looked?" signal.
    revision: u64,
    /// Current provenance classification per entity (Task 059). Absent for
    /// an entity means "never explicitly classified" (most tests fall into
    /// this bucket); real usage sets it as part of the same transaction
    /// that inserts the entity.
    provenance: BTreeMap<EntityId, ProvenanceState>,
}

impl Document {
    /// Create a new document with a single empty page, matching Article 300
    /// "Blank Notebook as the Default".
    pub fn new(title: impl Into<String>, now_seconds: f64) -> Self {
        let page = Page::new(PageId::new(), "Page 1");
        let active_page = Some(page.id);
        let mut pages = BTreeMap::new();
        pages.insert(page.id, page);
        Self {
            schema_version: SchemaVersion::CURRENT,
            metadata: DocumentMetadata::new(title, now_seconds),
            units: DocumentUnits::default(),
            pages,
            active_page,
            revision: 0,
            provenance: BTreeMap::new(),
        }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Advance the revision counter by one. Called once per committed
    /// transaction by `DocumentHistory::commit`; not intended for direct
    /// use outside that path.
    pub fn bump_revision(&mut self) {
        self.revision += 1;
    }

    pub fn provenance_of(&self, id: EntityId) -> Option<ProvenanceState> {
        self.provenance.get(&id).copied()
    }

    /// Set `id`'s provenance state, returning the previous value (if any)
    /// so a caller building an undo record can restore it exactly.
    pub fn set_provenance(
        &mut self,
        id: EntityId,
        state: ProvenanceState,
    ) -> Option<ProvenanceState> {
        self.provenance.insert(id, state)
    }

    /// Remove `id`'s provenance entry entirely, restoring "never
    /// classified." Used to invert a `SetProvenance` change whose prior
    /// state was `None`.
    pub fn clear_provenance(&mut self, id: EntityId) {
        self.provenance.remove(&id);
    }

    pub fn add_page(&mut self, name: impl Into<String>) -> PageId {
        let page = Page::new(PageId::new(), name);
        let id = page.id;
        self.pages.insert(id, page);
        if self.active_page.is_none() {
            self.active_page = Some(id);
        }
        id
    }

    pub fn page(&self, id: PageId) -> Option<&Page> {
        self.pages.get(&id)
    }

    pub fn page_mut(&mut self, id: PageId) -> DomainResult<&mut Page> {
        self.pages
            .get_mut(&id)
            .ok_or_else(|| DomainError::Document {
                kind: DocumentErrorKind::UnknownReference,
                detail: format!("no page with id {id:?}"),
            })
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.values()
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn active_page(&self) -> Option<PageId> {
        self.active_page
    }

    pub fn set_active_page(&mut self, id: PageId) -> DomainResult<()> {
        if !self.pages.contains_key(&id) {
            return Err(DomainError::Document {
                kind: DocumentErrorKind::UnknownReference,
                detail: format!("cannot activate unknown page {id:?}"),
            });
        }
        self.active_page = Some(id);
        Ok(())
    }

    /// Structural integrity check independent of file I/O (Task 053 builds
    /// the load-time version of this on top): every invariant this type's
    /// own methods are supposed to guarantee, re-verified. Used after
    /// deserializing a document loaded from disk, where a corrupted file
    /// could produce a structurally-valid-JSON-but-semantically-broken
    /// value no constructor here would ever create.
    pub fn validate(&self) -> DomainResult<()> {
        if let Some(active) = self.active_page {
            if !self.pages.contains_key(&active) {
                return Err(DomainError::Document {
                    kind: DocumentErrorKind::UnknownReference,
                    detail: format!(
                        "active_page {active:?} does not exist among this document's pages"
                    ),
                });
            }
        }
        if !self.schema_version.is_readable_by(SchemaVersion::CURRENT) {
            return Err(DomainError::Persistence {
                kind: craftloop_errors::PersistenceErrorKind::SchemaVersionMismatch,
                detail: format!(
                    "document schema {} is newer than this build's {}",
                    self.schema_version,
                    SchemaVersion::CURRENT
                ),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_has_one_page_and_it_is_active() {
        let doc = Document::new("Untitled", 0.0);
        assert_eq!(doc.page_count(), 1);
        assert!(doc.active_page().is_some());
        assert!(doc.page(doc.active_page().unwrap()).is_some());
    }

    #[test]
    fn add_page_increases_page_count_and_keeps_the_original_active_page() {
        let mut doc = Document::new("Untitled", 0.0);
        let first_active = doc.active_page().unwrap();
        let second = doc.add_page("Page 2");
        assert_eq!(doc.page_count(), 2);
        assert_eq!(doc.active_page(), Some(first_active));
        assert!(doc.page(second).is_some());
    }

    #[test]
    fn set_active_page_rejects_an_unknown_page_id() {
        let mut doc = Document::new("Untitled", 0.0);
        let bogus = PageId::new();
        assert!(doc.set_active_page(bogus).is_err());
        // Active page is unchanged after the rejected call.
        assert_ne!(doc.active_page(), Some(bogus));
    }

    #[test]
    fn page_mut_returns_a_structured_error_for_an_unknown_page() {
        let mut doc = Document::new("Untitled", 0.0);
        assert!(doc.page_mut(PageId::new()).is_err());
    }

    #[test]
    fn a_freshly_constructed_document_always_validates() {
        let doc = Document::new("Untitled", 0.0);
        assert!(doc.validate().is_ok());
    }

    #[test]
    fn a_future_schema_version_fails_validation() {
        let mut doc = Document::new("Untitled", 0.0);
        doc.schema_version = SchemaVersion(SchemaVersion::CURRENT.0 + 1);
        assert!(doc.validate().is_err());
    }

    #[test]
    fn a_new_document_starts_at_revision_zero() {
        assert_eq!(Document::new("Untitled", 0.0).revision(), 0);
    }

    #[test]
    fn bump_revision_increments_monotonically() {
        let mut doc = Document::new("Untitled", 0.0);
        doc.bump_revision();
        doc.bump_revision();
        assert_eq!(doc.revision(), 2);
    }

    #[test]
    fn provenance_defaults_to_none_and_can_be_set_and_cleared() {
        let mut doc = Document::new("Untitled", 0.0);
        let id = EntityId::Note(craftloop_ids::NoteId::new());
        assert_eq!(doc.provenance_of(id), None);

        let previous = doc.set_provenance(id, ProvenanceState::Suggested);
        assert_eq!(previous, None);
        assert_eq!(doc.provenance_of(id), Some(ProvenanceState::Suggested));

        let previous = doc.set_provenance(id, ProvenanceState::Accepted);
        assert_eq!(previous, Some(ProvenanceState::Suggested));

        doc.clear_provenance(id);
        assert_eq!(doc.provenance_of(id), None);
    }
}
