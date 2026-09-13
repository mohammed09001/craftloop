//! Unit consistency.
//!
//! Execution 01, Phase 14, Task 103. Authority: MCP Article 27; the
//! "no silent scale change" invariant `craftloop-units` (Phase 09)
//! already enforces at parse time.
//!
//! Phase 09 already guarantees a *single parse* honors an explicit unit
//! suffix regardless of the document's default. What that phase's own
//! tests cannot catch is a *later* silent misapplication: a value stored
//! in canonical millimeters drifting away from what its own recorded raw
//! input would produce if honestly re-parsed today (e.g. a hypothetical
//! future bug that re-scales a stored value using the page's default unit
//! instead of the unit the user actually wrote). This module re-parses
//! the original input and compares -- the same "recompute independently,
//! don't trust the cached result" pattern Phase 11 used for solver
//! residual diagnostics.

use craftloop_errors::{DomainResult, Severity};
use craftloop_ids::{ConflictId, CraftLoopId};
use craftloop_units::{parse_length, DecimalLocale, LengthUnit};

use crate::conflict::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};

/// Re-parses `raw_input` (exactly as the user originally typed it, e.g.
/// via `craftloop_units`'s raw-input-preserving parsers, Phase 09) and
/// compares the result to `stored_value_mm`. A mismatch beyond
/// `tolerance_mm` means the stored value no longer matches what its own
/// input says -- a silent unit misapplication, not a rounding artifact.
pub fn validate_unit_consistency(
    raw_input: &str,
    default_unit: LengthUnit,
    locale: DecimalLocale,
    stored_value_mm: f64,
    tolerance_mm: f64,
) -> DomainResult<Option<Conflict>> {
    let reparsed = parse_length(raw_input, default_unit, locale)?;
    if (reparsed.value_mm - stored_value_mm).abs() > tolerance_mm {
        Ok(Some(Conflict {
            id: ConflictId::new(),
            kind: ConflictKind::UnitMisapplication,
            severity: Severity::Error,
            affected_entities: Vec::new(),
            existing_truth: format!("stored value = {stored_value_mm}mm"),
            proposed_truth: format!(
                "re-parsing {raw_input:?} gives {}mm{}",
                reparsed.value_mm,
                reparsed
                    .unit_written
                    .map(|u| format!(" (explicit unit: {u:?})"))
                    .unwrap_or_default()
            ),
            evidence: format!(
                "difference = {}mm, tolerance = {tolerance_mm}mm",
                (reparsed.value_mm - stored_value_mm).abs()
            ),
            resolution_choices: vec![
                ResolutionChoice::KeepExisting,
                ResolutionChoice::ReplaceAndPropagate,
            ],
            status: ConflictStatus::Unresolved,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_matching_its_own_raw_input_has_no_conflict() {
        let conflict = validate_unit_consistency(
            "50mm",
            LengthUnit::Centimeters,
            DecimalLocale::PeriodDecimal,
            50.0,
            1e-6,
        )
        .unwrap();
        assert!(conflict.is_none());
    }

    #[test]
    fn a_stored_value_that_drifted_from_its_own_raw_input_is_flagged() {
        // The raw input explicitly says millimeters (50mm = 50mm), but the
        // stored value looks like it was instead (wrongly) scaled as if
        // "50" meant centimeters (50cm = 500mm) -- exactly the silent
        // misapplication Phase 09's "explicit unit always wins" rule
        // exists to prevent, caught here as a stored-value drift.
        let conflict = validate_unit_consistency(
            "50mm",
            LengthUnit::Centimeters,
            DecimalLocale::PeriodDecimal,
            500.0,
            1e-6,
        )
        .unwrap()
        .unwrap();
        assert_eq!(conflict.kind, ConflictKind::UnitMisapplication);
        assert!(conflict.proposed_truth.contains("Millimeters"));
    }

    #[test]
    fn a_bare_number_relies_on_the_default_unit_both_times_consistently() {
        let conflict = validate_unit_consistency(
            "5",
            LengthUnit::Centimeters,
            DecimalLocale::PeriodDecimal,
            50.0,
            1e-6,
        )
        .unwrap();
        assert!(conflict.is_none());
    }

    #[test]
    fn an_unparseable_raw_input_surfaces_the_parser_error_not_a_panic() {
        let result = validate_unit_consistency(
            "not a number",
            LengthUnit::Millimeters,
            DecimalLocale::PeriodDecimal,
            50.0,
            1e-6,
        );
        assert!(result.is_err());
    }

    #[test]
    fn differences_within_tolerance_are_not_flagged() {
        let conflict = validate_unit_consistency(
            "50mm",
            LengthUnit::Centimeters,
            DecimalLocale::PeriodDecimal,
            50.0000001,
            1e-3,
        )
        .unwrap();
        assert!(conflict.is_none());
    }
}
