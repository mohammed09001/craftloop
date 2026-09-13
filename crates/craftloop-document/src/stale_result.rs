//! Stale asynchronous result rejection.
//!
//! Execution 01, Phase 08, Task 060. Authority: Engine Contract 15;
//! Architecture Principle: Stale Result Rejection (Article 398); "Associate
//! async results with source revision IDs and discard them when the
//! document changes."
//!
//! No async runtime is wired into any crate yet (the harness is
//! single-threaded; a real background recognizer is Phase 16+). What this
//! phase can and does build is the freshness-check primitive every future
//! async producer must go through: capture [`Document::revision`] when the
//! background work *starts*, and refuse to apply its result if the
//! document's revision has moved on by the time it *finishes*.

use crate::document::Document;

/// A value computed asynchronously against a document at a specific
/// revision. Call [`AsyncResult::accept_if_fresh`] when the background
/// work completes; a `None` means the document changed in the meantime and
/// the result must be discarded, not applied.
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncResult<T> {
    source_revision: u64,
    value: T,
}

impl<T> AsyncResult<T> {
    /// Capture `value` as computed against `document` at its current
    /// revision. Call this at the *start* of the async work, using the
    /// document state the work is actually based on.
    pub fn new(document: &Document, value: T) -> Self {
        Self {
            source_revision: document.revision(),
            value,
        }
    }

    pub fn source_revision(&self) -> u64 {
        self.source_revision
    }

    pub fn is_stale(&self, document: &Document) -> bool {
        document.revision() != self.source_revision
    }

    /// Consume this result, returning `Some(value)` only if `document` is
    /// still at the revision the result was computed against.
    pub fn accept_if_fresh(self, document: &Document) -> Option<T> {
        if self.is_stale(document) {
            None
        } else {
            Some(self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::SemanticEntity;
    use crate::history::{DocumentChange, DocumentHistory};
    use crate::note::Note;
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, NoteId};

    #[test]
    fn a_result_computed_and_accepted_with_no_intervening_change_is_fresh() {
        let document = Document::new("Untitled", 0.0);
        let result = AsyncResult::new(&document, "recognized: circle");
        assert!(!result.is_stale(&document));
        assert_eq!(
            result.accept_if_fresh(&document),
            Some("recognized: circle")
        );
    }

    #[test]
    fn a_result_is_discarded_if_the_document_changed_before_it_completed() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let result = AsyncResult::new(&document, "stale suggestion");

        // Something else committed to the document while the async work
        // was in flight.
        let mut history = DocumentHistory::new();
        let note = Note::new(NoteId::new(), Point2::ORIGIN, "unrelated edit");
        history
            .commit(
                &mut document,
                vec![DocumentChange::InsertEntity {
                    page_id,
                    entity: SemanticEntity::Note(note),
                }],
            )
            .unwrap();

        assert!(result.is_stale(&document));
        assert_eq!(result.accept_if_fresh(&document), None);
    }

    #[test]
    fn source_revision_reflects_the_revision_at_capture_time() {
        let mut document = Document::new("Untitled", 0.0);
        document.bump_revision();
        document.bump_revision();
        let result = AsyncResult::new(&document, ());
        assert_eq!(result.source_revision(), 2);
    }
}
