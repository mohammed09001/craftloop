//! Semantic JSON diagnostic export.
//!
//! Execution 01, Phase 26, Task 186. Authority: Engine Contract 15
//! (Document); MCP Article 128 "Document Model".
//!
//! Distinct from `craftloop_document::persistence::save_document_atomically`
//! (Phase 07), which serializes `Document` in its compact, round-trip
//! *storage* shape. This export is for a developer reading the output
//! directly -- debugging a live session, or pinning a regression fixture
//! -- so it denormalizes what storage keeps separate: each entity's
//! `craftloop_document::ProvenanceState` (stored in `Document` as its own
//! `BTreeMap<EntityId, ProvenanceState>`, requiring a join to read
//! alongside the entity it describes) is inlined next to the entity it
//! belongs to. Nothing is filtered here -- unlike `svg`/`pdf`, this export
//! deliberately includes `SemanticEntity::Conflict` entries and any
//! not-yet-removed command ink, because seeing the document's full
//! internal state, diagnostics included, is exactly the point of a
//! debugging export.

use craftloop_document::{Document, ProvenanceState, SemanticEntity};
use craftloop_errors::DomainResult;
use serde::Serialize;

/// One page entity, with its provenance inlined.
#[derive(Debug, Serialize)]
pub struct DiagnosticEntityRecord {
    pub id: String,
    pub provenance: Option<ProvenanceState>,
    #[serde(flatten)]
    pub entity: SemanticEntity,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticPageRecord {
    pub page_id: String,
    pub name: String,
    pub entities: Vec<DiagnosticEntityRecord>,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticExport {
    pub schema_version: craftloop_serialization::SchemaVersion,
    pub title: String,
    pub units: craftloop_document::DocumentUnits,
    pub revision: u64,
    pub pages: Vec<DiagnosticPageRecord>,
}

fn entity_record(document: &Document, entity: &SemanticEntity) -> DiagnosticEntityRecord {
    let id = entity.id();
    DiagnosticEntityRecord {
        id: id.to_string(),
        provenance: document.provenance_of(id),
        entity: entity.clone(),
    }
}

/// Build the full diagnostic view of `document`: every page, every entity
/// (unfiltered), each annotated with its current provenance.
pub fn diagnostic_export(document: &Document) -> DiagnosticExport {
    let pages = document
        .pages()
        .map(|page| DiagnosticPageRecord {
            page_id: page.id.to_string(),
            name: page.name.clone(),
            entities: page
                .entities()
                .map(|entity| entity_record(document, entity))
                .collect(),
        })
        .collect();
    DiagnosticExport {
        schema_version: document.schema_version,
        title: document.metadata.title.clone(),
        units: document.units,
        revision: document.revision(),
        pages,
    }
}

/// `diagnostic_export`, serialized to a pretty-printed JSON string ready to
/// write to a regression fixture file.
pub fn export_diagnostic_json(document: &Document) -> DomainResult<String> {
    let export = diagnostic_export(document);
    serde_json::to_string_pretty(&export).map_err(|e| craftloop_errors::DomainError::Export {
        kind: craftloop_errors::ExportErrorKind::UnrepresentableValue,
        detail: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_document::{EntityId, Note};
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, NoteId};

    #[test]
    fn every_page_and_entity_appears_in_the_export() {
        let mut document = Document::new("Bracket", 0.0);
        let page_id = document.active_page().unwrap();
        let note_id = NoteId::new();
        document
            .page_mut(page_id)
            .unwrap()
            .insert(SemanticEntity::Note(Note::new(
                note_id,
                Point2::ORIGIN,
                "hello",
            )))
            .unwrap();

        let export = diagnostic_export(&document);
        assert_eq!(export.pages.len(), 1);
        assert_eq!(export.pages[0].entities.len(), 1);
        assert_eq!(
            export.pages[0].entities[0].id,
            EntityId::Note(note_id).to_string()
        );
    }

    #[test]
    fn provenance_is_inlined_next_to_its_entity() {
        let mut document = Document::new("Bracket", 0.0);
        let page_id = document.active_page().unwrap();
        let note_id = NoteId::new();
        let entity_id = EntityId::Note(note_id);
        document
            .page_mut(page_id)
            .unwrap()
            .insert(SemanticEntity::Note(Note::new(
                note_id,
                Point2::ORIGIN,
                "hello",
            )))
            .unwrap();
        document.set_provenance(entity_id, ProvenanceState::UserCreated);

        let export = diagnostic_export(&document);
        assert_eq!(
            export.pages[0].entities[0].provenance,
            Some(ProvenanceState::UserCreated)
        );
    }

    #[test]
    fn an_entity_never_explicitly_classified_has_no_provenance_not_a_fabricated_default() {
        let mut document = Document::new("Bracket", 0.0);
        let page_id = document.active_page().unwrap();
        document
            .page_mut(page_id)
            .unwrap()
            .insert(SemanticEntity::Note(Note::new(
                NoteId::new(),
                Point2::ORIGIN,
                "hello",
            )))
            .unwrap();

        let export = diagnostic_export(&document);
        assert_eq!(export.pages[0].entities[0].provenance, None);
    }

    #[test]
    fn the_export_serializes_to_valid_json() {
        let document = Document::new("Bracket", 0.0);
        let json = export_diagnostic_json(&document).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value.get("pages").is_some());
        assert_eq!(value.get("title").unwrap(), "Bracket");
    }
}
