//! One page/canvas of a notebook document.
//!
//! Execution 01, Phase 07, Task 048/050. Authority: MCP Article 65 "Pages
//! and Infinite Paper".

use std::collections::BTreeMap;

use craftloop_errors::{DocumentErrorKind, DomainError, DomainResult};
use craftloop_ids::PageId;
use serde::{Deserialize, Serialize};

use crate::entity::{EntityId, SemanticEntity};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: PageId,
    pub name: String,
    entities: BTreeMap<EntityId, SemanticEntity>,
}

impl Page {
    pub fn new(id: PageId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            entities: BTreeMap::new(),
        }
    }

    /// Insert a new entity. Rejects (without mutating) an entity whose ID
    /// already exists on this page -- silently overwriting a distinct
    /// stored entity under the same ID would be a real data-loss bug, not
    /// a convenience.
    pub fn insert(&mut self, entity: SemanticEntity) -> DomainResult<()> {
        let id = entity.id();
        if self.entities.contains_key(&id) {
            return Err(DomainError::Document {
                kind: DocumentErrorKind::DuplicateEntityId,
                detail: format!("entity {id:?} already exists on page {:?}", self.id),
            });
        }
        self.entities.insert(id, entity);
        Ok(())
    }

    pub fn get(&self, id: EntityId) -> Option<&SemanticEntity> {
        self.entities.get(&id)
    }

    pub fn remove(&mut self, id: EntityId) -> Option<SemanticEntity> {
        self.entities.remove(&id)
    }

    pub fn entities(&self) -> impl Iterator<Item = &SemanticEntity> {
        self.entities.values()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::Note;
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, NoteId};

    #[test]
    fn insert_then_get_round_trips() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let note = Note::new(NoteId::new(), Point2::ORIGIN, "hello");
        let id = EntityId::Note(note.id);
        page.insert(SemanticEntity::Note(note.clone())).unwrap();
        assert_eq!(page.get(id), Some(&SemanticEntity::Note(note)));
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn duplicate_id_insert_is_rejected_and_does_not_overwrite() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let note_id = NoteId::new();
        let first = Note::new(note_id, Point2::ORIGIN, "first");
        let second = Note::new(note_id, Point2::new(9.0, 9.0), "second");
        page.insert(SemanticEntity::Note(first.clone())).unwrap();

        let result = page.insert(SemanticEntity::Note(second));
        assert!(result.is_err());
        assert_eq!(
            page.get(EntityId::Note(note_id)),
            Some(&SemanticEntity::Note(first))
        );
    }

    #[test]
    fn remove_takes_an_entity_out_of_the_page() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let note = Note::new(NoteId::new(), Point2::ORIGIN, "hello");
        let id = EntityId::Note(note.id);
        page.insert(SemanticEntity::Note(note)).unwrap();
        assert!(page.remove(id).is_some());
        assert!(page.is_empty());
    }

    #[test]
    fn entities_iterates_in_deterministic_id_order() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let a = NoteId::from_u128(5);
        let b = NoteId::from_u128(1);
        page.insert(SemanticEntity::Note(Note::new(a, Point2::ORIGIN, "a")))
            .unwrap();
        page.insert(SemanticEntity::Note(Note::new(b, Point2::ORIGIN, "b")))
            .unwrap();

        let ids: Vec<EntityId> = page.entities().map(|e| e.id()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted, "BTreeMap iteration must already be sorted");
    }
}
