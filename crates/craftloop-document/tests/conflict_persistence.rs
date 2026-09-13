//! Conflict persistence tests.
//!
//! Execution 01, Phase 14, Task 106. Authority: MCP Article 27; Engine
//! Contract 15 (Document persistence).
//!
//! Task 106's exact objective: "Unresolved conflicts must survive
//! save/reopen without corrupting last valid geometry." Builds a document
//! holding both a valid primitive and an `Unresolved` `Conflict`
//! referencing it, round-trips through the same atomic
//! save/load path every other entity kind already uses (Task 055,
//! `save_reopen_golden.rs`), and asserts neither degrades: the primitive
//! is untouched, and the conflict is still present, still `Unresolved`,
//! with every field intact.

use craftloop_consistency::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};
use craftloop_document::{
    load_document, save_document_atomically, Document, EntityId, SemanticEntity,
};
use craftloop_errors::Severity;
use craftloop_geometry::{Point2, Segment2};
use craftloop_ids::{ConflictId, CraftLoopId, PrimitiveId};
use craftloop_recognition::{Beautified, BeautifiedPrimitive};

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-conflict-{}-{name}.json",
        std::process::id()
    ))
}

fn valid_line_primitive() -> (PrimitiveId, Beautified) {
    let id = PrimitiveId::new();
    let beautified = Beautified {
        primitive: BeautifiedPrimitive::Line(Segment2::new(
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
        )),
        displacement: 0.0,
    };
    (id, beautified)
}

fn unresolved_conflict(affected: PrimitiveId) -> Conflict {
    Conflict {
        id: ConflictId::new(),
        kind: ConflictKind::DegenerateGeometry,
        severity: Severity::Error,
        affected_entities: vec![format!("{affected:?}")],
        existing_truth: "line has nonzero length".to_string(),
        proposed_truth: "a sibling edit would collapse it to a point".to_string(),
        evidence: "length = 0.0000000001".to_string(),
        resolution_choices: vec![ResolutionChoice::KeepExisting, ResolutionChoice::Cancel],
        status: ConflictStatus::Unresolved,
    }
}

#[test]
fn an_unresolved_conflict_survives_save_and_reopen_without_corrupting_the_geometry_it_references() {
    let path = temp_path("unresolved");
    let _ = std::fs::remove_file(&path);

    let mut document = Document::new("Conflict Fixture", 1_700_000_000.0);
    let page_id = document.active_page().unwrap();
    let (primitive_id, beautified) = valid_line_primitive();
    let conflict = unresolved_conflict(primitive_id);
    let conflict_id = conflict.id;

    let page = document.page_mut(page_id).unwrap();
    page.insert(SemanticEntity::Primitive {
        id: primitive_id,
        beautified: beautified.clone(),
    })
    .unwrap();
    page.insert(SemanticEntity::Conflict(conflict.clone()))
        .unwrap();

    save_document_atomically(&path, &document).unwrap();
    let reopened = load_document(&path).unwrap();

    assert_eq!(document, reopened);

    let reopened_page = reopened.page(page_id).unwrap();
    // The geometry the conflict refers to is untouched.
    assert_eq!(
        reopened_page.get(EntityId::Primitive(primitive_id)),
        Some(&SemanticEntity::Primitive {
            id: primitive_id,
            beautified
        })
    );
    // The conflict itself is still there, still unresolved, every field
    // intact.
    let reopened_conflict = reopened_page.get(EntityId::Conflict(conflict_id));
    assert_eq!(reopened_conflict, Some(&SemanticEntity::Conflict(conflict)));
    match reopened_conflict {
        Some(SemanticEntity::Conflict(c)) => assert!(c.is_unresolved()),
        other => panic!("expected a Conflict entity, got {other:?}"),
    }

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}

#[test]
fn a_resolved_conflict_survives_save_and_reopen_with_its_resolution_choice_intact() {
    let path = temp_path("resolved");
    let _ = std::fs::remove_file(&path);

    let mut document = Document::new("Conflict Fixture", 1_700_000_000.0);
    let page_id = document.active_page().unwrap();
    let (primitive_id, beautified) = valid_line_primitive();
    let mut conflict = unresolved_conflict(primitive_id);
    craftloop_consistency::resolve(&mut conflict, ResolutionChoice::KeepExisting).unwrap();
    let conflict_id = conflict.id;

    let page = document.page_mut(page_id).unwrap();
    page.insert(SemanticEntity::Primitive {
        id: primitive_id,
        beautified,
    })
    .unwrap();
    page.insert(SemanticEntity::Conflict(conflict)).unwrap();

    save_document_atomically(&path, &document).unwrap();
    let reopened = load_document(&path).unwrap();
    let reopened_page = reopened.page(page_id).unwrap();

    match reopened_page.get(EntityId::Conflict(conflict_id)) {
        Some(SemanticEntity::Conflict(c)) => {
            assert_eq!(
                c.status,
                ConflictStatus::Resolved {
                    choice: ResolutionChoice::KeepExisting
                }
            );
        }
        other => panic!("expected a resolved Conflict entity, got {other:?}"),
    }

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}

#[test]
fn repeated_round_trips_of_an_unresolved_conflict_do_not_drift() {
    let path = temp_path("stable");
    let _ = std::fs::remove_file(&path);

    let mut document = Document::new("Conflict Fixture", 1_700_000_000.0);
    let page_id = document.active_page().unwrap();
    let (primitive_id, beautified) = valid_line_primitive();
    let conflict = unresolved_conflict(primitive_id);

    let page = document.page_mut(page_id).unwrap();
    page.insert(SemanticEntity::Primitive {
        id: primitive_id,
        beautified,
    })
    .unwrap();
    page.insert(SemanticEntity::Conflict(conflict)).unwrap();

    save_document_atomically(&path, &document).unwrap();
    let first_reopen = load_document(&path).unwrap();
    save_document_atomically(&path, &first_reopen).unwrap();
    let second_reopen = load_document(&path).unwrap();

    assert_eq!(document, first_reopen);
    assert_eq!(first_reopen, second_reopen);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{}.bak", path.display()));
}
