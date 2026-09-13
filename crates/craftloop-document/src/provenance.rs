//! Document-level provenance states.
//!
//! Execution 01, Phase 08, Task 059. Authority: Engine Contract 14
//! ("One visible action should undo as one coherent action" -- provenance
//! is part of what a transaction can change); MCP Article 540 "Term:
//! Provenance".
//!
//! Distinct from `craftloop_ink::StrokeProvenance` (Phase 05), which is an
//! append-only *event log* scoped to one stroke's ink-level history
//! (created, and later recognition/interpretation events). This is a
//! document-wide *current-state* classification -- "what is this entity's
//! status right now" -- that applies to any `SemanticEntity`, tracked and
//! mutated through the same transaction/undo mechanism as the entities
//! themselves (`history.rs`).

use serde::{Deserialize, Serialize};

/// The seven outcomes Task 059 names, verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceState {
    /// The user drew/typed this directly; no recognition or AI involved.
    UserCreated,
    /// An engine proposed this; the user has not yet accepted or rejected
    /// it (Article 5: suggestions remain suggestions until confirmed).
    Suggested,
    /// The user confirmed a suggestion.
    Accepted,
    /// User-created or accepted content the user has since edited.
    Modified,
    /// Computed from other confirmed entities (e.g. a value implied by
    /// constraints), not drawn or typed directly.
    Derived,
    /// Copied/propagated from another linked location (e.g. a shared
    /// dimension appearing in a second view, once views exist).
    Propagated,
    /// The user explicitly rejected a suggestion for this entity.
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_state_serializes_and_round_trips() {
        let all = [
            ProvenanceState::UserCreated,
            ProvenanceState::Suggested,
            ProvenanceState::Accepted,
            ProvenanceState::Modified,
            ProvenanceState::Derived,
            ProvenanceState::Propagated,
            ProvenanceState::Rejected,
        ];
        for state in all {
            let json = serde_json::to_string(&state).unwrap();
            let back: ProvenanceState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back);
        }
    }
}
