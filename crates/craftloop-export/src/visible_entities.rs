//! Shared "what actually belongs in a normal export" filter.
//!
//! Execution 01, Phase 26, Task 190. Authority: MCP Article 236
//! ("Documents should not depend on the current meaning of ephemeral
//! commands after execution.").
//!
//! Two independent things can leak into a visual export if nothing
//! filters them: confirmed command ink that has not yet been physically
//! removed from the page (Article 236 guarantees the *record* survives
//! via `craftloop_command::UndoMetadata`, not that removal from the page
//! is instantaneous), and internal diagnostic entities
//! (`SemanticEntity::Conflict`), which describe the document's own
//! consistency state, not its engineering content. Every export target in
//! this crate that renders *visible* output (`svg`, `pdf`) shares this one
//! filter so the exclusion rule cannot drift between them. The developer
//! diagnostic export (`diagnostic_json`, Task 186) deliberately does not
//! use this filter -- seeing conflicts and in-flight command ink is
//! exactly its point.

use std::collections::BTreeSet;

use craftloop_document::{Page, SemanticEntity};
use craftloop_ids::StrokeId;

/// True if `entity` belongs in a normal (non-diagnostic) visual export.
/// `ephemeral_strokes` names strokes that are command text/confirmation
/// gestures a caller has confirmed (typically sourced from
/// `craftloop_command::EphemeralInkDisposition::strokes_to_remove`) but
/// which may still physically be present on the page.
pub fn is_export_visible(entity: &SemanticEntity, ephemeral_strokes: &BTreeSet<StrokeId>) -> bool {
    match entity {
        SemanticEntity::Stroke(stroke) => !ephemeral_strokes.contains(&stroke.id),
        SemanticEntity::Conflict(_) => false,
        SemanticEntity::Primitive { .. }
        | SemanticEntity::Note(_)
        | SemanticEntity::Dimension(_) => true,
    }
}

/// The subset of `page`'s entities a normal visual export should render,
/// in the page's own deterministic (`EntityId`-sorted) order.
pub fn visible_entities<'a>(
    page: &'a Page,
    ephemeral_strokes: &'a BTreeSet<StrokeId>,
) -> impl Iterator<Item = &'a SemanticEntity> {
    page.entities()
        .filter(move |e| is_export_visible(e, ephemeral_strokes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_consistency::{ConflictKind, ConflictStatus};
    use craftloop_errors::Severity;
    use craftloop_geometry::Point2;
    use craftloop_ids::{ConflictId, CraftLoopId, NoteId, StrokeId};
    use craftloop_ink::Stroke;
    use craftloop_input::{PointerSample, PointerSource};

    fn one_sample_stroke(id: StrokeId) -> SemanticEntity {
        let sample = PointerSample {
            position: Point2::ORIGIN,
            timestamp_seconds: 0.0,
            pressure: None,
            tilt_x_deg: None,
            tilt_y_deg: None,
            source: PointerSource::Stylus,
            buttons: Default::default(),
            capabilities: Default::default(),
        };
        SemanticEntity::Stroke(Stroke::new(id, vec![sample]).unwrap())
    }

    #[test]
    fn an_ordinary_stroke_is_visible() {
        let entity = one_sample_stroke(StrokeId::new());
        assert!(is_export_visible(&entity, &BTreeSet::new()));
    }

    #[test]
    fn a_stroke_named_as_ephemeral_command_ink_is_excluded() {
        let id = StrokeId::new();
        let entity = one_sample_stroke(id);
        let mut ephemeral = BTreeSet::new();
        ephemeral.insert(id);
        assert!(!is_export_visible(&entity, &ephemeral));
    }

    #[test]
    fn a_conflict_entity_is_always_excluded_from_visual_export() {
        let conflict = craftloop_consistency::Conflict {
            id: ConflictId::new(),
            kind: ConflictKind::DegenerateGeometry,
            severity: Severity::Error,
            affected_entities: Vec::new(),
            existing_truth: String::new(),
            proposed_truth: String::new(),
            evidence: String::new(),
            resolution_choices: Vec::new(),
            status: ConflictStatus::Unresolved,
        };
        let entity = SemanticEntity::Conflict(conflict);
        assert!(!is_export_visible(&entity, &BTreeSet::new()));
    }

    #[test]
    fn a_note_is_visible_and_unaffected_by_the_ephemeral_set() {
        let entity = SemanticEntity::Note(craftloop_document::Note::new(
            NoteId::new(),
            Point2::ORIGIN,
            "hello",
        ));
        assert!(is_export_visible(&entity, &BTreeSet::new()));
    }

    #[test]
    fn visible_entities_filters_a_real_page() {
        let mut page = Page::new(craftloop_ids::PageId::new(), "Page 1");
        let ephemeral_id = StrokeId::new();
        let ordinary_id = StrokeId::new();
        page.insert(one_sample_stroke(ephemeral_id)).unwrap();
        page.insert(one_sample_stroke(ordinary_id)).unwrap();

        let mut ephemeral = BTreeSet::new();
        ephemeral.insert(ephemeral_id);

        let remaining: Vec<_> = visible_entities(&page, &ephemeral).collect();
        assert_eq!(remaining.len(), 1);
        assert_eq!(
            remaining[0].id(),
            craftloop_document::EntityId::Stroke(ordinary_id)
        );
    }
}
