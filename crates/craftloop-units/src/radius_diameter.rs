//! Radius and diameter notation parsing.
//!
//! Execution 01, Phase 09, Task 067. Authority: Engine Contract 07 ("Parser
//! does not choose a geometry target"); MCP Article 595 "Requirement
//! Group: Radius and Diameter".
//!
//! Recognizes the standard drafting prefixes -- `R` for radius, `D`/`⌀`/`Ø`
//! for diameter -- and returns a [`RadialCandidate`] naming *which* was
//! written. This module deliberately does **not** decide which circle/arc
//! a parsed value attaches to: per Engine Contract 07's authority
//! boundary, that is a downstream (dimension association, Phase 18)
//! decision. It is also only ever meant to be invoked once a caller has
//! already decided a token plausibly *is* radial notation (e.g. one
//! candidate word from handwriting recognition) -- it is not a general
//! text scanner run over arbitrary notes, where a stray leading "R" or "D"
//! would be common and meaningless.

use craftloop_errors::{DomainError, DomainResult, GeometryErrorKind, ParserErrorKind};
use serde::{Deserialize, Serialize};

use crate::length::parse_length;
use crate::length_unit::LengthUnit;
use crate::numeric::{with_original_input, DecimalLocale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadialKind {
    Radius,
    Diameter,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RadialCandidate {
    pub kind: RadialKind,
    pub value_mm: f64,
}

/// Parse a radius/diameter token such as `"R5"`, `"R 12.5mm"`, `"D10"`, or
/// `"⌀8"`.
pub fn parse_radial(
    input: &str,
    default_unit: LengthUnit,
    locale: DecimalLocale,
) -> DomainResult<RadialCandidate> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::EmptyInput,
            input: input.to_string(),
            detail: "radial input was empty or whitespace-only".to_string(),
        });
    }

    let (kind, rest) = if let Some(rest) = strip_prefix_ci(trimmed, "R") {
        (RadialKind::Radius, rest)
    } else if let Some(rest) = strip_any_prefix(trimmed, &['⌀', 'Ø', 'ø']) {
        (RadialKind::Diameter, rest)
    } else if let Some(rest) = strip_prefix_ci(trimmed, "D") {
        (RadialKind::Diameter, rest)
    } else {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::InvalidUnit,
            input: input.to_string(),
            detail: format!("{trimmed:?} has no recognized radius/diameter prefix (R, D, ⌀, Ø)"),
        });
    };

    let parsed_length = parse_length(rest.trim_start(), default_unit, locale)
        .map_err(|e| with_original_input(e, input))?;
    if parsed_length.value_mm <= 0.0 {
        return Err(DomainError::Geometry {
            kind: GeometryErrorKind::ToleranceViolation,
            detail: format!(
                "radius/diameter must be positive, got {} mm",
                parsed_length.value_mm
            ),
        });
    }

    Ok(RadialCandidate {
        kind,
        value_mm: parsed_length.value_mm,
    })
}

fn strip_prefix_ci<'a>(input: &'a str, prefix: &str) -> Option<&'a str> {
    let mut chars = input.chars();
    let first = chars.next()?;
    if first.to_string().eq_ignore_ascii_case(prefix) {
        Some(chars.as_str())
    } else {
        None
    }
}

fn strip_any_prefix<'a>(input: &'a str, candidates: &[char]) -> Option<&'a str> {
    let mut chars = input.chars();
    let first = chars.next()?;
    if candidates.contains(&first) {
        Some(chars.as_str())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_prefix_is_read_as_radius() {
        let candidate =
            parse_radial("R5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_eq!(candidate.kind, RadialKind::Radius);
        assert_eq!(candidate.value_mm, 5.0);
    }

    #[test]
    fn d_prefix_is_read_as_diameter() {
        let candidate =
            parse_radial("D10", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_eq!(candidate.kind, RadialKind::Diameter);
        assert_eq!(candidate.value_mm, 10.0);
    }

    #[test]
    fn diameter_symbol_prefixes_are_recognized() {
        for symbol in ["⌀8", "Ø8", "ø8"] {
            let candidate = parse_radial(
                symbol,
                LengthUnit::Millimeters,
                DecimalLocale::PeriodDecimal,
            )
            .unwrap();
            assert_eq!(candidate.kind, RadialKind::Diameter, "failed for {symbol}");
            assert_eq!(candidate.value_mm, 8.0);
        }
    }

    #[test]
    fn radius_and_diameter_are_never_confused_for_the_same_prefix() {
        let r = parse_radial("R5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        let d = parse_radial("D5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).unwrap();
        assert_ne!(r.kind, d.kind);
    }

    #[test]
    fn an_explicit_unit_after_the_prefix_is_honored() {
        let candidate = parse_radial(
            "R 1cm",
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        )
        .unwrap();
        assert_eq!(candidate.value_mm, 10.0);
    }

    #[test]
    fn a_zero_or_negative_value_is_rejected() {
        assert!(parse_radial("R0", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).is_err());
        assert!(
            parse_radial("R-5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal).is_err()
        );
    }

    #[test]
    fn input_without_a_radial_prefix_is_a_structured_unit_error() {
        let result = parse_radial("5", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal);
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::InvalidUnit,
                ..
            })
        ));
    }

    #[test]
    fn empty_input_is_a_structured_error() {
        assert!(matches!(
            parse_radial("", LengthUnit::Millimeters, DecimalLocale::PeriodDecimal),
            Err(DomainError::Parser {
                kind: ParserErrorKind::EmptyInput,
                ..
            })
        ));
    }
}
