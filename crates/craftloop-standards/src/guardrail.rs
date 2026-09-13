//! Standards claim guardrail.
//!
//! Execution 01, Phase 25, Task 184. Authority: MCP Article 235 ("Version
//! 1 should avoid claiming full standards compliance unless the
//! implemented subset has been reviewed professionally... The MCP
//! document should therefore distinguish standards awareness from
//! certified compliance.").
//!
//! Not merely a naming convention: [`StandardsProfile::new`]
//! (`profile.rs`) actually calls [`contains_forbidden_compliance_claim`]
//! and rejects a profile name that trips it, so this guardrail is
//! enforced at construction, not left as documentation a future caller
//! could forget to follow.

use craftloop_errors::{DomainError, DomainResult, ParserErrorKind};

/// Words/phrases that assert certification or full compliance -- exactly
/// what Article 235 says this product must not claim for its Version 1
/// implemented subset. `"standard"`/`"standards"` on their own are not
/// forbidden (this crate's whole name uses the word); only claims of
/// having actually *met* one are.
const FORBIDDEN_TERMS: &[&str] = &[
    "compliant",
    "compliance",
    "certified",
    "certification",
    "conforms to",
    "conformant",
];

/// Does `text` contain a forbidden certified/compliance claim?
/// Case-insensitive.
pub fn contains_forbidden_compliance_claim(text: &str) -> bool {
    let lower = text.to_lowercase();
    FORBIDDEN_TERMS.iter().any(|term| lower.contains(term))
}

/// Reject `text` outright if it makes a forbidden claim -- the real
/// enforcement point, not just a checker a caller might not call.
pub(crate) fn reject_forbidden_claim(text: &str) -> DomainResult<()> {
    if contains_forbidden_compliance_claim(text) {
        return Err(DomainError::Parser {
            kind: ParserErrorKind::InvalidUnit,
            input: text.to_string(),
            detail: "standards profile text must not claim certified compliance (Article 235; use standards-aware wording instead)".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_standards_aware_wording_is_allowed() {
        assert!(!contains_forbidden_compliance_claim(
            "Third-angle projection, ASME-inspired line role mapping"
        ));
    }

    #[test]
    fn the_word_standard_alone_is_not_forbidden() {
        assert!(!contains_forbidden_compliance_claim(
            "Standards-aware baseline profile"
        ));
    }

    #[test]
    fn compliance_claims_are_caught_case_insensitively() {
        for phrase in [
            "Fully COMPLIANT with ASME Y14.5",
            "ISO Certified",
            "Conforms to ISO 128",
        ] {
            assert!(
                contains_forbidden_compliance_claim(phrase),
                "should have flagged: {phrase}"
            );
        }
    }
}
