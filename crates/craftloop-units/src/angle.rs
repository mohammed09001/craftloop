//! Angle parsing, distinct from length parsing.
//!
//! Execution 01, Phase 09, Task 066. Authority: Engine Contract 07; MCP
//! Article 594 "Requirement Group: Angular Dimensions".
//!
//! Canonical internal angle unit is **radians** (matching
//! `craftloop-geometry`'s existing `Arc2`/trigonometric convention from
//! Phase 02). User-facing angle text is always degrees, so this module's
//! whole job is a clean degrees-in, radians-out boundary that also
//! actively rejects a length suffix showing up where an angle was
//! expected, rather than silently treating "45mm" as "45 degrees."

use craftloop_errors::{DomainError, DomainResult, ParserErrorKind};

use crate::numeric::{parse_decimal, with_original_input, DecimalLocale};

const DEGREE_SUFFIXES: &[&str] = &["°", "deg", "degrees", "degree"];
const LENGTH_SUFFIXES: &[&str] = &["mm", "cm", "m", "in", "\""];

/// Parse `input` as an angle in degrees, returning **radians**. A degree
/// suffix (`°`, `deg`, `degrees`) is optional -- callers typically only
/// invoke this in a context (an angular dimension field) where "degrees"
/// is already the only sensible reading of a bare number, per Engine
/// Contract 07's boundary that the parser itself never chooses *which*
/// geometry a value binds to; it only needs to know it is parsing an
/// angle, which the caller establishes by calling this function at all.
pub fn parse_angle_degrees(input: &str, locale: DecimalLocale) -> DomainResult<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::EmptyInput,
            input: input.to_string(),
            detail: "angle input was empty or whitespace-only".to_string(),
        });
    }

    let lower = trimmed.to_lowercase();
    for suffix in LENGTH_SUFFIXES {
        if lower.ends_with(suffix) {
            return Err(DomainError::Parser {
                kind: ParserErrorKind::InvalidUnit,
                input: input.to_string(),
                detail: format!("{trimmed:?} has a length unit suffix but an angle was expected"),
            });
        }
    }

    let mut numeric_part = trimmed;
    for suffix in DEGREE_SUFFIXES {
        if lower.ends_with(suffix) {
            numeric_part = trimmed[..trimmed.len() - suffix.len()].trim_end();
            break;
        }
    }

    let degrees = parse_decimal(numeric_part, locale).map_err(|e| with_original_input(e, input))?;
    Ok(degrees.to_radians())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_number_is_read_as_degrees() {
        let radians = parse_angle_degrees("90", DecimalLocale::PeriodDecimal).unwrap();
        assert!((radians - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    }

    #[test]
    fn a_degree_symbol_suffix_is_accepted() {
        let radians = parse_angle_degrees("45°", DecimalLocale::PeriodDecimal).unwrap();
        assert!((radians - std::f64::consts::FRAC_PI_4).abs() < 1e-9);
    }

    #[test]
    fn word_suffixes_deg_and_degrees_are_accepted_case_insensitively() {
        let a = parse_angle_degrees("30deg", DecimalLocale::PeriodDecimal).unwrap();
        let b = parse_angle_degrees("30 DEGREES", DecimalLocale::PeriodDecimal).unwrap();
        assert!((a - 30f64.to_radians()).abs() < 1e-9);
        assert!((b - 30f64.to_radians()).abs() < 1e-9);
    }

    #[test]
    fn a_length_suffix_on_an_angle_field_is_a_structured_unit_error() {
        let result = parse_angle_degrees("45mm", DecimalLocale::PeriodDecimal);
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::InvalidUnit,
                ..
            })
        ));
    }

    #[test]
    fn negative_angles_parse_correctly() {
        let radians = parse_angle_degrees("-90", DecimalLocale::PeriodDecimal).unwrap();
        assert!((radians + std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    }

    #[test]
    fn empty_input_is_a_structured_error() {
        assert!(matches!(
            parse_angle_degrees("", DecimalLocale::PeriodDecimal),
            Err(DomainError::Parser {
                kind: ParserErrorKind::EmptyInput,
                ..
            })
        ));
    }
}
