//! Deterministic recorded-input playback.
//!
//! Execution 01, Phase 03, Task 025. Authority: Loop Engineering Contract
//! ("Test: Write or update the smallest meaningful test" -- a recorded
//! trace is exactly this for interaction bugs); MCP Article 92
//! "Determinism".
//!
//! A [`PointerTrace`] is a named, ordered, serializable list of
//! [`PointerEvent`]s. The point of this type is narrow: turn "a user did
//! something that broke the app" into a file a test can load and replay
//! byte-for-byte, every time, on any machine.

use craftloop_errors::DomainResult;
use craftloop_serialization::to_canonical_json;
use serde::{Deserialize, Serialize};

use crate::lifecycle::{validate_sequence, PointerEvent};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerTrace {
    /// Short human-readable identifier, e.g. `"circle-then-drag-selection"`.
    /// Not a `craftloop-ids` typed ID: a trace is a test fixture on disk,
    /// not a live document entity.
    pub name: String,
    pub events: Vec<PointerEvent>,
}

impl PointerTrace {
    pub fn new(name: impl Into<String>, events: Vec<PointerEvent>) -> Self {
        Self {
            name: name.into(),
            events,
        }
    }

    /// Validate that `events` forms a coherent stroke lifecycle (Task 024)
    /// before anyone tries to replay it.
    pub fn validate(&self) -> DomainResult<()> {
        validate_sequence(&self.events)
    }

    /// Canonical JSON serialization (sorted object keys), so two runs that
    /// produce an equal trace produce byte-identical output -- required for
    /// this to work as a diffable regression fixture.
    pub fn to_canonical_json(&self) -> serde_json::Result<String> {
        to_canonical_json(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Deterministically replay every event, in recorded order, into
    /// `sink`. "Deterministic" here means exactly what it sounds like: the
    /// same trace produces the same call sequence every time, with no
    /// hidden clock, randomness, or reordering.
    pub fn replay<F: FnMut(&PointerEvent)>(&self, mut sink: F) {
        for event in &self.events {
            sink(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::InputCapabilities;
    use crate::sample::{PointerButtons, PointerSample, PointerSource};
    use craftloop_geometry::Point2;

    fn sample_at(x: f64, t: f64) -> PointerSample {
        PointerSample::new(
            Point2::new(x, 0.0),
            t,
            None,
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    fn sample_trace() -> PointerTrace {
        PointerTrace::new(
            "simple-line",
            vec![
                PointerEvent::Down(sample_at(0.0, 0.0)),
                PointerEvent::Move(sample_at(1.0, 0.1)),
                PointerEvent::Move(sample_at(2.0, 0.2)),
                PointerEvent::Up(sample_at(2.0, 0.3)),
            ],
        )
    }

    #[test]
    fn a_well_formed_trace_validates() {
        assert!(sample_trace().validate().is_ok());
    }

    #[test]
    fn a_malformed_trace_fails_validation() {
        let bad = PointerTrace::new("bad", vec![PointerEvent::Up(sample_at(0.0, 0.0))]);
        assert!(bad.validate().is_err());
    }

    #[test]
    fn replay_visits_every_event_in_recorded_order() {
        let trace = sample_trace();
        let mut visited = Vec::new();
        trace.replay(|event| visited.push(*event));
        assert_eq!(visited, trace.events);
    }

    #[test]
    fn canonical_json_round_trips_to_an_equal_trace() {
        let trace = sample_trace();
        let json = trace.to_canonical_json().unwrap();
        let back = PointerTrace::from_json(&json).unwrap();
        assert_eq!(trace, back);
    }

    #[test]
    fn canonical_json_is_byte_identical_across_repeated_serializations() {
        let trace = sample_trace();
        let first = trace.to_canonical_json().unwrap();
        let second = trace.to_canonical_json().unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn replaying_twice_produces_the_same_visited_sequence() {
        let trace = sample_trace();
        let mut first = Vec::new();
        trace.replay(|e| first.push(*e));
        let mut second = Vec::new();
        trace.replay(|e| second.push(*e));
        assert_eq!(first, second);
    }
}
