//! Route recognized text to the semantic (numeric) parser -- never to
//! geometry directly.
//!
//! Execution 01, Phase 17, Task 121. Authority: Engine Contract 05/07
//! ("Parser does not choose a geometry target").
//!
//! `RoutedValue`'s variants are exactly the semantic value kinds
//! `craftloop-units` (Phase 09) already parses: a length, an angle, or a
//! radius/diameter reading. This module's functions never accept or
//! return a `PrimitiveId`, a `BeautifiedPrimitive`, or any other geometry
//! type -- that omission is deliberate and structural, not an oversight:
//! it is the actual enforcement mechanism behind "do not modify geometry
//! directly from recognizer output." Deciding which primitive a routed
//! value binds to is a downstream (dimension association, Phase 18)
//! decision this crate cannot make even if it wanted to, because it does
//! not have the types to express it.

use craftloop_errors::DomainResult;
use craftloop_units::{
    parse_angle_degrees, parse_length, parse_radial, DecimalLocale, LengthUnit, ParsedLength,
    RadialCandidate,
};

use crate::recognizer::TextCandidate;

/// A recognized string, successfully parsed as one specific kind of
/// semantic value. Which variant a caller asks for is established by
/// calling the matching `route_as_*` function -- exactly like
/// `craftloop-units`' own parser functions, this module never guesses
/// *which* semantic type a string should become.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoutedValue {
    Length(ParsedLength),
    AngleRadians(f64),
    Radial(RadialCandidate),
}

/// Try each candidate, in ranked order, as a length. Returns one
/// `DomainResult` per input candidate, preserving every candidate's own
/// outcome (Task 123: ambiguous input often has more than one plausible
/// reading, and this router must not silently collapse to "the first
/// one that happens to parse").
pub fn route_as_length(
    candidates: &[TextCandidate],
    default_unit: LengthUnit,
    locale: DecimalLocale,
) -> Vec<DomainResult<RoutedValue>> {
    candidates
        .iter()
        .map(|candidate| {
            parse_length(&candidate.text, default_unit, locale).map(RoutedValue::Length)
        })
        .collect()
}

pub fn route_as_angle(
    candidates: &[TextCandidate],
    locale: DecimalLocale,
) -> Vec<DomainResult<RoutedValue>> {
    candidates
        .iter()
        .map(|candidate| {
            parse_angle_degrees(&candidate.text, locale).map(RoutedValue::AngleRadians)
        })
        .collect()
}

pub fn route_as_radial(
    candidates: &[TextCandidate],
    default_unit: LengthUnit,
    locale: DecimalLocale,
) -> Vec<DomainResult<RoutedValue>> {
    candidates
        .iter()
        .map(|candidate| {
            parse_radial(&candidate.text, default_unit, locale).map(RoutedValue::Radial)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_recognition::Confidence;

    #[test]
    fn a_length_candidate_routes_to_a_parsed_length() {
        let candidates = vec![TextCandidate::new("12.5mm", Confidence::new(0.9))];
        let routed = route_as_length(
            &candidates,
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        );
        assert_eq!(routed.len(), 1);
        assert!(matches!(routed[0], Ok(RoutedValue::Length(l)) if l.value_mm == 12.5));
    }

    #[test]
    fn every_candidate_gets_its_own_independent_outcome() {
        // One plausible, one garbage -- the garbage one must not corrupt
        // or suppress the plausible one's own result.
        let candidates = vec![
            TextCandidate::new("10mm", Confidence::new(0.8)),
            TextCandidate::new("not a number", Confidence::new(0.2)),
        ];
        let routed = route_as_length(
            &candidates,
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        );
        assert!(routed[0].is_ok());
        assert!(routed[1].is_err());
    }

    #[test]
    fn radial_notation_routes_correctly() {
        let candidates = vec![TextCandidate::new("R5", Confidence::new(0.9))];
        let routed = route_as_radial(
            &candidates,
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
        );
        assert!(matches!(
            routed[0],
            Ok(RoutedValue::Radial(RadialCandidate {
                kind: craftloop_units::RadialKind::Radius,
                value_mm
            })) if value_mm == 5.0
        ));
    }

    #[test]
    fn angle_notation_routes_correctly() {
        let candidates = vec![TextCandidate::new("45°", Confidence::new(0.9))];
        let routed = route_as_angle(&candidates, DecimalLocale::PeriodDecimal);
        assert!(
            matches!(routed[0], Ok(RoutedValue::AngleRadians(r)) if (r - std::f64::consts::FRAC_PI_4).abs() < 1e-9)
        );
    }
}
