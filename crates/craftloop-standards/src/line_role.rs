//! Line semantic roles.
//!
//! Execution 01, Phase 25, Task 180. Authority: MCP Article 235
//! ("Line type definitions. Line weight relationships.").
//!
//! Exactly the six roles Task 180 names -- kept as pure semantics,
//! entirely separate from any concrete visual styling (dash pattern,
//! weight, color). `line_style.rs`'s job is the mapping from a role to a
//! style; this type never carries styling data itself, so "which lines
//! mean what" and "how do those meanings currently render" can never be
//! confused with each other.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LineRole {
    /// A real, visible edge of the represented geometry.
    Visible,
    /// A construction aid, not part of the represented geometry itself.
    Construction,
    /// Marks the axis/center of a circular or symmetric feature.
    Centerline,
    /// An edge that exists but is hidden from the current view.
    Hidden,
    /// The line of a dimension annotation, distinct from the geometry it
    /// measures.
    DimensionLine,
    /// Connects a dimension line to the geometry it measures.
    ExtensionLine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_role_is_distinct() {
        let roles = [
            LineRole::Visible,
            LineRole::Construction,
            LineRole::Centerline,
            LineRole::Hidden,
            LineRole::DimensionLine,
            LineRole::ExtensionLine,
        ];
        for (i, a) in roles.iter().enumerate() {
            for (j, b) in roles.iter().enumerate() {
                assert_eq!(i == j, a == b);
            }
        }
    }

    #[test]
    fn serialization_round_trips_for_every_role() {
        for role in [
            LineRole::Visible,
            LineRole::Construction,
            LineRole::Centerline,
            LineRole::Hidden,
            LineRole::DimensionLine,
            LineRole::ExtensionLine,
        ] {
            let json = serde_json::to_string(&role).unwrap();
            let back: LineRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, back);
        }
    }
}
