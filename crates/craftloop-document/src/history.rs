//! Command transactions, undo, and redo over a `Document`.
//!
//! Execution 01, Phase 08, Tasks 056-058. Authority: Engine Contract 14
//! (Transactions: "Atomic commits, undo/redo, event history"; "One visible
//! action should undo as one coherent action"); MCP Article 79 "Undo and
//! Redo", Article 138 "Interaction Transaction Model".
//!
//! This is where Phase 01's generic `craftloop_transactions::Transaction`/
//! `TransactionLog` finally meet a real domain: [`DocumentChange`] is the
//! concrete change type, and [`DocumentHistory`] wires commit/undo/redo to
//! actually mutate a [`Document`].
//!
//! **Undo strategy (Task 057): state deltas, not inverse functions.**
//! `DocumentChange::RemoveEntity` and `SetProvenance` snapshot the value
//! they are overwriting *at the moment they are recorded*, so `invert()` is
//! a pure data transformation (swap old/new) rather than a function that
//! has to re-derive what "undo this insert" even means. This was chosen
//! over computing inverses structurally because a `SemanticEntity` can be
//! large and heterogeneous (a `Stroke` with many samples, a `Beautified`
//! primitive, ...) -- storing the actual prior value is simpler and
//! strictly more correct than trying to synthesize an inverse operation
//! for each entity kind.
//!
//! "Cross-view propagation" (as Task 057's objective names it) had no real
//! multi-view mechanism to test when this module was first written --
//! view blocks were Phase 20. The closest real analog Phase 08 could
//! exercise then was a single transaction touching **multiple pages** at
//! once (see the module tests), which exercises the same atomicity
//! property multi-view propagation needs: one visible action, several
//! entities across several containers, undone/redone as one unit.
//!
//! Execution 02, Phase 04 adds the real mechanism itself:
//! `SetViewBlock`/`SetOrthographicSet`/`SetMultiviewBinding`/
//! `SetDimension` follow the exact same "state delta" convention as
//! `SetProvenance` above (an `Option` `previous`/`new` pair, `invert()` is
//! just swapping them) so every `Document` field Phase 04 added
//! (`view_blocks`, `orthographic_sets`, `multiview_graph`,
//! `dimension_store`) is undoable/redoable/persistable through this same
//! one mechanism, not a second one.

use craftloop_dimension::SemanticDimension;
use craftloop_errors::DomainResult;
use craftloop_ids::{
    ConstraintId, DimensionId, OrthographicSetId, PageId, PrimitiveId, TransactionId, ViewId,
};
use craftloop_recognition::Beautified;
use craftloop_sketch::{ConstraintProvenance, SketchConstraintKind};
use craftloop_transactions::{Transaction, TransactionLog};
use serde::{Deserialize, Serialize};

use crate::document::Document;
use crate::entity::{EntityId, SemanticEntity};
use crate::multiview::SharedAxis;
use crate::provenance::ProvenanceState;
use crate::view::{OrthographicSet, ViewBlock};

/// One atomic step within a transaction. A single user-visible action
/// (Task 056) is a `Vec<DocumentChange>` committed together via
/// [`DocumentHistory::commit`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentChange {
    InsertEntity {
        page_id: PageId,
        entity: SemanticEntity,
    },
    /// `entity` is the full value being removed, snapshotted at record
    /// time -- see the module doc's "state deltas" note.
    RemoveEntity {
        page_id: PageId,
        entity: SemanticEntity,
    },
    /// `previous`/`new` are both `Option` (not just `new`) so that undoing
    /// a first-ever classification (`previous: None`) restores true
    /// "never classified" -- via `Document::clear_provenance` -- rather
    /// than having to invent some fallback state to set it to.
    SetProvenance {
        entity_id: EntityId,
        previous: Option<ProvenanceState>,
        new: Option<ProvenanceState>,
    },
    /// Execution 02, Phase 04: create, update, or remove (`new: None`) one
    /// `ViewBlock`. Same "state delta" convention as `SetProvenance`.
    SetViewBlock {
        id: ViewId,
        previous: Option<ViewBlock>,
        new: Option<ViewBlock>,
    },
    /// Execution 02, Phase 04: create, update, or remove one
    /// `OrthographicSet`.
    SetOrthographicSet {
        id: OrthographicSetId,
        previous: Option<OrthographicSet>,
        new: Option<OrthographicSet>,
    },
    /// Execution 02, Phase 04: bind or unbind one view's one shared axis in
    /// the `MultiviewGraph` (Article 35/37).
    SetMultiviewBinding {
        view: ViewId,
        axis: SharedAxis,
        previous: Option<DimensionId>,
        new: Option<DimensionId>,
    },
    /// Execution 02, Phase 04: the authoritative `DimensionStore` copy of a
    /// dimension's full value (see `document.rs`'s doc comment on how this
    /// relates to a page's own `SemanticEntity::Dimension` copy -- a
    /// caller committing a dimension edit/creation normally pairs this
    /// with an `InsertEntity`/`RemoveEntity` change in the same
    /// transaction so both copies move together atomically).
    SetDimension {
        id: DimensionId,
        previous: Option<SemanticDimension>,
        new: Option<SemanticDimension>,
    },
    /// Execution 02, Phase 04 (Article 17's Constraint Alpha Workflow):
    /// insert one constraint-bearing primitive into the document's
    /// `Sketch`.
    InsertSketchPrimitive {
        id: PrimitiveId,
        beautified: Beautified,
    },
    /// `beautified` is the full value being removed, snapshotted at
    /// record time -- same "state delta" convention as `RemoveEntity`.
    RemoveSketchPrimitive {
        id: PrimitiveId,
        beautified: Beautified,
    },
    /// Create, update, or remove one stored constraint in the document's
    /// `Sketch`.
    SetSketchConstraint {
        id: ConstraintId,
        previous: Option<(SketchConstraintKind, ConstraintProvenance)>,
        new: Option<(SketchConstraintKind, ConstraintProvenance)>,
    },
}

impl DocumentChange {
    fn apply(&self, document: &mut Document) -> DomainResult<()> {
        match self {
            DocumentChange::InsertEntity { page_id, entity } => {
                document.page_mut(*page_id)?.insert(entity.clone())
            }
            DocumentChange::RemoveEntity { page_id, entity } => {
                document.page_mut(*page_id)?.remove(entity.id());
                Ok(())
            }
            DocumentChange::SetProvenance { entity_id, new, .. } => {
                match new {
                    Some(state) => {
                        document.set_provenance(*entity_id, *state);
                    }
                    None => document.clear_provenance(*entity_id),
                }
                Ok(())
            }
            DocumentChange::SetViewBlock { id, new, .. } => {
                document.set_view_block(*id, new.clone());
                Ok(())
            }
            DocumentChange::SetOrthographicSet { id, new, .. } => {
                document.set_orthographic_set(*id, new.clone());
                Ok(())
            }
            DocumentChange::SetMultiviewBinding {
                view, axis, new, ..
            } => {
                match new {
                    Some(dimension) => {
                        // The graph's own `bind_axis` re-validates identity/
                        // axis compatibility every time (it has no way to
                        // trust a previously-valid binding is still valid
                        // once undo/redo can rewrite view identity too) --
                        // this needs the real `ViewBlock` to do that, which
                        // this change's own `view`/`axis` pair does not
                        // carry (only the ids). A binding recorded by
                        // `CraftLoopSession` is always validated once at
                        // creation time through `bind_axis` itself before
                        // this change is ever built (see its module docs),
                        // so re-deriving a `ViewBlock` here would only
                        // duplicate that check; this apply step is
                        // therefore the direct, already-validated write.
                        document
                            .multiview_graph_mut()
                            .insert_binding_unchecked(*view, *axis, *dimension);
                    }
                    None => {
                        document.multiview_graph_mut().unlink_axis(*view, *axis);
                    }
                }
                Ok(())
            }
            DocumentChange::SetDimension { id, new, .. } => {
                match new {
                    Some(dimension) => {
                        document
                            .dimension_store_mut()
                            .set_dimension(dimension.clone());
                    }
                    None => {
                        document.dimension_store_mut().remove_dimension(*id);
                    }
                }
                Ok(())
            }
            DocumentChange::InsertSketchPrimitive { id, beautified } => {
                document
                    .sketch_mut()
                    .insert_primitive(*id, beautified.clone());
                Ok(())
            }
            DocumentChange::RemoveSketchPrimitive { id, .. } => {
                document.sketch_mut().remove_primitive(*id);
                Ok(())
            }
            DocumentChange::SetSketchConstraint { id, new, .. } => {
                match new {
                    Some((kind, provenance)) => {
                        document
                            .sketch_mut()
                            .set_constraint_unchecked(*id, *kind, *provenance);
                    }
                    None => {
                        document.sketch_mut().remove_constraint_unchecked(*id);
                    }
                }
                Ok(())
            }
        }
    }

    /// The change that undoes this one exactly.
    fn invert(&self) -> DocumentChange {
        match self {
            DocumentChange::InsertEntity { page_id, entity } => DocumentChange::RemoveEntity {
                page_id: *page_id,
                entity: entity.clone(),
            },
            DocumentChange::RemoveEntity { page_id, entity } => DocumentChange::InsertEntity {
                page_id: *page_id,
                entity: entity.clone(),
            },
            DocumentChange::SetProvenance {
                entity_id,
                previous,
                new,
            } => DocumentChange::SetProvenance {
                entity_id: *entity_id,
                previous: *new,
                new: *previous,
            },
            DocumentChange::SetViewBlock { id, previous, new } => DocumentChange::SetViewBlock {
                id: *id,
                previous: new.clone(),
                new: previous.clone(),
            },
            DocumentChange::SetOrthographicSet { id, previous, new } => {
                DocumentChange::SetOrthographicSet {
                    id: *id,
                    previous: new.clone(),
                    new: previous.clone(),
                }
            }
            DocumentChange::SetMultiviewBinding {
                view,
                axis,
                previous,
                new,
            } => DocumentChange::SetMultiviewBinding {
                view: *view,
                axis: *axis,
                previous: *new,
                new: *previous,
            },
            DocumentChange::SetDimension { id, previous, new } => DocumentChange::SetDimension {
                id: *id,
                previous: new.clone(),
                new: previous.clone(),
            },
            DocumentChange::InsertSketchPrimitive { id, beautified } => {
                DocumentChange::RemoveSketchPrimitive {
                    id: *id,
                    beautified: beautified.clone(),
                }
            }
            DocumentChange::RemoveSketchPrimitive { id, beautified } => {
                DocumentChange::InsertSketchPrimitive {
                    id: *id,
                    beautified: beautified.clone(),
                }
            }
            DocumentChange::SetSketchConstraint { id, previous, new } => {
                DocumentChange::SetSketchConstraint {
                    id: *id,
                    previous: *new,
                    new: *previous,
                }
            }
        }
    }
}

/// Undo/redo history bound to a specific `Document`.
pub struct DocumentHistory {
    log: TransactionLog<DocumentChange>,
}

impl DocumentHistory {
    pub fn new() -> Self {
        Self {
            log: TransactionLog::new(),
        }
    }

    /// Apply `changes` to `document` as one atomic transaction (Task 056).
    /// If any change fails partway through, every change already applied
    /// in this call is rolled back (via its inverse) before returning the
    /// error, so a failed commit leaves `document` exactly as it was
    /// (Task 061's "only complete prior/new states restore").
    pub fn commit(
        &mut self,
        document: &mut Document,
        changes: Vec<DocumentChange>,
    ) -> DomainResult<TransactionId> {
        let mut transaction = Transaction::new();
        let mut applied: Vec<DocumentChange> = Vec::new();

        for change in changes {
            match change.apply(document) {
                Ok(()) => {
                    applied.push(change.clone());
                    if let Err(err) = transaction.record(change) {
                        Self::rollback(document, &applied);
                        return Err(err);
                    }
                }
                Err(err) => {
                    Self::rollback(document, &applied);
                    return Err(err);
                }
            }
        }

        let committed = transaction.commit()?;
        let id = committed.id;
        document.bump_revision();
        self.log.push(committed);
        Ok(id)
    }

    fn rollback(document: &mut Document, applied: &[DocumentChange]) {
        for change in applied.iter().rev() {
            // Rolling back a just-applied change is expected to always
            // succeed (it is undoing something this exact call just did to
            // this exact document); if it somehow does not, there is
            // nothing more coherent to do than leave the document as-is
            // and let `commit`'s error surface the original failure.
            let _ = change.invert().apply(document);
        }
    }

    /// Undo the most recent transaction. Applies each change's inverse in
    /// reverse order (Task 057), so a transaction of
    /// `[insert A, insert B]` undoes as `[remove B, remove A]`.
    pub fn undo(&mut self, document: &mut Document) -> DomainResult<TransactionId> {
        let committed = self.log.undo()?;
        for change in committed.changes.iter().rev() {
            change.invert().apply(document)?;
        }
        document.bump_revision();
        Ok(committed.id)
    }

    /// Redo the most recently undone transaction. Re-applies the original
    /// forward changes in their original order -- pure data replay, so no
    /// probabilistic recognition (or anything else nondeterministic) ever
    /// re-runs (Task 058).
    pub fn redo(&mut self, document: &mut Document) -> DomainResult<TransactionId> {
        let committed = self.log.redo()?;
        for change in &committed.changes {
            change.apply(document)?;
        }
        document.bump_revision();
        Ok(committed.id)
    }

    pub fn can_undo(&self) -> bool {
        self.log.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.log.can_redo()
    }

    pub fn transaction_count(&self) -> usize {
        self.log.history().len()
    }
}

impl Default for DocumentHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::Note;
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, NoteId};

    fn note_change(page_id: PageId, text: &str) -> (DocumentChange, EntityId) {
        let note = Note::new(NoteId::new(), Point2::ORIGIN, text);
        let id = EntityId::Note(note.id);
        (
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Note(note),
            },
            id,
        )
    }

    #[test]
    fn a_multi_entity_commit_is_one_transaction_and_applies_all_changes() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();

        let (change_a, id_a) = note_change(page_id, "first");
        let (change_b, id_b) = note_change(page_id, "second");
        history
            .commit(&mut document, vec![change_a, change_b])
            .unwrap();

        let page = document.page(page_id).unwrap();
        assert!(page.get(id_a).is_some());
        assert!(page.get(id_b).is_some());
        assert_eq!(history.transaction_count(), 1);
    }

    #[test]
    fn undo_reverses_every_change_in_the_transaction_as_one_unit() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();

        let (change_a, id_a) = note_change(page_id, "first");
        let (change_b, id_b) = note_change(page_id, "second");
        history
            .commit(&mut document, vec![change_a, change_b])
            .unwrap();

        history.undo(&mut document).unwrap();

        let page = document.page(page_id).unwrap();
        assert!(page.get(id_a).is_none());
        assert!(page.get(id_b).is_none());
        assert!(page.is_empty());
    }

    #[test]
    fn redo_restores_the_same_semantic_state_via_pure_replay() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();

        let (change, id) = note_change(page_id, "hello");
        history.commit(&mut document, vec![change]).unwrap();
        history.undo(&mut document).unwrap();
        history.redo(&mut document).unwrap();

        assert!(document.page(page_id).unwrap().get(id).is_some());
    }

    #[test]
    fn a_transaction_spanning_two_pages_undoes_and_redoes_as_one_action() {
        // Stand-in for "cross-view propagation" (Task 057) until Phase 20's
        // real view blocks exist: one transaction touching two distinct
        // containers, undone/redone atomically -- see module docs.
        let mut document = Document::new("Untitled", 0.0);
        let page_a = document.active_page().unwrap();
        let page_b = document.add_page("Page 2");
        let mut history = DocumentHistory::new();

        let (change_a, id_a) = note_change(page_a, "on page a");
        let (change_b, id_b) = note_change(page_b, "on page b");
        history
            .commit(&mut document, vec![change_a, change_b])
            .unwrap();

        assert!(document.page(page_a).unwrap().get(id_a).is_some());
        assert!(document.page(page_b).unwrap().get(id_b).is_some());

        history.undo(&mut document).unwrap();
        assert!(document.page(page_a).unwrap().get(id_a).is_none());
        assert!(document.page(page_b).unwrap().get(id_b).is_none());

        history.redo(&mut document).unwrap();
        assert!(document.page(page_a).unwrap().get(id_a).is_some());
        assert!(document.page(page_b).unwrap().get(id_b).is_some());
    }

    #[test]
    fn set_provenance_undoes_to_the_prior_state_including_none() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();

        let (insert, id) = note_change(page_id, "hello");
        history.commit(&mut document, vec![insert]).unwrap();

        let previous = document.provenance_of(id);
        history
            .commit(
                &mut document,
                vec![DocumentChange::SetProvenance {
                    entity_id: id,
                    previous,
                    new: Some(ProvenanceState::Suggested),
                }],
            )
            .unwrap();
        assert_eq!(document.provenance_of(id), Some(ProvenanceState::Suggested));

        history.undo(&mut document).unwrap();
        assert_eq!(document.provenance_of(id), None);
    }

    #[test]
    fn commit_increments_revision_and_undo_redo_also_change_it() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();
        let start = document.revision();

        let (change, _) = note_change(page_id, "hello");
        history.commit(&mut document, vec![change]).unwrap();
        assert_eq!(document.revision(), start + 1);

        history.undo(&mut document).unwrap();
        assert_eq!(document.revision(), start + 2);
    }

    #[test]
    fn undoing_with_no_history_is_a_structured_error_not_a_panic() {
        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        assert!(history.undo(&mut document).is_err());
    }

    #[test]
    fn a_new_commit_after_undo_clears_the_redo_branch() {
        let mut document = Document::new("Untitled", 0.0);
        let page_id = document.active_page().unwrap();
        let mut history = DocumentHistory::new();

        let (first, _) = note_change(page_id, "first");
        history.commit(&mut document, vec![first]).unwrap();
        history.undo(&mut document).unwrap();
        assert!(history.can_redo());

        let (second, _) = note_change(page_id, "second");
        history.commit(&mut document, vec![second]).unwrap();
        assert!(!history.can_redo());
    }

    // --- Execution 02, Phase 04: view/orthographic/multiview/dimension --

    #[test]
    fn set_view_block_undoes_to_the_prior_state_including_none() {
        use crate::view::{PrincipalViewIdentity, ViewBlock};
        use craftloop_ids::ViewId;

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let view_id = ViewId::new();
        let mut block = ViewBlock::new(view_id);
        block.set_identity(PrincipalViewIdentity::Front);

        history
            .commit(
                &mut document,
                vec![DocumentChange::SetViewBlock {
                    id: view_id,
                    previous: None,
                    new: Some(block.clone()),
                }],
            )
            .unwrap();
        assert_eq!(
            document.view_block(view_id).and_then(|b| b.identity()),
            Some(PrincipalViewIdentity::Front)
        );

        history.undo(&mut document).unwrap();
        assert!(document.view_block(view_id).is_none());

        history.redo(&mut document).unwrap();
        assert_eq!(
            document.view_block(view_id).and_then(|b| b.identity()),
            Some(PrincipalViewIdentity::Front)
        );
    }

    #[test]
    fn set_orthographic_set_undoes_to_the_prior_state_including_none() {
        use crate::view::OrthographicSet;
        use craftloop_ids::OrthographicSetId;

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let set_id = OrthographicSetId::new();
        let set = OrthographicSet::new(set_id);

        history
            .commit(
                &mut document,
                vec![DocumentChange::SetOrthographicSet {
                    id: set_id,
                    previous: None,
                    new: Some(set.clone()),
                }],
            )
            .unwrap();
        assert!(document.orthographic_set(set_id).is_some());

        history.undo(&mut document).unwrap();
        assert!(document.orthographic_set(set_id).is_none());

        history.redo(&mut document).unwrap();
        assert!(document.orthographic_set(set_id).is_some());
    }

    #[test]
    fn set_multiview_binding_undoes_to_the_prior_state_including_none() {
        use craftloop_ids::{DimensionId, ViewId};

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let view_id = ViewId::new();
        let dimension_id = DimensionId::new();

        history
            .commit(
                &mut document,
                vec![DocumentChange::SetMultiviewBinding {
                    view: view_id,
                    axis: SharedAxis::Width,
                    previous: None,
                    new: Some(dimension_id),
                }],
            )
            .unwrap();
        assert_eq!(
            document
                .multiview_graph()
                .axis_of(view_id, SharedAxis::Width),
            Some(dimension_id)
        );

        history.undo(&mut document).unwrap();
        assert_eq!(
            document
                .multiview_graph()
                .axis_of(view_id, SharedAxis::Width),
            None
        );

        history.redo(&mut document).unwrap();
        assert_eq!(
            document
                .multiview_graph()
                .axis_of(view_id, SharedAxis::Width),
            Some(dimension_id)
        );
    }

    #[test]
    fn set_dimension_undoes_to_the_prior_state_including_none() {
        use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget};
        use craftloop_ids::PrimitiveId;

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            25.0,
        )
        .unwrap();
        let dimension_id = dimension.id;

        history
            .commit(
                &mut document,
                vec![DocumentChange::SetDimension {
                    id: dimension_id,
                    previous: None,
                    new: Some(dimension),
                }],
            )
            .unwrap();
        assert_eq!(
            document
                .dimension_store()
                .dimension(dimension_id)
                .map(|d| d.value()),
            Some(25.0)
        );

        history.undo(&mut document).unwrap();
        assert!(document.dimension_store().dimension(dimension_id).is_none());

        history.redo(&mut document).unwrap();
        assert_eq!(
            document
                .dimension_store()
                .dimension(dimension_id)
                .map(|d| d.value()),
            Some(25.0)
        );
    }

    #[test]
    fn insert_sketch_primitive_undoes_and_redoes() {
        use craftloop_geometry::{Point2, Segment2};
        use craftloop_ids::PrimitiveId;
        use craftloop_recognition::BeautifiedPrimitive;

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let id = PrimitiveId::new();
        let beautified = Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 0.0),
            )),
            displacement: 0.0,
        };

        history
            .commit(
                &mut document,
                vec![DocumentChange::InsertSketchPrimitive {
                    id,
                    beautified: beautified.clone(),
                }],
            )
            .unwrap();
        assert_eq!(document.sketch().primitive(id), Some(&beautified));

        history.undo(&mut document).unwrap();
        assert_eq!(document.sketch().primitive(id), None);

        history.redo(&mut document).unwrap();
        assert_eq!(document.sketch().primitive(id), Some(&beautified));
    }

    #[test]
    fn set_sketch_constraint_undoes_to_the_prior_state_including_none() {
        use craftloop_ids::PrimitiveId;
        use craftloop_sketch::{ConstraintProvenance, SketchConstraintKind};

        let mut document = Document::new("Untitled", 0.0);
        let mut history = DocumentHistory::new();
        let primitive_id = PrimitiveId::new();
        let constraint_id = ConstraintId::new();
        let kind = SketchConstraintKind::Horizontal(primitive_id);

        history
            .commit(
                &mut document,
                vec![DocumentChange::SetSketchConstraint {
                    id: constraint_id,
                    previous: None,
                    new: Some((kind, ConstraintProvenance::UserCreated)),
                }],
            )
            .unwrap();
        assert!(document.sketch().constraint(constraint_id).is_some());

        history.undo(&mut document).unwrap();
        assert!(document.sketch().constraint(constraint_id).is_none());

        history.redo(&mut document).unwrap();
        assert!(document.sketch().constraint(constraint_id).is_some());
    }
}
