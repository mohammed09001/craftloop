//! Conflict resolution transactions.
//!
//! Execution 01, Phase 14, Task 104. Authority: MCP Article 27; Article
//! 311 ("The user can replace an old constraint or cancel the new one").
//!
//! Resolving is a one-time, atomic transition (mirroring
//! `craftloop-transactions`' own commit-once philosophy, Phase 04): a
//! [`Conflict`] can only move from `Unresolved` to `Resolved` once, and
//! only with a choice the conflict itself actually offered -- both
//! checked before anything is mutated, so a rejected resolution attempt
//! never leaves the conflict in a half-applied state.

use craftloop_errors::{ConsistencyErrorKind, DomainError, DomainResult};

use crate::conflict::{Conflict, ConflictStatus, ResolutionChoice};

/// Resolve `conflict` with `choice`. Rejects (without mutating anything)
/// if the conflict was already resolved, or if `choice` is not one of the
/// conflict's own `resolution_choices` (Task 100's schema is the source
/// of truth for which choices are even meaningful for a given conflict --
/// e.g. `Unlink` is nonsensical for a `DegenerateGeometry` conflict, which
/// never offers it).
pub fn resolve(conflict: &mut Conflict, choice: ResolutionChoice) -> DomainResult<()> {
    if !conflict.is_unresolved() {
        return Err(DomainError::Consistency {
            kind: ConsistencyErrorKind::AlreadyResolved,
            detail: format!("conflict {:?} was already resolved", conflict.id),
        });
    }
    if !conflict.resolution_choices.contains(&choice) {
        return Err(DomainError::Consistency {
            kind: ConsistencyErrorKind::ChoiceNotOffered,
            detail: format!("{choice:?} is not among this conflict's offered resolution choices"),
        });
    }
    conflict.status = ConflictStatus::Resolved { choice };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conflict::ConflictKind;
    use craftloop_errors::Severity;
    use craftloop_ids::{ConflictId, CraftLoopId};

    fn sample() -> Conflict {
        Conflict {
            id: ConflictId::new(),
            kind: ConflictKind::DegenerateGeometry,
            severity: Severity::Error,
            affected_entities: Vec::new(),
            existing_truth: "a".to_string(),
            proposed_truth: "b".to_string(),
            evidence: "c".to_string(),
            resolution_choices: vec![ResolutionChoice::KeepExisting, ResolutionChoice::Cancel],
            status: ConflictStatus::Unresolved,
        }
    }

    #[test]
    fn resolving_with_an_offered_choice_succeeds() {
        let mut conflict = sample();
        resolve(&mut conflict, ResolutionChoice::KeepExisting).unwrap();
        assert_eq!(
            conflict.status,
            ConflictStatus::Resolved {
                choice: ResolutionChoice::KeepExisting
            }
        );
    }

    #[test]
    fn resolving_twice_is_rejected_and_the_first_resolution_survives() {
        let mut conflict = sample();
        resolve(&mut conflict, ResolutionChoice::KeepExisting).unwrap();
        let err = resolve(&mut conflict, ResolutionChoice::Cancel).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Consistency {
                kind: ConsistencyErrorKind::AlreadyResolved,
                ..
            }
        ));
        assert_eq!(
            conflict.status,
            ConflictStatus::Resolved {
                choice: ResolutionChoice::KeepExisting
            }
        );
    }

    #[test]
    fn resolving_with_a_choice_the_conflict_never_offered_is_rejected() {
        let mut conflict = sample();
        let err = resolve(&mut conflict, ResolutionChoice::Unlink).unwrap_err();
        assert!(matches!(
            err,
            DomainError::Consistency {
                kind: ConsistencyErrorKind::ChoiceNotOffered,
                ..
            }
        ));
        // Rejected: still unresolved, not partially mutated.
        assert!(conflict.is_unresolved());
    }
}
