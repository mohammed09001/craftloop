//! Task 213 — Blank page to structured line.
//!
//! Execution 01, Phase 30, Task 213. Exercises input (Phase 03), the
//! stroke model (Phase 05), recognition (Phase 06), beautification
//! (Phase 06), transactions (Phase 08), undo (Phase 08), and persistence
//! (Phase 07) in one continuous pipeline -- a real drawing session from a
//! blank document to a saved-and-reopened structured line, using every
//! crate's own real API (no mocks, no duplicated logic).

use std::path::{Path, PathBuf};

use craftloop_document::{Document, DocumentChange, DocumentHistory, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, PrimitiveId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{MouseSimulator, PointerButtons};
use craftloop_recognition::{beautify, recognize, RecognitionCandidate};

fn temp_doc_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-scenario-213-{}-{name}.json",
        std::process::id()
    ))
}

fn cleanup(path: &Path) {
    let _ = std::fs::remove_file(path);
    let mut backup = path.as_os_str().to_owned();
    backup.push(".bak");
    let _ = std::fs::remove_file(backup);
}

#[test]
fn a_stroke_is_recognized_beautified_committed_undone_and_survives_reopen() {
    // 1. Input + stroke model: a near-straight, real pointer-sample
    //    sequence, not synthetic geometry handed directly to the
    //    recognizer.
    let raw_points: Vec<Point2> = (0..12)
        .map(|i| Point2::new(i as f64 * 2.0, i as f64 * 2.0 + 0.05 * (i as f64 % 2.0)))
        .collect();
    let samples: Vec<_> = raw_points
        .iter()
        .enumerate()
        .map(|(i, &p)| MouseSimulator::sample(p, i as f64 * 0.05, PointerButtons::default()))
        .collect();
    let stroke_id = StrokeId::new();
    let stroke = Stroke::new(stroke_id, samples).unwrap();
    assert_eq!(stroke.len(), 12);

    // 2. Recognition: the raw stroke's own points, fitted for real.
    let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
    let candidates = recognize(&points);
    let line_candidate = candidates
        .iter()
        .find(|c| matches!(c, RecognitionCandidate::Line(_)))
        .expect("a near-straight stroke must produce a real Line candidate");

    // 3. Beautification: a clean geometric primitive from the accepted
    //    candidate.
    let beautified = beautify(line_candidate).expect("a Line candidate always beautifies");

    // 4. Document + transaction: both the raw stroke and the beautified
    //    primitive committed as one atomic, user-visible action.
    let mut document = Document::new("Scenario 213", 0.0);
    let page_id = document.active_page().unwrap();
    let primitive_id = PrimitiveId::new();
    let mut history = DocumentHistory::new();

    let transaction_id = history
        .commit(
            &mut document,
            vec![
                DocumentChange::InsertEntity {
                    page_id,
                    entity: SemanticEntity::Stroke(stroke.clone()),
                },
                DocumentChange::InsertEntity {
                    page_id,
                    entity: SemanticEntity::Primitive {
                        id: primitive_id,
                        beautified: beautified.clone(),
                    },
                },
            ],
        )
        .unwrap();
    assert_eq!(document.page(page_id).unwrap().len(), 2);

    // 5. Undo: one coherent action reverses both inserts together
    //    (Engine Contract 14's "one visible action undoes as one
    //    coherent action").
    let undone_id = history.undo(&mut document).unwrap();
    assert_eq!(undone_id, transaction_id);
    assert_eq!(document.page(page_id).unwrap().len(), 0);
    assert!(history.can_redo());

    // Redo brings the whole action back in one step, proving the undo
    // was a true, replayable inverse, not a destructive rewrite.
    history.redo(&mut document).unwrap();
    assert_eq!(document.page(page_id).unwrap().len(), 2);

    // 6. Persistence: save the post-redo state, reopen it, and confirm
    //    semantic equivalence -- the structured line survives a real
    //    save/reopen cycle, not just an in-memory undo/redo cycle.
    let path = temp_doc_path("roundtrip");
    cleanup(&path);
    craftloop_document::save_document_atomically(&path, &document).unwrap();
    let reopened = craftloop_document::load_document(&path).unwrap();
    cleanup(&path);

    assert_eq!(reopened.page(page_id).unwrap().len(), 2);
    let reopened_page = reopened.page(page_id).unwrap();
    assert!(reopened_page
        .entities()
        .any(|e| matches!(e, SemanticEntity::Stroke(s) if s.id == stroke_id)));
    assert!(reopened_page
        .entities()
        .any(|e| matches!(e, SemanticEntity::Primitive { id, .. } if *id == primitive_id)));
}
