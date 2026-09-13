//! Atomic domain transaction boundary for the Craft Loop shared engineering
//! core.
//!
//! Execution 01, Phase 01, Task 011. Authority: Engine Contract 14
//! ("Atomic commits, undo/redo, event history"; "One visible action should
//! undo as one coherent action"); MCP Article 138 "Interaction Transaction
//! Model", Article 385 "Architecture Principle: Transactional Domain
//! Operations".
//!
//! This crate defines the *boundary*, not the concrete undo inverses for any
//! specific engine (that lands with real domain state in Phase 08). It is
//! generic over a change type `C` so every later domain crate (geometry,
//! dimensions, constraints, views, ...) can reuse the same atomicity and
//! history mechanics instead of re-implementing them per engine, which would
//! violate Engine Contract 16's "no duplicate business logic per input
//! surface."
//!
//! Two invariants this module makes hard to violate:
//! - a transaction's changes become visible either all at once (`commit`) or
//!   not at all (`rollback` or simply dropping an open transaction);
//! - undo/redo operate on whole committed transactions, never on individual
//!   changes inside one, so "one visible user action" always undoes as one
//!   coherent action (Engine Contract 14's authority boundary).

use craftloop_errors::{DomainError, DomainResult, TransactionErrorKind};
use craftloop_ids::{CraftLoopId, TransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransactionStatus {
    Open,
    Committed,
    RolledBack,
}

/// An in-progress atomic transaction accumulating changes of type `C`.
///
/// `C` is intentionally an opaque, caller-defined type: this crate does not
/// know what a "change" is for geometry vs. dimensions vs. views. Later
/// crates supply their own change enum and get atomicity/history for free.
#[derive(Debug)]
pub struct Transaction<C> {
    id: TransactionId,
    changes: Vec<C>,
    status: TransactionStatus,
}

impl<C> Transaction<C> {
    pub fn new() -> Self {
        Self {
            id: TransactionId::new(),
            changes: Vec::new(),
            status: TransactionStatus::Open,
        }
    }

    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub fn changes(&self) -> &[C] {
        &self.changes
    }

    pub fn is_open(&self) -> bool {
        self.status == TransactionStatus::Open
    }

    /// Append one change to this transaction. Fails once the transaction has
    /// already been committed or rolled back: a resolved transaction is
    /// immutable evidence, not a container you can keep editing.
    pub fn record(&mut self, change: C) -> DomainResult<()> {
        self.ensure_open("record")?;
        self.changes.push(change);
        Ok(())
    }

    /// Resolve this transaction as committed, making its changes visible as
    /// one atomic unit. Returns the immutable [`CommittedTransaction`]
    /// evidence for the caller to fold into document state and transaction
    /// history.
    pub fn commit(&mut self) -> DomainResult<CommittedTransaction<C>>
    where
        C: Clone,
    {
        self.ensure_open("commit")?;
        self.status = TransactionStatus::Committed;
        Ok(CommittedTransaction {
            id: self.id,
            changes: self.changes.clone(),
        })
    }

    /// Resolve this transaction as rolled back: none of its recorded changes
    /// take effect and it never enters transaction history.
    pub fn rollback(&mut self) -> DomainResult<()> {
        self.ensure_open("rollback")?;
        self.status = TransactionStatus::RolledBack;
        Ok(())
    }

    fn ensure_open(&self, action: &str) -> DomainResult<()> {
        if self.status == TransactionStatus::Open {
            Ok(())
        } else {
            Err(DomainError::Transaction {
                kind: TransactionErrorKind::AlreadyResolved,
                detail: format!(
                    "cannot {action} transaction {}: already {:?}",
                    self.id, self.status
                ),
            })
        }
    }
}

impl<C> Default for Transaction<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable evidence that a [`Transaction`] committed. This is the unit
/// undo/redo operate on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedTransaction<C> {
    pub id: TransactionId,
    pub changes: Vec<C>,
}

/// Ordered history of committed transactions with undo/redo bookkeeping.
///
/// `redo` is invalidated by any new commit, matching MCP Article 79 (Undo
/// and Redo): redoing after taking a genuinely new action would silently
/// resurrect a superseded branch of history, which Article 92 (Determinism)
/// and Engine Contract 14 ("never silently drop confirmed constraints" /
/// coherent action semantics) both rule out.
#[derive(Debug, Default)]
pub struct TransactionLog<C> {
    committed: Vec<CommittedTransaction<C>>,
    undone: Vec<CommittedTransaction<C>>,
}

impl<C> TransactionLog<C> {
    pub fn new() -> Self {
        Self {
            committed: Vec::new(),
            undone: Vec::new(),
        }
    }

    /// Record a freshly committed transaction as the newest point in
    /// history. Clears the redo stack.
    pub fn push(&mut self, transaction: CommittedTransaction<C>) {
        self.committed.push(transaction);
        self.undone.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.committed.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.undone.is_empty()
    }

    /// Move the most recent transaction from committed to undone history and
    /// return a copy so the caller can apply its inverse.
    pub fn undo(&mut self) -> DomainResult<CommittedTransaction<C>>
    where
        C: Clone,
    {
        match self.committed.pop() {
            Some(tx) => {
                self.undone.push(tx.clone());
                Ok(tx)
            }
            None => Err(DomainError::Transaction {
                kind: TransactionErrorKind::NoMatchingHistoryEntry,
                detail: "undo requested but transaction history is empty".to_string(),
            }),
        }
    }

    /// Move the most recently undone transaction back onto committed history
    /// and return it so the caller can re-apply its forward changes.
    pub fn redo(&mut self) -> DomainResult<CommittedTransaction<C>>
    where
        C: Clone,
    {
        match self.undone.pop() {
            Some(tx) => {
                self.committed.push(tx.clone());
                Ok(tx)
            }
            None => Err(DomainError::Transaction {
                kind: TransactionErrorKind::NoMatchingHistoryEntry,
                detail: "redo requested but there is nothing undone".to_string(),
            }),
        }
    }

    pub fn history(&self) -> &[CommittedTransaction<C>] {
        &self.committed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestChange {
        SetValue(&'static str, i64),
    }

    #[test]
    fn commit_makes_all_recorded_changes_visible_together() {
        let mut tx = Transaction::new();
        tx.record(TestChange::SetValue("a", 1)).unwrap();
        tx.record(TestChange::SetValue("b", 2)).unwrap();
        let committed = tx.commit().expect("commit should succeed");
        assert_eq!(
            committed.changes,
            vec![TestChange::SetValue("a", 1), TestChange::SetValue("b", 2)]
        );
    }

    #[test]
    fn recording_after_commit_fails_instead_of_silently_mutating_history() {
        let mut tx: Transaction<TestChange> = Transaction::new();
        tx.commit().expect("commit should succeed");
        let result = tx.record(TestChange::SetValue("late", 99));
        assert!(matches!(
            result,
            Err(DomainError::Transaction {
                kind: TransactionErrorKind::AlreadyResolved,
                ..
            })
        ));
    }

    #[test]
    fn committing_twice_fails() {
        let mut tx: Transaction<TestChange> = Transaction::new();
        tx.commit().expect("first commit should succeed");
        let second = tx.commit();
        assert!(second.is_err());
    }

    #[test]
    fn rollback_discards_changes_and_they_never_reach_history() {
        let mut tx = Transaction::new();
        tx.record(TestChange::SetValue("discarded", 1)).unwrap();
        tx.rollback().expect("rollback should succeed");
        assert!(!tx.is_open());
        // A rolled-back transaction was never pushed into any
        // TransactionLog, so there is nothing further to assert against
        // history -- its changes simply never became visible.
    }

    #[test]
    fn each_transaction_has_a_unique_stable_id() {
        let a: Transaction<TestChange> = Transaction::new();
        let b: Transaction<TestChange> = Transaction::new();
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn undo_and_redo_operate_on_whole_transactions_not_individual_changes() {
        let mut log: TransactionLog<TestChange> = TransactionLog::new();

        let mut tx1 = Transaction::new();
        tx1.record(TestChange::SetValue("x", 1)).unwrap();
        tx1.record(TestChange::SetValue("y", 2)).unwrap();
        log.push(tx1.commit().unwrap());

        let mut tx2 = Transaction::new();
        tx2.record(TestChange::SetValue("z", 3)).unwrap();
        log.push(tx2.commit().unwrap());

        assert_eq!(log.history().len(), 2);

        let undone = log.undo().expect("undo should return tx2");
        assert_eq!(undone.changes, vec![TestChange::SetValue("z", 3)]);
        assert_eq!(log.history().len(), 1);
        assert!(log.can_redo());

        let redone = log.redo().expect("redo should restore tx2");
        assert_eq!(redone.changes, vec![TestChange::SetValue("z", 3)]);
        assert_eq!(log.history().len(), 2);
    }

    #[test]
    fn new_commit_after_undo_clears_the_redo_stack() {
        let mut log: TransactionLog<TestChange> = TransactionLog::new();

        let mut tx1 = Transaction::new();
        tx1.record(TestChange::SetValue("first", 1)).unwrap();
        log.push(tx1.commit().unwrap());

        log.undo().unwrap();
        assert!(log.can_redo());

        let mut tx2 = Transaction::new();
        tx2.record(TestChange::SetValue("second", 2)).unwrap();
        log.push(tx2.commit().unwrap());

        assert!(
            !log.can_redo(),
            "a fresh commit must invalidate the old redo branch"
        );
    }

    #[test]
    fn undo_on_empty_history_is_a_structured_error_not_a_panic() {
        let mut log: TransactionLog<TestChange> = TransactionLog::new();
        let result = log.undo();
        assert!(matches!(
            result,
            Err(DomainError::Transaction {
                kind: TransactionErrorKind::NoMatchingHistoryEntry,
                ..
            })
        ));
    }
}
