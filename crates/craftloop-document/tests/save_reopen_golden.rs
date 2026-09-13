//! Save/reopen golden tests.
//!
//! Execution 01, Phase 07, Task 055. Authority: Engine Contract 15; MCP
//! Article 203 "Acceptance Criterion: Persistence".
//!
//! Builds a representative document -- multiple pages, a raw stroke, an
//! accepted beautified primitive, and a note -- persists it, reopens it,
//! and asserts the reopened semantic state is exactly equal to the
//! original. This is the "golden" document Task 055 asks for: a fixture
//! deliberately touching every semantic entity kind this execution has
//! implemented so far, not just one trivial field.

use craftloop_document::{
    load_document, save_document_atomically, Document, EntityId, Note, SemanticEntity,
};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, NoteId, PrimitiveId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{InputCapabilities, MouseSimulator, PointerButtons};
use craftloop_recognition::{beautify, recognize, RecognitionCandidate};

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-golden-{}-{name}.json",
        std::process::id()
    ))
}

fn build_representative_document() -> Document {
    let mut document = Document::new("Golden Fixture Notebook", 1_700_000_000.0);
    let page_id = document
        .active_page()
        .expect("new document has an active page");

    // A raw stroke, built the same way the harness would: through the
    // mouse simulator, so it carries honest (non-authoritative) capability
    // metadata, matching real production provenance.
    let stroke_samples = vec![
        MouseSimulator::sample(Point2::new(0.0, 0.0), 0.0, PointerButtons::default()),
        MouseSimulator::sample(Point2::new(10.0, 0.0), 0.1, PointerButtons::default()),
        MouseSimulator::sample(Point2::new(20.0, 0.0), 0.2, PointerButtons::default()),
    ];
    let stroke = Stroke::new(StrokeId::new(), stroke_samples).unwrap();

    // An accepted, beautified primitive: recognize a clean line and
    // beautify the top candidate, exactly as a real accept flow would.
    let line_points: Vec<Point2> = (0..12)
        .map(|i| Point2::new(i as f64, 2.0 * i as f64))
        .collect();
    let candidates = recognize(&line_points);
    let top = candidates
        .into_iter()
        .find(|c| !matches!(c, RecognitionCandidate::KeepAsInk))
        .expect("a clean line always yields a non-ink candidate");
    let beautified =
        beautify(&top).expect("a Line/Circle/Arc/Rectangle candidate always beautifies");

    let note = Note::new(
        NoteId::new(),
        Point2::new(5.0, 30.0),
        "This line looks load-bearing -- double check.",
    );

    let page = document.page_mut(page_id).unwrap();
    page.insert(SemanticEntity::Stroke(stroke)).unwrap();
    page.insert(SemanticEntity::Primitive {
        id: PrimitiveId::new(),
        beautified,
    })
    .unwrap();
    page.insert(SemanticEntity::Note(note)).unwrap();

    document.add_page("Page 2");

    document
}

#[test]
fn a_representative_document_survives_save_and_reopen_exactly() {
    let path = temp_path("representative");
    let _ = std::fs::remove_file(&path);

    let original = build_representative_document();
    save_document_atomically(&path, &original).unwrap();
    let reopened = load_document(&path).unwrap();

    assert_eq!(original, reopened);
    assert_eq!(reopened.page_count(), 2);

    let page = reopened.page(reopened.active_page().unwrap()).unwrap();
    assert_eq!(page.len(), 3);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}

#[test]
fn every_semantic_entity_kind_is_individually_recoverable_after_reopen() {
    let path = temp_path("entity-kinds");
    let _ = std::fs::remove_file(&path);

    let original = build_representative_document();
    let page_id = original.active_page().unwrap();
    let original_page = original.page(page_id).unwrap();

    let stroke_id = original_page.entities().find_map(|e| match e {
        SemanticEntity::Stroke(s) => Some(s.id),
        _ => None,
    });
    let primitive_id = original_page.entities().find_map(|e| match e {
        SemanticEntity::Primitive { id, .. } => Some(*id),
        _ => None,
    });
    let note_id = original_page.entities().find_map(|e| match e {
        SemanticEntity::Note(n) => Some(n.id),
        _ => None,
    });

    save_document_atomically(&path, &original).unwrap();
    let reopened = load_document(&path).unwrap();
    let reopened_page = reopened.page(page_id).unwrap();

    assert!(matches!(
        reopened_page.get(EntityId::Stroke(stroke_id.unwrap())),
        Some(SemanticEntity::Stroke(_))
    ));
    assert!(matches!(
        reopened_page.get(EntityId::Primitive(primitive_id.unwrap())),
        Some(SemanticEntity::Primitive { .. })
    ));
    assert!(matches!(
        reopened_page.get(EntityId::Note(note_id.unwrap())),
        Some(SemanticEntity::Note(_))
    ));

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}

#[test]
fn reopening_twice_in_a_row_is_stable_no_drift_across_repeated_round_trips() {
    let path = temp_path("stable");
    let _ = std::fs::remove_file(&path);

    let original = build_representative_document();
    save_document_atomically(&path, &original).unwrap();
    let first_reopen = load_document(&path).unwrap();
    save_document_atomically(&path, &first_reopen).unwrap();
    let second_reopen = load_document(&path).unwrap();

    assert_eq!(original, first_reopen);
    assert_eq!(first_reopen, second_reopen);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}

#[test]
fn an_empty_document_also_round_trips() {
    let path = temp_path("empty");
    let _ = std::fs::remove_file(&path);

    let original = Document::new("Blank Notebook", 0.0);
    save_document_atomically(&path, &original).unwrap();
    let reopened = load_document(&path).unwrap();
    assert_eq!(original, reopened);

    let _ = std::fs::remove_file(&path);
}

// InputCapabilities is imported only to document, in one visible assertion,
// that the persisted stroke's capability metadata survives the round trip
// honestly (still non-authoritative for a simulated stroke) -- not just
// that *some* value round-trips.
#[test]
fn persisted_stroke_capabilities_still_honestly_report_simulated_input_after_reopen() {
    let path = temp_path("capabilities-honesty");
    let _ = std::fs::remove_file(&path);

    let original = build_representative_document();
    save_document_atomically(&path, &original).unwrap();
    let reopened = load_document(&path).unwrap();

    let page = reopened.page(reopened.active_page().unwrap()).unwrap();
    let stroke = page.entities().find_map(|e| match e {
        SemanticEntity::Stroke(s) => Some(s),
        _ => None,
    });
    let capabilities = stroke.unwrap().samples()[0].capabilities;
    assert_eq!(capabilities, InputCapabilities::NONE);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}
