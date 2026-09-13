//! Diagnostic export: current harness state and event trace as JSON,
//! independent of screenshots.
//!
//! Execution 01, Phase 04, Task 032. Authority: Engine Contract 26 (Export:
//! "preserve engineering values"); MCP Article 92 "Determinism".

use craftloop_serialization::SchemaVersion;
use serde::{Deserialize, Serialize};

use crate::state::{HarnessState, RecordedStroke};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessDiagnostic {
    pub schema_version: SchemaVersion,
    pub active_tool: String,
    pub simulated_pressure_override: f64,
    pub viewport_zoom: f64,
    pub viewport_pan_x: f64,
    pub viewport_pan_y: f64,
    pub completed_stroke_count: usize,
    pub completed_strokes: Vec<RecordedStroke>,
    pub stroke_in_progress: bool,
    pub simulator_disclaimer: String,
}

impl HarnessDiagnostic {
    pub fn capture(state: &HarnessState) -> Self {
        Self {
            schema_version: SchemaVersion::CURRENT,
            active_tool: state.active_tool.label().to_string(),
            simulated_pressure_override: state.simulated_controls.pressure_override(),
            viewport_zoom: state.viewport.zoom(),
            viewport_pan_x: state.viewport.pan().x,
            viewport_pan_y: state.viewport.pan().y,
            completed_stroke_count: state.completed_strokes.len(),
            completed_strokes: state.completed_strokes.clone(),
            stroke_in_progress: state.is_stroke_in_progress(),
            simulator_disclaimer: craftloop_input::SIMULATOR_DISCLAIMER.to_string(),
        }
    }

    pub fn to_canonical_json(&self) -> serde_json::Result<String> {
        craftloop_serialization::to_canonical_json(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_input::{InputCapabilities, PointerButtons, PointerEvent, PointerSource};

    fn sample() -> craftloop_input::PointerSample {
        craftloop_input::PointerSample::new(
            Point2::ORIGIN,
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
    fn capture_reflects_completed_strokes_and_tool() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample())).unwrap();
        state.handle_event(PointerEvent::Up(sample())).unwrap();

        let diagnostic = HarnessDiagnostic::capture(&state);
        assert_eq!(diagnostic.completed_stroke_count, 1);
        assert_eq!(diagnostic.active_tool, "Pen");
        assert!(!diagnostic.stroke_in_progress);
    }

    #[test]
    fn capture_reflects_an_in_progress_stroke() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample())).unwrap();
        let diagnostic = HarnessDiagnostic::capture(&state);
        assert!(diagnostic.stroke_in_progress);
        assert_eq!(diagnostic.completed_stroke_count, 0);
    }

    #[test]
    fn export_always_includes_the_simulator_disclaimer() {
        let state = HarnessState::new();
        let diagnostic = HarnessDiagnostic::capture(&state);
        assert_eq!(
            diagnostic.simulator_disclaimer,
            craftloop_input::SIMULATOR_DISCLAIMER
        );
    }

    #[test]
    fn canonical_json_export_round_trips() {
        let mut state = HarnessState::new();
        state.handle_event(PointerEvent::Down(sample())).unwrap();
        state.handle_event(PointerEvent::Up(sample())).unwrap();
        let diagnostic = HarnessDiagnostic::capture(&state);

        let json = diagnostic.to_canonical_json().unwrap();
        let back: HarnessDiagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(diagnostic, back);
    }
}
