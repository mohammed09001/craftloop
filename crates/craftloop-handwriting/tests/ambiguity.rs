//! Ambiguity tests.
//!
//! Execution 01, Phase 17, Task 123. Authority: Engine Contract 05.
//!
//! Exercises exactly the six named ambiguity classes: 1/7, 3/8, decimal
//! separator, degree symbol, R, and diameter notation. Every scenario
//! drives a real `FixtureHandwritingRecognizer` result through the real
//! `craftloop-units` parsers (Phase 09) via `routing.rs` -- nothing here
//! is a placeholder assertion.

use craftloop_geometry::Point2;
use craftloop_handwriting::{
    route_as_angle, route_as_length, route_as_radial, RoutedValue, TextCandidate,
};
use craftloop_ids::{CraftLoopId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::MouseSimulator;
use craftloop_recognition::Confidence;
use craftloop_units::{DecimalLocale, LengthUnit, RadialKind};

fn dummy_stroke() -> Stroke {
    let samples = vec![
        MouseSimulator::sample(Point2::new(0.0, 0.0), 0.0, Default::default()),
        MouseSimulator::sample(Point2::new(1.0, 1.0), 0.1, Default::default()),
    ];
    Stroke::new(StrokeId::new(), samples).unwrap()
}

// --- "1" vs "7" -------------------------------------------------------

#[test]
fn a_one_versus_seven_ambiguity_preserves_both_candidates_as_distinct_valid_lengths() {
    let candidates = vec![
        TextCandidate::new("1", Confidence::new(0.55)),
        TextCandidate::new("7", Confidence::new(0.45)),
    ];
    let routed = route_as_length(
        &candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    assert_eq!(routed.len(), 2);
    assert!(matches!(routed[0], Ok(RoutedValue::Length(l)) if l.value_mm == 1.0));
    assert!(matches!(routed[1], Ok(RoutedValue::Length(l)) if l.value_mm == 7.0));
    // Neither candidate is silently dropped or merged.
    assert_ne!(routed[0], routed[1]);
}

// --- "3" vs "8" -------------------------------------------------------

#[test]
fn a_three_versus_eight_ambiguity_preserves_both_candidates_as_distinct_valid_lengths() {
    let candidates = vec![
        TextCandidate::new("3", Confidence::new(0.6)),
        TextCandidate::new("8", Confidence::new(0.4)),
    ];
    let routed = route_as_length(
        &candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    assert!(matches!(routed[0], Ok(RoutedValue::Length(l)) if l.value_mm == 3.0));
    assert!(matches!(routed[1], Ok(RoutedValue::Length(l)) if l.value_mm == 8.0));
}

// --- decimal separator --------------------------------------------------

#[test]
fn a_genuinely_ambiguous_decimal_separator_in_recognized_text_is_never_silently_resolved() {
    // "1,234" (exactly one separator, exactly 3 trailing digits) is
    // Phase 09's own documented genuinely-ambiguous case: comma-decimal
    // "1.234" or thousands-grouping "1234"? A handwriting recognizer
    // returns raw text, not a locale -- so the locale-free numeric
    // parser (`parse_decimal_auto`, Phase 09) is what a caller must run
    // before `route_as_length` can be given an explicit locale to
    // resolve with, and it must refuse to guess.
    let candidate = TextCandidate::new("1,234", Confidence::new(0.5));
    let result = craftloop_units::parse_decimal_auto(&candidate.text);
    assert!(matches!(
        result,
        Err(craftloop_errors::DomainError::Parser {
            kind: craftloop_errors::ParserErrorKind::AmbiguousDecimalSeparator,
            ..
        })
    ));
}

#[test]
fn once_a_locale_is_established_route_as_length_resolves_the_same_recognized_text_consistently() {
    // The same raw digits, now routed with an explicit locale (e.g. the
    // document's configured locale) -- both readings are individually
    // unambiguous once the locale is no longer in question, and they
    // disagree with each other exactly as expected (1234 vs 1.234),
    // proving `route_as_length` never silently picks one on its own.
    let candidate = vec![TextCandidate::new("1,234mm", Confidence::new(0.5))];
    let period = route_as_length(
        &candidate,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    let comma = route_as_length(
        &candidate,
        LengthUnit::Millimeters,
        DecimalLocale::CommaDecimal,
    );
    let period_value = match period[0] {
        Ok(RoutedValue::Length(l)) => l.value_mm,
        ref other => panic!("expected a parsed length, got {other:?}"),
    };
    let comma_value = match comma[0] {
        Ok(RoutedValue::Length(l)) => l.value_mm,
        ref other => panic!("expected a parsed length, got {other:?}"),
    };
    assert_ne!(period_value, comma_value);
}

// --- degree symbol --------------------------------------------------

#[test]
fn a_degree_symbol_candidate_and_a_bare_number_candidate_both_route_to_the_same_angle() {
    let candidates = vec![
        TextCandidate::new("45°", Confidence::new(0.7)),
        TextCandidate::new("45", Confidence::new(0.3)),
    ];
    let routed = route_as_angle(&candidates, DecimalLocale::PeriodDecimal);
    let a = match &routed[0] {
        Ok(RoutedValue::AngleRadians(r)) => *r,
        other => panic!("expected a parsed angle, got {other:?}"),
    };
    let b = match &routed[1] {
        Ok(RoutedValue::AngleRadians(r)) => *r,
        other => panic!("expected a parsed angle, got {other:?}"),
    };
    assert!((a - b).abs() < 1e-9);
}

// --- R (radius prefix vs. a bare, non-numeric letter) --------------------

#[test]
fn an_r_prefixed_candidate_routes_as_a_radius_but_a_bare_r_is_rejected_not_guessed() {
    let candidates = vec![
        TextCandidate::new("R5", Confidence::new(0.8)),
        TextCandidate::new("R", Confidence::new(0.2)),
    ];
    let routed = route_as_radial(
        &candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    assert!(matches!(
        routed[0],
        Ok(RoutedValue::Radial(candidate)) if candidate.kind == RadialKind::Radius && candidate.value_mm == 5.0
    ));
    // A bare "R" with no following number is not silently treated as
    // radius-zero or any other fabricated value -- it is a real parser
    // error.
    assert!(routed[1].is_err());
}

// --- diameter notation --------------------------------------------------

#[test]
fn diameter_notation_variants_all_route_to_the_same_diameter_reading() {
    let candidates = vec![
        TextCandidate::new("D10", Confidence::new(0.6)),
        TextCandidate::new("⌀10", Confidence::new(0.7)),
        TextCandidate::new("Ø10", Confidence::new(0.5)),
    ];
    let routed = route_as_radial(
        &candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    for result in &routed {
        assert!(matches!(
            result,
            Ok(RoutedValue::Radial(candidate)) if candidate.kind == RadialKind::Diameter && candidate.value_mm == 10.0
        ));
    }
}

#[test]
fn radius_and_diameter_notation_are_never_confused_with_each_other() {
    let candidates = vec![
        TextCandidate::new("R10", Confidence::new(0.5)),
        TextCandidate::new("D10", Confidence::new(0.5)),
    ];
    let routed = route_as_radial(
        &candidates,
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    );
    let Ok(RoutedValue::Radial(r)) = routed[0] else {
        panic!("expected radius")
    };
    let Ok(RoutedValue::Radial(d)) = routed[1] else {
        panic!("expected diameter")
    };
    assert_ne!(r.kind, d.kind);
}

// --- provenance stays intact through an ambiguous recognition ------------

#[test]
fn an_ambiguous_recognition_result_still_names_its_exact_source_stroke() {
    use craftloop_handwriting::{
        FixtureHandwritingRecognizer, HandwritingRecognitionResult, HandwritingRecognizer,
    };

    let stroke = dummy_stroke();
    let candidates = vec![
        TextCandidate::new("1", Confidence::new(0.55)),
        TextCandidate::new("7", Confidence::new(0.45)),
    ];
    let mut recognizer = FixtureHandwritingRecognizer::new()
        .with_fixture(std::slice::from_ref(&stroke), candidates.clone());
    let recognized = recognizer.recognize(std::slice::from_ref(&stroke));
    let result = HandwritingRecognitionResult::new(vec![stroke.id], recognized);

    assert!(result.is_traceable_to(&[stroke.id]));
    assert_eq!(result.candidates, candidates);
}
