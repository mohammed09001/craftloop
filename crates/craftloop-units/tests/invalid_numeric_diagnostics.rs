//! Invalid numeric diagnostics.
//!
//! Execution 01, Phase 09, Task 069. Authority: Engine Contract 07;
//! Article 170 "Error Philosophy". "Return structured parse errors that
//! the UI can explain without losing raw input."
//!
//! Cross-cutting proof (not specific to one parser) that every failure
//! mode across every entry point in this crate satisfies both halves of
//! that requirement: the error's `kind` is a typed enum a UI can `match`
//! on to choose an explanation, and `input` is always the exact original
//! string the user typed/wrote -- never lost, truncated, or replaced with
//! a generic message.

use craftloop_errors::DomainError;
use craftloop_units::{
    parse_angle_degrees, parse_decimal, parse_length, parse_radial, DecimalLocale, LengthUnit,
};

fn extract_input(err: &DomainError) -> &str {
    match err {
        DomainError::Parser { input, .. } => input,
        other => panic!("expected a Parser error, got {other:?}"),
    }
}

#[test]
fn every_parser_preserves_the_exact_raw_input_on_failure() {
    let raw_inputs = ["", "   ", "not a number", "abcmm", "5xyz", "R", "45xyz"];

    for raw in raw_inputs {
        if let Err(err) = parse_decimal(raw, DecimalLocale::PeriodDecimal) {
            assert_eq!(
                extract_input(&err),
                raw,
                "parse_decimal lost the raw input for {raw:?}"
            );
        }
        if let Err(err) = parse_length(raw, LengthUnit::Millimeters, DecimalLocale::PeriodDecimal) {
            assert_eq!(
                extract_input(&err),
                raw,
                "parse_length lost the raw input for {raw:?}"
            );
        }
        if let Err(err) = parse_angle_degrees(raw, DecimalLocale::PeriodDecimal) {
            assert_eq!(
                extract_input(&err),
                raw,
                "parse_angle_degrees lost the raw input for {raw:?}"
            );
        }
        if let Err(err) = parse_radial(raw, LengthUnit::Millimeters, DecimalLocale::PeriodDecimal) {
            assert_eq!(
                extract_input(&err),
                raw,
                "parse_radial lost the raw input for {raw:?}"
            );
        }
    }
}

#[test]
fn every_failure_carries_a_typed_kind_a_ui_can_match_on_without_parsing_the_message() {
    let cases: Vec<DomainError> = vec![
        parse_decimal("", DecimalLocale::PeriodDecimal).unwrap_err(),
        parse_decimal("garbage", DecimalLocale::PeriodDecimal).unwrap_err(),
        parse_angle_degrees("45mm", DecimalLocale::PeriodDecimal).unwrap_err(),
        parse_radial("5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap_err(),
    ];

    for err in cases {
        // The mere fact that this matches at all (rather than needing a
        // string comparison against `.to_string()`) is the property under
        // test.
        assert!(matches!(
            err,
            DomainError::Parser { .. } | DomainError::Geometry { .. }
        ));
    }
}

#[test]
fn the_display_message_still_includes_the_raw_input_for_human_reading() {
    let err = parse_length(
        "not-a-length",
        LengthUnit::Millimeters,
        DecimalLocale::PeriodDecimal,
    )
    .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("not-a-length"),
        "human-readable message should still show the raw input: {message}"
    );
}
