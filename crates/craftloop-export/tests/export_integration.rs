//! End-to-end export tests.
//!
//! Execution 01, Phase 26, Tasks 186-191. Exercises a document with a
//! realistic mix of visible geometry, a dimension, a note, ephemeral
//! command ink, and a diagnostic conflict, the way a real export pass
//! would: through every target this phase implements at once, confirming
//! the exclusion rule (Task 190) and the unit-preservation rule (Task 189)
//! hold consistently across all of them, and that the diagnostic JSON
//! export (Task 186) is the one place that still sees everything.

use std::collections::BTreeSet;

use craftloop_command::{dispose_of_confirmed_command_ink, UndoMetadata};
use craftloop_consistency::{Conflict, ConflictKind, ConflictStatus};
use craftloop_dimension::{
    DimensionAnnotation, DimensionKind, DimensionRole, DimensionTarget, SemanticDimension,
};
use craftloop_document::{Document, DocumentUnits, Note, SemanticEntity};
use craftloop_errors::Severity;
use craftloop_export::{
    export_diagnostic_json, export_pdf_bounded, export_svg, UNSUPPORTED_EXPORT_TARGETS,
};
use craftloop_geometry::Point2;
use craftloop_ids::{
    ConflictId, CraftLoopId, DimensionAnnotationId, DimensionId, NoteId, PrimitiveId, StrokeId,
};
use craftloop_ink::Stroke;
use craftloop_input::{PointerSample, PointerSource};

fn sample_at(x: f64, y: f64) -> PointerSample {
    PointerSample {
        position: Point2::new(x, y),
        timestamp_seconds: 0.0,
        pressure: None,
        tilt_x_deg: None,
        tilt_y_deg: None,
        source: PointerSource::Stylus,
        buttons: Default::default(),
        capabilities: Default::default(),
    }
}

#[test]
fn a_mixed_document_exports_consistently_across_every_target() {
    let mut document = Document::new("Bracket", 0.0);
    document.units = DocumentUnits::Inches;
    let page_id = document.active_page().unwrap();

    // Ordinary, permanent ink.
    let ink_id = StrokeId::new();
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Stroke(
            Stroke::new(ink_id, vec![sample_at(0.0, 0.0), sample_at(20.0, 0.0)]).unwrap(),
        ))
        .unwrap();

    // Ephemeral command ink -- confirmed and disposed of via the Phase 19
    // mechanism, but (as in a real session) not yet physically removed
    // from the page by the time export runs.
    let command_stroke_id = StrokeId::new();
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Stroke(
            Stroke::new(
                command_stroke_id,
                vec![sample_at(50.0, 50.0), sample_at(60.0, 60.0)],
            )
            .unwrap(),
        ))
        .unwrap();
    let disposition = dispose_of_confirmed_command_ink(
        vec![command_stroke_id],
        None,
        UndoMetadata::undoable("Entered Sketch mode"),
    );
    let ephemeral_strokes: BTreeSet<StrokeId> =
        disposition.strokes_to_remove.iter().copied().collect();

    // A note.
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Note(Note::new(
            NoteId::new(),
            Point2::new(0.0, 5.0),
            "M6 THRU ALL",
        )))
        .unwrap();

    // A dimension: exactly 1 inch, stored canonically as 25.4mm.
    let dimension = SemanticDimension::new(
        DimensionId::new(),
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        25.4,
    )
    .unwrap();
    let dimension_id = dimension.id;
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Dimension(dimension))
        .unwrap();
    let annotation = DimensionAnnotation::new(
        DimensionAnnotationId::new(),
        dimension_id,
        Point2::new(10.0, -3.0),
    );

    // A diagnostic conflict -- internal state, never part of the drawing.
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Conflict(Conflict {
            id: ConflictId::new(),
            kind: ConflictKind::DegenerateGeometry,
            severity: Severity::Error,
            affected_entities: vec![ink_id.to_string()],
            existing_truth: "nonzero length".to_string(),
            proposed_truth: "zero length".to_string(),
            evidence: "length = 0".to_string(),
            resolution_choices: Vec::new(),
            status: ConflictStatus::Unresolved,
        }))
        .unwrap();

    let page = document.page(page_id).unwrap();

    // Task 186: diagnostic JSON sees everything, unfiltered.
    let diagnostic = export_diagnostic_json(&document).unwrap();
    assert!(diagnostic.contains("M6 THRU ALL"));
    assert!(
        diagnostic.contains("DegenerateGeometry"),
        "the diagnostic export must still show conflicts -- that is its purpose"
    );

    // Task 187 + 190: SVG shows the note, the permanent ink, and the
    // unit-converted dimension -- never the ephemeral command ink or the
    // conflict.
    let svg = export_svg(
        page,
        document.units,
        std::slice::from_ref(&annotation),
        &ephemeral_strokes,
    );
    assert!(svg.contains("M6 THRU ALL"));
    assert!(
        svg.contains("1.000in"),
        "dimension must show the inch-converted value"
    );
    assert!(
        !svg.contains("25.4"),
        "must not leak the raw millimeter value"
    );
    assert!(
        !svg.contains("50,50"),
        "ephemeral command ink must not appear"
    );
    assert!(!svg.contains("DegenerateGeometry"));

    // Task 188 + 190: the PDF prototype obeys the same rules.
    let pdf_bytes = export_pdf_bounded(page, document.units, &[annotation], &ephemeral_strokes);
    let pdf_text = String::from_utf8(pdf_bytes).unwrap();
    assert!(pdf_text.contains("(M6 THRU ALL)"));
    assert!(pdf_text.contains("(1.000in)"));
    assert!(!pdf_text.contains("50 50 m"));
    assert!(!pdf_text.contains("DegenerateGeometry"));

    // Task 191: DXF's absence is a documented decision, not a silent gap.
    assert!(UNSUPPORTED_EXPORT_TARGETS.iter().any(|e| e.contains("DXF")));
}
