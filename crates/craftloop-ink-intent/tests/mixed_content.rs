//! Mixed-content tests.
//!
//! Execution 01, Phase 16, Task 117. Authority: Engine Contract 04.
//!
//! Task 117 names four specific, genuinely hard cases: "numbers in
//! sentences, command words in notes, letter O versus circle, and
//! circles used for selection." This workspace has no real
//! handwriting-content reader yet (that is Phase 17's job) -- so the
//! honest, testable claim this baseline can make about the first two
//! cases is that it never *fabricates* a `Text`/`Command`/
//! `NumericDimensionCandidate` interpretation it has no real evidence
//! for, at commit-worthy confidence. The latter two cases (`O` vs.
//! circle, circle-for-selection) *are* fully decidable with the
//! deterministic signals this crate has (tool context, Task 114) and are
//! tested as real, resolved classifications, not just restraint.

use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_ink_intent::{
    ContextFeatures, DeterministicBaselineClassifier, IntentCategory, IntentClassifier,
    ToolContext, COMMIT_THRESHOLD,
};
use craftloop_input::MouseSimulator;
use craftloop_recognition::recognize;

fn stroke_from(points: &[Point2]) -> Stroke {
    let samples = points
        .iter()
        .enumerate()
        .map(|(i, p)| MouseSimulator::sample(*p, i as f64 * 0.02, Default::default()))
        .collect();
    Stroke::new(StrokeId::new(), samples).unwrap()
}

fn best_non_ink_confidence(points: &[Point2]) -> Option<craftloop_recognition::Confidence> {
    recognize(points)
        .into_iter()
        .filter(|c| !matches!(c, craftloop_recognition::RecognitionCandidate::KeepAsInk))
        .map(|c| c.confidence())
        .max_by(|a, b| a.get().partial_cmp(&b.get()).unwrap())
}

fn circle_points(center: Point2, radius: f64, count: usize) -> Vec<Point2> {
    (0..=count)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (count as f64);
            Point2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

/// A generic, moderately clean stroke shape -- not chosen to look like
/// any letter or digit in particular, since this baseline has no way to
/// read letters/digits at all. Standing in for "some ordinary
/// handwriting-shaped mark," per the case names ("numbers in sentences",
/// "command words in notes").
fn generic_handwriting_shaped_stroke() -> Stroke {
    let points = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(2.0, 0.0),
        Point2::new(3.0, 1.0),
        Point2::new(4.0, 0.0),
    ];
    stroke_from(&points)
}

#[test]
fn numbers_in_sentences_never_yield_a_committable_numeric_dimension_claim() {
    // No task in this phase, and no capability in this crate, can
    // actually distinguish a handwritten "5" from any other mark -- so
    // the correct, honest behavior is to never claim
    // `NumericDimensionCandidate` at all here, let alone at commit
    // confidence.
    let stroke = generic_handwriting_shaped_stroke();
    let features = ContextFeatures::compute(&stroke, ToolContext::Pen, 0, false, false);
    let classifier = DeterministicBaselineClassifier;
    let candidates = classifier.classify(&stroke, &features, None);

    assert!(
        !candidates.iter().any(|c| c.category == IntentCategory::NumericDimensionCandidate),
        "baseline must never fabricate a NumericDimensionCandidate it cannot actually read: {candidates:?}"
    );
}

#[test]
fn command_words_in_notes_never_yield_a_committable_command_claim() {
    let stroke = generic_handwriting_shaped_stroke();
    let features = ContextFeatures::compute(&stroke, ToolContext::Pen, 0, false, false);
    let classifier = DeterministicBaselineClassifier;
    let candidates = classifier.classify(&stroke, &features, None);

    assert!(
        !candidates
            .iter()
            .any(|c| c.category == IntentCategory::Command),
        "baseline must never fabricate a Command it cannot actually read: {candidates:?}"
    );
    // RawInk must be the top-ranked, committable outcome for ordinary
    // handwriting-shaped ink with no other evidence.
    assert_eq!(candidates[0].category, IntentCategory::RawInk);
}

#[test]
fn a_clean_loop_drawn_with_the_pen_tool_is_offered_as_a_geometry_candidate_not_selection() {
    // "Letter O versus circle": the same closed, clean loop shape. With
    // the pen tool active, it is a real geometry candidate.
    let points = circle_points(Point2::new(0.0, 0.0), 5.0, 24);
    let stroke = stroke_from(&points);
    let geometry_confidence = best_non_ink_confidence(&points);

    let features = ContextFeatures::compute(&stroke, ToolContext::Pen, 0, false, false);
    let classifier = DeterministicBaselineClassifier;
    let candidates = classifier.classify(&stroke, &features, geometry_confidence);

    assert!(candidates
        .iter()
        .any(|c| c.category == IntentCategory::GeometryCandidate));
    assert!(!candidates
        .iter()
        .any(|c| c.category == IntentCategory::SelectionGesture));
}

#[test]
fn the_same_clean_loop_drawn_with_the_selector_tool_is_a_selection_gesture_not_geometry() {
    // "Circles used for selection": identical shape, different tool
    // context -- Task 114's "current tool/context" is exactly the signal
    // that resolves this ambiguity deterministically, without needing to
    // analyze the shape any differently.
    let points = circle_points(Point2::new(0.0, 0.0), 5.0, 24);
    let stroke = stroke_from(&points);
    let geometry_confidence = best_non_ink_confidence(&points);

    let features = ContextFeatures::compute(&stroke, ToolContext::Selector, 0, true, false);
    let classifier = DeterministicBaselineClassifier;
    let candidates = classifier.classify(&stroke, &features, geometry_confidence);

    let top = &candidates[0];
    assert_eq!(top.category, IntentCategory::SelectionGesture);
    assert!(
        top.is_committable(),
        "selection gesture should be committable: {top:?}"
    );
    assert!(
        !candidates
            .iter()
            .any(|c| c.category == IntentCategory::GeometryCandidate),
        "a selector-tool stroke must never also be offered as geometry: {candidates:?}"
    );
}

#[test]
fn every_returned_candidate_list_always_includes_raw_ink() {
    for tool in [
        ToolContext::Pen,
        ToolContext::Eraser,
        ToolContext::Selector,
        ToolContext::Unknown,
    ] {
        let stroke = generic_handwriting_shaped_stroke();
        let features = ContextFeatures::compute(&stroke, tool, 0, false, false);
        let candidates = DeterministicBaselineClassifier.classify(&stroke, &features, None);
        assert!(
            candidates
                .iter()
                .any(|c| c.category == IntentCategory::RawInk),
            "RawInk must always be offered for tool {tool:?}"
        );
    }
}

#[test]
fn candidates_are_ranked_by_descending_confidence() {
    let points = circle_points(Point2::new(0.0, 0.0), 5.0, 24);
    let stroke = stroke_from(&points);
    let features = ContextFeatures::compute(&stroke, ToolContext::Selector, 0, true, false);
    let candidates = DeterministicBaselineClassifier.classify(&stroke, &features, None);
    for window in candidates.windows(2) {
        assert!(window[0].confidence.get() >= window[1].confidence.get());
    }
}

#[test]
fn commit_threshold_matches_the_established_recognition_engine_convention() {
    // Article 96: "confidence" must mean the same thing everywhere.
    assert_eq!(COMMIT_THRESHOLD, 0.6);
}
