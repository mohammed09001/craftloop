//! Task 226 — Test stale-result cancellation.
//!
//! Execution 01, Phase 31, Task 226. Forces an async recognition result
//! and an async correspondence result to "arrive" (in this
//! single-threaded workspace, "arrive" means the caller finally calls
//! `AsyncResult::accept_if_fresh`, Phase 08) only *after* their own
//! source entity has been deleted from the document, and confirms both
//! are discarded -- real `AsyncResult<T>` usage with real recognition
//! (`craftloop_recognition::RecognitionCandidate`) and correspondence
//! (`craftloop_document::CorrespondenceCandidate`) payload types, not a
//! generic placeholder value.

use craftloop_document::{
    stale_result::AsyncResult, CorrespondenceCandidate, Document, DocumentChange, DocumentHistory,
    Evidence, EvidenceKind, SemanticEntity,
};
use craftloop_geometry::{Circle2, Point2, Segment2};
use craftloop_ids::{CraftLoopId, PrimitiveId};
use craftloop_recognition::{Beautified, BeautifiedPrimitive, Confidence, RecognitionCandidate};

#[test]
fn a_recognition_result_arriving_after_its_source_stroke_is_deleted_is_discarded() {
    let mut document = Document::new("Scenario 226", 0.0);
    let page_id = document.active_page().unwrap();
    let mut history = DocumentHistory::new();

    let primitive_id = PrimitiveId::new();
    let entity = SemanticEntity::Primitive {
        id: primitive_id,
        beautified: Beautified {
            primitive: BeautifiedPrimitive::Circle(Circle2::new(Point2::ORIGIN, 5.0).unwrap()),
            displacement: 0.01,
        },
    };
    history
        .commit(
            &mut document,
            vec![DocumentChange::InsertEntity {
                page_id,
                entity: entity.clone(),
            }],
        )
        .unwrap();

    // Background recognition work starts, capturing the document's
    // current revision -- a real `RecognitionCandidate`, not a
    // placeholder string.
    let in_flight = AsyncResult::new(&document, RecognitionCandidate::KeepAsInk);
    assert!(!in_flight.is_stale(&document));

    // The source primitive is deleted before the background work
    // completes -- a real, separate transaction.
    history
        .commit(
            &mut document,
            vec![DocumentChange::RemoveEntity { page_id, entity }],
        )
        .unwrap();

    // The recognition result "arrives" now: stale, and discarded.
    assert!(in_flight.is_stale(&document));
    assert_eq!(in_flight.accept_if_fresh(&document), None);
}

#[test]
fn a_correspondence_result_arriving_after_one_of_its_two_primitives_is_deleted_is_discarded() {
    let mut document = Document::new("Scenario 226b", 0.0);
    let page_id = document.active_page().unwrap();
    let mut history = DocumentHistory::new();

    let source_id = PrimitiveId::new();
    let target_id = PrimitiveId::new();
    let source_entity = SemanticEntity::Primitive {
        id: source_id,
        beautified: Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::ORIGIN,
                Point2::new(10.0, 0.0),
            )),
            displacement: 0.0,
        },
    };
    let target_entity = SemanticEntity::Primitive {
        id: target_id,
        beautified: Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(0.0, 5.0),
                Point2::new(10.0, 5.0),
            )),
            displacement: 0.0,
        },
    };
    history
        .commit(
            &mut document,
            vec![
                DocumentChange::InsertEntity {
                    page_id,
                    entity: source_entity.clone(),
                },
                DocumentChange::InsertEntity {
                    page_id,
                    entity: target_entity,
                },
            ],
        )
        .unwrap();

    // Background correspondence-evaluation work starts.
    let candidate = CorrespondenceCandidate {
        source: source_id,
        target: target_id,
        confidence: Confidence::new(0.8),
        evidence: vec![Evidence {
            kind: EvidenceKind::SharedExtent,
            detail: "matching characteristic length".to_string(),
        }],
    };
    let in_flight = AsyncResult::new(&document, candidate);
    assert!(!in_flight.is_stale(&document));

    // The source primitive is deleted (e.g. the user erased that edge)
    // before the correspondence evaluation completes.
    history
        .commit(
            &mut document,
            vec![DocumentChange::RemoveEntity {
                page_id,
                entity: source_entity,
            }],
        )
        .unwrap();

    assert!(in_flight.is_stale(&document));
    assert_eq!(in_flight.accept_if_fresh(&document), None);
}

#[test]
fn a_result_that_completes_before_any_intervening_change_is_still_accepted() {
    // The negative-of-the-negative case: staleness must be a real,
    // conditional check, not a blanket "always discard" that would make
    // the two tests above meaningless.
    let document = Document::new("Scenario 226c", 0.0);
    let in_flight = AsyncResult::new(&document, RecognitionCandidate::KeepAsInk);
    assert_eq!(
        in_flight.accept_if_fresh(&document),
        Some(RecognitionCandidate::KeepAsInk)
    );
}
