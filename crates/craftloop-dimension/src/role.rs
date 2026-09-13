//! Dimension role: driving, reference, derived, shared, or bounded.
//!
//! Execution 01, Phase 10, Tasks 072-073. Authority: Engine Contract 09;
//! MCP Article 24 "Driving, Derived, Shared, Bounded, and Reference
//! Dimensions".
//!
//! Modeled as one role enum (matching how Article 24 itself groups all
//! five under a single title) rather than several independent boolean
//! flags, because a dimension is exactly one of these at a time -- "driving
//! and derived simultaneously" is not a coherent state, so an enum makes
//! the invalid combination unrepresentable instead of merely undocumented.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionRole {
    /// Controls geometry: editing this dimension's value is how the user
    /// changes the design (Task 072).
    Driving,
    /// Reports geometry without controlling it: read-only, informational
    /// (Task 072).
    Reference,
    /// Computed from other confirmed entities, not drawn/typed/edited
    /// directly (Task 073).
    Derived,
    /// The same semantic dimension is shown/used across more than one
    /// linked view (Task 073; real cross-view linkage is Phase 20, but the
    /// role itself is meaningful and testable now).
    Shared,
    /// Restricted to a feasible range rather than pinned to one exact
    /// value (Task 073) -- see `SemanticDimension::feasible_range`.
    Bounded,
}

impl DimensionRole {
    /// Only `Driving` dimensions accept a direct value edit (Task 075);
    /// every other role's value comes from somewhere else (the user
    /// reading it off existing geometry, a computation, propagation from
    /// another view, or a solver picking a point in a range).
    pub fn is_directly_editable(&self) -> bool {
        matches!(self, DimensionRole::Driving)
    }

    /// Execution 01, Phase 13, Task 099: UI-ready explanation for this
    /// role, free of internal/solver terminology (Article 99's "prefer
    /// 'This width is already defined in the front view' over
    /// 'Redundant driving dimension'"). Covers this crate's half of Task
    /// 099's named list ("derived, shared" and, here, the rest of the
    /// role vocabulary too); `craftloop-sketch::DegreesOfFreedomState::
    /// explain` covers the geometric half ("free", "conflicting").
    pub fn explain(&self) -> &'static str {
        match self {
            DimensionRole::Driving => "Editing this value changes the design.",
            DimensionRole::Reference => {
                "Reports the current geometry; editing it does not change anything."
            }
            DimensionRole::Derived => {
                "Calculated from other relationships and cannot be edited directly."
            }
            DimensionRole::Shared => "The same value as another view -- change it there instead.",
            DimensionRole::Bounded => "Restricted to a feasible range rather than one fixed value.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_driving_is_directly_editable() {
        assert!(DimensionRole::Driving.is_directly_editable());
        for role in [
            DimensionRole::Reference,
            DimensionRole::Derived,
            DimensionRole::Shared,
            DimensionRole::Bounded,
        ] {
            assert!(
                !role.is_directly_editable(),
                "{role:?} must not be directly editable"
            );
        }
    }

    #[test]
    fn every_role_has_a_jargon_free_explanation() {
        for role in [
            DimensionRole::Driving,
            DimensionRole::Reference,
            DimensionRole::Derived,
            DimensionRole::Shared,
            DimensionRole::Bounded,
        ] {
            let text = role.explain();
            assert!(!text.is_empty());
            assert!(
                !text.to_lowercase().contains("solver")
                    && !text.to_lowercase().contains("driving dimension"),
                "explanation for {role:?} leaked internal terminology: {text:?}"
            );
        }
    }

    #[test]
    fn serialization_round_trips_for_every_role() {
        for role in [
            DimensionRole::Driving,
            DimensionRole::Reference,
            DimensionRole::Derived,
            DimensionRole::Shared,
            DimensionRole::Bounded,
        ] {
            let json = serde_json::to_string(&role).unwrap();
            let back: DimensionRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, back);
        }
    }
}
