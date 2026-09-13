//! Integer/decimal parsing and locale-sensitive decimal separator policy.
//!
//! Execution 01, Phase 09, Tasks 064, 068, 069. Authority: Engine Contract
//! 07 ("Numbers, units, angle, radius, diameter"; "Parser does not choose a
//! geometry target"); MCP Article 301/302 "Edge Case: Ambiguous Handwritten
//! Number / Decimal Separator".
//!
//! [`parse_decimal`] is the one function every higher-level parser in this
//! crate (length, angle, radius/diameter) bottoms out in, and it is also
//! meant to be the same function a future handwriting-recognition text
//! candidate (Phase 17) is parsed with -- Task 064's "prepare the same
//! semantic parser for handwriting recognition output" is satisfied by
//! having exactly one number-parsing entry point rather than a
//! keyboard-only one and a separate recognition-only one.

use craftloop_errors::{DomainError, DomainResult, ParserErrorKind};
use serde::{Deserialize, Serialize};

/// Rewrite a `DomainError::Parser`'s `input` field to `original` (any other
/// variant passes through unchanged).
///
/// Every higher-level parser in this crate (length, angle, radius/
/// diameter) strips a unit suffix before delegating to [`parse_decimal`]
/// on the remaining numeric substring. Without this rewrite, a failure
/// from that inner call would report the *substring* (e.g. `"abc"` from
/// `"abcmm"`) as the raw input, silently dropping the suffix -- exactly
/// the "losing raw input" Task 069 forbids. Callers apply this via
/// `.map_err(|e| with_original_input(e, input))` at every suffix-stripping
/// call site.
pub(crate) fn with_original_input(err: DomainError, original: &str) -> DomainError {
    match err {
        DomainError::Parser { kind, detail, .. } => DomainError::Parser {
            kind,
            input: original.to_string(),
            detail,
        },
        other => other,
    }
}

/// Which character is the decimal point when a caller knows their locale.
/// See Task 068: this crate never *guesses* a locale, only applies one it
/// is given -- guessing would be exactly the kind of silent scale change
/// (Task 065) or fabricated certainty (Article 4) this execution forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecimalLocale {
    /// `.` is the decimal point; `,` (if present) groups thousands.
    PeriodDecimal,
    /// `,` is the decimal point; `.` (if present) groups thousands.
    CommaDecimal,
}

fn strip_and_normalize(input: &str, decimal_char: char, thousands_char: char) -> String {
    let mut normalized = String::with_capacity(input.len());
    for c in input.chars() {
        if c == thousands_char {
            continue; // grouping separator: drop it
        }
        if c == decimal_char {
            normalized.push('.');
        } else {
            normalized.push(c);
        }
    }
    normalized
}

/// Parse `input` as a decimal number under an explicitly known locale.
/// Deterministic: the same input and locale always produce the same
/// result, with no auto-detection involved.
pub fn parse_decimal(input: &str, locale: DecimalLocale) -> DomainResult<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::EmptyInput,
            input: input.to_string(),
            detail: "numeric input was empty or whitespace-only".to_string(),
        });
    }

    let (decimal_char, thousands_char) = match locale {
        DecimalLocale::PeriodDecimal => ('.', ','),
        DecimalLocale::CommaDecimal => (',', '.'),
    };
    let normalized = strip_and_normalize(trimmed, decimal_char, thousands_char);

    normalized.parse::<f64>().map_err(|_| DomainError::Parser {
        kind: ParserErrorKind::InvalidNumber,
        input: input.to_string(),
        detail: format!("could not parse {trimmed:?} as a number under {locale:?}"),
    })
}

/// Parse `input` without a known locale, using the documented policy for
/// when that is and is not safe (Task 068):
///
/// - no separator, or two different separators present: unambiguous,
///   parsed directly (the later-occurring one is the decimal point, the
///   earlier one a thousands group, matching both US `1,234.56` and EU
///   `1.234,56` conventions).
/// - exactly one separator with a trailing run of exactly three digits
///   (e.g. `"1,234"`): genuinely ambiguous -- could be a thousands-grouped
///   integer or a three-decimal-place fraction, depending on locale this
///   function does not have. Returns a structured
///   `AmbiguousDecimalSeparator` error rather than guessing.
/// - exactly one separator with any other trailing digit count (e.g.
///   `"12,5"` or `"12,3456"`): unambiguous decimal point.
pub fn parse_decimal_auto(input: &str) -> DomainResult<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::EmptyInput,
            input: input.to_string(),
            detail: "numeric input was empty or whitespace-only".to_string(),
        });
    }

    let comma_positions: Vec<usize> = trimmed.match_indices(',').map(|(i, _)| i).collect();
    let period_positions: Vec<usize> = trimmed.match_indices('.').map(|(i, _)| i).collect();
    let comma_count = comma_positions.len();
    let period_count = period_positions.len();

    if comma_count == 0 && period_count == 0 {
        // No separator at all: locale is irrelevant.
        return parse_decimal(input, DecimalLocale::PeriodDecimal);
    }

    if comma_count > 0 && period_count > 0 {
        // Both characters present: whichever occurs last is the decimal
        // point (standard convention for both US `1,234.56` and EU
        // `1.234,56` grouped notation, regardless of how many grouping
        // separators came before it).
        let comma_last = *comma_positions.last().unwrap();
        let period_last = *period_positions.last().unwrap();
        let locale = if comma_last > period_last {
            DecimalLocale::CommaDecimal
        } else {
            DecimalLocale::PeriodDecimal
        };
        return parse_decimal(input, locale);
    }

    if comma_count >= 2 {
        // Multiple commas and no period at all cannot be multiple decimal
        // points (that would never parse); it is unambiguous
        // thousands-grouping under the period-decimal convention, e.g.
        // "1,234,567" = 1234567.
        return parse_decimal(input, DecimalLocale::PeriodDecimal);
    }
    if period_count >= 2 {
        // Mirror image: repeated periods with no comma imply
        // thousands-grouping under the comma-decimal convention.
        return parse_decimal(input, DecimalLocale::CommaDecimal);
    }

    // Exactly one separator, of one kind, and nothing of the other: this
    // is the genuinely ambiguous case -- resolve it (or reject it) by
    // trailing digit count, per the function doc above.
    let implied_locale = if comma_count == 1 {
        DecimalLocale::CommaDecimal
    } else {
        DecimalLocale::PeriodDecimal
    };
    check_single_separator_then_parse(trimmed, input, implied_locale)
}

fn check_single_separator_then_parse(
    trimmed: &str,
    original: &str,
    implied_locale: DecimalLocale,
) -> DomainResult<f64> {
    let sep = match implied_locale {
        DecimalLocale::CommaDecimal => ',',
        DecimalLocale::PeriodDecimal => '.',
    };
    if let Some(pos) = trimmed.find(sep) {
        let suffix = &trimmed[pos + sep.len_utf8()..];
        let suffix_digit_count = suffix.chars().filter(|c| c.is_ascii_digit()).count();
        if suffix_digit_count == 3 && suffix.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::Parser {
                kind: ParserErrorKind::AmbiguousDecimalSeparator,
                input: original.to_string(),
                detail: format!(
                    "{trimmed:?} has exactly one separator followed by three digits; locale is required to resolve it"
                ),
            });
        }
    }
    parse_decimal(original, implied_locale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_integers_parse_under_either_locale() {
        assert_eq!(
            parse_decimal("42", DecimalLocale::PeriodDecimal).unwrap(),
            42.0
        );
        assert_eq!(
            parse_decimal("42", DecimalLocale::CommaDecimal).unwrap(),
            42.0
        );
    }

    #[test]
    fn period_locale_parses_a_period_as_the_decimal_point() {
        assert_eq!(
            parse_decimal("12.5", DecimalLocale::PeriodDecimal).unwrap(),
            12.5
        );
    }

    #[test]
    fn comma_locale_parses_a_comma_as_the_decimal_point() {
        assert_eq!(
            parse_decimal("12,5", DecimalLocale::CommaDecimal).unwrap(),
            12.5
        );
    }

    #[test]
    fn period_locale_treats_commas_as_thousands_separators() {
        assert_eq!(
            parse_decimal("1,234.5", DecimalLocale::PeriodDecimal).unwrap(),
            1234.5
        );
    }

    #[test]
    fn comma_locale_treats_periods_as_thousands_separators() {
        assert_eq!(
            parse_decimal("1.234,5", DecimalLocale::CommaDecimal).unwrap(),
            1234.5
        );
    }

    #[test]
    fn negative_numbers_parse_correctly() {
        assert_eq!(
            parse_decimal("-7.5", DecimalLocale::PeriodDecimal).unwrap(),
            -7.5
        );
    }

    #[test]
    fn empty_input_is_a_structured_error() {
        let result = parse_decimal("   ", DecimalLocale::PeriodDecimal);
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::EmptyInput,
                ..
            })
        ));
    }

    #[test]
    fn garbage_input_is_invalid_number_not_a_panic() {
        let result = parse_decimal("not a number", DecimalLocale::PeriodDecimal);
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::InvalidNumber,
                ..
            })
        ));
    }

    #[test]
    fn auto_parses_unambiguous_single_separator_cases() {
        assert_eq!(parse_decimal_auto("12.5").unwrap(), 12.5);
        assert_eq!(parse_decimal_auto("12,5").unwrap(), 12.5);
        assert_eq!(parse_decimal_auto("12.3456").unwrap(), 12.3456);
    }

    #[test]
    fn auto_parses_unambiguous_dual_separator_cases_us_and_eu_style() {
        assert_eq!(parse_decimal_auto("1,234.56").unwrap(), 1234.56); // US
        assert_eq!(parse_decimal_auto("1.234,56").unwrap(), 1234.56); // EU
    }

    #[test]
    fn auto_parses_repeated_grouping_separator_with_no_decimal_point_as_a_whole_number() {
        // Multiple commas and no period at all cannot mean multiple
        // decimal points, so this is unambiguous thousands grouping, not
        // the three-digit-suffix ambiguous case.
        assert_eq!(parse_decimal_auto("1,234,567").unwrap(), 1_234_567.0);
        assert_eq!(parse_decimal_auto("1.234.567").unwrap(), 1_234_567.0);
    }

    #[test]
    fn auto_flags_the_genuinely_ambiguous_three_digit_single_separator_case() {
        let result = parse_decimal_auto("1,234");
        assert!(matches!(
            result,
            Err(DomainError::Parser {
                kind: ParserErrorKind::AmbiguousDecimalSeparator,
                ..
            })
        ));
        let result2 = parse_decimal_auto("1.234");
        assert!(matches!(
            result2,
            Err(DomainError::Parser {
                kind: ParserErrorKind::AmbiguousDecimalSeparator,
                ..
            })
        ));
    }

    #[test]
    fn ambiguous_input_can_still_be_resolved_by_supplying_an_explicit_locale() {
        // Exactly the recovery path a real UI would offer after the
        // ambiguous error: ask the user (or use a saved preference), then
        // call the explicit-locale parser instead of guessing.
        assert_eq!(
            parse_decimal("1,234", DecimalLocale::PeriodDecimal).unwrap(),
            1234.0
        );
        assert_eq!(
            parse_decimal("1,234", DecimalLocale::CommaDecimal).unwrap(),
            1.234
        );
    }
}
