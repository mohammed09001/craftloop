//! Task 214 — Handwritten-dimension stub scenario.
//!
//! Execution 01, Phase 30, Task 214. Uses deterministic handwriting
//! fixtures (`FixtureHandwritingRecognizer`, Phase 17) to exercise
//! parse (Phase 09), association (Phase 18), solver (Phase 11/12),
//! annotation (Phase 10), undo (Phase 08), and reopen (Phase 07) -- on
//! this Windows development machine, matching the task's own "on
//! Windows" scope (no real handwriting recognizer exists to call into
//! here; see `FixtureHandwritingRecognizer`'s own doc comment).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use craftloop_constraint::{
    ConstraintRequest, ConstraintSolver, GeometricConstraint, PointVariables, Variable, VariableId,
};
use craftloop_dimension::{associate, AssociationCandidate};
use craftloop_dimension::{
    DimensionAnnotation, DimensionKind, DimensionRole, DimensionStore, DimensionTarget,
    SemanticDimension,
};
use craftloop_document::{Document, DocumentChange, DocumentHistory, SemanticEntity};
use craftloop_geometry::{Point2, Segment2};
use craftloop_handwriting::{
    route_as_length, FixtureHandwritingRecognizer, HandwritingRecognizer, RoutedValue,
    TextCandidate,
};
use craftloop_ids::{
    ConstraintId, CraftLoopId, DimensionAnnotationId, DimensionId, PrimitiveId, StrokeId,
};
use craftloop_ink::Stroke;
use craftloop_input::{MouseSimulator, PointerButtons};
use craftloop_recognition::{BeautifiedPrimitive, Confidence};
use craftloop_sketch::EzpzSolver;
use craftloop_units::{DecimalLocale, LengthUnit};

fn temp_doc_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "craftloop-scenario-214-{}-{name}.json",
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
fn a_handwritten_dimension_fixture_flows_through_parse_association_solver_annotation_undo_and_reopen(
) {
    // Setup: one existing line primitive, roughly (but not exactly) 25mm
    // long -- the solver's job below is to correct it to exactly 25mm.
    let primitive_id = PrimitiveId::new();
    let initial_line = Segment2::new(Point2::new(0.0, 0.0), Point2::new(24.0, 0.0));

    // 1. Handwriting fixture -> parse (Phase 09/17): a deterministic
    //    stub, never a real recognizer, per this crate's own contract.
    let handwriting_stroke = Stroke::new(
        StrokeId::new(),
        vec![MouseSimulator::sample(
            Point2::new(12.0, -5.0),
            0.0,
            PointerButtons::default(),
        )],
    )
    .unwrap();
    let mut recognizer = FixtureHandwritingRecognizer::new().with_fixture(
        std::slice::from_ref(&handwriting_stroke),
        vec![TextCandidate::new("25mm", Confidence::new(0.95))],
    );
    let text_candidates = recognizer.recognize(std::slice::from_ref(&handwriting_stroke));
    assert_eq!(text_candidates.len(), 1);

    let routed = route_as_length(
        &text_candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    let parsed_value_mm = match routed.into_iter().next().unwrap().unwrap() {
        RoutedValue::Length(parsed) => parsed.value_mm,
        other => panic!("expected a Length, got {other:?}"),
    };
    assert_eq!(parsed_value_mm, 25.0);

    // 2. Association (Phase 18): the handwritten text's position is
    //    near the line's own midpoint -- proximity + orientation
    //    evidence should bind it to this primitive, with no explicit
    //    selection or dimension-guide short-circuit needed.
    let candidates = vec![AssociationCandidate {
        primitive_id,
        primitive: BeautifiedPrimitive::Line(initial_line),
        view: None,
    }];
    let text_position = Point2::new(12.0, -5.0);
    let association_results = associate(text_position, &candidates, None, None, None);
    let top = association_results
        .first()
        .expect("at least one association candidate");
    assert_eq!(top.primitive_id, primitive_id);
    assert!(
        top.is_committable(),
        "a close, well-oriented candidate must be commit-worthy, not merely suggested"
    );

    // 3. Solver (Phase 11/12): correct the line's actual length to the
    //    handwriting-parsed value via the real `ezpz`-backed solver, not
    //    a hand-computed replacement.
    let (ax, ay, bx, by) = (VariableId(0), VariableId(1), VariableId(2), VariableId(3));
    let a = PointVariables::new(ax, ay);
    let b = PointVariables::new(bx, by);
    let variables = vec![
        Variable::new(ax, initial_line.a.x),
        Variable::new(ay, initial_line.a.y),
        Variable::new(bx, initial_line.b.x),
        Variable::new(by, initial_line.b.y),
    ];
    let constraints = vec![
        ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::FixedValue {
                variable: ax,
                value: 0.0,
            },
        },
        ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::FixedValue {
                variable: ay,
                value: 0.0,
            },
        },
        ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::Horizontal { a, b },
        },
        ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::Distance {
                a,
                b,
                value: parsed_value_mm,
            },
        },
    ];
    let mut solver = EzpzSolver;
    let solve_result = solver.solve(&variables, &constraints);
    assert_eq!(
        solve_result.status,
        craftloop_constraint::SolveStatus::Solved
    );
    let solved_length = solved_distance(&solve_result.values, a, b);
    assert!(
        (solved_length - 25.0).abs() < 1e-6,
        "solved length was {solved_length}, expected 25.0"
    );

    // 4. Annotation (Phase 10): a driving dimension at the solved value,
    //    with a visible annotation at the handwriting's own position.
    let mut store = DimensionStore::new();
    let dimension_id = DimensionId::new();
    let dimension = SemanticDimension::new(
        dimension_id,
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(primitive_id),
        solved_length,
    )
    .unwrap();
    store.insert_dimension(dimension).unwrap();
    let annotation =
        DimensionAnnotation::new(DimensionAnnotationId::new(), dimension_id, text_position);
    store.add_annotation(annotation).unwrap();
    assert_eq!(store.annotations_for(dimension_id).len(), 1);

    // 5. Undo (Phase 08): commit the dimension into the document as one
    //    transaction, then undo it -- one coherent action reversed.
    let mut document = Document::new("Scenario 214", 0.0);
    let page_id = document.active_page().unwrap();
    let mut history = DocumentHistory::new();
    let semantic_dimension = store.dimension(dimension_id).unwrap().clone();
    history
        .commit(
            &mut document,
            vec![DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Dimension(semantic_dimension),
            }],
        )
        .unwrap();
    assert_eq!(document.page(page_id).unwrap().len(), 1);
    history.undo(&mut document).unwrap();
    assert_eq!(document.page(page_id).unwrap().len(), 0);
    history.redo(&mut document).unwrap();
    assert_eq!(document.page(page_id).unwrap().len(), 1);

    // 6. Reopen (Phase 07), on this Windows machine.
    let path = temp_doc_path("roundtrip");
    cleanup(&path);
    craftloop_document::save_document_atomically(&path, &document).unwrap();
    let reopened = craftloop_document::load_document(&path).unwrap();
    cleanup(&path);
    let reopened_dimension = reopened
        .page(page_id)
        .unwrap()
        .entities()
        .find_map(|e| match e {
            SemanticEntity::Dimension(d) if d.id == dimension_id => Some(d),
            _ => None,
        })
        .expect("the dimension must survive save/reopen");
    // Same tolerance as the solved-length check above: the numeric
    // solver converges to within floating-point precision of 25.0, not
    // bit-for-bit exact, and persistence must not lose or add precision
    // beyond that.
    assert!((reopened_dimension.value() - 25.0).abs() < 1e-6);
}

fn solved_distance(
    values: &BTreeMap<VariableId, f64>,
    a: PointVariables,
    b: PointVariables,
) -> f64 {
    let ax = values[&a.x];
    let ay = values[&a.y];
    let bx = values[&b.x];
    let by = values[&b.y];
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}
