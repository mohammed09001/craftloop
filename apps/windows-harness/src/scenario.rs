//! Deterministic scenario loader.
//!
//! Execution 01, Phase 04, Task 031. Authority: Loop Engineering Contract
//! ("Test: Write or update the smallest meaningful test"); builds directly
//! on `craftloop_input::PointerTrace` (Phase 03, Task 025).
//!
//! A scenario is a named, on-disk JSON fixture that replays one or more
//! recorded pointer traces to reach a known state, so a bug found while
//! clicking around the harness can become `scenarios/some-bug.json` and a
//! regression test instead of a one-off manual repro.

use std::fs;
use std::path::Path;

use craftloop_errors::DomainError;
use craftloop_input::PointerTrace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub traces: Vec<PointerTrace>,
}

#[derive(Debug, thiserror::Error)]
pub enum ScenarioLoadError {
    #[error("could not read scenario file: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse scenario JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("scenario failed validation: {0}")]
    Invalid(#[from] DomainError),
}

impl Scenario {
    pub fn validate(&self) -> Result<(), DomainError> {
        for trace in &self.traces {
            trace.validate()?;
        }
        Ok(())
    }

    pub fn from_json(json: &str) -> Result<Self, ScenarioLoadError> {
        let scenario: Scenario = serde_json::from_str(json)?;
        scenario.validate()?;
        Ok(scenario)
    }

    pub fn load_from_file(path: &Path) -> Result<Self, ScenarioLoadError> {
        let contents = fs::read_to_string(path)?;
        Self::from_json(&contents)
    }

    pub fn to_canonical_json(&self) -> serde_json::Result<String> {
        craftloop_serialization::to_canonical_json(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_input::{
        InputCapabilities, PointerButtons, PointerEvent, PointerSample, PointerSource,
    };

    fn sample() -> PointerSample {
        PointerSample::new(
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

    fn valid_scenario() -> Scenario {
        Scenario {
            name: "draw-one-line".to_string(),
            description: "Down, move, up to draw a simple line.".to_string(),
            traces: vec![PointerTrace::new(
                "line",
                vec![
                    PointerEvent::Down(sample()),
                    PointerEvent::Move(sample()),
                    PointerEvent::Up(sample()),
                ],
            )],
        }
    }

    #[test]
    fn a_valid_scenario_round_trips_through_json() {
        let scenario = valid_scenario();
        let json = scenario.to_canonical_json().unwrap();
        let back = Scenario::from_json(&json).unwrap();
        assert_eq!(scenario, back);
    }

    #[test]
    fn from_json_rejects_a_scenario_with_an_invalid_trace() {
        let bad = Scenario {
            name: "bad".to_string(),
            description: "".to_string(),
            traces: vec![PointerTrace::new(
                "bad-trace",
                vec![PointerEvent::Up(sample())],
            )],
        };
        let json = serde_json::to_string(&bad).unwrap();
        assert!(matches!(
            Scenario::from_json(&json),
            Err(ScenarioLoadError::Invalid(_))
        ));
    }

    #[test]
    fn from_json_rejects_malformed_json() {
        assert!(matches!(
            Scenario::from_json("not json"),
            Err(ScenarioLoadError::Parse(_))
        ));
    }

    #[test]
    fn load_from_file_round_trips_through_a_real_temp_file() {
        let scenario = valid_scenario();
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "craftloop-scenario-test-{}.json",
            std::process::id()
        ));
        fs::write(&path, scenario.to_canonical_json().unwrap()).unwrap();

        let loaded = Scenario::load_from_file(&path).unwrap();
        assert_eq!(loaded, scenario);

        fs::remove_file(&path).ok();
    }

    #[test]
    fn load_from_file_reports_io_error_for_a_missing_file() {
        let missing = Path::new("this/path/does/not/exist.json");
        assert!(matches!(
            Scenario::load_from_file(missing),
            Err(ScenarioLoadError::Io(_))
        ));
    }

    /// The checked-in `scenarios/example-line.json` fixture (produced by
    /// `examples/generate_scenario.rs`) must actually load through the real
    /// `Scenario::load_from_file` path the harness UI uses -- not merely be
    /// assumed compatible because it was written by similar-looking code.
    #[test]
    fn checked_in_example_scenario_loads_and_validates() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("scenarios/example-line.json");
        let scenario = Scenario::load_from_file(&path).expect("example-line.json must load");
        assert_eq!(scenario.name, "example-line");
        assert_eq!(scenario.traces.len(), 1);
        assert_eq!(scenario.traces[0].events.len(), 4);
    }
}
