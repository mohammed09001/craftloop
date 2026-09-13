//! Command object model.
//!
//! Execution 01, Phase 19, Task 133. Authority: MCP Article 239
//! ("Every command object should contain: Command type. Target context.
//! Parameters. Source channel. Timestamp. Undo metadata. Risk level.").
//!
//! `CommandAction` is the full, flat vocabulary across every namespace
//! (Article 237's Notebook/Sketch/Orthographic word lists) -- a shared
//! action like `Dimension` (valid in both Sketch and Orthographic) is one
//! variant, not duplicated per namespace, matching this workspace's
//! "no duplicated truth" convention. Which actions are actually valid in
//! which namespace is `grammar.rs`'s job, not this type's.

use std::collections::BTreeMap;

use craftloop_ids::CommandId;
use serde::{Deserialize, Serialize};

/// The full command vocabulary, flattened across namespaces (Article
/// 237). Exactly the words that article names -- no speculative
/// additions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CommandAction {
    // Notebook context
    Pen,
    Eraser,
    Select,
    Sketch,
    Orthographic,
    // Sketch context
    Line,
    Circle,
    Arc,
    Rectangle,
    Dimension,
    ExitSketch,
    // Orthographic context
    AddView,
    LabelView,
    Link,
    Resolve,
}

/// Which namespace's vocabulary is currently active (Task 134). Article
/// 237 names exactly these three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandNamespace {
    Notebook,
    Sketch,
    Orthographic,
}

/// Where a command originated (Article 239's exact channel list, plus
/// the "future" ones it already names so this type does not need to
/// change again when they arrive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandSource {
    Toolbar,
    InkCommand,
    Keyboard,
    Accessibility,
    Voice,
    Gesture,
}

/// Article 141/236's risk policy tiers. `Low` can execute directly;
/// anything higher requires stronger confirmation (`risk.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Article 236: "The resulting operation is stored, not the original
/// command string as the sole authority" -- undo metadata names the
/// operation in a way that survives independent of the ink/text that
/// triggered it (Task 140's ephemeral-ink requirement depends on this).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndoMetadata {
    /// Human-readable description of what this command did, usable in an
    /// undo-history list without needing to re-interpret the original
    /// command source.
    pub description: String,
    pub is_undoable: bool,
}

impl UndoMetadata {
    pub fn undoable(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            is_undoable: true,
        }
    }

    pub fn not_undoable(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            is_undoable: false,
        }
    }
}

/// One command object (Task 133), regardless of which source channel
/// produced it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub id: CommandId,
    pub action: CommandAction,
    pub source: CommandSource,
    pub namespace: CommandNamespace,
    /// A simple string-keyed parameter bag: command parameters vary
    /// enough by action (a `Dimension` command's target vs. a `LabelView`
    /// command's label text) that a single shared struct would either
    /// need one field per action (most always empty) or its own enum
    /// duplicating `CommandAction`'s own shape. A plain map is the
    /// smallest coherent representation Task 133 actually asks for
    /// ("parameters"), not a premature schema for every action's own
    /// argument shape.
    pub parameters: BTreeMap<String, String>,
    pub risk: RiskLevel,
    pub timestamp_seconds: f64,
    pub undo: UndoMetadata,
}
