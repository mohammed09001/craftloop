//! Harness engine state: everything the UI displays or mutates, with zero
//! dependency on `egui`.
//!
//! Execution 01, Phase 04, Tasks 029/033. Authority: Engine Contract 27
//! ("It is a test adapter, never production authority"); this phase's own
//! Task 033 ("enforce module boundaries so no production mobile semantics
//! depend on egui types or Windows-specific code").
//!
//! `HarnessState` intentionally holds only what Phases 00-03 have actually
//! built: raw pointer strokes and lifecycle events. It does not have fields
//! for "recognized primitives," "dimensions," or "constraints" yet, because
//! those engines do not exist until Phases 06/10/12 -- adding placeholder
//! fields for them now would be exactly the kind of fabricated completeness
//! the No-Hallucination Contract forbids. Task 029's remaining inspector
//! panels (recognized primitives, dimensions, constraints, transactions,
//! conflicts, orthographic relationships) are added to this struct
//! alongside the engines that produce that state.

use craftloop_input::{PointerEvent, PointerTrace, StrokeLifecycleValidator};
use serde::{Deserialize, Serialize};

use crate::tool::{HarnessTool, SimulatedControls};
use crate::viewport::Viewport;

/// A completed raw stroke: the ordered events from one `Down` through its
/// terminating `Up`/`Cancel`/`CaptureLost`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordedStroke {
    pub events: Vec<PointerEvent>,
}

pub struct HarnessState {
    pub active_tool: HarnessTool,
    pub simulated_controls: SimulatedControls,
    pub viewport: Viewport,
    /// Strokes that have completed (Up/Cancel/CaptureLost seen).
    pub completed_strokes: Vec<RecordedStroke>,
    /// Events for the stroke currently in progress, if any.
    in_progress_events: Vec<PointerEvent>,
    lifecycle: StrokeLifecycleValidator,
}

impl HarnessState {
    pub fn new() -> Self {
        Self {
            active_tool: HarnessTool::Pen,
            simulated_controls: SimulatedControls::new(),
            viewport: Viewport::new(),
            completed_strokes: Vec::new(),
            in_progress_events: Vec::new(),
            lifecycle: StrokeLifecycleValidator::new(),
        }
    }

    /// Feed one lifecycle event into the harness. Returns an error (and
    /// leaves state unchanged) if the event is not valid in the current
    /// lifecycle state, exactly like `StrokeLifecycleValidator` alone would
    /// -- the harness must not silently swallow a malformed sequence.
    pub fn handle_event(&mut self, event: PointerEvent) -> craftloop_errors::DomainResult<()> {
        self.lifecycle.accept(&event)?;
        let stroke_ended = matches!(
            event,
            PointerEvent::Up(_) | PointerEvent::Cancel(_) | PointerEvent::CaptureLost
        );
        if !matches!(event, PointerEvent::Hover(_)) || self.lifecycle.is_stroke_active() {
            self.in_progress_events.push(event);
        }
        if stroke_ended {
            let events = std::mem::take(&mut self.in_progress_events);
            self.completed_strokes.push(RecordedStroke { events });
        }
        Ok(())
    }

    pub fn is_stroke_in_progress(&self) -> bool {
        self.lifecycle.is_stroke_active()
    }

    pub fn clear(&mut self) {
        self.completed_strokes.clear();
        self.in_progress_events.clear();
        self.lifecycle = StrokeLifecycleValidator::new();
    }

    /// Load a scenario's traces as if they had just been drawn, replacing
    /// current state. Returns an error without mutating state if any trace
    /// is malformed (traces are pre-validated by `Scenario::validate`, but
    /// this is checked again defensively).
    pub fn load_traces(&mut self, traces: &[PointerTrace]) -> craftloop_errors::DomainResult<()> {
        for trace in traces {
            trace.validate()?;
        }
        self.clear();
        for trace in traces {
            for event in &trace.events {
                self.handle_event(*event)?;
            }
        }
        Ok(())
    }
}

impl Default for HarnessState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_input::{InputCapabilities, PointerButtons, PointerSource};

    fn sample(x: f64) -> craftloop_input::PointerSample {
        craftloop_input::PointerSample::new(
            Point2::new(x, 0.0),
            0.0,
            None,
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    #[test]
    fn a_complete_down_move_up_sequence_becomes_one_recorded_stroke() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample(0.0))).unwrap();
        state.handle_event(PointerEvent::Move(sample(1.0))).unwrap();
        state.handle_event(PointerEvent::Up(sample(2.0))).unwrap();
        assert_eq!(state.completed_strokes.len(), 1);
        assert_eq!(state.completed_strokes[0].events.len(), 3);
        assert!(!state.is_stroke_in_progress());
    }

    #[test]
    fn an_invalid_event_is_rejected_and_does_not_corrupt_state() {
        let mut state = HarnessState::new();
        let result = state.handle_event(PointerEvent::Up(sample(0.0)));
        assert!(result.is_err());
        assert!(state.completed_strokes.is_empty());
        assert!(!state.is_stroke_in_progress());
    }

    #[test]
    fn clear_resets_strokes_and_lifecycle() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample(0.0))).unwrap();
        state.clear();
        assert!(state.completed_strokes.is_empty());
        assert!(!state.is_stroke_in_progress());
        // Lifecycle was reset too, so a fresh Down is valid again.
        assert!(state.handle_event(PointerEvent::Down(sample(0.0))).is_ok());
    }

    #[test]
    fn load_traces_replays_multiple_traces_as_completed_strokes() {
        let mut state = HarnessState::new();
        let traces = vec![
            PointerTrace::new(
                "a",
                vec![
                    PointerEvent::Down(sample(0.0)),
                    PointerEvent::Up(sample(1.0)),
                ],
            ),
            PointerTrace::new(
                "b",
                vec![
                    PointerEvent::Down(sample(2.0)),
                    PointerEvent::Up(sample(3.0)),
                ],
            ),
        ];
        state.load_traces(&traces).unwrap();
        assert_eq!(state.completed_strokes.len(), 2);
    }

    #[test]
    fn load_traces_leaves_state_unchanged_when_a_trace_is_invalid() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample(0.0))).unwrap();
        state.handle_event(PointerEvent::Up(sample(1.0))).unwrap();

        let bad_traces = vec![PointerTrace::new(
            "bad",
            vec![PointerEvent::Up(sample(0.0))],
        )];
        let result = state.load_traces(&bad_traces);
        assert!(result.is_err());
        // Original stroke is still there -- the invalid load never cleared it.
        assert_eq!(state.completed_strokes.len(), 1);
    }
}
