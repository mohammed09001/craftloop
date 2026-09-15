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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebStrokeSummary {
    pub id: String,
    pub sample_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebPrimitiveSummary {
    pub id: String,
    pub kind: WebPrimitiveKind,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebDimensionSummary {
    pub id: String,
    pub kind: WebDimensionKind,
    pub role: WebDimensionRole,
    pub value: f64,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebConflictSummary {
    pub id: String,
    pub kind: WebConflictKind,
    pub unresolved: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebViewBlockSummary {
    pub id: String,
    pub identity: Option<WebPrincipalViewIdentity>,
    pub geometry_member_count: u32,
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
    pub conflicts: Vec<WebConflictSummary>,
    pub view_blocks: Vec<WebViewBlockSummary>,
    pub orthographic_sets: Vec<WebOrthographicSetSummary>,
    pub selected_entity_ids: Vec<String>,
    pub revision: u64,
    pub can_undo: bool,
    pub can_redo: bool,
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
