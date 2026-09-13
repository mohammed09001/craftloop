//! Ephemeral command ink.
//!
//! Execution 01, Phase 19, Task 140. Authority: MCP Article 236
//! ("Documents should not depend on the current meaning of ephemeral
//! commands after execution. The resulting operation is stored, not the
//! original command string as the sole authority.").
//!
//! Once a command has been confirmed and executed, the raw strokes that
//! spelled it out (and the gesture that confirmed it) are not meant to
//! remain on the page as ordinary ink -- but *removing* them must never
//! mean the fact that the command happened is lost. `UndoMetadata`
//! (Task 133) is exactly the durable record that survives; this module
//! is the small, explicit seam that says which strokes are safe to
//! remove and confirms the operation's own record does not reference
//! them.

use craftloop_ids::StrokeId;
use serde::{Deserialize, Serialize};

use crate::command::UndoMetadata;

/// What to do with the ink that made up a confirmed command. `undo`
/// carries the durable record (Task 133); `strokes_to_remove` names
/// exactly which raw strokes (the command text and/or the confirmation
/// gesture) may now be taken off the page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EphemeralInkDisposition {
    pub strokes_to_remove: Vec<StrokeId>,
    pub undo: UndoMetadata,
}

/// Build the disposition for one confirmed command. Fails (structurally,
/// by simply never being constructible any other way) to lose the
/// association between the removed strokes and what they meant: a
/// caller cannot have an `EphemeralInkDisposition` without also having
/// the `UndoMetadata` that survives after the strokes are gone.
pub fn dispose_of_confirmed_command_ink(
    command_strokes: Vec<StrokeId>,
    confirmation_gesture_stroke: Option<StrokeId>,
    undo: UndoMetadata,
) -> EphemeralInkDisposition {
    let mut strokes_to_remove = command_strokes;
    if let Some(gesture) = confirmation_gesture_stroke {
        strokes_to_remove.push(gesture);
    }
    EphemeralInkDisposition {
        strokes_to_remove,
        undo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    #[test]
    fn both_the_command_text_and_the_confirmation_gesture_are_marked_for_removal() {
        let text_stroke = StrokeId::new();
        let gesture_stroke = StrokeId::new();
        let disposition = dispose_of_confirmed_command_ink(
            vec![text_stroke],
            Some(gesture_stroke),
            UndoMetadata::undoable("Switched tool to Pen"),
        );
        assert!(disposition.strokes_to_remove.contains(&text_stroke));
        assert!(disposition.strokes_to_remove.contains(&gesture_stroke));
    }

    #[test]
    fn the_undo_record_survives_independent_of_which_strokes_are_removed() {
        let disposition = dispose_of_confirmed_command_ink(
            vec![StrokeId::new(), StrokeId::new()],
            None,
            UndoMetadata::undoable("Entered Sketch mode"),
        );
        // The record does not reference stroke identity at all -- the
        // operation's meaning has already been captured independently
        // of the raw ink (Article 236's own requirement).
        assert_eq!(disposition.undo.description, "Entered Sketch mode");
        assert!(disposition.undo.is_undoable);
    }

    #[test]
    fn a_command_with_no_separate_confirmation_stroke_only_removes_its_own_text() {
        let text_stroke = StrokeId::new();
        let disposition = dispose_of_confirmed_command_ink(
            vec![text_stroke],
            None,
            UndoMetadata::not_undoable("Selected entity"),
        );
        assert_eq!(disposition.strokes_to_remove, vec![text_stroke]);
    }
}
