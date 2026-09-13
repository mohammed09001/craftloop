//! Freeform notes.
//!
//! Execution 01, Phase 07, Task 050 ("Support raw ink, geometry, notes,
//! dimensions, view labels, suggestions, conflicts, and ephemeral
//! interaction records"). Authority: MCP Article 67 "Notes Around
//! Engineering Views", Article 145 "General Notes Versus Engineering
//! Notes".
//!
//! Deliberately minimal: a positioned piece of text. Article 145's
//! distinction between general and engineering notes, and Article 146's
//! handwriting-style preservation, are presentation/recognition concerns
//! for a later phase to layer on top of this storage primitive, not
//! something this phase needs to model to satisfy Task 050's "notes"
//! requirement.

use craftloop_geometry::Point2;
use craftloop_ids::NoteId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub position: Point2,
    pub text: String,
}

impl Note {
    pub fn new(id: NoteId, position: Point2, text: impl Into<String>) -> Self {
        Self {
            id,
            position,
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    #[test]
    fn serialization_round_trips() {
        let note = Note::new(
            NoteId::new(),
            Point2::new(1.0, 2.0),
            "remember to check tolerance",
        );
        let json = serde_json::to_string(&note).unwrap();
        let back: Note = serde_json::from_str(&json).unwrap();
        assert_eq!(note, back);
    }
}
