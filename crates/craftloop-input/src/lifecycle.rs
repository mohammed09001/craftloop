//! Stroke lifecycle events.
//!
//! Execution 01, Phase 03, Task 024. Authority: Engine Contract 02 (Raw
//! Ink: "Ordered samples, stroke identity, grouping"); MCP Article 244
//! "Detailed Specification of Pen Tools".
//!
//! Normalizes platform-specific pointer callbacks (Windows mouse
//! down/move/up, future Android/iOS touch/stylus callbacks) into one event
//! enum, and provides [`StrokeLifecycleValidator`] so "is this a coherent
//! stroke" is a single, testable state machine instead of duplicated
//! ad hoc checks in every adapter.

use craftloop_errors::{DomainError, DomainResult, InputErrorKind};
use serde::{Deserialize, Serialize};

use crate::sample::PointerSample;

/// One normalized pointer lifecycle event.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PointerEvent {
    /// The pointer made contact (mouse button pressed / stylus touched
    /// down / finger touched down).
    Down(PointerSample),
    /// The pointer moved while in contact.
    Move(PointerSample),
    /// The pointer lifted (stroke completed normally).
    Up(PointerSample),
    /// The stroke was cancelled by the platform (e.g. an incoming system
    /// gesture) rather than completed by the user.
    Cancel(PointerSample),
    /// The pointer is near the surface but not in contact (stylus hover;
    /// never produced by the mouse simulator, which has no hover
    /// capability).
    Hover(PointerSample),
    /// The platform revoked pointer capture outside the normal down/up
    /// flow (e.g. window lost focus mid-stroke). No sample position is
    /// necessarily meaningful, since this can be reported asynchronously.
    CaptureLost,
}

impl PointerEvent {
    /// The sample carried by this event, if any (`CaptureLost` has none).
    pub fn sample(&self) -> Option<&PointerSample> {
        match self {
            PointerEvent::Down(s)
            | PointerEvent::Move(s)
            | PointerEvent::Up(s)
            | PointerEvent::Cancel(s)
            | PointerEvent::Hover(s) => Some(s),
            PointerEvent::CaptureLost => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifecycleState {
    /// No stroke in progress. `Hover` is valid here (a stylus can hover
    /// without ever touching down); `Down` starts a stroke.
    Idle,
    /// A stroke is in progress: `Move`/`Hover` continue it, `Up`/`Cancel`/
    /// `CaptureLost` end it.
    Active,
}

/// Validates that a sequence of [`PointerEvent`]s forms a coherent stroke
/// lifecycle: at most one stroke active at a time, no `Move`/`Up`/`Cancel`
/// without a preceding `Down`, no doubled `Down`.
#[derive(Debug)]
pub struct StrokeLifecycleValidator {
    state: LifecycleState,
}

impl StrokeLifecycleValidator {
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Idle,
        }
    }

    pub fn accept(&mut self, event: &PointerEvent) -> DomainResult<()> {
        use LifecycleState::*;
        use PointerEvent::*;

        match (self.state, event) {
            (Idle, Hover(_)) => Ok(()),
            (Idle, Down(_)) => {
                self.state = Active;
                Ok(())
            }
            (Active, Move(_) | Hover(_)) => Ok(()),
            (Active, Up(_) | Cancel(_) | CaptureLost) => {
                self.state = Idle;
                Ok(())
            }
            (Idle, Move(_) | Up(_) | Cancel(_) | CaptureLost) => Err(DomainError::Input {
                kind: InputErrorKind::InvalidLifecycleSequence,
                detail: format!("{event:?} received with no active stroke (state was Idle)"),
            }),
            (Active, Down(_)) => Err(DomainError::Input {
                kind: InputErrorKind::InvalidLifecycleSequence,
                detail: "Down received while a stroke is already active".to_string(),
            }),
        }
    }

    pub fn is_stroke_active(&self) -> bool {
        self.state == LifecycleState::Active
    }
}

impl Default for StrokeLifecycleValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a whole recorded sequence at once, e.g. before replaying it
/// (Task 025). Returns the first ordering violation, if any.
pub fn validate_sequence(events: &[PointerEvent]) -> DomainResult<()> {
    let mut validator = StrokeLifecycleValidator::new();
    for event in events {
        validator.accept(event)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::InputCapabilities;
    use crate::sample::PointerButtons;
    use craftloop_geometry::Point2;

    fn sample() -> PointerSample {
        PointerSample::new(
            Point2::ORIGIN,
            0.0,
            None,
            None,
            None,
            crate::sample::PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    #[test]
    fn down_move_up_is_a_valid_sequence() {
        assert!(validate_sequence(&[
            PointerEvent::Down(sample()),
            PointerEvent::Move(sample()),
            PointerEvent::Move(sample()),
            PointerEvent::Up(sample()),
        ])
        .is_ok());
    }

    #[test]
    fn move_without_down_is_rejected() {
        let result = validate_sequence(&[PointerEvent::Move(sample())]);
        assert!(matches!(
            result,
            Err(DomainError::Input {
                kind: InputErrorKind::InvalidLifecycleSequence,
                ..
            })
        ));
    }

    #[test]
    fn double_down_without_up_is_rejected() {
        let result =
            validate_sequence(&[PointerEvent::Down(sample()), PointerEvent::Down(sample())]);
        assert!(result.is_err());
    }

    #[test]
    fn hover_is_valid_both_before_and_during_a_stroke() {
        assert!(validate_sequence(&[
            PointerEvent::Hover(sample()),
            PointerEvent::Down(sample()),
            PointerEvent::Hover(sample()),
            PointerEvent::Up(sample()),
        ])
        .is_ok());
    }

    #[test]
    fn cancel_ends_a_stroke_just_like_up() {
        assert!(
            validate_sequence(&[PointerEvent::Down(sample()), PointerEvent::Cancel(sample())])
                .is_ok()
        );
    }

    #[test]
    fn capture_lost_ends_a_stroke_and_needs_no_sample() {
        assert!(
            validate_sequence(&[PointerEvent::Down(sample()), PointerEvent::CaptureLost]).is_ok()
        );
    }

    #[test]
    fn a_new_stroke_can_start_after_a_clean_end() {
        assert!(validate_sequence(&[
            PointerEvent::Down(sample()),
            PointerEvent::Up(sample()),
            PointerEvent::Down(sample()),
            PointerEvent::Up(sample()),
        ])
        .is_ok());
    }

    #[test]
    fn validator_reports_active_state_correctly() {
        let mut v = StrokeLifecycleValidator::new();
        assert!(!v.is_stroke_active());
        v.accept(&PointerEvent::Down(sample())).unwrap();
        assert!(v.is_stroke_active());
        v.accept(&PointerEvent::Up(sample())).unwrap();
        assert!(!v.is_stroke_active());
    }

    #[test]
    fn up_without_down_is_rejected() {
        assert!(validate_sequence(&[PointerEvent::Up(sample())]).is_err());
    }
}
