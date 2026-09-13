//! Structured domain errors and diagnostics for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 01, Task 009. Authority: Engine Contracts 01-30
//! ("Structured diagnostics when invalid input is possible" is a mandatory
//! evidence item for every contract); MCP Article 170 "Error Philosophy".
//!
//! Forbidden shortcut this crate exists to close: "do not use free-form
//! strings as the only error contract for solver, parser, consistency, or
//! persistence failures." Every [`DomainError`] variant carries a typed
//! `kind` enum a caller can `match` on; the `detail` string is present only
//! for human-readable context, never as the sole signal.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// How serious a diagnostic is. Distinct from `DomainError` itself because
/// the same underlying condition can be informational in one context and
/// blocking in another (e.g. an unresolved dimension is a `Warning` while a
/// view is being sketched but may need to be a `Blocker` at an export gate).
///
/// MCP Article 4 "Never fabricate engineering certainty" and Article 29
/// "Under-Constrained Does Not Mean Wrong" both depend on severity being
/// separate from mere presence of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Purely informational; no action required.
    Info,
    /// Worth surfacing to the user but does not block the current action.
    Warning,
    /// The requested operation could not complete as specified.
    Error,
    /// A true blocker: nothing downstream can proceed until this resolves.
    Blocker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometryErrorKind {
    /// Input collapses to a degenerate case (e.g. zero-length segment used
    /// as a direction vector).
    DegenerateInput,
    /// A value fell outside what `craftloop-geometry`'s tolerance policy
    /// considers valid (e.g. non-positive circle radius).
    ToleranceViolation,
    /// A computed value is finite but outside an expected domain range.
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParserErrorKind {
    /// The input could not be parsed as a number at all.
    InvalidNumber,
    /// A unit token was present but not recognized/supported.
    InvalidUnit,
    /// A decimal separator could not be resolved without locale context.
    AmbiguousDecimalSeparator,
    /// Input was empty or whitespace-only.
    EmptyInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolverErrorKind {
    /// More constraints were confirmed than the system has degrees of
    /// freedom to satisfy.
    OverConstrained,
    /// The constraint set has no solution, independent of DOF counting.
    Infeasible,
    /// The numerical solve did not converge within budget.
    DidNotConverge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsistencyErrorKind {
    /// Two or more confirmed facts contradict each other.
    Conflict,
    /// A reference points at an entity that does not exist in the document.
    UnknownReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistenceErrorKind {
    /// On-disk schema version is newer or incompatible with this build.
    SchemaVersionMismatch,
    /// The document failed an integrity check on load.
    CorruptedDocument,
    /// The underlying storage operation failed (disk, permissions, etc.).
    IoFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionErrorKind {
    /// A commit was attempted on a transaction that already committed or
    /// rolled back.
    AlreadyResolved,
    /// One of the operations inside the transaction failed, so the whole
    /// transaction must not apply (atomicity).
    OperationFailed,
    /// An undo/redo was requested but history does not contain a matching
    /// entry.
    NoMatchingHistoryEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputErrorKind {
    /// A numeric field (pressure, tilt, ...) was outside its valid range.
    OutOfRange,
    /// An event arrived that is not valid in the current stroke lifecycle
    /// state (e.g. `Move` before any `Down`).
    InvalidLifecycleSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentErrorKind {
    /// An entity was inserted under an ID that already exists in the same
    /// page (would silently overwrite a distinct entity).
    DuplicateEntityId,
    /// A reference (e.g. a page ID) does not resolve to anything in the
    /// document.
    UnknownReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InkErrorKind {
    /// A stroke was constructed with zero samples.
    EmptyStroke,
    /// A stroke's samples did not all report the same `PointerSource`.
    MixedSource,
}

/// A structured domain error. Every variant is namespaced by subsystem and
/// carries a typed `kind`; `detail` is supplementary human-readable context
/// only, per the forbidden-shortcut rule above.
#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
pub enum DomainError {
    #[error("geometry error ({kind:?}): {detail}")]
    Geometry {
        kind: GeometryErrorKind,
        detail: String,
    },
    #[error("parser error ({kind:?}) on input {input:?}: {detail}")]
    Parser {
        kind: ParserErrorKind,
        input: String,
        detail: String,
    },
    #[error("solver error ({kind:?}): {detail}")]
    Solver {
        kind: SolverErrorKind,
        detail: String,
    },
    #[error("consistency error ({kind:?}): {detail}")]
    Consistency {
        kind: ConsistencyErrorKind,
        detail: String,
    },
    #[error("persistence error ({kind:?}): {detail}")]
    Persistence {
        kind: PersistenceErrorKind,
        detail: String,
    },
    #[error("transaction error ({kind:?}): {detail}")]
    Transaction {
        kind: TransactionErrorKind,
        detail: String,
    },
    #[error("input error ({kind:?}): {detail}")]
    Input {
        kind: InputErrorKind,
        detail: String,
    },
    #[error("ink error ({kind:?}): {detail}")]
    Ink { kind: InkErrorKind, detail: String },
    #[error("document error ({kind:?}): {detail}")]
    Document {
        kind: DocumentErrorKind,
        detail: String,
    },
}

impl DomainError {
    /// The default severity for this error's kind when no richer context is
    /// available. Callers with more context (e.g. "this dimension is merely
    /// unresolved, not conflicting") should build a [`Diagnostic`] with an
    /// explicit severity instead of relying on this default.
    pub fn default_severity(&self) -> Severity {
        match self {
            DomainError::Geometry { .. } => Severity::Error,
            DomainError::Parser { .. } => Severity::Warning,
            DomainError::Solver {
                kind: SolverErrorKind::OverConstrained,
                ..
            } => Severity::Error,
            DomainError::Solver { .. } => Severity::Error,
            DomainError::Consistency { .. } => Severity::Error,
            DomainError::Persistence {
                kind: PersistenceErrorKind::CorruptedDocument,
                ..
            } => Severity::Blocker,
            DomainError::Persistence { .. } => Severity::Error,
            DomainError::Transaction { .. } => Severity::Error,
            DomainError::Input { .. } => Severity::Error,
            DomainError::Ink { .. } => Severity::Error,
            DomainError::Document { .. } => Severity::Error,
        }
    }
}

/// A diagnostic ready for display to the user or export (Engine Contract
/// 26). Wraps a [`DomainError`] with an explicit [`Severity`] and optional
/// references to the entities involved, so a UI can highlight exactly what
/// the diagnostic is about without parsing a message string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub error: DomainError,
    /// Stable string form of related entity IDs (e.g. from
    /// `craftloop-ids`). Kept as strings here so this low-level crate has no
    /// dependency on the ID crate; callers format their typed IDs with
    /// `Display` before attaching them.
    pub related_entities: Vec<String>,
}

impl Diagnostic {
    pub fn new(error: DomainError) -> Self {
        let severity = error.default_severity();
        Self {
            severity,
            error,
            related_entities: Vec::new(),
        }
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_related_entity(mut self, entity: impl Into<String>) -> Self {
        self.related_entities.push(entity.into());
        self
    }

    pub fn is_blocking(&self) -> bool {
        matches!(self.severity, Severity::Blocker)
    }
}

pub type DomainResult<T> = Result<T, DomainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_never_loses_the_typed_kind() {
        let err = DomainError::Parser {
            kind: ParserErrorKind::AmbiguousDecimalSeparator,
            input: "1,234".to_string(),
            detail: "locale not specified".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("AmbiguousDecimalSeparator"));
        assert!(msg.contains("1,234"));
    }

    #[test]
    fn callers_can_match_on_kind_without_string_parsing() {
        let err = DomainError::Solver {
            kind: SolverErrorKind::OverConstrained,
            detail: "3 extra constraints".to_string(),
        };
        let is_over_constrained = matches!(
            err,
            DomainError::Solver {
                kind: SolverErrorKind::OverConstrained,
                ..
            }
        );
        assert!(is_over_constrained);
    }

    #[test]
    fn corrupted_document_defaults_to_blocker_severity() {
        let err = DomainError::Persistence {
            kind: PersistenceErrorKind::CorruptedDocument,
            detail: "checksum mismatch".to_string(),
        };
        assert_eq!(err.default_severity(), Severity::Blocker);
    }

    #[test]
    fn diagnostic_builder_attaches_related_entities() {
        let diag = Diagnostic::new(DomainError::Consistency {
            kind: ConsistencyErrorKind::Conflict,
            detail: "two driving dimensions disagree".to_string(),
        })
        .with_related_entity("DimensionId(11111111-1111-1111-1111-111111111111)")
        .with_related_entity("DimensionId(22222222-2222-2222-2222-222222222222)");

        assert_eq!(diag.related_entities.len(), 2);
        assert_eq!(diag.severity, Severity::Error);
        assert!(!diag.is_blocking());
    }

    #[test]
    fn severity_can_be_overridden_for_context_the_default_cannot_know() {
        // Article 29: under-constrained is not automatically an error. A
        // caller mid-sketch may downgrade a solver diagnostic to Info.
        let diag = Diagnostic::new(DomainError::Solver {
            kind: SolverErrorKind::Infeasible,
            detail: "sketch is mid-edit".to_string(),
        })
        .with_severity(Severity::Info);
        assert_eq!(diag.severity, Severity::Info);
    }

    #[test]
    fn diagnostic_round_trips_through_json() {
        let diag = Diagnostic::new(DomainError::Geometry {
            kind: GeometryErrorKind::ToleranceViolation,
            detail: "radius <= 0".to_string(),
        });
        let json = serde_json::to_string(&diag).expect("serialize");
        let back: Diagnostic = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(diag, back);
    }

    #[test]
    fn severity_ordering_supports_finding_the_worst_diagnostic() {
        let mut severities = vec![Severity::Warning, Severity::Blocker, Severity::Info];
        severities.sort();
        assert_eq!(
            severities,
            vec![Severity::Info, Severity::Warning, Severity::Blocker]
        );
        assert_eq!(severities.iter().max(), Some(&Severity::Blocker));
    }
}
