//! Ink intent categories.
//!
//! Execution 01, Phase 16, Task 113. Authority: Engine Contract 04 (Ink
//! Intent Engine); MCP Article 1's framing of ink as needing to resolve
//! into distinct meanings before it becomes structured data.
//!
//! Exactly the nine categories Task 113 names -- not a speculative
//! superset. `RawInk` is always a legitimate outcome (mirrors
//! `craftloop_recognition::RecognitionCandidate::KeepAsInk`, Phase 06):
//! ink that resolves into nothing more specific is not a failure state.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentCategory {
    /// Stays ordinary handwriting/drawing; no more specific meaning
    /// applies (or none could be determined with enough confidence).
    RawInk,
    /// A candidate for `craftloop-recognition`'s geometric fitting
    /// (Phase 06) -- this category answers "should this even be offered
    /// to the geometry fitter," not which shape it is.
    GeometryCandidate,
    /// Ordinary handwritten words/sentences, not a command or label.
    Text,
    /// A number meant as a dimension value, not incidental text (Article
    /// 27's "300 cannot form this triangle" example is what this
    /// category, once accepted, eventually feeds).
    NumericDimensionCandidate,
    /// A principal-view identity label (Front/Top/Right/...), MCP Article
    /// 30.
    ViewLabel,
    /// A recognized reserved command word/gesture (Article 647's `Pen`
    /// example).
    Command,
    /// A gesture selecting existing entities rather than creating new
    /// geometry.
    SelectionGesture,
    /// A gesture erasing/removing existing ink or geometry.
    EraseGesture,
    /// A freeform note/annotation attached to nearby geometry, distinct
    /// from a dimension or command.
    Annotation,
}
