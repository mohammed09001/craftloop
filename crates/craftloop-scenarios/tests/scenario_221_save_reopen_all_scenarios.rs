//! Task 221 — Save/reopen all scenarios.
//!
//! Execution 01, Phase 30, Task 221. Persists one combined document
//! carrying every entity kind the other eight scenarios in this phase
//! exercise individually (stroke, beautified primitive, dimension, note,
//! conflict) and verifies full semantic equivalence after reopen --
//! using `Document`'s own derived `PartialEq` for a real, structural
//! equality check across the whole document, not a hand-picked field
//! subset that could hide a real regression.

use std::path::{Path, PathBuf};

use craftloop_consistency::{Conflict, ConflictKind, ConflictStatus};
use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget, SemanticDimension};
use craftloop_document::{Document, Note, SemanticEntity};
use craftloop_errors::Severity;
use craftloop_geometry::{Point2, Segment2};
use craftloop_ids::{ConflictId, CraftLoopId, DimensionId, NoteId, PrimitiveId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{MouseSimulator, PointerButtons};
use craftloop_recognition::{Beautified, BeautifiedPrimitive};

fn temp_doc_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-scenario-221-{}-{name}.json",
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
fn a_document_carrying_every_scenario_entity_kind_round_trips_with_full_equality() {
    let mut document = Document::new("Scenario 221: everything at once", 0.0);
    let page_id = document.active_page().unwrap();
    let page = document.page_mut(page_id).unwrap();

    // Stroke + stroke model (Task 213's slice).
    let stroke = Stroke::new(
        StrokeId::new(),
        vec![
            MouseSimulator::sample(Point2::new(0.0, 0.0), 0.0, PointerButtons::default()),
            MouseSimulator::sample(Point2::new(5.0, 5.0), 0.1, PointerButtons::default()),
        ],
    )
    .unwrap();
    page.insert(SemanticEntity::Stroke(stroke)).unwrap();

    // Beautified primitive (Tasks 213/214/216's slice).
    page.insert(SemanticEntity::Primitive {
        id: PrimitiveId::new(),
        beautified: Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(0.0, 0.0),
                Point2::new(25.0, 0.0),
            )),
            displacement: 0.02,
        },
    })
    .unwrap();

    // Dimension (Tasks 214/215/217/218's slice).
    let dimension = SemanticDimension::new(
        DimensionId::new(),
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        25.0,
    )
    .unwrap();
    page.insert(SemanticEntity::Dimension(dimension)).unwrap();

    // Note (Task 220's slice) -- including the exact word-boundary-safe
    // sentence that scenario proves stays a note.
    page.insert(SemanticEntity::Note(Note::new(
        NoteId::new(),
        Point2::new(0.0, 10.0),
        "Buy 5 pens and 3 rulers today",
    )))
    .unwrap();

    // Conflict (Task 218's slice) -- a resolved one, proving resolution
    // status itself survives the round trip, not just unresolved state.
    let mut conflict = Conflict {
        id: ConflictId::new(),
        kind: ConflictKind::CrossViewMismatch,
        severity: Severity::Error,
        affected_entities: vec!["Width".to_string()],
        existing_truth: "Width = 100".to_string(),
        proposed_truth: "Width = 130".to_string(),
        evidence: "difference = 30".to_string(),
        resolution_choices: vec![
            craftloop_consistency::ResolutionChoice::KeepExisting,
            craftloop_consistency::ResolutionChoice::ReplaceAndPropagate,
        ],
        status: ConflictStatus::Unresolved,
    };
    craftloop_consistency::resolve(
        &mut conflict,
        craftloop_consistency::ResolutionChoice::ReplaceAndPropagate,
    )
    .unwrap();
    page.insert(SemanticEntity::Conflict(conflict)).unwrap();

    assert_eq!(document.page(page_id).unwrap().len(), 5);

    // Save, reopen, and check full structural equality -- not a
    // field-by-field re-derivation of what "equivalent" means, but the
    // same `PartialEq` every other test in this workspace already
    // trusts for `Document`.
    let path = temp_doc_path("everything");
    cleanup(&path);
    craftloop_document::save_document_atomically(&path, &document).unwrap();
    let reopened = craftloop_document::load_document(&path).unwrap();
    cleanup(&path);

    assert_eq!(document, reopened);
    assert_eq!(reopened.page(page_id).unwrap().len(), 5);
}
