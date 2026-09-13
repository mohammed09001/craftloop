//! Explicit unit parsing for lengths.
//!
//! Execution 01, Phase 09, Task 065. Authority: Engine Contract 07; MCP
//! Article 593 "Requirement Group: Linear Dimensions".
//!
//! Forbidden shortcut this module exists to close: "without silent scale
//! changes." If a unit is written (`"50mm"`), that unit is honored
//! *regardless* of the document's configured default -- writing "50mm" in
//! a centimeter-default document must never silently become 50 cm. Only a
//! bare number with no unit at all falls back to the caller-supplied
//! default.

use craftloop_errors::{DomainError, DomainResult, ParserErrorKind};
use serde::{Deserialize, Serialize};

use crate::length_unit::LengthUnit;
use crate::numeric::{parse_decimal, with_original_input, DecimalLocale};

/// Recognized suffixes, longest first so `"mm"` is matched before the
/// bare `"m"` it contains.
const UNIT_SUFFIXES: &[(&str, LengthUnit)] = &[
    ("mm", LengthUnit::Millimeters),
    ("cm", LengthUnit::Centimeters),
    ("in", LengthUnit::Inches),
    ("\"", LengthUnit::Inches),
    ("m", LengthUnit::Meters),
];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParsedLength {
    pub value_mm: f64,
    /// The unit actually written in the input, or `None` if the input was
    /// a bare number interpreted against the caller's default unit.
    pub unit_written: Option<LengthUnit>,
}

/// Parse `input` as a length. `default_unit` is used **only** when `input`
/// has no recognizable unit suffix at all.
pub fn parse_length(
    input: &str,
    default_unit: LengthUnit,
    locale: DecimalLocale,
) -> DomainResult<ParsedLength> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::EmptyInput,
            input: input.to_string(),
            detail: "length input was empty or whitespace-only".to_string(),
        });
    }

    let lower = trimmed.to_lowercase();
    for (suffix, unit) in UNIT_SUFFIXES {
        if lower.ends_with(suffix) {
            let numeric_part = trimmed[..trimmed.len() - suffix.len()].trim_end();
            let value =
                parse_decimal(numeric_part, locale).map_err(|e| with_original_input(e, input))?;
            return Ok(ParsedLength {
                value_mm: unit.to_millimeters(value),
                unit_written: Some(*unit),
            });
        }
    }

    let value = parse_decimal(trimmed, locale).map_err(|e| with_original_input(e, input))?;
    Ok(ParsedLength {
        value_mm: default_unit.to_millimeters(value),
        unit_written: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_number_uses_the_default_unit() {
        let parsed =
            parse_length("50", LengthUnit::Centimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_eq!(parsed.value_mm, 500.0);
        assert_eq!(parsed.unit_written, None);
    }

    #[test]
    fn an_explicit_unit_is_honored_regardless_of_the_default() {
        // The core "no silent scale change" invariant: writing "50mm" in a
        // centimeter-default document must still mean 50 millimeters, not
        // 50 centimeters.
        let parsed = parse_length(
            "50mm",
            LengthUnit::Centimeters,
            DecimalLocale::PeriodDecimal,
        )
        .unwrap();
        assert_eq!(parsed.value_mm, 50.0);
        assert_eq!(parsed.unit_written, Some(LengthUnit::Millimeters));
    }

    #[test]
    fn centimeters_convert_correctly() {
        let parsed =
            parse_length("5cm", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_eq!(parsed.value_mm, 50.0);
    }

    #[test]
    fn meters_are_distinguished_from_millimeters_despite_shared_letter() {
        let parsed =
            parse_length("2m", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_eq!(parsed.value_mm, 2000.0);
        assert_eq!(parsed.unit_written, Some(LengthUnit::Meters));
    }

    #[test]
    fn inches_accept_both_the_word_and_the_double_quote_symbol() {
        let word =
            parse_length("2in", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        let symbol =
            parse_length("2\"", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert!((word.value_mm - 50.8).abs() < 1e-9);
        assert!((symbol.value_mm - 50.8).abs() < 1e-9);
    }

    #[test]
    fn decimal_values_with_units_parse_correctly() {
        let parsed = parse_length(
            "12.5mm",
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        )
        .unwrap();
        assert_eq!(parsed.value_mm, 12.5);
    }

    #[test]
    fn empty_input_is_a_structured_error() {
        assert!(matches!(
            parse_length("", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal),
            Err(DomainError::Parser {
                kind: ParserErrorKind::EmptyInput,
                ..
            })
        ));
    }

    #[test]
    fn garbage_after_a_valid_unit_suffix_is_an_invalid_number_error() {
        let result = parse_length(
            "abcmm",
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        );
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::InvalidNumber,
                ..
            })
        ));
    }
}
