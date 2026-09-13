//! Standards profile metadata.
//!
//! Execution 01, Phase 25, Task 181. Authority: MCP Article 235 ("The
//! Standards Engine should separate normative representation rules from
//! product interaction. Its data should be profile-based.").
//!
//! `StandardsProfile` stores exactly what this phase's tasks give it
//! real content for: a name and a projection convention (reusing
//! `craftloop_document::ProjectionConvention`, Phase 21, rather than a
//! duplicate enum). Article 235's fuller profile shape (line weight
//! relationships beyond what `line_style.rs` maps, dimension typography
//! rules, leader/centerline/hidden-line *conventions* as opposed to this
//! phase's flat role-to-style mapping, section-view rules) is named
//! there as future capability, not built speculatively here -- see
//! `unsupported_conventions` (Task 185) for the explicit, honest list.

use craftloop_document::ProjectionConvention;
use craftloop_errors::DomainResult;

use crate::guardrail::reject_forbidden_claim;

/// A named standards profile. Construction itself enforces Task 184's
/// guardrail (`name` may not contain a certified/compliance claim) --
/// see `guardrail.rs`.
#[derive(Debug, Clone, PartialEq)]
pub struct StandardsProfile {
    name: String,
    pub projection_convention: ProjectionConvention,
}

impl StandardsProfile {
    pub fn new(
        name: impl Into<String>,
        projection_convention: ProjectionConvention,
    ) -> DomainResult<Self> {
        let name = name.into();
        reject_forbidden_claim(&name)?;
        Ok(Self {
            name,
            projection_convention,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_standards_aware_name_is_accepted() {
        let profile =
            StandardsProfile::new("ASME-inspired baseline", ProjectionConvention::ThirdAngle)
                .unwrap();
        assert_eq!(profile.name(), "ASME-inspired baseline");
        assert_eq!(
            profile.projection_convention,
            ProjectionConvention::ThirdAngle
        );
    }

    #[test]
    fn a_compliance_claiming_name_is_rejected_at_construction() {
        let result =
            StandardsProfile::new("ASME Y14.5 Compliant", ProjectionConvention::FirstAngle);
        assert!(
            result.is_err(),
            "construction itself must enforce the guardrail, not just a linter run separately"
        );
    }
}
