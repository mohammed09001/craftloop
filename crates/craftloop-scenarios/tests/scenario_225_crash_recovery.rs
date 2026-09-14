//! Task 225 — Test crash recovery.
//!
//! Execution 01, Phase 31, Task 225. Interrupts persistence *during a
//! sequence of transactions* and verifies atomic recovery -- an
//! end-to-end combination of `DocumentHistory`'s in-memory transaction
//! atomicity (Phase 08) and `save_document_atomically`/`load_document`'s
//! file-level crash recovery (Phase 07, Task 053), which until this
//! phase were only ever tested independently of each other.
//!
//! **A real, precise finding from writing this test carefully, not
//! assumed:** `save_document_atomically`'s temp-file + `fs::rename`
//! design means the *primary* file itself can never be observed torn or
//! half-written by a crash during that function -- a crash either
//! happens before the atomic rename (primary untouched) or the rename
//! itself is atomic on this filesystem (primary fully becomes the new
//! content). So "the primary is corrupted" can only realistically
//! represent an *external* cause (disk error, manual truncation, a
//! non-atomic writer touching the file some other way), not a torn
//! write from this mechanism itself -- and when that external
//! corruption happens **after** the most recent successful save, this
//! test's own assertions show recovery falls back to `.bak`, which is
//! the state as of the **second-most-recent** save (`.bak` is
//! overwritten with the outgoing primary at the *start* of every save,
//! before the new content becomes primary) -- not the most recent one.
//! Recovery is never partial or inconsistent, but it can roll back
//! further than "just the last unsaved transaction" if the newest saved
//! copy itself becomes unreadable afterward. That distinction is worth
//! stating precisely rather than assuming "recovery gets you the latest
//! save."

use std::path::{Path, PathBuf};

use craftloop_document::{Document, DocumentChange, DocumentHistory, Note, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, NoteId};

fn temp_doc_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-scenario-225-{}-{name}.json",
        std::process::id()
    ))
}

fn backup_path(path: &Path) -> PathBuf {
    let mut backup = path.as_os_str().to_owned();
    backup.push(".bak");
    PathBuf::from(backup)
}

fn cleanup(path: &Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(backup_path(path));
}

fn commit_note(
    history: &mut DocumentHistory,
    document: &mut Document,
    page_id: craftloop_ids::PageId,
    text: &str,
) {
    history
        .commit(
            document,
            vec![DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Note(Note::new(NoteId::new(), Point2::ORIGIN, text)),
            }],
        )
        .unwrap();
}

#[test]
fn corruption_after_the_newest_save_recovers_the_second_most_recent_save_not_a_torn_state() {
    let path = temp_doc_path("interrupted");
    cleanup(&path);

    let mut document = Document::new("Scenario 225", 0.0);
    let page_id = document.active_page().unwrap();
    let mut history = DocumentHistory::new();

    // Transaction A, saved -- no `.bak` exists yet (nothing to back up).
    commit_note(&mut history, &mut document, page_id, "first");
    craftloop_document::save_document_atomically(&path, &document).unwrap();
    let state_after_a = document.clone();

    // Transaction B, saved -- `.bak` now holds state A (copied from the
    // outgoing primary *before* B replaces it); primary now holds B.
    commit_note(&mut history, &mut document, page_id, "second");
    craftloop_document::save_document_atomically(&path, &document).unwrap();

    // Transaction C: committed in memory (Phase 08's own atomicity
    // already guarantees it is fully applied or not applied at all to
    // `document`) but never saved -- representing an in-flight edit at
    // the moment of the "crash" below.
    commit_note(&mut history, &mut document, page_id, "third");

    // Simulate an external corruption of the primary file after B's
    // save completed (see module doc: this cannot be a torn write from
    // `save_document_atomically` itself).
    std::fs::write(&path, b"{ this is not valid JSON }").unwrap();

    let recovered = craftloop_document::load_document(&path).unwrap();
    cleanup(&path);

    // Recovery lands on state A (the `.bak` generation), not state B and
    // certainly not the never-saved state C -- proven exactly, not
    // approximately.
    assert_eq!(recovered, state_after_a);
    assert_eq!(recovered.page(page_id).unwrap().len(), 1);
    assert!(recovered
        .page(page_id)
        .unwrap()
        .entities()
        .any(|e| matches!(e, SemanticEntity::Note(n) if n.text == "first")));
    assert!(recovered
        .page(page_id)
        .unwrap()
        .entities()
        .all(|e| !matches!(e, SemanticEntity::Note(n) if n.text == "second" || n.text == "third")));
}

#[test]
fn a_corrupted_backup_never_blocks_loading_a_healthy_primary() {
    // The inverse failure mode: `.bak` itself is damaged (e.g. the
    // best-effort `fs::copy` step was interrupted), but the primary --
    // the real source of truth -- is perfectly fine. `load_document`
    // must succeed by reading the primary directly and never even
    // consult a healthy-vs-not `.bak` in that case.
    let path = temp_doc_path("healthy-primary-bad-backup");
    cleanup(&path);

    let mut document = Document::new("Scenario 225b", 0.0);
    let page_id = document.active_page().unwrap();
    let mut history = DocumentHistory::new();
    commit_note(&mut history, &mut document, page_id, "intact");
    craftloop_document::save_document_atomically(&path, &document).unwrap();

    std::fs::write(backup_path(&path), b"not valid json at all").unwrap();

    let recovered = craftloop_document::load_document(&path).unwrap();
    cleanup(&path);
    assert_eq!(recovered, document);
}

#[test]
fn recovery_is_a_real_error_not_a_silent_default_when_both_copies_are_corrupted() {
    let path = temp_doc_path("both-corrupted");
    cleanup(&path);
    std::fs::write(&path, b"not json").unwrap();
    std::fs::write(backup_path(&path), b"also not json").unwrap();

    let result = craftloop_document::load_document(&path);
    cleanup(&path);
    assert!(
        result.is_err(),
        "with no recoverable copy at all, this must be a real, surfaced error -- never a fabricated blank document"
    );
}
