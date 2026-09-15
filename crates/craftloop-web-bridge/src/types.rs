//! JSON-shaped read models and command payloads crossing the
//! `wasm-bindgen` boundary.
//!
//! Execution 03, Phase 04. Mirrors native `craftloop-mobile-ffi`'s
//! `Ffi*` types (Article 10: "a session API conceptually aligned with
//! native `CraftLoopSession`") but for a different reason than that
//! crate's own `Ffi*` mirrors exist: `wasm-bindgen` (unlike UniFFI) has
//! no support for exporting a Rust enum that carries data (only
//! fieldless C-like enums cross as real JS/TS enums) and no ergonomic
//! support for `Vec<CustomStruct>` parameters/returns. Serializing a
//! plain-data DTO to a JSON string and letting the JS side
//! `JSON.parse`/`JSON.stringify` it is the standard, honest
//! `wasm-bindgen` pattern for exactly this shape of problem -- these
//! types exist to give that JSON a stable, typed Rust source of truth
//! rather than hand-building object literals at each call site.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

// ---------------------------------------------------------------------
// Fieldless enums: real wasm-bindgen enums, not JSON (Article 10 parity
// with native's FFI-safe enum types where wasm-bindgen actually
// supports it -- these four cross as method parameters directly).
// ---------------------------------------------------------------------

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebDimensionKind {
    Linear,
    Angular,
    Radius,
    Diameter,
}

impl From<WebDimensionKind> for craftloop_dimension::DimensionKind {
    fn from(kind: WebDimensionKind) -> Self {
        match kind {
            WebDimensionKind::Linear => craftloop_dimension::DimensionKind::Linear,
            WebDimensionKind::Angular => craftloop_dimension::DimensionKind::Angular,
            WebDimensionKind::Radius => craftloop_dimension::DimensionKind::Radius,
            WebDimensionKind::Diameter => craftloop_dimension::DimensionKind::Diameter,
        }
    }
}

impl From<craftloop_dimension::DimensionKind> for WebDimensionKind {
    fn from(kind: craftloop_dimension::DimensionKind) -> Self {
        match kind {
            craftloop_dimension::DimensionKind::Linear => WebDimensionKind::Linear,
            craftloop_dimension::DimensionKind::Angular => WebDimensionKind::Angular,
            craftloop_dimension::DimensionKind::Radius => WebDimensionKind::Radius,
            craftloop_dimension::DimensionKind::Diameter => WebDimensionKind::Diameter,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebDimensionRole {
    Driving,
    Reference,
    Derived,
    Shared,
    Bounded,
}

impl From<craftloop_dimension::DimensionRole> for WebDimensionRole {
    fn from(role: craftloop_dimension::DimensionRole) -> Self {
        match role {
            craftloop_dimension::DimensionRole::Driving => WebDimensionRole::Driving,
            craftloop_dimension::DimensionRole::Reference => WebDimensionRole::Reference,
            craftloop_dimension::DimensionRole::Derived => WebDimensionRole::Derived,
            craftloop_dimension::DimensionRole::Shared => WebDimensionRole::Shared,
            craftloop_dimension::DimensionRole::Bounded => WebDimensionRole::Bounded,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebPrincipalViewIdentity {
    Front,
    Top,
    Right,
    Back,
}

impl From<WebPrincipalViewIdentity> for craftloop_document::PrincipalViewIdentity {
    fn from(identity: WebPrincipalViewIdentity) -> Self {
        match identity {
            WebPrincipalViewIdentity::Front => craftloop_document::PrincipalViewIdentity::Front,
            WebPrincipalViewIdentity::Top => craftloop_document::PrincipalViewIdentity::Top,
            WebPrincipalViewIdentity::Right => craftloop_document::PrincipalViewIdentity::Right,
            WebPrincipalViewIdentity::Back => craftloop_document::PrincipalViewIdentity::Back,
        }
    }
}

impl From<craftloop_document::PrincipalViewIdentity> for WebPrincipalViewIdentity {
    fn from(identity: craftloop_document::PrincipalViewIdentity) -> Self {
        match identity {
            craftloop_document::PrincipalViewIdentity::Front => WebPrincipalViewIdentity::Front,
            craftloop_document::PrincipalViewIdentity::Top => WebPrincipalViewIdentity::Top,
            craftloop_document::PrincipalViewIdentity::Right => WebPrincipalViewIdentity::Right,
            craftloop_document::PrincipalViewIdentity::Back => WebPrincipalViewIdentity::Back,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebSharedAxis {
    Width,
    Height,
    Depth,
}

impl From<WebSharedAxis> for craftloop_document::SharedAxis {
    fn from(axis: WebSharedAxis) -> Self {
        match axis {
            WebSharedAxis::Width => craftloop_document::SharedAxis::Width,
            WebSharedAxis::Height => craftloop_document::SharedAxis::Height,
            WebSharedAxis::Depth => craftloop_document::SharedAxis::Depth,
        }
    }
}

impl From<craftloop_document::SharedAxis> for WebSharedAxis {
    fn from(axis: craftloop_document::SharedAxis) -> Self {
        match axis {
            craftloop_document::SharedAxis::Width => WebSharedAxis::Width,
            craftloop_document::SharedAxis::Height => WebSharedAxis::Height,
            craftloop_document::SharedAxis::Depth => WebSharedAxis::Depth,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebResolutionChoice {
    KeepExisting,
    ReplaceAndPropagate,
    Unlink,
    RemoveConstraint,
    Cancel,
}

impl From<WebResolutionChoice> for craftloop_consistency::ResolutionChoice {
    fn from(choice: WebResolutionChoice) -> Self {
        match choice {
            WebResolutionChoice::KeepExisting => {
                craftloop_consistency::ResolutionChoice::KeepExisting
            }
            WebResolutionChoice::ReplaceAndPropagate => {
                craftloop_consistency::ResolutionChoice::ReplaceAndPropagate
            }
            WebResolutionChoice::Unlink => craftloop_consistency::ResolutionChoice::Unlink,
            WebResolutionChoice::RemoveConstraint => {
                craftloop_consistency::ResolutionChoice::RemoveConstraint
            }
            WebResolutionChoice::Cancel => craftloop_consistency::ResolutionChoice::Cancel,
        }
    }
}

impl From<craftloop_consistency::ResolutionChoice> for WebResolutionChoice {
    fn from(choice: craftloop_consistency::ResolutionChoice) -> Self {
        match choice {
            craftloop_consistency::ResolutionChoice::KeepExisting => {
                WebResolutionChoice::KeepExisting
            }
            craftloop_consistency::ResolutionChoice::ReplaceAndPropagate => {
                WebResolutionChoice::ReplaceAndPropagate
            }
            craftloop_consistency::ResolutionChoice::Unlink => WebResolutionChoice::Unlink,
            craftloop_consistency::ResolutionChoice::RemoveConstraint => {
                WebResolutionChoice::RemoveConstraint
            }
            craftloop_consistency::ResolutionChoice::Cancel => WebResolutionChoice::Cancel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebPointerSource {
    SimulatedMouse,
    Stylus,
    Touch,
}

impl From<WebPointerSource> for craftloop_input::PointerSource {
    fn from(source: WebPointerSource) -> Self {
        match source {
            WebPointerSource::SimulatedMouse => craftloop_input::PointerSource::SimulatedMouse,
            WebPointerSource::Stylus => craftloop_input::PointerSource::Stylus,
            WebPointerSource::Touch => craftloop_input::PointerSource::Touch,
        }
    }
}

// ---------------------------------------------------------------------
// JSON DTOs (mirror the Ffi* Records; see module doc for why JSON).
// ---------------------------------------------------------------------

/// Mirrors native `FfiPointerSample`. Article 15's full field list
/// (pointerType/position/timestamp/pressure/tilt/buttons) plus the same
/// real, queried capability flags native requires -- Phase 06's Pointer
/// Events adapter is what actually populates these from a browser
/// event; this type only defines the shape they arrive in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebPointerSample {
    pub x: f64,
    pub y: f64,
    pub timestamp_seconds: f64,
    pub pressure: Option<f64>,
    pub tilt_x_deg: Option<f64>,
    pub tilt_y_deg: Option<f64>,
    pub source: WebPointerSource,
    pub button_primary: bool,
    pub button_secondary: bool,
    pub button_barrel: bool,
    pub capability_pressure: bool,
    pub capability_tilt: bool,
    pub capability_hover: bool,
    pub capability_palm_rejection: bool,
    pub capability_eraser: bool,
}

impl From<&WebPointerSample> for craftloop_input::PointerSample {
    fn from(sample: &WebPointerSample) -> Self {
        craftloop_input::PointerSample {
            position: craftloop_geometry::Point2::new(sample.x, sample.y),
            timestamp_seconds: sample.timestamp_seconds,
            pressure: sample.pressure,
            tilt_x_deg: sample.tilt_x_deg,
            tilt_y_deg: sample.tilt_y_deg,
            source: sample.source.into(),
            buttons: craftloop_input::PointerButtons {
                primary: sample.button_primary,
                secondary: sample.button_secondary,
                barrel: sample.button_barrel,
            },
            capabilities: craftloop_input::InputCapabilities {
                pressure: sample.capability_pressure,
                tilt: sample.capability_tilt,
                hover: sample.capability_hover,
                palm_rejection: sample.capability_palm_rejection,
                eraser: sample.capability_eraser,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebStrokeOutcome {
    pub stroke_id: String,
    pub eligible_for_recognition: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebPrimitiveKind {
    Line,
    Circle,
    Arc,
    Rectangle,
}

/// Execution 03, Phase 06, Task 045: `points` (each sample's real
/// `Point2` position) was added alongside the original `sample_count`
/// so the structured SVG layer can actually render a committed stroke
/// -- without it, a submitted stroke would visibly vanish the instant
/// the transient ink Canvas stopped drawing it, which is not "mouse/
/// pen drawing appears immediately" (Task 044), it is drawing that
/// disappears.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebStrokeSummary {
    pub id: String,
    pub sample_count: u32,
    pub points: Vec<craftloop_geometry::Point2>,
}

/// Execution 03, Phase 05, Task 036: `min_*`/`max_*` (Phase 04's
/// original bounding box, still useful for hit-testing/fit-to-content
/// per native's own rationale) plus `geometry` -- the real
/// `craftloop_recognition::BeautifiedPrimitive`, serialized as-is
/// (`{"Line":{"a":{...},"b":{...}}}`, `{"Circle":{"center":...,
/// "radius":...}}`, etc.) so the frontend has actual coordinates to
/// render a Segment/Circle/Arc/Rectangle, not just its box. There is no
/// `Ellipse` variant here because none exists anywhere in the shared
/// core yet (`BeautifiedPrimitive` has exactly four variants) -- adding
/// one would be exactly the fabricated-capability Article 65 forbids.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebPrimitiveSummary {
    pub id: String,
    pub kind: WebPrimitiveKind,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub geometry: craftloop_recognition::BeautifiedPrimitive,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebDimensionSummary {
    pub id: String,
    pub kind: WebDimensionKind,
    pub role: WebDimensionRole,
    pub value: f64,
    /// Task 037's "anchors": which primitive(s) this dimension targets,
    /// from the real `DimensionTarget::primitive_ids()`.
    pub target_primitive_ids: Vec<String>,
}

/// Task 038: real relation metadata for a stored sketch constraint.
/// `label` is `SketchConstraintKind`'s own `Debug` rendering (e.g.
/// `Horizontal(PrimitiveId(..))`) rather than a second, hand-maintained
/// name for each of its eleven variants -- honest and always in sync
/// with the real enum, at the cost of not being pretty-printed; a
/// nicer per-kind label is exactly the kind of presentation polish
/// Phase 09-12's toolbar/badge work owns, not this read model.
/// `primitive_ids` comes from the same real `point_refs()` the solver
/// itself uses to build its variable set (`Sketch::variable_map`), so
/// it can never disagree with what the constraint actually constrains.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebConstraintSummary {
    pub id: String,
    pub label: String,
    pub primitive_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebConflictKind {
    DegenerateGeometry,
    DimensionConstraintMismatch,
    UnitMisapplication,
    CrossViewMismatch,
}

impl From<craftloop_consistency::ConflictKind> for WebConflictKind {
    fn from(kind: craftloop_consistency::ConflictKind) -> Self {
        match kind {
            craftloop_consistency::ConflictKind::DegenerateGeometry => {
                WebConflictKind::DegenerateGeometry
            }
            craftloop_consistency::ConflictKind::DimensionConstraintMismatch => {
                WebConflictKind::DimensionConstraintMismatch
            }
            craftloop_consistency::ConflictKind::UnitMisapplication => {
                WebConflictKind::UnitMisapplication
            }
            craftloop_consistency::ConflictKind::CrossViewMismatch => {
                WebConflictKind::CrossViewMismatch
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebSeverity {
    Info,
    Warning,
    Error,
    Blocker,
}

impl From<craftloop_errors::Severity> for WebSeverity {
    fn from(severity: craftloop_errors::Severity) -> Self {
        match severity {
            craftloop_errors::Severity::Info => WebSeverity::Info,
            craftloop_errors::Severity::Warning => WebSeverity::Warning,
            craftloop_errors::Severity::Error => WebSeverity::Error,
            craftloop_errors::Severity::Blocker => WebSeverity::Blocker,
        }
    }
}

/// Task 041: everything a conflict-resolution UI needs, read directly
/// from the real `craftloop_consistency::Conflict` -- never
/// reconstructed or guessed at this boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebConflictSummary {
    pub id: String,
    pub kind: WebConflictKind,
    pub severity: WebSeverity,
    pub unresolved: bool,
    pub affected_entities: Vec<String>,
    pub existing_truth: String,
    pub proposed_truth: String,
    pub evidence: String,
    pub allowed_resolutions: Vec<WebResolutionChoice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebOrthographicReadiness {
    DraftReady,
    IdentityReady,
    LinkReady,
    Resolved,
    Constrained,
}

impl From<craftloop_document::OrthographicReadiness> for WebOrthographicReadiness {
    fn from(readiness: craftloop_document::OrthographicReadiness) -> Self {
        use craftloop_document::OrthographicReadiness as R;
        match readiness {
            R::DraftReady => WebOrthographicReadiness::DraftReady,
            R::IdentityReady => WebOrthographicReadiness::IdentityReady,
            R::LinkReady => WebOrthographicReadiness::LinkReady,
            R::Resolved => WebOrthographicReadiness::Resolved,
            R::Constrained => WebOrthographicReadiness::Constrained,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebAxisBinding {
    pub axis: WebSharedAxis,
    pub dimension_id: String,
}

/// Task 039: identity, real member ids (not just a count), and real
/// readiness/shared-axis state -- the last two computed by the same
/// `evaluate_readiness`/`MultiviewGraph` calls `enterOrthographic`/
/// `propagateSharedValue` already use, not a separate approximation.
/// Deliberately does not include `layout` (`PageLayoutTransform`):
/// Article 38 puts computing the visual 2D engineering-region layout in
/// the frontend, not in this read model.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebViewBlockSummary {
    pub id: String,
    pub identity: Option<WebPrincipalViewIdentity>,
    pub geometry_member_ids: Vec<String>,
    pub readiness: WebOrthographicReadiness,
    pub blockers: Vec<String>,
    pub axis_bindings: Vec<WebAxisBinding>,
    pub unresolved_axes: Vec<WebSharedAxis>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebOrthographicSetSummary {
    pub id: String,
    pub view_ids: Vec<String>,
}

/// Mirrors native `FfiSceneSnapshot` (Article 13). Every field is real
/// presentation data read from the live `Document`/`DocumentHistory` --
/// see `session.rs::scene_snapshot`, not this module -- never invented
/// here.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebSceneSnapshot {
    pub strokes: Vec<WebStrokeSummary>,
    pub primitives: Vec<WebPrimitiveSummary>,
    pub dimensions: Vec<WebDimensionSummary>,
    pub constraints: Vec<WebConstraintSummary>,
    pub conflicts: Vec<WebConflictSummary>,
    pub view_blocks: Vec<WebViewBlockSummary>,
    pub orthographic_sets: Vec<WebOrthographicSetSummary>,
    pub selected_entity_ids: Vec<String>,
    pub revision: u64,
    pub can_undo: bool,
    pub can_redo: bool,
    pub workspace_mode: WebWorkspaceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebDebugState {
    pub revision: u64,
    pub transaction_count: u32,
    pub can_undo: bool,
    pub can_redo: bool,
    pub unresolved_conflict_count: u32,
    pub view_block_count: u32,
    pub orthographic_set_count: u32,
}

/// Article 17's stable constraint list -- see native `FfiConstraintKind`'s
/// own doc comment for exactly why only these eight variants are
/// exposed. JSON-tagged externally (serde's default), e.g.
/// `{"Horizontal":{"line":"<uuid>"}}`, so the JS side builds a plain
/// object literal rather than needing a bespoke encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WebConstraintKind {
    Coincident { a: String, b: String },
    Horizontal { line: String },
    Vertical { line: String },
    Parallel { a: String, b: String },
    Perpendicular { a: String, b: String },
    EqualLength { a: String, b: String },
    EqualRadius { a: String, b: String },
    Concentric { a: String, b: String },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum WebConstraintOutcome {
    Added { constraint_id: String },
    Redundant { existing_constraint_id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebSolveStatus {
    Solved,
    Unsatisfied,
    Failed,
}

impl From<craftloop_constraint::SolveStatus> for WebSolveStatus {
    fn from(status: craftloop_constraint::SolveStatus) -> Self {
        match status {
            craftloop_constraint::SolveStatus::Solved => WebSolveStatus::Solved,
            craftloop_constraint::SolveStatus::Unsatisfied => WebSolveStatus::Unsatisfied,
            craftloop_constraint::SolveStatus::Failed => WebSolveStatus::Failed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebSolveOutcome {
    pub status: WebSolveStatus,
    pub unsatisfied_constraint_ids: Vec<String>,
    pub updated_primitive_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum WebPropagateOutcome {
    Propagated { affected_views: Vec<String> },
    Conflict { conflict_id: String },
}

// ---------------------------------------------------------------------
// Workspace mode and command grammar (Execution 03, Phase 08).
// ---------------------------------------------------------------------

/// Task 056: Creative and Sketch2D as ephemeral session interaction
/// state -- the same ephemeral-vs-semantic distinction `selection`
/// already established (Phase 04's `CraftLoopSession` doc comment):
/// never a `DocumentChange`, never touches `DocumentHistory` (Task
/// 063). Deliberately a small session-owned type of its own rather
/// than reusing `craftloop_command::CommandNamespace` directly --
/// `CommandNamespace` is the *command-grammar's* concept (three
/// values, including `Orthographic`, which is not a toolbar drawing
/// mode); `WorkspaceMode` is the narrower UI-facing "what does the
/// canvas/toolbar currently look like" concept Article 19-21 describe.
/// `enter_sketch_mode`/`enter_creative_pen_mode` map each transition
/// onto the one real `CommandNamespace` it corresponds to when
/// submitting through the Command Bus, so the two concepts stay in
/// lockstep without literally being the same type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebWorkspaceMode {
    Creative,
    Sketch2D,
}

/// Mirrors `craftloop_command::CommandNamespace`, exposed as a real
/// wasm-bindgen enum since it crosses as a plain function parameter
/// (`resolveCommand`'s second argument) with no associated data.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebCommandNamespace {
    Notebook,
    Sketch,
    Orthographic,
}

impl From<WebCommandNamespace> for craftloop_command::CommandNamespace {
    fn from(namespace: WebCommandNamespace) -> Self {
        match namespace {
            WebCommandNamespace::Notebook => craftloop_command::CommandNamespace::Notebook,
            WebCommandNamespace::Sketch => craftloop_command::CommandNamespace::Sketch,
            WebCommandNamespace::Orthographic => craftloop_command::CommandNamespace::Orthographic,
        }
    }
}

/// Mirrors every `craftloop_command::CommandAction` variant -- the
/// command simulator (Task 062; full UI in Phase 15) needs to report
/// exactly which real action a piece of recognized text resolved to,
/// not a narrowed-down subset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WebCommandAction {
    Pen,
    Eraser,
    Select,
    Sketch,
    Orthographic,
    Line,
    Circle,
    Arc,
    Rectangle,
    Dimension,
    ExitSketch,
    AddView,
    LabelView,
    Link,
    Resolve,
}

impl From<craftloop_command::CommandAction> for WebCommandAction {
    fn from(action: craftloop_command::CommandAction) -> Self {
        use craftloop_command::CommandAction as A;
        match action {
            A::Pen => WebCommandAction::Pen,
            A::Eraser => WebCommandAction::Eraser,
            A::Select => WebCommandAction::Select,
            A::Sketch => WebCommandAction::Sketch,
            A::Orthographic => WebCommandAction::Orthographic,
            A::Line => WebCommandAction::Line,
            A::Circle => WebCommandAction::Circle,
            A::Arc => WebCommandAction::Arc,
            A::Rectangle => WebCommandAction::Rectangle,
            A::Dimension => WebCommandAction::Dimension,
            A::ExitSketch => WebCommandAction::ExitSketch,
            A::AddView => WebCommandAction::AddView,
            A::LabelView => WebCommandAction::LabelView,
            A::Link => WebCommandAction::Link,
            A::Resolve => WebCommandAction::Resolve,
        }
    }
}

/// Mirrors `craftloop_command::grammar::GrammarMatch` (Task 059/062):
/// "never guess" is the whole point of this type existing, so
/// `Ambiguous`'s candidate list is preserved exactly as the real
/// resolver produced it, not collapsed to a single guess.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum WebGrammarMatch {
    Exact { action: WebCommandAction },
    UniquePrefix { action: WebCommandAction },
    Ambiguous { candidates: Vec<String> },
    NoMatch,
}

impl From<craftloop_command::GrammarMatch> for WebGrammarMatch {
    fn from(grammar_match: craftloop_command::GrammarMatch) -> Self {
        use craftloop_command::GrammarMatch as G;
        match grammar_match {
            G::Exact(action) => WebGrammarMatch::Exact {
                action: action.into(),
            },
            G::UniquePrefix(action) => WebGrammarMatch::UniquePrefix {
                action: action.into(),
            },
            G::Ambiguous(candidates) => WebGrammarMatch::Ambiguous {
                candidates: candidates.iter().map(|s| s.to_string()).collect(),
            },
            G::NoMatch => WebGrammarMatch::NoMatch,
        }
    }
}
