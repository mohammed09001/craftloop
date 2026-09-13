//! Wrong-target regression suite.
//!
//! Execution 01, Phase 18, Task 132. Authority: Engine Contract 09.
//!
//! "Correct number on wrong entity" is a critical class of error: the
//! numeric value the user wrote is exactly right, but the dimension
//! engine bound it to the wrong piece of geometry, and every downstream
//! consumer of that dimension (the constraint solver, the consistency
//! engine, an exported drawing) now silently trusts a wrong association.
//! This suite proves two things end-to-end, against the real
//! `association`/`store` code (not a mock): the association engine
//! itself resists the easy way to get this wrong, and when a wrong
//! association slips through anyway, `DimensionStore::reassign_target`
//! (Task 131) corrects it without losing the dimension's value, role, or
//! annotations.

use craftloop_dimension::{
    associate, AssociationCandidate, AssociationSource, DimensionAnnotation, DimensionKind,
    DimensionRole, DimensionStore, DimensionTarget, SemanticDimension,
};
use craftloop_geometry::{Point2, Segment2};
use craftloop_ids::{CraftLoopId, DimensionAnnotationId, DimensionId, PrimitiveId};
use craftloop_recognition::BeautifiedPrimitive;

fn line(id: PrimitiveId, a: Point2, b: Point2) -> AssociationCandidate {
    AssociationCandidate {
        primitive_id: id,
        primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
        view: None,
    }
}

#[test]
fn a_value_written_near_two_similarly_plausible_lines_never_silently_locks_onto_one() {
    // Two parallel lines a modest distance apart -- a genuinely tricky
    // real-world layout (e.g. two nearby edges of a part). Without
    // explicit selection or a dimension guide, the engine must not
    // collapse to a single confident answer; a caller (a real UI) is
    // expected to require confirmation here, not silently commit.
    let top = PrimitiveId::new();
    let bottom = PrimitiveId::new();
    let candidates = vec![
        line(top, Point2::new(0.0, 30.0), Point2::new(10.0, 30.0)),
        line(bottom, Point2::new(0.0, -30.0), Point2::new(10.0, -30.0)),
    ];
    let results = associate(Point2::new(5.0, 0.0), &candidates, None, None, None);

    assert_eq!(
        results.len(),
        2,
        "both plausible candidates must be surfaced"
    );
    assert!(
        !results[0].is_committable(),
        "an ambiguous pair must not auto-commit to either one"
    );
}

#[test]
fn explicit_selection_prevents_the_wrong_target_from_ever_being_proposed() {
    let correct = PrimitiveId::new();
    let visually_closer_but_wrong = PrimitiveId::new();
    let candidates = vec![
        line(correct, Point2::new(0.0, 50.0), Point2::new(10.0, 50.0)),
        line(
            visually_closer_but_wrong,
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
        ),
    ];
    // The user explicitly selected the far line before writing the
    // value -- the near one, however visually tempting, must never even
    // appear as a candidate.
    let results = associate(
        Point2::new(5.0, 1.0),
        &candidates,
        Some(correct),
        None,
        None,
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].primitive_id, correct);
    assert_eq!(results[0].source, AssociationSource::ExplicitSelection);
}

#[test]
fn a_wrong_association_that_already_committed_is_corrected_without_losing_the_dimensions_value_or_annotations(
) {
    // Simulates the failure this whole suite exists to guard against:
    // somehow (a bad UI moment, a stale selection) a dimension ends up
    // bound to the wrong primitive. Task 131's reassign_target is the
    // fix -- it must not require deleting the dimension and re-entering
    // its value, which would itself be a second, compounding source of
    // error (a typo on re-entry, lost annotations).
    let wrong_target = PrimitiveId::new();
    let correct_target = PrimitiveId::new();

    let mut store = DimensionStore::new();
    let dimension = SemanticDimension::new(
        DimensionId::new(),
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(wrong_target),
        42.5,
    )
    .unwrap();
    let dimension_id = dimension.id;
    store.insert_dimension(dimension).unwrap();
    let annotation = DimensionAnnotation::new(
        DimensionAnnotationId::new(),
        dimension_id,
        Point2::new(3.0, 3.0),
    );
    store.add_annotation(annotation.clone()).unwrap();

    // Discover and fix the wrong association.
    let previous = store
        .reassign_target(dimension_id, DimensionTarget::Single(correct_target))
        .unwrap();
    assert_eq!(previous, DimensionTarget::Single(wrong_target));

    let fixed = store.dimension(dimension_id).unwrap();
    assert_eq!(fixed.target, DimensionTarget::Single(correct_target));
    // The number the user actually wrote is untouched -- that was never
    // wrong, only its target was.
    assert_eq!(fixed.value(), 42.5);
    assert_eq!(fixed.role, DimensionRole::Driving);
    assert_eq!(store.annotations_for(dimension_id), vec![&annotation]);
}
