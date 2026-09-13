//! Basic line style mapping.
//!
//! Execution 01, Phase 25, Task 182. Authority: MCP Article 235 ("Line
//! type definitions. Line weight relationships.").
//!
//! Deterministic role-to-style mapping for test rendering and export --
//! explicitly the *baseline* Task 182 asks for, one style per role, not
//! a per-profile-configurable stylesheet (Article 235's fuller
//! "profile-based" styling is named as future capability, see
//! `unsupported_conventions`, Task 185).

use serde::{Deserialize, Serialize};

use crate::line_role::LineRole;

/// A minimal, real line style: enough to actually draw or export
/// something distinguishable per role, not a placeholder struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineStyle {
    /// Alternating on/off segment lengths, in document millimeters. An
    /// empty pattern means solid.
    pub dash_pattern: Vec<f64>,
    /// Relative line weight -- `1.0` is the baseline (`Visible`); other
    /// roles scale from it, matching Article 235's "line weight
    /// relationships" (a *relationship*, not an absolute value a real
    /// standards profile would ultimately own).
    pub weight: f64,
}

/// The deterministic baseline mapping. Every `LineRole` maps to exactly
/// one style; there is no "unmapped role" case to worry about, since the
/// match is exhaustive.
pub fn style_for_role(role: LineRole) -> LineStyle {
    match role {
        LineRole::Visible => LineStyle {
            dash_pattern: vec![],
            weight: 1.0,
        },
        LineRole::Construction => LineStyle {
            dash_pattern: vec![1.0, 1.0],
            weight: 0.5,
        },
        LineRole::Centerline => LineStyle {
            dash_pattern: vec![6.0, 1.5, 1.0, 1.5],
            weight: 0.5,
        },
        LineRole::Hidden => LineStyle {
            dash_pattern: vec![3.0, 1.5],
            weight: 0.7,
        },
        LineRole::DimensionLine => LineStyle {
            dash_pattern: vec![],
            weight: 0.35,
        },
        LineRole::ExtensionLine => LineStyle {
            dash_pattern: vec![],
            weight: 0.25,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_lines_are_solid_and_the_baseline_weight() {
        let style = style_for_role(LineRole::Visible);
        assert!(style.dash_pattern.is_empty());
        assert_eq!(style.weight, 1.0);
    }

    #[test]
    fn hidden_and_centerline_are_both_dashed_but_distinguishably_so() {
        let hidden = style_for_role(LineRole::Hidden);
        let centerline = style_for_role(LineRole::Centerline);
        assert!(!hidden.dash_pattern.is_empty());
        assert!(!centerline.dash_pattern.is_empty());
        assert_ne!(
            hidden.dash_pattern, centerline.dash_pattern,
            "hidden and centerline must be visually distinguishable"
        );
    }

    #[test]
    fn every_role_maps_to_a_style_the_match_is_exhaustive() {
        for role in [
            LineRole::Visible,
            LineRole::Construction,
            LineRole::Centerline,
            LineRole::Hidden,
            LineRole::DimensionLine,
            LineRole::ExtensionLine,
        ] {
            // Just calling this for every role, relying on the match in
            // `style_for_role` being exhaustive (a compile-time
            // guarantee) -- this test exists to make that guarantee
            // visible in the evidence trail, not to re-derive it.
            let _ = style_for_role(role);
        }
    }

    #[test]
    fn dimension_and_extension_lines_are_lighter_than_visible_geometry() {
        let visible = style_for_role(LineRole::Visible);
        let dimension = style_for_role(LineRole::DimensionLine);
        let extension = style_for_role(LineRole::ExtensionLine);
        assert!(dimension.weight < visible.weight);
        assert!(extension.weight < visible.weight);
    }
}
