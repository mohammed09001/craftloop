//! The conflict object schema.
//!
//! Execution 01, Phase 14, Task 100. Authority: MCP Article 27
//! "Geometric Consistency Engine" ("The engine should return structured
//! explanations, not merely a boolean failure").
//!
//! A [`Conflict`] carries everything Article 27 names: type ([`ConflictKind`]),
//! severity ([`craftloop_errors::Severity`], reused rather than duplicated
//! -- a conflict's severity is the same concept `Diagnostic` already
//! models, and Task 100 does not ask for a conflict-specific severity
//! scale), affected entities, existing truth, proposed truth, evidence,
//! and resolution choices ([`ResolutionChoice`], Task 104).

use craftloop_errors::Severity;
use craftloop_ids::ConflictId;
use serde::{Deserialize, Serialize};

/// What kind of consistency problem this is. One variant per validator in
/// this crate (Tasks 101-103); grows with future consistency checks
/// rather than being a free-form string, so callers can match on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictKind {
    /// Task 101: an entity's own geometry is degenerate (e.g. a
    /// near-zero-length line) -- impossible/meaningless on its own,
    /// independent of any relationship to other entities.
    DegenerateGeometry,
    /// Task 102: a dimension's driving value is incompatible with what
    /// the constraint solver has already determined.
    DimensionConstraintMismatch,
    /// Task 103: a stored value no longer matches what its own recorded
    /// raw input would produce if re-parsed -- evidence that an explicit
    /// unit was silently misapplied somewhere between input and storage.
    UnitMisapplication,
    /// Execution 01, Phase 23, Task 171: a second, incompatible value was
    /// proposed for a dimension already shared across linked orthographic
    /// views (MCP Article 37: "The top view should not be allowed to
    /// establish a contradictory 130-millimeter width for the same linked
    /// geometry... the system should explain the conflict").
    CrossViewMismatch,
}

/// Task 104: how a conflict can be resolved. Matches Article 27/Task
/// 104's five choices exactly -- not a speculative superset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionChoice {
    /// Keep the existing (previously confirmed) truth; discard the new
    /// proposal.
    KeepExisting,
    /// Accept the new proposal and propagate it to whatever depended on
    /// the old value.
    ReplaceAndPropagate,
    /// Remove the relationship that ties the conflicting entities
    /// together, where that is semantically valid (e.g. un-share a
    /// cross-view dimension) -- leaving both sides independently valid.
    Unlink,
    /// Remove the constraint responsible for the contradiction.
    RemoveConstraint,
    /// Abandon the change that produced this conflict; nothing commits.
    Cancel,
}

/// Task 105: a conflict is either still open, or has been resolved with a
/// specific, recorded choice -- never silently discarded. This is the
/// type-level enforcement of "unresolved conflicts must survive
/// save/reopen" (Task 106): there is no variant that loses the conflict's
/// existence once created, only one that records how it ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStatus {
    Unresolved,
    Resolved { choice: ResolutionChoice },
}

/// A structured, persisted record of one consistency problem. Task 105's
/// invariant applies to every constructor in this crate: a `Conflict` is
/// only ever created for a genuine contradiction, never for an
/// unresolved-but-not-wrong state (Article 29) -- see each validator
/// module's own tests proving this.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    pub id: ConflictId,
    pub kind: ConflictKind,
    pub severity: Severity,
    /// Stable string references to the entities involved (same
    /// convention as `craftloop_errors::Diagnostic::related_entities`:
    /// this low-level crate has no dependency on `craftloop-document`'s
    /// entity types, so callers format their typed IDs before attaching
    /// them).
    pub affected_entities: Vec<String>,
    /// What was true before the proposed change (or currently is true,
    /// for a conflict not tied to a specific edit).
    pub existing_truth: String,
    /// What the new input/edit claims should be true instead.
    pub proposed_truth: String,
    /// Supporting detail (e.g. a residual value, a re-parsed number) a
    /// presentation layer can show on demand without needing to
    /// re-derive it.
    pub evidence: String,
    pub resolution_choices: Vec<ResolutionChoice>,
    pub status: ConflictStatus,
}

impl Conflict {
    pub fn is_unresolved(&self) -> bool {
        matches!(self.status, ConflictStatus::Unresolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    fn sample() -> Conflict {
        Conflict {
            id: ConflictId::new(),
            kind: ConflictKind::DegenerateGeometry,
            severity: Severity::Error,
            affected_entities: vec!["PrimitiveId(...)".to_string()],
            existing_truth: "line has nonzero length".to_string(),
            proposed_truth: "line collapses to a point".to_string(),
            evidence: "length = 0.00000012mm".to_string(),
            resolution_choices: vec![ResolutionChoice::KeepExisting, ResolutionChoice::Cancel],
            status: ConflictStatus::Unresolved,
        }
    }

    #[test]
    fn a_fresh_conflict_is_unresolved() {
        assert!(sample().is_unresolved());
    }

    #[test]
    fn a_resolved_conflict_is_not_unresolved_but_still_exists() {
        let mut conflict = sample();
        conflict.status = ConflictStatus::Resolved {
            choice: ResolutionChoice::KeepExisting,
        };
        assert!(!conflict.is_unresolved());
        // Task 105: resolving a conflict records the outcome, it does not
        // erase that the conflict existed.
        assert_eq!(conflict.kind, ConflictKind::DegenerateGeometry);
    }

    #[test]
    fn serialization_round_trips() {
        let conflict = sample();
        let json = serde_json::to_string(&conflict).unwrap();
        let back: Conflict = serde_json::from_str(&json).unwrap();
        assert_eq!(conflict, back);
    }

    #[test]
    fn a_resolved_status_round_trips_with_its_choice() {
        let mut conflict = sample();
        conflict.status = ConflictStatus::Resolved {
            choice: ResolutionChoice::RemoveConstraint,
        };
        let json = serde_json::to_string(&conflict).unwrap();
        let back: Conflict = serde_json::from_str(&json).unwrap();
        assert_eq!(
            back.status,
            ConflictStatus::Resolved {
                choice: ResolutionChoice::RemoveConstraint
            }
        );
    }
}
