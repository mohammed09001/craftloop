//! Stroke provenance.
//!
//! Execution 01, Phase 05, Task 038. Authority: Engine Contract 02; MCP
//! Article 540 "Term: Provenance".
//!
//! A structured, append-only log of where a stroke came from and what has
//! happened to it since -- "without turning provenance into user-visible
//! clutter" means this is data a diagnostic panel or export can read, not
//! something rendered inline in the notebook by default.
//!
//! Only [`ProvenanceEvent::Created`] exists yet, because recognition
//! (Phase 06), beautification (Phase 06), and command interpretation
//! (Phase 19) -- the sources of later interpretation events -- do not exist
//! yet. Their event variants are added alongside those engines, not
//! speculatively stubbed here.

use craftloop_input::PointerSource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProvenanceEvent {
    /// The stroke was first recorded from `source` at `timestamp_seconds`
    /// (matching the first sample's timestamp).
    Created {
        source: PointerSource,
        timestamp_seconds: f64,
    },
}

/// The ordered provenance log for one stroke.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StrokeProvenance {
    events: Vec<ProvenanceEvent>,
}

impl StrokeProvenance {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn created(source: PointerSource, timestamp_seconds: f64) -> Self {
        let mut provenance = Self::new();
        provenance.record(ProvenanceEvent::Created {
            source,
            timestamp_seconds,
        });
        provenance
    }

    /// Append an event. Provenance is append-only: there is no method to
    /// remove or reorder past events, so it stays a trustworthy history.
    pub fn record(&mut self, event: ProvenanceEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[ProvenanceEvent] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_seeds_exactly_one_event() {
        let provenance = StrokeProvenance::created(PointerSource::SimulatedMouse, 1.5);
        assert_eq!(provenance.events().len(), 1);
        assert!(matches!(
            provenance.events()[0],
            ProvenanceEvent::Created {
                source: PointerSource::SimulatedMouse,
                timestamp_seconds
            } if timestamp_seconds == 1.5
        ));
    }

    #[test]
    fn record_appends_without_disturbing_earlier_events() {
        let mut provenance = StrokeProvenance::created(PointerSource::Stylus, 0.0);
        provenance.record(ProvenanceEvent::Created {
            source: PointerSource::Stylus,
            timestamp_seconds: 5.0,
        });
        assert_eq!(provenance.events().len(), 2);
        assert!(matches!(
            provenance.events()[0],
            ProvenanceEvent::Created { timestamp_seconds, .. } if timestamp_seconds == 0.0
        ));
    }

    #[test]
    fn serialization_round_trips() {
        let provenance = StrokeProvenance::created(PointerSource::Touch, 2.0);
        let json = serde_json::to_string(&provenance).unwrap();
        let back: StrokeProvenance = serde_json::from_str(&json).unwrap();
        assert_eq!(provenance, back);
    }
}
