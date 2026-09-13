//! Standards-Aware Representation Baseline for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 25, Tasks 180-185. Authority: MCP Article 235
//! "Detailed Specification of the Standards Engine".
//!
//! Explicitly **not** a certified-compliance implementation of any real
//! drafting standard -- Article 235's own words: "Version 1 should avoid
//! claiming full standards compliance unless the implemented subset has
//! been reviewed professionally... The MCP document should therefore
//! distinguish standards awareness from certified compliance." Every
//! public item in this crate is named and documented with that
//! distinction in mind (see `guardrail.rs`, which enforces it at
//! construction, not just in prose).

pub mod dimension_presentation;
pub mod guardrail;
pub mod line_role;
pub mod line_style;
pub mod profile;

pub use dimension_presentation::{present, DimensionTypography, PresentedDimension};
pub use guardrail::contains_forbidden_compliance_claim;
pub use line_role::LineRole;
pub use line_style::{style_for_role, LineStyle};
pub use profile::StandardsProfile;

/// Task 185: "document unsupported conventions" -- Article 235's own
/// fuller profile-capability list (leader conventions, centerline
/// *conventions* beyond this baseline's flat role mapping, hidden-line
/// *conventions* beyond a single dash style, future section-view rules,
/// and any real ISO/ASME-licensed rule text) named explicitly as **not**
/// implemented, rather than left for a reader to discover by absence.
pub const UNSUPPORTED_CONVENTIONS: &[&str] = &[
    "Leader conventions",
    "Per-profile-configurable line weight/typography (this baseline is one fixed role-to-style mapping, not yet profile-driven)",
    "Section-view rules",
    "Any rule text requiring access to the licensed ISO/ASME standard documents themselves",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_conventions_is_a_real_nonempty_documented_list() {
        assert!(!UNSUPPORTED_CONVENTIONS.is_empty());
        for entry in UNSUPPORTED_CONVENTIONS {
            assert!(!entry.is_empty());
        }
    }

    #[test]
    fn no_entry_in_the_crates_own_public_surface_claims_compliance() {
        // Task 184's guardrail applied to this crate's own README-style
        // list, so the "document unsupported conventions" text itself
        // never backslides into a compliance claim.
        for entry in UNSUPPORTED_CONVENTIONS {
            assert!(!contains_forbidden_compliance_claim(entry));
        }
    }
}
