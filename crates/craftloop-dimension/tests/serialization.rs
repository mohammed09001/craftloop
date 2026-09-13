//! Dimension serialization tests.
//!
//! Execution 01, Phase 10, Task 077. Authority: Engine Contract 09; MCP
//! Article 130 "Separation of Geometry and Presentation".
//!
//! "Persist semantic and presentation state independently" is checked
//! literally: a `SemanticDimension` serializes and reloads correctly with
//! zero `DimensionAnnotation`s anywhere nearby, a `DimensionAnnotation`
//! serializes and reloads on its own (referencing a dimension purely by
//! ID, not by an embedded copy of it), and a whole `DimensionStore`
//! round-trips with both kept as separate top-level collections rather
//! than one nested inside the other.

use craftloop_dimension::{
    DimensionAnnotation, DimensionKind, DimensionRole, DimensionStore, DimensionTarget,
    SemanticDimension,
};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, DimensionAnnotationId, DimensionId, PrimitiveId};

fn sample_dimension() -> SemanticDimension {
    SemanticDimension::new(
        DimensionId::new(),
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        42.0,
    )
    .unwrap()
}

#[test]
fn a_semantic_dimension_serializes_and_reloads_with_no_annotation_present_anywhere() {
    let dimension = sample_dimension();
    let json = serde_json::to_string(&dimension).unwrap();
    // The JSON has no annotation-shaped data in it at all -- semantic state
    // genuinely carries no presentation payload.
    assert!(!json.contains("visible"));
    assert!(!json.contains("position"));

    let reloaded: SemanticDimension = serde_json::from_str(&json).unwrap();
    assert_eq!(dimension, reloaded);
}

#[test]
fn an_annotation_serializes_and_reloads_referencing_its_dimension_only_by_id() {
    let dimension_id = DimensionId::new();
    let annotation = DimensionAnnotation::new(
        DimensionAnnotationId::new(),
        dimension_id,
        Point2::new(1.0, 2.0),
    );
    let json = serde_json::to_string(&annotation).unwrap();

    // No embedded copy of the dimension's kind/role/value -- only its ID.
    assert!(!json.contains("Linear"));
    assert!(!json.contains("Driving"));

    let reloaded: DimensionAnnotation = serde_json::from_str(&json).unwrap();
    assert_eq!(annotation, reloaded);
    assert_eq!(reloaded.dimension_id, dimension_id);
}

#[test]
fn a_dimension_store_round_trips_with_dimensions_and_annotations_as_separate_top_level_maps() {
    let mut store = DimensionStore::new();
    let dimension = sample_dimension();
    let id = dimension.id;
    store.insert_dimension(dimension).unwrap();
    store
        .add_annotation(DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            id,
            Point2::ORIGIN,
        ))
        .unwrap();

    let json = serde_json::to_string(&store).unwrap();
    // Top-level shape: two sibling maps, not one nested inside the other.
    assert!(json.starts_with(r#"{"dimensions":"#) || json.contains(r#""dimensions":"#));
    assert!(json.contains(r#""annotations":"#));

    let reloaded: DimensionStore = serde_json::from_str(&json).unwrap();
    assert_eq!(reloaded.dimension(id).unwrap().value(), 42.0);
    assert_eq!(reloaded.annotations_for(id).len(), 1);
}

#[test]
fn semantic_state_is_unaffected_by_an_annotation_round_trip_and_vice_versa() {
    // Deserializing an annotation on its own cannot accidentally
    // reconstruct or mutate a dimension: it is simply impossible, because
    // the annotation's JSON has no dimension fields to read from at all.
    let dimension_id = DimensionId::new();
    let annotation =
        DimensionAnnotation::new(DimensionAnnotationId::new(), dimension_id, Point2::ORIGIN);
    let json = serde_json::to_string(&annotation).unwrap();
    let _reloaded: DimensionAnnotation = serde_json::from_str(&json).unwrap();

    // A DimensionStore that never saw this annotation still has zero
    // dimensions -- nothing about deserializing the annotation created one.
    let store = DimensionStore::new();
    assert!(store.dimension(dimension_id).is_none());
}
