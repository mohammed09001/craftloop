//! Normalized pointer samples.
//!
//! Execution 01, Phase 03, Task 021. Authority: Engine Contract 01
//! (Normalized Input: "No platform UI types may cross into domain crates");
//! MCP Article 582 "Requirement Group: Pen Input".
//!
//! `PointerSample` is the one shape every input adapter (Windows mouse
//! simulator today; Android Jetpack Ink and iPad PencilKit adapters later)
//! must normalize into, so nothing downstream of this crate ever needs to
//! know which platform an event came from.

use craftloop_errors::{DomainError, DomainResult, InputErrorKind};
use serde::{Deserialize, Serialize};

use craftloop_geometry::Point2;

use crate::capabilities::InputCapabilities;

/// Where a pointer sample originated. Distinct from `InputCapabilities`:
/// `source` says *what kind* of input device this is, `capabilities` says
/// *what it can do*. Kept separate because two devices of the same
/// `source` could plausibly report different capabilities in the future.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointerSource {
    /// Windows mouse mapped to a simulated pen (Task 023). Never real
    /// stylus hardware.
    SimulatedMouse,
    /// A genuine stylus (Apple Pencil, S Pen, or similar).
    Stylus,
    /// A bare-finger touch point.
    Touch,
}

impl PointerSource {
    /// A short, user-facing label. For `SimulatedMouse` this always
    /// contains the word "simulated" so no downstream UI text can
    /// accidentally present it as real stylus hardware (Task 026, and the
    /// No-Hallucination Contract's "real Apple Pencil behavior was tested
    /// on Windows" prohibition).
    pub fn label(&self) -> &'static str {
        match self {
            PointerSource::SimulatedMouse => "Simulated pen (mouse)",
            PointerSource::Stylus => "Stylus",
            PointerSource::Touch => "Touch",
        }
    }
}

/// Which pointer buttons are held, if any. Barrel is the stylus side
/// button; mice typically report primary/secondary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PointerButtons {
    pub primary: bool,
    pub secondary: bool,
    pub barrel: bool,
}

/// One normalized pointer sample.
///
/// `pressure`/`tilt_x_deg`/`tilt_y_deg` are `Option` because a source that
/// cannot sense them must not report a fabricated value (Task 022). The
/// mouse simulator is a deliberate, documented exception for `pressure`
/// only (Task 023: "explicit constant pressure"): it supplies a constant
/// placeholder so downstream rendering has *something* to draw stroke width
/// from, while `capabilities.pressure == false` tells callers that value is
/// not authoritative sensor data. Use [`PointerSample::is_pressure_authoritative`]
/// rather than checking `pressure.is_some()` to tell the two cases apart.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PointerSample {
    pub position: Point2,
    /// Seconds since an adapter-defined stream epoch (not wall-clock time),
    /// so recorded traces (Task 025) replay identically regardless of when
    /// they are replayed.
    pub timestamp_seconds: f64,
    pub pressure: Option<f64>,
    pub tilt_x_deg: Option<f64>,
    pub tilt_y_deg: Option<f64>,
    pub source: PointerSource,
    pub buttons: PointerButtons,
    pub capabilities: InputCapabilities,
}

impl PointerSample {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position: Point2,
        timestamp_seconds: f64,
        pressure: Option<f64>,
        tilt_x_deg: Option<f64>,
        tilt_y_deg: Option<f64>,
        source: PointerSource,
        buttons: PointerButtons,
        capabilities: InputCapabilities,
    ) -> DomainResult<Self> {
        if !timestamp_seconds.is_finite() {
            return Err(DomainError::Input {
                kind: InputErrorKind::OutOfRange,
                detail: format!("timestamp_seconds must be finite, got {timestamp_seconds}"),
            });
        }
        if let Some(p) = pressure {
            if !(0.0..=1.0).contains(&p) {
                return Err(DomainError::Input {
                    kind: InputErrorKind::OutOfRange,
                    detail: format!("pressure must be within [0, 1], got {p}"),
                });
            }
        }
        for (label, tilt) in [("tilt_x_deg", tilt_x_deg), ("tilt_y_deg", tilt_y_deg)] {
            if let Some(t) = tilt {
                if !(-90.0..=90.0).contains(&t) {
                    return Err(DomainError::Input {
                        kind: InputErrorKind::OutOfRange,
                        detail: format!("{label} must be within [-90, 90], got {t}"),
                    });
                }
            }
        }
        Ok(Self {
            position,
            timestamp_seconds,
            pressure,
            tilt_x_deg,
            tilt_y_deg,
            source,
            buttons,
            capabilities,
        })
    }

    /// True if `pressure` (when present) reflects genuine hardware sensing
    /// rather than a source-specific placeholder such as the mouse
    /// simulator's constant value.
    pub fn is_pressure_authoritative(&self) -> bool {
        self.pressure.is_some() && self.capabilities.pressure
    }

    pub fn is_tilt_authoritative(&self) -> bool {
        (self.tilt_x_deg.is_some() || self.tilt_y_deg.is_some()) && self.capabilities.tilt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_sample() -> PointerSample {
        PointerSample::new(
            Point2::new(1.0, 2.0),
            0.5,
            Some(0.5),
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap()
    }

    #[test]
    fn valid_sample_constructs_successfully() {
        let s = valid_sample();
        assert_eq!(s.source, PointerSource::SimulatedMouse);
    }

    #[test]
    fn pressure_out_of_range_is_rejected() {
        let result = PointerSample::new(
            Point2::ORIGIN,
            0.0,
            Some(1.5),
            None,
            None,
            PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities::NONE,
        );
        assert!(matches!(
            result,
            Err(DomainError::Input {
                kind: InputErrorKind::OutOfRange,
                ..
            })
        ));
    }

    #[test]
    fn negative_pressure_is_rejected() {
        let result = PointerSample::new(
            Point2::ORIGIN,
            0.0,
            Some(-0.1),
            None,
            None,
            PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities::NONE,
        );
        assert!(result.is_err());
    }

    #[test]
    fn tilt_out_of_range_is_rejected() {
        let result = PointerSample::new(
            Point2::ORIGIN,
            0.0,
            None,
            Some(91.0),
            None,
            PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities::NONE,
        );
        assert!(result.is_err());
    }

    #[test]
    fn non_finite_timestamp_is_rejected() {
        let result = PointerSample::new(
            Point2::ORIGIN,
            f64::NAN,
            None,
            None,
            None,
            PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities::NONE,
        );
        assert!(result.is_err());
    }

    #[test]
    fn simulated_mouse_pressure_is_not_authoritative_even_when_present() {
        let s = valid_sample();
        assert!(s.pressure.is_some());
        assert!(!s.is_pressure_authoritative());
    }

    #[test]
    fn real_stylus_pressure_with_matching_capability_is_authoritative() {
        let s = PointerSample::new(
            Point2::ORIGIN,
            0.0,
            Some(0.8),
            None,
            None,
            PointerSource::Stylus,
            PointerButtons::default(),
            InputCapabilities {
                pressure: true,
                ..InputCapabilities::NONE
            },
        )
        .unwrap();
        assert!(s.is_pressure_authoritative());
    }

    #[test]
    fn simulated_source_label_always_says_simulated() {
        assert!(PointerSource::SimulatedMouse
            .label()
            .to_lowercase()
            .contains("simulated"));
    }

    #[test]
    fn serialization_round_trips() {
        let s = valid_sample();
        let json = serde_json::to_string(&s).unwrap();
        let back: PointerSample = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}
