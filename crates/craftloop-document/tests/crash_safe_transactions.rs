//! Crash-safe transaction tests.
//!
//! Execution 01, Phase 08, Task 061. Authority: Engine Contract 14; the
//! task's own wording: "Simulate interruption during multi-view change and
//! prove only complete prior/new states restore."
//!
//! Real view blocks are Phase 20 (see `history.rs`'s module doc for why),
//! so "multi-view change" is exercised here as a multi-entity, multi-page
//! transaction where one step is engineered to fail partway through --
//! standing in for an interruption mid-propagation. What Task 061 actually
//! asks to be proven is atomicity: a failed transaction must never leave
//! the document holding *some* of its changes -- only the complete prior
//! state (nothing applied) or, on success, the complete new state.

use craftloop_document::{
    Document, DocumentChange, DocumentHistory, EntityId, Note, SemanticEntity,
};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, NoteId, PageId};

fn insert_note(page_id: PageId, text: &str) -> (DocumentChange, EntityId) {
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
fn a_transaction_interrupted_partway_through_leaves_no_partial_changes() {
    let mut document = Document::new("Untitled", 0.0);
    let real_page = document.active_page().unwrap();
    let bogus_page = PageId::new(); // never added to the document

    let (good_change_1, id_1) = insert_note(real_page, "first");
    let (good_change_2, id_2) = insert_note(real_page, "second");
    let (failing_change, _id_3) = insert_note(bogus_page, "never lands");

    let mut history = DocumentHistory::new();
    let result = history.commit(
        &mut document,
        vec![good_change_1, good_change_2, failing_change],
    );

    assert!(
        result.is_err(),
        "a transaction touching an unknown page must fail"
    );

    // Neither of the two changes that succeeded before the interruption
    // are visible -- the "complete prior state," not a partial mix.
    let page = document.page(real_page).unwrap();
    assert!(
        page.is_empty(),
        "no partial changes may remain after a failed transaction"
    );
    assert!(page.get(id_1).is_none());
    assert!(page.get(id_2).is_none());

    // No half-committed transaction entered history either.
    assert_eq!(history.transaction_count(), 0);
    assert!(!history.can_undo());
}

#[test]
fn a_transaction_that_fails_after_a_successful_one_does_not_corrupt_prior_committed_state() {
    let mut document = Document::new("Untitled", 0.0);
    let real_page = document.active_page().unwrap();
    let bogus_page = PageId::new();
    let mut history = DocumentHistory::new();

    // A genuinely successful, committed transaction first.
    let (committed_change, committed_id) = insert_note(real_page, "safely committed");
    history
        .commit(&mut document, vec![committed_change])
        .unwrap();
    let revision_after_first_commit = document.revision();

    // A second transaction that partially succeeds, then fails.
    let (partial_change, partial_id) = insert_note(real_page, "should not survive");
    let (failing_change, _) = insert_note(bogus_page, "never lands");
    let result = history.commit(&mut document, vec![partial_change, failing_change]);
    assert!(result.is_err());

    // The first transaction's entity is still exactly there -- the failed
    // second transaction did not touch or roll back unrelated prior state.
    let page = document.page(real_page).unwrap();
    assert!(page.get(committed_id).is_some());
    // The second transaction's own partial change is not present.
    assert!(page.get(partial_id).is_none());
    assert_eq!(page.len(), 1);

    // Revision did not advance for the failed transaction.
    assert_eq!(document.revision(), revision_after_first_commit);
    assert_eq!(history.transaction_count(), 1);
}

#[test]
fn undo_after_a_failed_transaction_still_correctly_reverts_the_last_real_commit() {
    // Proves the failed-transaction rollback did not corrupt the undo
    // stack itself -- undo still finds and reverts the last *successful*
    // transaction, not some artifact of the failed one.
    let mut document = Document::new("Untitled", 0.0);
    let real_page = document.active_page().unwrap();
    let bogus_page = PageId::new();
    let mut history = DocumentHistory::new();

    let (committed_change, committed_id) = insert_note(real_page, "will be undone");
    history
        .commit(&mut document, vec![committed_change])
        .unwrap();

    let (_, _) = insert_note(bogus_page, "irrelevant"); // unused, just documenting intent
    let (failing_change, _) = insert_note(bogus_page, "fails");
    assert!(history.commit(&mut document, vec![failing_change]).is_err());

    history.undo(&mut document).unwrap();
    assert!(document
        .page(real_page)
        .unwrap()
        .get(committed_id)
        .is_none());
    assert!(!history.can_undo());
}
