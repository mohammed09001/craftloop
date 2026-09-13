//! Constraint provenance.
//!
//! Execution 01, Phase 12, Task 091. Objective: "Distinguish explicit user
//! constraints from accepted suggestions and deterministic relationships."
//!
//! Deliberately a small, constraint-specific enum rather than reusing
//! `craftloop-document`'s `ProvenanceState` (Phase 07): that type's
//! `Propagated`/`Rejected` variants describe a *document entity's*
//! lifecycle (a stroke recognized into a primitive, later un-recognized),
//! which is not the question this enum answers. A constraint's provenance
//! only ever needs to say who/what asserted the relationship -- reusing an
//! unrelated five-variant enum "because it already exists" would be
//! exactly the duplicated-truth shortcut the Loop Engineering Contract
//! forbids (the two enums would drift out of sync the moment either grows
//! a variant the other does not need).
//!
//! `Sketch` (this crate) intentionally does not thread provenance into
//! solver priority (Task 091 asks only to *distinguish* the categories as
//! data, not to change how they are solved) -- doing so without a task
//! asking for it would be speculative scope.

use serde::{Deserialize, Serialize};

/// Mutually exclusive: a constraint was asserted by exactly one of these
/// origins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintProvenance {
    /// The user explicitly drew or applied this constraint.
    UserCreated,
    /// The engine suggested this relationship (e.g. from ink recognition
    /// snapping) and the user accepted it.
    AcceptedSuggestion,
    /// The relationship is implied deterministically by other confirmed
    /// constraints/geometry, not asserted on its own (e.g. a rectangle's
    /// fourth right angle once the other three and the side lengths are
    /// fixed).
    Derived,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_three_origins_are_distinguishable_by_equality() {
        assert_ne!(
            ConstraintProvenance::UserCreated,
            ConstraintProvenance::AcceptedSuggestion
        );
        assert_ne!(
            ConstraintProvenance::AcceptedSuggestion,
            ConstraintProvenance::Derived
        );
        assert_ne!(
            ConstraintProvenance::UserCreated,
            ConstraintProvenance::Derived
        );
    }

    #[test]
    fn serialization_round_trips_for_every_variant() {
        for provenance in [
            ConstraintProvenance::UserCreated,
            ConstraintProvenance::AcceptedSuggestion,
            ConstraintProvenance::Derived,
        ] {
            let json = serde_json::to_string(&provenance).unwrap();
            let back: ConstraintProvenance = serde_json::from_str(&json).unwrap();
            assert_eq!(provenance, back);
        }
    }
}
