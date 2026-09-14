//! The native document root.
//!
//! Execution 01, Phase 07, Task 048. Authority: Engine Contract 15
//! (Document: "Schema, storage, migration, integrity, autosave"); MCP
//! Article 128 "Document Model".
//!
//! Holds exactly what Tasks 048/050 ask for and this execution has real
//! types for: schema version, metadata, a unit setting, and pages of
//! semantic entities. "Asset references" (also named in Task 048) are
//! deliberately absent -- reference-image asset storage has no owning
//! phase reached yet, and gets a field here when it does.
//!
//! Execution 02, Phase 04 (`CraftLoopSession` Mobile API, Article 12):
//! `view_blocks`/`orthographic_sets`/`multiview_graph`/`dimension_store`
//! were added here. Execution 01 built `ViewBlock`/`OrthographicSet`
//! (Phase 20), `MultiviewGraph` (Phase 22), and `DimensionStore` (Phase
//! 10) as fully tested, independent types, but every scenario test that
//! used more than one of them (e.g. `scenario_216_front_to_orthographic`)
//! constructed them as bare local variables -- none of them was ever a
//! real, persisted `Document` field, so "save, kill the app, reopen it"
//! (Execution 02 Article 35's Golden Alpha Journey) had nothing to
//! actually round-trip. This is that wiring: each field lives here, next
//! to the `SemanticEntity` pages that already reference the same
//! `PrimitiveId`/`DimensionId` space, so one `Document` is the single
//! persisted unit of engineering truth the session layer needs.
//!
//! `dimension_store` is deliberately *not* a replacement for the
//! `SemanticEntity::Dimension` copies `Page` already stores (Phase 07):
//! the page copy is what renders and what Phase 07's existing tests
//! already exercise; `dimension_store` is the authoritative copy the
//! multiview/propagation engine (`propagation.rs`) actually mutates
//! (`DimensionStore::edit_driving_value`/`set_dimension`), since
//! reusing that engine's own validation is required rather than
//! reimplementing it. `CraftLoopSession` (`craftloop-mobile-ffi`) is
//! responsible for keeping the two in sync, always inside one atomic
//! `DocumentHistory::commit` transaction -- see its module doc.

use std::collections::BTreeMap;

use craftloop_dimension::DimensionStore;
use craftloop_errors::{DocumentErrorKind, DomainError, DomainResult};
use craftloop_ids::{CraftLoopId, OrthographicSetId, PageId, ViewId};
use craftloop_serialization::SchemaVersion;
use craftloop_sketch::Sketch;
use serde::{Deserialize, Serialize};

use crate::entity::EntityId;
use crate::metadata::DocumentMetadata;
use crate::multiview::MultiviewGraph;
use crate::page::Page;
use crate::provenance::ProvenanceState;
use crate::units::DocumentUnits;
use crate::view::{OrthographicSet, ViewBlock};

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
    /// Execution 02, Phase 04: every `ViewBlock` this document has ever
    /// created (Article 30's semantic identity/layout/membership state),
    /// keyed by its stable `ViewId`.
    view_blocks: BTreeMap<ViewId, ViewBlock>,
    /// Execution 02, Phase 04: every `OrthographicSet` (a group of linked
    /// views describing one design state, Article 30/327).
    orthographic_sets: BTreeMap<OrthographicSetId, OrthographicSet>,
    /// Execution 02, Phase 04: which shared `DimensionId` each view's each
    /// consumed axis is bound to (Article 35/37's "same width in front and
    /// top"). See `crate::multiview`.
    multiview_graph: MultiviewGraph,
    /// Execution 02, Phase 04: the authoritative dimension values the
    /// multiview/propagation engine reads and edits. See this struct's own
    /// doc comment above for how this relates to `Page`'s
    /// `SemanticEntity::Dimension` copies.
    dimension_store: DimensionStore,
    /// Execution 02, Phase 04 (Article 17's Constraint Alpha Workflow):
    /// constraint-bearing geometry and the constraints relating it
    /// (`craftloop-sketch`, Phase 12). A document has exactly one
    /// `Sketch` for Execution 02's Alpha scope -- no task before this one
    /// asks for more than one constrainable geometry set per document.
    sketch: Sketch,
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
            view_blocks: BTreeMap::new(),
            orthographic_sets: BTreeMap::new(),
            multiview_graph: MultiviewGraph::new(),
            dimension_store: DimensionStore::new(),
            sketch: Sketch::new(),
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

    // -- View blocks / orthographic sets / multiview graph / dimension
    // -- store (Execution 02, Phase 04). Mutation is `pub(crate)`-only:
    // -- every change to these collections must go through
    // -- `history::DocumentChange` so it is undoable and so
    // -- `bump_revision` stays the single source of "did anything
    // -- change" (matching `provenance`'s own convention above).

    pub fn view_block(&self, id: ViewId) -> Option<&ViewBlock> {
        self.view_blocks.get(&id)
    }

    pub fn view_blocks(&self) -> impl Iterator<Item = &ViewBlock> {
        self.view_blocks.values()
    }

    pub(crate) fn set_view_block(&mut self, id: ViewId, block: Option<ViewBlock>) {
        match block {
            Some(block) => {
                self.view_blocks.insert(id, block);
            }
            None => {
                self.view_blocks.remove(&id);
            }
        }
    }

    pub fn orthographic_set(&self, id: OrthographicSetId) -> Option<&OrthographicSet> {
        self.orthographic_sets.get(&id)
    }

    pub fn orthographic_sets(&self) -> impl Iterator<Item = &OrthographicSet> {
        self.orthographic_sets.values()
    }

    pub(crate) fn set_orthographic_set(
        &mut self,
        id: OrthographicSetId,
        set: Option<OrthographicSet>,
    ) {
        match set {
            Some(set) => {
                self.orthographic_sets.insert(id, set);
            }
            None => {
                self.orthographic_sets.remove(&id);
            }
        }
    }

    pub fn multiview_graph(&self) -> &MultiviewGraph {
        &self.multiview_graph
    }

    pub(crate) fn multiview_graph_mut(&mut self) -> &mut MultiviewGraph {
        &mut self.multiview_graph
    }

    pub fn dimension_store(&self) -> &DimensionStore {
        &self.dimension_store
    }

    pub(crate) fn dimension_store_mut(&mut self) -> &mut DimensionStore {
        &mut self.dimension_store
    }

    pub fn sketch(&self) -> &Sketch {
        &self.sketch
    }

    pub(crate) fn sketch_mut(&mut self) -> &mut Sketch {
        &mut self.sketch
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
