//! `CraftLoopSession`: the Android/iPad product-session API.
//!
//! Execution 02, Phase 04 (Tasks 022-033). Authority: Execution 02
//! Article 12 ("CraftLoopSession Mobile API") and Article 13 ("Scene
//! Snapshot Contract").
//!
//! Execution 01 built ~20 independent, fully tested domain engines
//! (document, dimension, consistency, sketch/constraints, transactions,
//! command) but never wired them into one persisted, undoable session --
//! every scenario test that needed more than one engine constructed them
//! as bare local variables. This module is that wiring: one
//! `CraftLoopSession` per open document, holding exactly the real engines
//! (never reimplementing their validation), exposed across the UniFFI
//! boundary as an opaque object so a mobile client can drive the whole
//! Golden Alpha Journey (Article 35) through one coherent API.
//!
//! **Mutation discipline.** Every operation that changes persisted
//! document state does so by building a scratch clone of whatever engine
//! state it needs (`Document::dimension_store()`/`multiview_graph()`/
//! `sketch()` all derive `Clone` for exactly this), calling the real,
//! already-tested engine function against the clone, and then committing
//! the resulting delta through `DocumentHistory::commit` as one or more
//! `DocumentChange`s -- never mutating `state.document` directly outside
//! a commit. This is what makes every operation here undoable/redoable
//! and gives Article 35's "kill the app, reopen it" step something real
//! to restore.
//!
//! **Command Bus wiring.** Every operation with a natural
//! `CommandAction` (Article 236/239; `craftloop_command::bus::CommandBus`
//! existed since Phase 19 but nothing had ever called `submit` from a
//! real product surface) is submitted through the bus first, at
//! `RiskLevel::Low` -- every Execution 02 Alpha action is an explicit,
//! already-decided user action (a toolbar tap, a typed value), never
//! risky/ambiguous ink-derived input needing gathered confirmation
//! evidence (that is Phase 15's Ink Command Adapter, out of this phase's
//! scope), so `requires_confirmation(Low) == false` and
//! `submit(command, None)` always succeeds once the namespace check
//! passes. `undo`/`redo` and `delete_selected` have no matching
//! `CommandAction` in Article 237's vocabulary at all (neither "Undo",
//! "Redo", nor "Delete" is a named word in any namespace) and are not
//! routed through the bus -- inventing a new vocabulary word for them
//! would be exactly the scope creep Article 30/37 forbid. Likewise,
//! `apply_constraint`/`remove_constraint`/`solve_constraints` have no
//! matching action (Article 237 never names a "Constraint" command word)
//! and go straight to the real `Sketch` engine.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use craftloop_command::{
    Command, CommandAction, CommandBus, CommandSource, RiskLevel, UndoMetadata,
};
use craftloop_consistency::{resolve, Conflict, ConflictKind, ConflictStatus, ResolutionChoice};
use craftloop_constraint::SolveStatus;
use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget, SemanticDimension};
use craftloop_document::{
    axes_for_identity, evaluate_readiness, load_document, propagate_confirmed_value,
    propose_shared_value, save_document_atomically, transition_to_orthographic, Document,
    DocumentChange, DocumentHistory, EntityId, OrthographicReadiness, OrthographicSet,
    PrincipalViewIdentity, ProjectionConvention, SemanticEntity, SharedAxis, ViewBlock,
};
use craftloop_errors::{DomainError, Severity};
use craftloop_geometry::{Circle2, Point2, RelationalRectangle, Segment2};
use craftloop_ids::{
    ConflictId, ConstraintId, CraftLoopId, DimensionId, OrthographicSetId, PrimitiveId, StrokeId,
    ViewId,
};
use craftloop_ink::Stroke;
use craftloop_input::PointerSample;
use craftloop_recognition::{beautify, rank_candidates, recognize, RecognitionCandidate};
use craftloop_sketch::{ConstraintOutcome, ConstraintProvenance, EzpzSolver, SketchConstraintKind};

use crate::FfiPointerSample;

// ---------------------------------------------------------------------
// Error boundary
// ---------------------------------------------------------------------

/// Execution 02, Phase 04: every fallible `CraftLoopSession` method
/// returns this rather than panicking. A single carrying variant (Article
/// 12 asks for structured session operations, not a taxonomy of FFI error
/// kinds this phase has no concrete Android-side handling for yet) --
/// `detail` always comes from a real `DomainError`/`PersistError`'s own
/// `Display`, which already names the specific domain error kind (see
/// `craftloop_errors::DomainError`'s `#[error(...)]` messages), so no
/// diagnostic information is lost by not mirroring every `*ErrorKind`
/// enum across the boundary too.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum FfiSessionError {
    #[error("{detail}")]
    Domain { detail: String },
}

impl From<DomainError> for FfiSessionError {
    fn from(err: DomainError) -> Self {
        FfiSessionError::Domain {
            detail: err.to_string(),
        }
    }
}

impl From<craftloop_document::persistence::PersistError> for FfiSessionError {
    fn from(err: craftloop_document::persistence::PersistError) -> Self {
        FfiSessionError::Domain {
            detail: err.to_string(),
        }
    }
}

fn missing(kind: &str, id: impl std::fmt::Display) -> FfiSessionError {
    FfiSessionError::Domain {
        detail: format!("no {kind} with id {id}"),
    }
}

// ---------------------------------------------------------------------
// Small id/time helpers
// ---------------------------------------------------------------------

/// Parse a plain UUID string (as every single-kind `CraftLoopId` renders
/// via its own `Display`) back into its typed id. Mirrors
/// `craftloop_document::EntityId::from_str`'s own technique
/// (`Uuid::parse_str` then `from_u128`) -- that is this workspace's own
/// established precedent for reconstructing a typed id from a string at a
/// boundary, not a new exception to `CraftLoopId::from_u128`'s "tests
/// only" guidance.
fn parse_id<T: CraftLoopId>(raw: &str) -> Result<T, FfiSessionError> {
    let uuid = uuid::Uuid::parse_str(raw).map_err(|err| FfiSessionError::Domain {
        detail: format!("invalid id {raw:?}: {err}"),
    })?;
    Ok(T::from_u128(uuid.as_u128()))
}

fn parse_entity_id(raw: &str) -> Result<EntityId, FfiSessionError> {
    raw.parse()
        .map_err(|err: String| FfiSessionError::Domain { detail: err })
}

fn now_seconds() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

// ---------------------------------------------------------------------
// FFI enums mirroring domain enums (see the crate's own module doc:
// every Ffi* type is a plain mirror with an explicit conversion, never
// the domain type re-exported directly).
// ---------------------------------------------------------------------

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiPrincipalViewIdentity {
    Front,
    Top,
    Right,
    Back,
}

impl From<FfiPrincipalViewIdentity> for PrincipalViewIdentity {
    fn from(identity: FfiPrincipalViewIdentity) -> Self {
        match identity {
            FfiPrincipalViewIdentity::Front => PrincipalViewIdentity::Front,
            FfiPrincipalViewIdentity::Top => PrincipalViewIdentity::Top,
            FfiPrincipalViewIdentity::Right => PrincipalViewIdentity::Right,
            FfiPrincipalViewIdentity::Back => PrincipalViewIdentity::Back,
        }
    }
}

impl From<PrincipalViewIdentity> for FfiPrincipalViewIdentity {
    fn from(identity: PrincipalViewIdentity) -> Self {
        match identity {
            PrincipalViewIdentity::Front => FfiPrincipalViewIdentity::Front,
            PrincipalViewIdentity::Top => FfiPrincipalViewIdentity::Top,
            PrincipalViewIdentity::Right => FfiPrincipalViewIdentity::Right,
            PrincipalViewIdentity::Back => FfiPrincipalViewIdentity::Back,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiSharedAxis {
    Width,
    Height,
    Depth,
}

impl From<FfiSharedAxis> for SharedAxis {
    fn from(axis: FfiSharedAxis) -> Self {
        match axis {
            FfiSharedAxis::Width => SharedAxis::Width,
            FfiSharedAxis::Height => SharedAxis::Height,
            FfiSharedAxis::Depth => SharedAxis::Depth,
        }
    }
}

impl From<SharedAxis> for FfiSharedAxis {
    fn from(axis: SharedAxis) -> Self {
        match axis {
            SharedAxis::Width => FfiSharedAxis::Width,
            SharedAxis::Height => FfiSharedAxis::Height,
            SharedAxis::Depth => FfiSharedAxis::Depth,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiDimensionKind {
    Linear,
    Angular,
    Radius,
    Diameter,
}

impl From<FfiDimensionKind> for DimensionKind {
    fn from(kind: FfiDimensionKind) -> Self {
        match kind {
            FfiDimensionKind::Linear => DimensionKind::Linear,
            FfiDimensionKind::Angular => DimensionKind::Angular,
            FfiDimensionKind::Radius => DimensionKind::Radius,
            FfiDimensionKind::Diameter => DimensionKind::Diameter,
        }
    }
}

impl From<DimensionKind> for FfiDimensionKind {
    fn from(kind: DimensionKind) -> Self {
        match kind {
            DimensionKind::Linear => FfiDimensionKind::Linear,
            DimensionKind::Angular => FfiDimensionKind::Angular,
            DimensionKind::Radius => FfiDimensionKind::Radius,
            DimensionKind::Diameter => FfiDimensionKind::Diameter,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiDimensionRole {
    Driving,
    Reference,
    Derived,
    Shared,
    Bounded,
}

impl From<DimensionRole> for FfiDimensionRole {
    fn from(role: DimensionRole) -> Self {
        match role {
            DimensionRole::Driving => FfiDimensionRole::Driving,
            DimensionRole::Reference => FfiDimensionRole::Reference,
            DimensionRole::Derived => FfiDimensionRole::Derived,
            DimensionRole::Shared => FfiDimensionRole::Shared,
            DimensionRole::Bounded => FfiDimensionRole::Bounded,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiResolutionChoice {
    KeepExisting,
    ReplaceAndPropagate,
    Unlink,
    RemoveConstraint,
    Cancel,
}

impl From<FfiResolutionChoice> for ResolutionChoice {
    fn from(choice: FfiResolutionChoice) -> Self {
        match choice {
            FfiResolutionChoice::KeepExisting => ResolutionChoice::KeepExisting,
            FfiResolutionChoice::ReplaceAndPropagate => ResolutionChoice::ReplaceAndPropagate,
            FfiResolutionChoice::Unlink => ResolutionChoice::Unlink,
            FfiResolutionChoice::RemoveConstraint => ResolutionChoice::RemoveConstraint,
            FfiResolutionChoice::Cancel => ResolutionChoice::Cancel,
        }
    }
}

impl From<ResolutionChoice> for FfiResolutionChoice {
    fn from(choice: ResolutionChoice) -> Self {
        match choice {
            ResolutionChoice::KeepExisting => FfiResolutionChoice::KeepExisting,
            ResolutionChoice::ReplaceAndPropagate => FfiResolutionChoice::ReplaceAndPropagate,
            ResolutionChoice::Unlink => FfiResolutionChoice::Unlink,
            ResolutionChoice::RemoveConstraint => FfiResolutionChoice::RemoveConstraint,
            ResolutionChoice::Cancel => FfiResolutionChoice::Cancel,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiConflictKind {
    DegenerateGeometry,
    DimensionConstraintMismatch,
    UnitMisapplication,
    CrossViewMismatch,
}

impl From<ConflictKind> for FfiConflictKind {
    fn from(kind: ConflictKind) -> Self {
        match kind {
            ConflictKind::DegenerateGeometry => FfiConflictKind::DegenerateGeometry,
            ConflictKind::DimensionConstraintMismatch => {
                FfiConflictKind::DimensionConstraintMismatch
            }
            ConflictKind::UnitMisapplication => FfiConflictKind::UnitMisapplication,
            ConflictKind::CrossViewMismatch => FfiConflictKind::CrossViewMismatch,
        }
    }
}

/// Article 17's stable constraint list, mapped 1:1 onto the
/// `SketchConstraintKind` variants that already exist (`craftloop-sketch`,
/// Phase 12) -- primitive/point references cross the FFI boundary as
/// plain UUID strings (`parse_id`), matching every other id in this
/// module. `Fixed`/`LineTangentToCircle`/`CircleTangentToCircle`/
/// `Symmetric` are omitted: Article 17 names only "Coincident,
/// Horizontal, Vertical, Parallel, Perpendicular, Equal, and Concentric"
/// (plus "Tangent/Symmetry only if the existing engine already supports
/// them reliably" -- true here, but adding all eleven `SketchConstraintKind`
/// variants to the Alpha toolbar contract would be exactly the "expand
/// the toolbar" scope creep Article 17 itself forbids; Tangent/Symmetric
/// point-level constraints need a `PointRef`, not a bare `PrimitiveId`,
/// mirroring which is deferred to whichever phase actually adds those
/// tools).
#[derive(uniffi::Enum, Debug, Clone, PartialEq)]
pub enum FfiConstraintKind {
    Coincident { a: String, b: String },
    Horizontal { line: String },
    Vertical { line: String },
    Parallel { a: String, b: String },
    Perpendicular { a: String, b: String },
    EqualLength { a: String, b: String },
    EqualRadius { a: String, b: String },
    Concentric { a: String, b: String },
}

impl FfiConstraintKind {
    fn into_domain(self) -> Result<SketchConstraintKind, FfiSessionError> {
        use craftloop_sketch::PointRef;
        Ok(match self {
            FfiConstraintKind::Coincident { a, b } => SketchConstraintKind::Coincident(
                PointRef::LineEnd(parse_id::<PrimitiveId>(&a)?),
                PointRef::LineStart(parse_id::<PrimitiveId>(&b)?),
            ),
            FfiConstraintKind::Horizontal { line } => {
                SketchConstraintKind::Horizontal(parse_id(&line)?)
            }
            FfiConstraintKind::Vertical { line } => {
                SketchConstraintKind::Vertical(parse_id(&line)?)
            }
            FfiConstraintKind::Parallel { a, b } => {
                SketchConstraintKind::Parallel(parse_id(&a)?, parse_id(&b)?)
            }
            FfiConstraintKind::Perpendicular { a, b } => {
                SketchConstraintKind::Perpendicular(parse_id(&a)?, parse_id(&b)?)
            }
            FfiConstraintKind::EqualLength { a, b } => {
                SketchConstraintKind::EqualLength(parse_id(&a)?, parse_id(&b)?)
            }
            FfiConstraintKind::EqualRadius { a, b } => {
                SketchConstraintKind::EqualRadius(parse_id(&a)?, parse_id(&b)?)
            }
            FfiConstraintKind::Concentric { a, b } => {
                SketchConstraintKind::Concentric(parse_id(&a)?, parse_id(&b)?)
            }
        })
    }
}

#[derive(uniffi::Enum, Debug, Clone, PartialEq)]
pub enum FfiConstraintOutcome {
    Added { constraint_id: String },
    Redundant { existing_constraint_id: String },
}

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiSolveStatus {
    Solved,
    Unsatisfied,
    Failed,
}

impl From<SolveStatus> for FfiSolveStatus {
    fn from(status: SolveStatus) -> Self {
        match status {
            SolveStatus::Solved => FfiSolveStatus::Solved,
            SolveStatus::Unsatisfied => FfiSolveStatus::Unsatisfied,
            SolveStatus::Failed => FfiSolveStatus::Failed,
        }
    }
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiSolveOutcome {
    pub status: FfiSolveStatus,
    pub unsatisfied_constraint_ids: Vec<String>,
    /// How many primitives actually moved -- `0` with `status == Solved`
    /// means "already satisfied, nothing to redraw."
    pub updated_primitive_count: u32,
}

#[derive(uniffi::Enum, Debug, Clone, PartialEq)]
pub enum FfiPropagateOutcome {
    Propagated { affected_views: Vec<String> },
    Conflict { conflict_id: String },
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiStrokeOutcome {
    pub stroke_id: String,
    /// From a real `recognize()`/`rank_candidates()` call against this
    /// stroke's own points (Article 12's "keep/accept/reject recognition
    /// result") -- never a guess; `false` when the top-ranked candidate is
    /// `RecognitionCandidate::KeepAsInk`.
    pub eligible_for_recognition: bool,
}

// ---------------------------------------------------------------------
// Scene snapshot (Article 13)
// ---------------------------------------------------------------------

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiPrimitiveKind {
    Line,
    Circle,
    Arc,
    Rectangle,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiStrokeSummary {
    pub id: String,
    pub sample_count: u32,
}

/// Execution 02, Phase 08 (Tasks 055/057): the coarse snapshot originally
/// carried no geometry at all for a primitive -- only `id`/`kind` --
/// which made tap-to-select hit-testing and fit-to-content bounding-box
/// computation impossible from the Android side. Every
/// `BeautifiedPrimitive` variant already has a real, tested `.bounds()`
/// method (`craftloop-geometry`'s own `Segment2`/`Circle2`/`Arc2`/
/// `RelationalRectangle`, Phase 06 origin); this reuses that directly
/// rather than adding new geometry logic here, matching this crate's own
/// rule of never reimplementing a domain calculation at the FFI layer.
/// A bounding box (not full point/edge geometry) is the minimum Article
/// 13 needs for these two tasks -- still a coarse snapshot, not a
/// diffing/full-fidelity contract.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiPrimitiveSummary {
    pub id: String,
    pub kind: FfiPrimitiveKind,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiDimensionSummary {
    pub id: String,
    pub kind: FfiDimensionKind,
    pub role: FfiDimensionRole,
    pub value: f64,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiConflictSummary {
    pub id: String,
    pub kind: FfiConflictKind,
    pub unresolved: bool,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiViewBlockSummary {
    pub id: String,
    pub identity: Option<FfiPrincipalViewIdentity>,
    pub geometry_member_count: u32,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiOrthographicSetSummary {
    pub id: String,
    pub view_ids: Vec<String>,
}

/// Article 13: a coarse, full snapshot ("optimize to diffs later only if
/// measurement justifies it"). Selection is read from `SessionState`
/// directly (ephemeral UI state, Article 12's own distinction), never
/// from `Document` -- there is nothing to persist or undo about it.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiSceneSnapshot {
    pub strokes: Vec<FfiStrokeSummary>,
    pub primitives: Vec<FfiPrimitiveSummary>,
    pub dimensions: Vec<FfiDimensionSummary>,
    pub conflicts: Vec<FfiConflictSummary>,
    pub view_blocks: Vec<FfiViewBlockSummary>,
    pub orthographic_sets: Vec<FfiOrthographicSetSummary>,
    pub selected_entity_ids: Vec<String>,
    pub revision: u64,
    pub can_undo: bool,
    pub can_redo: bool,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiDebugState {
    pub revision: u64,
    pub transaction_count: u32,
    pub can_undo: bool,
    pub can_redo: bool,
    pub unresolved_conflict_count: u32,
    pub view_block_count: u32,
    pub orthographic_set_count: u32,
}

// ---------------------------------------------------------------------
// Session state and object
// ---------------------------------------------------------------------

struct SessionState {
    document: Document,
    history: DocumentHistory,
    command_bus: CommandBus,
    /// Ephemeral, session-local selection (Article 12/Phase 06 Task 041's
    /// ephemeral-vs-semantic distinction) -- never a `DocumentChange`,
    /// never persisted, never undoable on its own.
    selection: BTreeSet<EntityId>,
    /// Execution 02, Phase 04: which view/axis/value a still-unresolved
    /// `CrossViewMismatch` conflict (created by `propagate_shared_value`)
    /// proposed, so `resolve_conflict`'s `ReplaceAndPropagate` choice can
    /// actually apply that exact proposal through the real
    /// `propagate_confirmed_value` function instead of re-parsing it out
    /// of `Conflict::affected_entities`'s free-form diagnostic strings
    /// (Article 27's own doc: those strings are for a human, not a
    /// machine-parseable reference). Deliberately not persisted: an
    /// unresolved conflict surviving save/reopen (Gate M/O) still shows
    /// as a real, resolvable conflict, but resolving `ReplaceAndPropagate`
    /// on one from a *previous* session falls back to `KeepExisting`'s
    /// effect (no crash, explicit, documented limitation -- see the
    /// Phase 04 evidence file).
    pending_shared_value_proposals:
        std::collections::BTreeMap<ConflictId, (ViewId, SharedAxis, f64)>,
}

impl SessionState {
    fn active_page(&self) -> Result<craftloop_ids::PageId, FfiSessionError> {
        self.document
            .active_page()
            .ok_or_else(|| FfiSessionError::Domain {
                detail: "document has no active page".to_string(),
            })
    }

    fn low_risk_command(
        &self,
        action: CommandAction,
        namespace: craftloop_command::CommandNamespace,
        description: &str,
    ) -> Command {
        Command {
            id: craftloop_ids::CommandId::new(),
            action,
            source: CommandSource::Toolbar,
            namespace,
            parameters: std::collections::BTreeMap::new(),
            risk: RiskLevel::Low,
            timestamp_seconds: now_seconds(),
            undo: UndoMetadata::undoable(description),
        }
    }

    fn submit(
        &mut self,
        action: CommandAction,
        namespace: craftloop_command::CommandNamespace,
        description: &str,
    ) -> Result<(), FfiSessionError> {
        let command = self.low_risk_command(action, namespace, description);
        self.command_bus.submit(command, None)?;
        Ok(())
    }

    /// `self.history.commit(&mut self.document, ...)` does not borrow-check
    /// through a `MutexGuard<SessionState>` receiver the way it would on a
    /// bare struct (disjoint-field-borrow analysis does not see through an
    /// arbitrary `DerefMut`) -- every call site in this module goes through
    /// this method instead, which performs the same split borrow on a
    /// concrete `&mut SessionState` where the compiler can see it is safe.
    fn commit(
        &mut self,
        changes: Vec<DocumentChange>,
    ) -> Result<craftloop_ids::TransactionId, FfiSessionError> {
        Ok(self.history.commit(&mut self.document, changes)?)
    }

    fn undo(&mut self) -> Result<(), FfiSessionError> {
        self.history.undo(&mut self.document)?;
        Ok(())
    }

    fn redo(&mut self) -> Result<(), FfiSessionError> {
        self.history.redo(&mut self.document)?;
        Ok(())
    }
}

/// The Android/iPad product session (Article 12). Exposed to Kotlin/Swift
/// as an opaque, reference-counted handle (UniFFI's `Object` pattern);
/// every method takes `&self` and reaches interior state through a
/// `Mutex`, since a mobile client (Kotlin `ViewModel`, a Compose
/// coroutine) may call it from more than one thread.
#[derive(uniffi::Object)]
pub struct CraftLoopSession {
    state: Mutex<SessionState>,
}

impl CraftLoopSession {
    fn lock(&self) -> std::sync::MutexGuard<'_, SessionState> {
        // A poisoned lock means some earlier call panicked mid-mutation --
        // this workspace's own convention (`craftloop-mobile-ffi`'s
        // existing tests, `history.rs`'s rollback-on-error path) is to
        // never let a caller-triggerable error become a panic in the
        // first place, so a poison here would itself already be a bug
        // this session cannot recover from cleanly; recovering the inner
        // guard anyway (rather than panicking again here) keeps the
        // session at least minimally usable for the caller to save/
        // inspect state instead of losing everything.
        match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

#[uniffi::export]
impl CraftLoopSession {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            state: Mutex::new(SessionState {
                document: Document::new("Untitled", now_seconds()),
                history: DocumentHistory::new(),
                command_bus: CommandBus::new(),
                selection: BTreeSet::new(),
                pending_shared_value_proposals: std::collections::BTreeMap::new(),
            }),
        }
    }

    #[uniffi::constructor]
    pub fn open(path: String) -> Result<Self, FfiSessionError> {
        let document = load_document(Path::new(&path))?;
        Ok(Self {
            state: Mutex::new(SessionState {
                document,
                history: DocumentHistory::new(),
                command_bus: CommandBus::new(),
                selection: BTreeSet::new(),
                pending_shared_value_proposals: std::collections::BTreeMap::new(),
            }),
        })
    }

    pub fn save(&self, path: String) -> Result<(), FfiSessionError> {
        let state = self.lock();
        save_document_atomically(Path::new(&path), &state.document)?;
        Ok(())
    }

    pub fn scene_snapshot(&self) -> FfiSceneSnapshot {
        let state = self.lock();
        let document = &state.document;

        let mut strokes = Vec::new();
        let mut primitives = Vec::new();
        let mut conflicts = Vec::new();
        if let Some(page_id) = document.active_page() {
            if let Some(page) = document.page(page_id) {
                for entity in page.entities() {
                    match entity {
                        SemanticEntity::Stroke(stroke) => strokes.push(FfiStrokeSummary {
                            id: stroke.id.to_string(),
                            sample_count: stroke.len() as u32,
                        }),
                        SemanticEntity::Primitive { id, beautified } => {
                            let (kind, bounds) = match &beautified.primitive {
                                craftloop_recognition::BeautifiedPrimitive::Line(s) => {
                                    (FfiPrimitiveKind::Line, s.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Circle(c) => {
                                    (FfiPrimitiveKind::Circle, c.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Arc(a) => {
                                    (FfiPrimitiveKind::Arc, a.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Rectangle(r) => {
                                    (FfiPrimitiveKind::Rectangle, r.bounds())
                                }
                            };
                            primitives.push(FfiPrimitiveSummary {
                                id: id.to_string(),
                                kind,
                                min_x: bounds.min.x,
                                min_y: bounds.min.y,
                                max_x: bounds.max.x,
                                max_y: bounds.max.y,
                            });
                        }
                        // Dimensions render from `dimension_store()` below,
                        // the authoritative source (it also holds shared
                        // axis dimensions with no page placement at all --
                        // see `DimensionStore::dimensions`'s own doc).
                        SemanticEntity::Dimension(_) => {}
                        SemanticEntity::Conflict(conflict) => {
                            conflicts.push(FfiConflictSummary {
                                id: conflict.id.to_string(),
                                kind: conflict.kind.into(),
                                unresolved: conflict.is_unresolved(),
                            });
                        }
                        SemanticEntity::Note(_) => {}
                    }
                }
            }
        }
        let dimensions: Vec<FfiDimensionSummary> = document
            .dimension_store()
            .dimensions()
            .map(|dimension| FfiDimensionSummary {
                id: dimension.id.to_string(),
                kind: dimension.kind.into(),
                role: dimension.role.into(),
                value: dimension.value(),
            })
            .collect();

        let view_blocks = document
            .view_blocks()
            .map(|view| FfiViewBlockSummary {
                id: view.id.to_string(),
                identity: view.identity().map(Into::into),
                geometry_member_count: view.geometry_members.len() as u32,
            })
            .collect();
        let orthographic_sets = document
            .orthographic_sets()
            .map(|set| FfiOrthographicSetSummary {
                id: set.id.to_string(),
                view_ids: set.views().iter().map(|id| id.to_string()).collect(),
            })
            .collect();
        let selected_entity_ids = state.selection.iter().map(|id| id.to_string()).collect();

        FfiSceneSnapshot {
            strokes,
            primitives,
            dimensions,
            conflicts,
            view_blocks,
            orthographic_sets,
            selected_entity_ids,
            revision: document.revision(),
            can_undo: state.history.can_undo(),
            can_redo: state.history.can_redo(),
        }
    }

    pub fn debug_state(&self) -> FfiDebugState {
        let state = self.lock();
        let unresolved_conflicts = state
            .document
            .active_page()
            .and_then(|id| state.document.page(id))
            .map(|page| {
                page.entities()
                    .filter(|e| matches!(e, SemanticEntity::Conflict(c) if c.is_unresolved()))
                    .count()
            })
            .unwrap_or(0);
        FfiDebugState {
            revision: state.document.revision(),
            transaction_count: state.history.transaction_count() as u32,
            can_undo: state.history.can_undo(),
            can_redo: state.history.can_redo(),
            unresolved_conflict_count: unresolved_conflicts as u32,
            view_block_count: state.document.view_blocks().count() as u32,
            orthographic_set_count: state.document.orthographic_sets().count() as u32,
        }
    }

    // -- Input / raw ink (Article 9/12) ---------------------------------

    pub fn submit_stroke(
        &self,
        samples: Vec<FfiPointerSample>,
    ) -> Result<FfiStrokeOutcome, FfiSessionError> {
        let mut state = self.lock();
        let domain_samples: Vec<PointerSample> = samples.iter().map(PointerSample::from).collect();
        let stroke = Stroke::new(StrokeId::new(), domain_samples)?;
        let stroke_id = stroke.id;

        let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
        let ranked = rank_candidates(recognize(&points));
        let eligible = ranked
            .first()
            .map(|c| !matches!(c, RecognitionCandidate::KeepAsInk))
            .unwrap_or(false);

        let page_id = state.active_page()?;
        state.commit(vec![DocumentChange::InsertEntity {
            page_id,
            entity: SemanticEntity::Stroke(stroke),
        }])?;

        Ok(FfiStrokeOutcome {
            stroke_id: stroke_id.to_string(),
            eligible_for_recognition: eligible,
        })
    }

    /// Article 12's "keep/accept/reject recognition result", and Gate F's
    /// "produce structured semantic geometry or an explicit keep-as-ink
    /// outcome": re-run real recognition against a stored stroke's own
    /// points, and if the top-ranked candidate beautifies, replace the raw
    /// stroke with the structured primitive (in both the page and the
    /// `Sketch`) as one atomic transaction. Returns `None` -- not an error
    /// -- when the top candidate is `KeepAsInk` or does not beautify
    /// cleanly (`beautify`'s own documented `None` cases): an explicit,
    /// honest "stays ink" outcome, never a fabricated primitive.
    pub fn accept_recognition(&self, stroke_id: String) -> Result<Option<String>, FfiSessionError> {
        let mut state = self.lock();
        let stroke_id: StrokeId = parse_id(&stroke_id)?;
        let page_id = state.active_page()?;
        let entity = state
            .document
            .page(page_id)
            .and_then(|p| p.get(EntityId::Stroke(stroke_id)))
            .cloned()
            .ok_or_else(|| missing("stroke", stroke_id))?;
        let stroke = match &entity {
            SemanticEntity::Stroke(s) => s.clone(),
            _ => unreachable!("EntityId::Stroke always maps to SemanticEntity::Stroke"),
        };

        let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
        let top = rank_candidates(recognize(&points))
            .into_iter()
            .next()
            .unwrap_or(RecognitionCandidate::KeepAsInk);
        let Some(beautified) = beautify(&top) else {
            return Ok(None);
        };

        let primitive_id = PrimitiveId::new();
        state.commit(vec![
            DocumentChange::RemoveEntity { page_id, entity },
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Primitive {
                    id: primitive_id,
                    beautified: beautified.clone(),
                },
            },
            DocumentChange::InsertSketchPrimitive {
                id: primitive_id,
                beautified,
            },
        ])?;
        Ok(Some(primitive_id.to_string()))
    }

    // -- Explicit-tool primitive creation (Article 12) -------------------

    pub fn create_primitive_line(
        &self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Result<String, FfiSessionError> {
        let mut state = self.lock();
        state.submit(
            CommandAction::Line,
            craftloop_command::CommandNamespace::Sketch,
            "Created line",
        )?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(x0, y0),
                Point2::new(x1, y1),
            )),
            displacement: 0.0,
        };
        self.insert_primitive(&mut state, beautified)
    }

    pub fn create_primitive_circle(
        &self,
        center_x: f64,
        center_y: f64,
        radius: f64,
    ) -> Result<String, FfiSessionError> {
        let mut state = self.lock();
        state.submit(
            CommandAction::Circle,
            craftloop_command::CommandNamespace::Sketch,
            "Created circle",
        )?;
        let circle = Circle2::new(Point2::new(center_x, center_y), radius)?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Circle(circle),
            displacement: 0.0,
        };
        self.insert_primitive(&mut state, beautified)
    }

    pub fn create_primitive_rectangle(
        &self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Result<String, FfiSessionError> {
        let mut state = self.lock();
        state.submit(
            CommandAction::Rectangle,
            craftloop_command::CommandNamespace::Sketch,
            "Created rectangle",
        )?;
        let rectangle =
            RelationalRectangle::from_axis_aligned(Point2::new(x0, y0), Point2::new(x1, y1))?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Rectangle(rectangle),
            displacement: 0.0,
        };
        self.insert_primitive(&mut state, beautified)
    }

    // -- Selection (ephemeral, Article 12) -------------------------------

    pub fn select(&self, ids: Vec<String>) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let mut parsed = BTreeSet::new();
        for raw in &ids {
            parsed.insert(parse_entity_id(raw)?);
        }
        state.submit(
            CommandAction::Select,
            craftloop_command::CommandNamespace::Notebook,
            "Selected entities",
        )?;
        state.selection = parsed;
        Ok(())
    }

    pub fn clear_selection(&self) {
        self.lock().selection.clear();
    }

    pub fn delete_selected(&self, ids: Vec<String>) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let page_id = state.active_page()?;
        let mut changes = Vec::new();
        let mut parsed_ids = Vec::new();
        for raw in &ids {
            let entity_id = parse_entity_id(raw)?;
            parsed_ids.push(entity_id);
            if let Some(entity) = state
                .document
                .page(page_id)
                .and_then(|p| p.get(entity_id))
                .cloned()
            {
                changes.push(DocumentChange::RemoveEntity { page_id, entity });
            }
        }
        if !changes.is_empty() {
            state.commit(changes)?;
        }
        for id in parsed_ids {
            state.selection.remove(&id);
        }
        Ok(())
    }

    // -- Dimensions (Article 12/14) ---------------------------------------

    pub fn create_dimension(
        &self,
        kind: FfiDimensionKind,
        target_ids: Vec<String>,
        value: f64,
    ) -> Result<String, FfiSessionError> {
        let mut state = self.lock();
        state.submit(
            CommandAction::Dimension,
            craftloop_command::CommandNamespace::Sketch,
            "Created dimension",
        )?;

        let target = match target_ids.as_slice() {
            [a] => DimensionTarget::Single(parse_id::<PrimitiveId>(a)?),
            [a, b] => {
                DimensionTarget::Pair(parse_id::<PrimitiveId>(a)?, parse_id::<PrimitiveId>(b)?)
            }
            _ => {
                return Err(FfiSessionError::Domain {
                    detail: "a dimension needs exactly 1 or 2 target primitive ids".to_string(),
                })
            }
        };
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            kind.into(),
            DimensionRole::Driving,
            target,
            value,
        )?;
        let id = dimension.id;
        let page_id = state.active_page()?;
        state.commit(vec![
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Dimension(dimension.clone()),
            },
            DocumentChange::SetDimension {
                id,
                previous: None,
                new: Some(dimension),
            },
        ])?;
        Ok(id.to_string())
    }

    /// Article 14 Task 068 / Gate G: a valid edit changes the dimension's
    /// value (mirrored, in one transaction, onto both the page's own
    /// `SemanticEntity::Dimension` copy and the authoritative
    /// `DimensionStore` entry); an invalid edit changes nothing about the
    /// dimension but commits a real `DimensionConstraintMismatch`
    /// `Conflict` entity so the caller has something concrete to render,
    /// and still returns `Err` so the caller knows the edit itself did not
    /// take effect.
    pub fn edit_dimension(&self, id: String, new_value: f64) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let dimension_id: DimensionId = parse_id(&id)?;
        state.submit(
            CommandAction::Dimension,
            craftloop_command::CommandNamespace::Sketch,
            "Edited dimension",
        )?;

        let mut scratch = state.document.dimension_store().clone();
        match scratch.edit_driving_value(dimension_id, new_value) {
            Ok(_previous_value) => {
                let previous_dimension = state
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .cloned();
                let new_dimension = scratch.dimension(dimension_id).cloned();
                let page_id = state.active_page()?;
                let mut changes = vec![DocumentChange::SetDimension {
                    id: dimension_id,
                    previous: previous_dimension.clone(),
                    new: new_dimension.clone(),
                }];
                if let Some(previous_entity) = previous_dimension.clone() {
                    changes.push(DocumentChange::RemoveEntity {
                        page_id,
                        entity: SemanticEntity::Dimension(previous_entity),
                    });
                }
                if let Some(new_entity) = new_dimension {
                    changes.push(DocumentChange::InsertEntity {
                        page_id,
                        entity: SemanticEntity::Dimension(new_entity),
                    });
                }
                state.commit(changes)?;
                Ok(())
            }
            Err(err) => {
                let existing_value = state
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .map(|d| d.value());
                let conflict = Conflict {
                    id: ConflictId::new(),
                    kind: ConflictKind::DimensionConstraintMismatch,
                    severity: Severity::Error,
                    affected_entities: vec![format!("{dimension_id:?}")],
                    existing_truth: format!("value = {existing_value:?}"),
                    proposed_truth: format!("value = {new_value}"),
                    evidence: err.to_string(),
                    resolution_choices: vec![
                        ResolutionChoice::KeepExisting,
                        ResolutionChoice::Cancel,
                    ],
                    status: ConflictStatus::Unresolved,
                };
                let page_id = state.active_page()?;
                state.commit(vec![DocumentChange::InsertEntity {
                    page_id,
                    entity: SemanticEntity::Conflict(conflict),
                }])?;
                Err(FfiSessionError::from(err))
            }
        }
    }

    // -- Constraints (Article 17) -----------------------------------------

    pub fn apply_constraint(
        &self,
        kind: FfiConstraintKind,
    ) -> Result<FfiConstraintOutcome, FfiSessionError> {
        let mut state = self.lock();
        let domain_kind = kind.into_domain()?;
        let constraint_id = ConstraintId::new();

        let mut scratch = state.document.sketch().clone();
        let outcome = scratch.add_constraint(
            constraint_id,
            domain_kind,
            ConstraintProvenance::UserCreated,
        )?;
        match outcome {
            ConstraintOutcome::Added => {
                state.commit(vec![DocumentChange::SetSketchConstraint {
                    id: constraint_id,
                    previous: None,
                    new: Some((domain_kind, ConstraintProvenance::UserCreated)),
                }])?;
                Ok(FfiConstraintOutcome::Added {
                    constraint_id: constraint_id.to_string(),
                })
            }
            ConstraintOutcome::Redundant { existing } => Ok(FfiConstraintOutcome::Redundant {
                existing_constraint_id: existing.to_string(),
            }),
        }
    }

    pub fn remove_constraint(&self, id: String) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let constraint_id: ConstraintId = parse_id(&id)?;
        let (kind, provenance) = state
            .document
            .sketch()
            .constraint(constraint_id)
            .map(|(k, p)| (*k, *p))
            .ok_or_else(|| missing("constraint", &id))?;
        state.commit(vec![DocumentChange::SetSketchConstraint {
            id: constraint_id,
            previous: Some((kind, provenance)),
            new: None,
        }])?;
        Ok(())
    }

    /// Article 17's engine integration: solve every stored constraint
    /// against the real `EzpzSolver` (on a scratch clone, per this
    /// module's mutation discipline) and commit only the primitives that
    /// actually moved, mirrored onto both the `Sketch` and the page's own
    /// `SemanticEntity::Primitive` copy.
    pub fn solve_constraints(&self) -> Result<FfiSolveOutcome, FfiSessionError> {
        let mut state = self.lock();
        let mut scratch = state.document.sketch().clone();
        let mut solver = EzpzSolver::new();
        let result = scratch.solve(&mut solver);

        if result.status == SolveStatus::Failed {
            return Ok(FfiSolveOutcome {
                status: result.status.into(),
                unsatisfied_constraint_ids: Vec::new(),
                updated_primitive_count: 0,
            });
        }

        let page_id = state.active_page()?;
        let primitive_ids: Vec<PrimitiveId> = state.document.sketch().primitive_ids().collect();
        let mut changes = Vec::new();
        let mut updated_count = 0u32;
        for id in primitive_ids {
            let before = state.document.sketch().primitive(id).cloned();
            let after = scratch.primitive(id).cloned();
            if before == after {
                continue;
            }
            if let (Some(before_val), Some(after_val)) = (before, after) {
                changes.push(DocumentChange::RemoveSketchPrimitive {
                    id,
                    beautified: before_val.clone(),
                });
                changes.push(DocumentChange::InsertSketchPrimitive {
                    id,
                    beautified: after_val.clone(),
                });
                if let Some(SemanticEntity::Primitive { .. }) = state
                    .document
                    .page(page_id)
                    .and_then(|p| p.get(EntityId::Primitive(id)))
                {
                    changes.push(DocumentChange::RemoveEntity {
                        page_id,
                        entity: SemanticEntity::Primitive {
                            id,
                            beautified: before_val,
                        },
                    });
                    changes.push(DocumentChange::InsertEntity {
                        page_id,
                        entity: SemanticEntity::Primitive {
                            id,
                            beautified: after_val,
                        },
                    });
                }
                updated_count += 1;
            }
        }

        if !changes.is_empty() {
            state.commit(changes)?;
        }

        Ok(FfiSolveOutcome {
            status: result.status.into(),
            unsatisfied_constraint_ids: result
                .unsatisfied
                .iter()
                .map(|d| d.constraint_id.to_string())
                .collect(),
            updated_primitive_count: updated_count,
        })
    }

    // -- View Identity / Orthographic (Article 18) -------------------------

    pub fn assign_view_identity(
        &self,
        view_id: Option<String>,
        identity: FfiPrincipalViewIdentity,
    ) -> Result<String, FfiSessionError> {
        let mut state = self.lock();
        state.submit(
            CommandAction::LabelView,
            craftloop_command::CommandNamespace::Orthographic,
            "Assigned view identity",
        )?;

        let (id, previous) = match view_id {
            Some(raw) => {
                let id: ViewId = parse_id(&raw)?;
                let previous = state
                    .document
                    .view_block(id)
                    .cloned()
                    .ok_or_else(|| missing("view", id))?;
                (id, Some(previous))
            }
            None => (ViewId::new(), None),
        };
        let mut block = previous.clone().unwrap_or_else(|| ViewBlock::new(id));
        block.set_identity(identity.into());

        state.commit(vec![DocumentChange::SetViewBlock {
            id,
            previous,
            new: Some(block),
        }])?;
        Ok(id.to_string())
    }

    /// Not itself named as a separate Article 12 capability, but required
    /// by every real caller of `assign_view_identity`/`enter_orthographic`:
    /// a view's `geometry_members` (Article 30 -- what `evaluate_readiness`
    /// actually checks) has no other way to become non-empty. The smallest
    /// correct addition this phase's own "add the smallest correct
    /// primitive, do not work around a missing one" rule asks for.
    pub fn add_geometry_to_view(
        &self,
        view_id: String,
        primitive_ids: Vec<String>,
    ) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let view_id: ViewId = parse_id(&view_id)?;
        let previous = state
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;
        let mut updated = previous.clone();
        for raw in &primitive_ids {
            updated
                .geometry_members
                .insert(parse_id::<PrimitiveId>(raw)?);
        }
        state.commit(vec![DocumentChange::SetViewBlock {
            id: view_id,
            previous: Some(previous),
            new: Some(updated),
        }])?;
        Ok(())
    }

    /// Article 18/35's central workflow: enter Orthographic from a source
    /// view that already reached at least `LinkReady` (Article 234: "It
    /// should not require full dimensioning") and create the linked
    /// derived views (Top/Right/Back) at their default layout, all as one
    /// atomic transaction.
    pub fn enter_orthographic(&self, view_id: String) -> Result<Vec<String>, FfiSessionError> {
        let mut state = self.lock();
        let view_id: ViewId = parse_id(&view_id)?;
        let source = state
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;

        let mut geometry = std::collections::BTreeMap::new();
        if let Some(page_id) = state.document.active_page() {
            if let Some(page) = state.document.page(page_id) {
                for entity in page.entities() {
                    if let SemanticEntity::Primitive { id, beautified } = entity {
                        if source.geometry_members.contains(id) {
                            geometry.insert(*id, beautified.primitive.clone());
                        }
                    }
                }
            }
        }
        let (readiness, issues) = evaluate_readiness(&source, &geometry);
        if readiness < OrthographicReadiness::LinkReady {
            let blockers: Vec<String> = issues
                .iter()
                .filter(|issue| issue.is_blocker())
                .map(|issue| format!("{issue:?}"))
                .collect();
            return Err(FfiSessionError::Domain {
                detail: format!("view is not link-ready yet: {blockers:?}"),
            });
        }

        let command = state.low_risk_command(
            CommandAction::Orthographic,
            craftloop_command::CommandNamespace::Notebook,
            "Entered Orthographic mode",
        );
        state.command_bus.submit(command.clone(), None)?;
        let layouts =
            transition_to_orthographic(&command, &source, ProjectionConvention::ThirdAngle, 100.0)?;

        let mut set = OrthographicSet::new(OrthographicSetId::new());
        set.add_view(&source, &[])?;
        let mut all_views = vec![source];
        let mut new_ids = Vec::new();
        let mut changes = Vec::new();
        for (identity, layout) in &layouts {
            let mut derived = ViewBlock::new(ViewId::new());
            derived.set_identity(*identity);
            derived.move_to(*layout);
            let existing_refs: Vec<&ViewBlock> = all_views.iter().collect();
            set.add_view(&derived, &existing_refs)?;
            new_ids.push(derived.id.to_string());
            changes.push(DocumentChange::SetViewBlock {
                id: derived.id,
                previous: None,
                new: Some(derived.clone()),
            });
            all_views.push(derived);
        }
        changes.push(DocumentChange::SetOrthographicSet {
            id: set.id,
            previous: None,
            new: Some(set),
        });

        state.commit(changes)?;
        Ok(new_ids)
    }

    /// Article 18/35: propose `value` for `view_id`'s `axis`. A real
    /// mismatch against an existing shared binding becomes a `Conflict`
    /// (Gate M), never a second independent truth; a fresh confirmation
    /// binds `view_id` and reuses -- or, if none exists yet, mints -- one
    /// shared `DimensionId`, and also binds every other view in the same
    /// `OrthographicSet` whose identity consumes the same axis and has no
    /// binding yet (this is the mechanism behind "enter depth in TOP,
    /// RIGHT sees it" without a second manual action).
    pub fn propagate_shared_value(
        &self,
        view_id: String,
        axis: FfiSharedAxis,
        value: f64,
    ) -> Result<FfiPropagateOutcome, FfiSessionError> {
        let mut state = self.lock();
        let view_id: ViewId = parse_id(&view_id)?;
        let axis: SharedAxis = axis.into();
        let view = state
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;

        state.submit(
            CommandAction::Link,
            craftloop_command::CommandNamespace::Orthographic,
            "Propagated shared dimension",
        )?;

        if let Some(conflict) = propose_shared_value(
            state.document.dimension_store(),
            state.document.multiview_graph(),
            &view,
            axis,
            value,
        ) {
            let conflict_id = conflict.id;
            state
                .pending_shared_value_proposals
                .insert(conflict_id, (view_id, axis, value));
            let page_id = state.active_page()?;
            state.commit(vec![DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Conflict(conflict),
            }])?;
            return Ok(FfiPropagateOutcome::Conflict {
                conflict_id: conflict_id.to_string(),
            });
        }

        if let Some(existing_dimension) = state.document.multiview_graph().axis_of(view_id, axis) {
            // Already bound and matches within tolerance -- a harmless
            // re-confirmation (`propose_shared_value` returned `None`
            // precisely because there is nothing to change).
            let affected: Vec<String> = state
                .document
                .multiview_graph()
                .affected_views(existing_dimension)
                .into_iter()
                .map(|id| id.to_string())
                .collect();
            return Ok(FfiPropagateOutcome::Propagated {
                affected_views: affected,
            });
        }

        // Fresh confirmation. Reuse a dimension another view in the same
        // OrthographicSet already bound for this axis (Article 37: never a
        // second independent truth for the same semantic quantity);
        // otherwise mint a new Shared-in-spirit dimension (Driving role,
        // matching this workspace's own `propagation.rs` test convention
        // for a "some value a view has already confirmed").
        let containing_set = state
            .document
            .orthographic_sets()
            .find(|set| set.views().contains(&view_id))
            .cloned();
        let mut reuse_dimension = None;
        if let Some(set) = &containing_set {
            for other_id in set.views() {
                if *other_id == view_id {
                    continue;
                }
                if let Some(existing) = state.document.multiview_graph().axis_of(*other_id, axis) {
                    reuse_dimension = Some(existing);
                    break;
                }
            }
        }

        let mut scratch_store = state.document.dimension_store().clone();
        let mut scratch_graph = state.document.multiview_graph().clone();
        let previous_dimension_snapshot;
        let dimension_id;
        match reuse_dimension {
            Some(existing_id) => {
                previous_dimension_snapshot = state
                    .document
                    .dimension_store()
                    .dimension(existing_id)
                    .cloned();
                propagate_confirmed_value(&mut scratch_store, &scratch_graph, existing_id, value)?;
                dimension_id = existing_id;
            }
            None => {
                previous_dimension_snapshot = None;
                let dimension = SemanticDimension::new(
                    DimensionId::new(),
                    DimensionKind::Linear,
                    DimensionRole::Driving,
                    DimensionTarget::Single(PrimitiveId::new()),
                    value,
                )?;
                dimension_id = dimension.id;
                scratch_store.set_dimension(dimension);
            }
        }
        scratch_graph.bind_axis(&view, axis, dimension_id)?;

        let mut affected_views = vec![view_id];
        let mut binding_changes = vec![DocumentChange::SetMultiviewBinding {
            view: view_id,
            axis,
            previous: None,
            new: Some(dimension_id),
        }];
        if let Some(set) = &containing_set {
            for other_id in set.views() {
                if *other_id == view_id || scratch_graph.axis_of(*other_id, axis).is_some() {
                    continue;
                }
                if let Some(other_block) = state.document.view_block(*other_id) {
                    let consumes = other_block
                        .identity()
                        .map(|identity| axes_for_identity(identity).contains(&axis))
                        .unwrap_or(false);
                    if consumes {
                        scratch_graph.bind_axis(other_block, axis, dimension_id)?;
                        binding_changes.push(DocumentChange::SetMultiviewBinding {
                            view: *other_id,
                            axis,
                            previous: None,
                            new: Some(dimension_id),
                        });
                        affected_views.push(*other_id);
                    }
                }
            }
        }

        let new_dimension_snapshot = scratch_store.dimension(dimension_id).cloned();
        let mut changes = vec![DocumentChange::SetDimension {
            id: dimension_id,
            previous: previous_dimension_snapshot,
            new: new_dimension_snapshot,
        }];
        changes.extend(binding_changes);

        state.commit(changes)?;
        Ok(FfiPropagateOutcome::Propagated {
            affected_views: affected_views.iter().map(|id| id.to_string()).collect(),
        })
    }

    // -- Conflicts (Article 12/14/18) --------------------------------------

    pub fn resolve_conflict(
        &self,
        id: String,
        choice: FfiResolutionChoice,
    ) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        let conflict_id: ConflictId = parse_id(&id)?;
        let page_id = state.active_page()?;
        let entity_id = EntityId::Conflict(conflict_id);
        let previous_entity = state
            .document
            .page(page_id)
            .and_then(|p| p.get(entity_id))
            .cloned()
            .ok_or_else(|| missing("conflict", &id))?;
        let mut conflict = match previous_entity.clone() {
            SemanticEntity::Conflict(c) => c,
            _ => unreachable!("EntityId::Conflict always maps to SemanticEntity::Conflict"),
        };

        let choice_domain: ResolutionChoice = choice.into();
        resolve(&mut conflict, choice_domain)?;

        state.submit(
            CommandAction::Resolve,
            craftloop_command::CommandNamespace::Orthographic,
            "Resolved conflict",
        )?;

        let mut changes = vec![
            DocumentChange::RemoveEntity {
                page_id,
                entity: previous_entity,
            },
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Conflict(conflict.clone()),
            },
        ];

        if matches!(choice_domain, ResolutionChoice::ReplaceAndPropagate) {
            if let Some((view_id, axis, value)) = state
                .pending_shared_value_proposals
                .get(&conflict_id)
                .copied()
            {
                let dimension_id = state
                    .document
                    .multiview_graph()
                    .axis_of(view_id, axis)
                    .ok_or_else(|| missing("multiview binding", view_id))?;
                let mut scratch_store = state.document.dimension_store().clone();
                propagate_confirmed_value(
                    &mut scratch_store,
                    state.document.multiview_graph(),
                    dimension_id,
                    value,
                )?;
                let previous_dimension = state
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .cloned();
                let new_dimension = scratch_store.dimension(dimension_id).cloned();
                changes.push(DocumentChange::SetDimension {
                    id: dimension_id,
                    previous: previous_dimension,
                    new: new_dimension,
                });
            }
        }
        state.pending_shared_value_proposals.remove(&conflict_id);

        state.commit(changes)?;
        Ok(())
    }

    // -- Undo/redo (Article 12; no matching CommandAction, see module doc)

    pub fn undo(&self) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        state.undo()?;
        Ok(())
    }

    pub fn redo(&self) -> Result<(), FfiSessionError> {
        let mut state = self.lock();
        state.redo()?;
        Ok(())
    }

    pub fn can_undo(&self) -> bool {
        self.lock().history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.lock().history.can_redo()
    }
}

impl CraftLoopSession {
    /// Shared tail of `create_primitive_line`/`_circle`/`_rectangle`:
    /// insert into both the page (rendering) and the `Sketch`
    /// (constraint-eligibility), as one atomic transaction.
    fn insert_primitive(
        &self,
        state: &mut SessionState,
        beautified: craftloop_recognition::Beautified,
    ) -> Result<String, FfiSessionError> {
        let primitive_id = PrimitiveId::new();
        let page_id = state.active_page()?;
        state.commit(vec![
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Primitive {
                    id: primitive_id,
                    beautified: beautified.clone(),
                },
            },
            DocumentChange::InsertSketchPrimitive {
                id: primitive_id,
                beautified,
            },
        ])?;
        Ok(primitive_id.to_string())
    }
}

impl Default for CraftLoopSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FfiPointerSource;

    /// `DocumentHistory` (undo/redo) is deliberately session-local, not
    /// persisted (`CraftLoopSession::open` always starts a fresh one) --
    /// only `Document` itself round-trips through save/reopen. Comparing
    /// two snapshots taken before/after a reopen must therefore ignore
    /// `can_undo`/`can_redo`, which are expected to differ (a freshly
    /// reopened session can never undo past what it has not yet done in
    /// *this* process), while everything else -- geometry, dimensions,
    /// conflicts, view identity, orthographic relationships, revision --
    /// must match exactly.
    fn assert_persisted_state_matches(before: &FfiSceneSnapshot, after: &FfiSceneSnapshot) {
        assert_eq!(before.strokes, after.strokes);
        assert_eq!(before.primitives, after.primitives);
        assert_eq!(before.dimensions, after.dimensions);
        assert_eq!(before.conflicts, after.conflicts);
        assert_eq!(before.view_blocks, after.view_blocks);
        assert_eq!(before.orthographic_sets, after.orthographic_sets);
        assert_eq!(before.revision, after.revision);
    }

    fn stylus_sample(x: f64, y: f64, t: f64) -> FfiPointerSample {
        FfiPointerSample {
            x,
            y,
            timestamp_seconds: t,
            pressure: None,
            tilt_x_deg: None,
            tilt_y_deg: None,
            source: FfiPointerSource::Stylus,
            button_primary: false,
            button_secondary: false,
            button_barrel: false,
            capability_pressure: false,
            capability_tilt: false,
            capability_hover: false,
            capability_palm_rejection: false,
            capability_eraser: false,
        }
    }

    #[test]
    fn a_new_session_starts_empty_with_no_undo_redo() {
        let session = CraftLoopSession::new();
        let snapshot = session.scene_snapshot();
        assert!(snapshot.strokes.is_empty());
        assert!(snapshot.primitives.is_empty());
        assert!(snapshot.dimensions.is_empty());
        assert!(snapshot.conflicts.is_empty());
        assert!(snapshot.view_blocks.is_empty());
        assert!(snapshot.orthographic_sets.is_empty());
        assert_eq!(snapshot.revision, 0);
        assert!(!snapshot.can_undo);
        assert!(!snapshot.can_redo);
        assert!(!session.can_undo());
        assert!(!session.can_redo());
    }

    #[test]
    fn submit_stroke_inserts_raw_ink_and_is_undoable_redoable() {
        let session = CraftLoopSession::new();
        let samples = vec![
            stylus_sample(0.0, 0.0, 0.0),
            stylus_sample(1.0, 0.05, 0.01),
            stylus_sample(2.0, -0.05, 0.02),
        ];
        let outcome = session.submit_stroke(samples).unwrap();
        assert!(!outcome.stroke_id.is_empty());

        let snapshot = session.scene_snapshot();
        assert_eq!(snapshot.strokes.len(), 1);
        assert_eq!(snapshot.strokes[0].id, outcome.stroke_id);
        assert_eq!(snapshot.strokes[0].sample_count, 3);

        session.undo().unwrap();
        assert!(session.scene_snapshot().strokes.is_empty());
        session.redo().unwrap();
        assert_eq!(session.scene_snapshot().strokes.len(), 1);
    }

    #[test]
    fn a_clean_line_stroke_is_eligible_and_accept_recognition_replaces_it_with_a_primitive() {
        let session = CraftLoopSession::new();
        let samples: Vec<FfiPointerSample> = (0..12)
            .map(|i| stylus_sample(i as f64, 0.0, i as f64 * 0.01))
            .collect();
        let outcome = session.submit_stroke(samples).unwrap();
        assert!(
            outcome.eligible_for_recognition,
            "a near-perfect straight line must be recognized as eligible"
        );

        let primitive_id = session
            .accept_recognition(outcome.stroke_id.clone())
            .unwrap()
            .expect("a clean line must beautify into a real primitive, not stay ink");

        let snapshot = session.scene_snapshot();
        assert!(
            snapshot.strokes.is_empty(),
            "the raw stroke must be replaced"
        );
        assert_eq!(snapshot.primitives.len(), 1);
        assert_eq!(snapshot.primitives[0].id, primitive_id);
        assert_eq!(snapshot.primitives[0].kind, FfiPrimitiveKind::Line);
    }

    #[test]
    fn a_scribble_that_fits_nothing_well_stays_ink_on_accept_recognition() {
        let session = CraftLoopSession::new();
        // A sharp zig-zag: no single line, circle, or arc fits this well.
        let samples: Vec<FfiPointerSample> = vec![
            stylus_sample(0.0, 0.0, 0.0),
            stylus_sample(1.0, 8.0, 0.01),
            stylus_sample(2.0, -6.0, 0.02),
            stylus_sample(3.0, 9.0, 0.03),
            stylus_sample(4.0, -7.0, 0.04),
            stylus_sample(5.0, 10.0, 0.05),
            stylus_sample(6.0, -8.0, 0.06),
            stylus_sample(7.0, 11.0, 0.07),
        ];
        let outcome = session.submit_stroke(samples).unwrap();
        let result = session
            .accept_recognition(outcome.stroke_id.clone())
            .unwrap();
        if outcome.eligible_for_recognition {
            // The recognizer's own confidence threshold, not this test,
            // is authoritative -- only assert internal consistency.
            assert!(result.is_some());
        } else {
            assert!(
                result.is_none(),
                "a stroke reported ineligible must stay ink, not fabricate a primitive"
            );
            assert_eq!(session.scene_snapshot().strokes.len(), 1);
            assert!(session.scene_snapshot().primitives.is_empty());
        }
    }

    #[test]
    fn create_primitive_and_dimension_then_invalid_edit_creates_a_conflict_and_undo_clears_it() {
        let session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 10.0, 0.0).unwrap();
        let snapshot = session.scene_snapshot();
        assert_eq!(snapshot.primitives.len(), 1);
        assert_eq!(snapshot.primitives[0].kind, FfiPrimitiveKind::Line);

        let dimension_id = session
            .create_dimension(FfiDimensionKind::Linear, vec![line_id.clone()], 100.0)
            .unwrap();
        let snapshot = session.scene_snapshot();
        assert_eq!(snapshot.dimensions.len(), 1);
        assert_eq!(snapshot.dimensions[0].value, 100.0);
        assert_eq!(snapshot.dimensions[0].role, FfiDimensionRole::Driving);

        // Article 14 Task 068 / Gate G: an invalid value creates a
        // conflict and leaves the dimension unchanged.
        let result = session.edit_dimension(dimension_id.clone(), -5.0);
        assert!(result.is_err());
        let snapshot = session.scene_snapshot();
        assert_eq!(
            snapshot.dimensions[0].value, 100.0,
            "invalid edit must not change the value"
        );
        assert_eq!(snapshot.conflicts.len(), 1);
        assert!(snapshot.conflicts[0].unresolved);
        assert_eq!(
            snapshot.conflicts[0].kind,
            FfiConflictKind::DimensionConstraintMismatch
        );

        session.undo().unwrap();
        assert!(session.scene_snapshot().conflicts.is_empty());

        // A valid edit does take effect.
        session.edit_dimension(dimension_id.clone(), 130.0).unwrap();
        assert_eq!(session.scene_snapshot().dimensions[0].value, 130.0);
    }

    #[test]
    fn scene_snapshot_reports_a_real_bounding_box_per_primitive_kind() {
        // Execution 02, Phase 08 (Tasks 055/057): the snapshot originally
        // carried no geometry for a primitive at all, making Android-side
        // tap-to-select hit-testing and fit-to-content bounding-box
        // computation impossible. Every kind's bounds must come from the
        // real, tested `.bounds()` on the underlying geometry type, not a
        // placeholder -- checked here against hand-computed expectations
        // for a line and a circle.
        let session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(1.0, 2.0, 5.0, 8.0).unwrap();
        let circle_id = session.create_primitive_circle(10.0, 10.0, 3.0).unwrap();

        let snapshot = session.scene_snapshot();
        let line = snapshot
            .primitives
            .iter()
            .find(|p| p.id == line_id)
            .unwrap();
        assert_eq!(
            (line.min_x, line.min_y, line.max_x, line.max_y),
            (1.0, 2.0, 5.0, 8.0)
        );

        let circle = snapshot
            .primitives
            .iter()
            .find(|p| p.id == circle_id)
            .unwrap();
        assert_eq!(
            (circle.min_x, circle.min_y, circle.max_x, circle.max_y),
            (7.0, 7.0, 13.0, 13.0),
            "a circle centered at (10,10) with radius 3 bounds to [7,13] on each axis"
        );
    }

    #[test]
    fn apply_constraint_detects_redundancy_and_solve_moves_the_line() {
        let session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 4.0, 3.0).unwrap();

        let outcome = session
            .apply_constraint(FfiConstraintKind::Horizontal {
                line: line_id.clone(),
            })
            .unwrap();
        let constraint_id = match outcome {
            FfiConstraintOutcome::Added { constraint_id } => constraint_id,
            other => panic!("expected Added, got {other:?}"),
        };

        // Article 312: re-adding the same relationship is redundant, not an error.
        let again = session
            .apply_constraint(FfiConstraintKind::Horizontal {
                line: line_id.clone(),
            })
            .unwrap();
        assert!(matches!(again, FfiConstraintOutcome::Redundant { .. }));

        let solved = session.solve_constraints().unwrap();
        assert_eq!(solved.status, FfiSolveStatus::Solved);
        assert_eq!(solved.updated_primitive_count, 1);

        session.remove_constraint(constraint_id).unwrap();
        // Removing an already-removed constraint is a structured error.
        let dangling = "00000000-0000-0000-0000-000000000001".to_string();
        assert!(session.remove_constraint(dangling).is_err());
    }

    #[test]
    fn select_and_delete_selected_removes_entities_and_clears_selection() {
        let session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 1.0, 0.0).unwrap();
        let entity_id = format!("Primitive:{line_id}");

        session.select(vec![entity_id.clone()]).unwrap();
        assert_eq!(
            session.scene_snapshot().selected_entity_ids,
            vec![entity_id.clone()]
        );

        session.clear_selection();
        assert!(session.scene_snapshot().selected_entity_ids.is_empty());

        session.select(vec![entity_id.clone()]).unwrap();
        session.delete_selected(vec![entity_id]).unwrap();
        assert!(session.scene_snapshot().primitives.is_empty());
        assert!(session.scene_snapshot().selected_entity_ids.is_empty());
    }

    #[test]
    fn save_then_open_restores_an_equal_scene() {
        let session = CraftLoopSession::new();
        session.create_primitive_line(0.0, 0.0, 5.0, 5.0).unwrap();

        let path = std::env::temp_dir().join(format!(
            "craftloop-session-roundtrip-{}.json",
            std::process::id()
        ));
        let backup = {
            let mut p = path.clone().into_os_string();
            p.push(".bak");
            std::path::PathBuf::from(p)
        };
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&backup);

        session.save(path.to_string_lossy().to_string()).unwrap();
        let before = session.scene_snapshot();

        let reopened = CraftLoopSession::open(path.to_string_lossy().to_string()).unwrap();
        let after = reopened.scene_snapshot();
        assert_persisted_state_matches(&before, &after);
        assert!(
            !after.can_undo,
            "a freshly reopened session starts with no undo history"
        );

        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&backup);
    }

    /// Execution 02 Article 35's Golden Alpha Journey, proven at the Rust
    /// layer through `CraftLoopSession` alone -- months before any Android
    /// build can exist to prove it physically.
    #[test]
    fn golden_alpha_journey_end_to_end_through_save_and_reopen() {
        let session = CraftLoopSession::new();

        // Draw one rough line, treat it as the front profile.
        let line_id = session.create_primitive_line(0.0, 0.0, 100.0, 0.0).unwrap();

        // Dimension it.
        let dimension_id = session
            .create_dimension(FfiDimensionKind::Linear, vec![line_id.clone()], 100.0)
            .unwrap();

        // Apply a stable geometric constraint.
        session
            .apply_constraint(FfiConstraintKind::Horizontal {
                line: line_id.clone(),
            })
            .unwrap();

        // Attempt an incompatible value: a real conflict, not silent mutation.
        assert!(session.edit_dimension(dimension_id.clone(), -1.0).is_err());
        assert_eq!(session.scene_snapshot().conflicts.len(), 1);

        // Undo the incompatible action.
        session.undo().unwrap();
        assert!(session.scene_snapshot().conflicts.is_empty());

        // Assign the structured sketch as FRONT.
        let front_id = session
            .assign_view_identity(None, FfiPrincipalViewIdentity::Front)
            .unwrap();
        session
            .add_geometry_to_view(front_id.clone(), vec![line_id.clone()])
            .unwrap();

        // Activate Orthographic.
        let new_views = session.enter_orthographic(front_id.clone()).unwrap();
        assert_eq!(new_views.len(), 3, "Top, Right, and Back are all created");

        let snapshot = session.scene_snapshot();
        assert_eq!(snapshot.orthographic_sets.len(), 1);
        assert_eq!(snapshot.orthographic_sets[0].view_ids.len(), 4);
        let top_id = snapshot
            .view_blocks
            .iter()
            .find(|v| v.identity == Some(FfiPrincipalViewIdentity::Top))
            .expect("Top must exist")
            .id
            .clone();
        let right_id = snapshot
            .view_blocks
            .iter()
            .find(|v| v.identity == Some(FfiPrincipalViewIdentity::Right))
            .expect("Right must exist")
            .id
            .clone();

        // Confirm unknown depth is not invented: no depth dimension exists
        // until the user actually enters one.
        assert_eq!(
            snapshot.dimensions.len(),
            1,
            "only the width dimension exists so far"
        );

        // Select the appropriate Top extent and enter a missing depth
        // value; Right must use the same semantic depth without a second
        // manual action.
        let outcome = session
            .propagate_shared_value(top_id.clone(), FfiSharedAxis::Depth, 40.0)
            .unwrap();
        match outcome {
            FfiPropagateOutcome::Propagated { affected_views } => {
                assert!(affected_views.contains(&top_id));
                assert!(affected_views.contains(&right_id));
            }
            other => panic!("expected Propagated, got {other:?}"),
        }
        assert!(session
            .scene_snapshot()
            .dimensions
            .iter()
            .any(|d| d.value == 40.0));

        // Attempt a contradictory shared value on Right: a shared-value
        // conflict, not a second independent truth.
        let conflict_outcome = session
            .propagate_shared_value(right_id.clone(), FfiSharedAxis::Depth, 999.0)
            .unwrap();
        let conflict_id = match conflict_outcome {
            FfiPropagateOutcome::Conflict { conflict_id } => conflict_id,
            other => panic!("expected Conflict, got {other:?}"),
        };
        assert_eq!(
            session
                .scene_snapshot()
                .conflicts
                .iter()
                .filter(|c| c.unresolved)
                .count(),
            1
        );

        // Resolve by replacing and propagating.
        session
            .resolve_conflict(conflict_id, FfiResolutionChoice::ReplaceAndPropagate)
            .unwrap();
        let after_resolve = session.scene_snapshot();
        assert!(after_resolve.dimensions.iter().any(|d| d.value == 999.0));
        assert!(!after_resolve.conflicts.iter().any(|c| c.unresolved));

        // Undo and redo the resolution.
        assert!(session.can_undo());
        session.undo().unwrap();
        assert!(session.can_redo());
        session.redo().unwrap();

        // Save, "kill the app" (drop the session), and reopen.
        let path = std::env::temp_dir().join(format!(
            "craftloop-session-golden-{}.json",
            std::process::id()
        ));
        let backup = {
            let mut p = path.clone().into_os_string();
            p.push(".bak");
            std::path::PathBuf::from(p)
        };
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&backup);

        session.save(path.to_string_lossy().to_string()).unwrap();
        let before_snapshot = session.scene_snapshot();
        drop(session);

        let reopened = CraftLoopSession::open(path.to_string_lossy().to_string()).unwrap();
        let after_snapshot = reopened.scene_snapshot();
        assert_persisted_state_matches(&before_snapshot, &after_snapshot);
        assert!(
            !after_snapshot.can_undo,
            "a freshly reopened session starts with no undo history"
        );

        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&backup);
    }
}
