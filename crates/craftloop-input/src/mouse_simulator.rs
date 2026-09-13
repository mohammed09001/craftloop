//! Mouse-to-pen simulation adapter.
//!
//! Execution 01, Phase 03, Task 023. Authority: MCP Article 6 "Pen-Native,
//! Not Merely Pen-Compatible"; "Execution 01" Agent Operating Directive on
//! never overclaiming simulated hardware.
//!
//! This is the shared, testable mapping logic; wiring it to actual Windows
//! `winit`/`egui` mouse events happens in the Phase 04 harness
//! (`apps/windows-harness`), which is expected to call
//! [`MouseSimulator::sample`] rather than constructing `PointerSample`
//! directly, so the "constant pressure, absent tilt, never-claim-real"
//! contract lives in exactly one place.

use craftloop_geometry::Point2;

use crate::capabilities::InputCapabilities;
use crate::sample::{PointerButtons, PointerSample, PointerSource};

/// Maps Windows mouse input to simulated pen samples.
///
/// Per Task 023: pressure is an explicit constant (not a fabricated
/// hardware reading), tilt is always absent, and `capabilities` never
/// claims support this adapter cannot provide.
pub struct MouseSimulator;

impl MouseSimulator {
    /// A mouse never has real pressure, tilt, hover, palm rejection, or
    /// eraser sensing.
    pub const CAPABILITIES: InputCapabilities = InputCapabilities::NONE;

    /// The constant pressure-like value used for rendering (e.g. stroke
    /// width) when no real sensor exists. Not sensor data -- see
    /// [`PointerSample::is_pressure_authoritative`].
    pub const CONSTANT_PRESSURE: f64 = 0.5;

    /// Build one normalized sample from a raw mouse position/time/buttons.
    /// Never fails: every value this adapter can supply is already within
    /// `PointerSample::new`'s valid ranges by construction.
    pub fn sample(
        position: Point2,
        timestamp_seconds: f64,
        buttons: PointerButtons,
    ) -> PointerSample {
        PointerSample::new(
            position,
            timestamp_seconds,
            Some(Self::CONSTANT_PRESSURE),
            None, // tilt: always absent, never fabricated
            None,
            PointerSource::SimulatedMouse,
            buttons,
            Self::CAPABILITIES,
        )
        .expect("MouseSimulator always constructs values within PointerSample's valid ranges")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulated_sample_has_constant_pressure_and_no_tilt() {
        let s = MouseSimulator::sample(Point2::new(10.0, 20.0), 1.0, PointerButtons::default());
        assert_eq!(s.pressure, Some(MouseSimulator::CONSTANT_PRESSURE));
        assert_eq!(s.tilt_x_deg, None);
        assert_eq!(s.tilt_y_deg, None);
    }

    #[test]
    fn simulated_sample_capabilities_never_claim_real_hardware_support() {
        let s = MouseSimulator::sample(Point2::ORIGIN, 0.0, PointerButtons::default());
        assert!(!s.capabilities.pressure);
        assert!(!s.capabilities.tilt);
        assert!(!s.capabilities.hover);
        assert!(!s.capabilities.palm_rejection);
        assert!(!s.capabilities.eraser);
        // The value is present for rendering, but explicitly non-authoritative.
        assert!(!s.is_pressure_authoritative());
    }

    #[test]
    fn simulated_sample_is_tagged_with_the_simulated_source() {
        let s = MouseSimulator::sample(Point2::ORIGIN, 0.0, PointerButtons::default());
        assert_eq!(s.source, PointerSource::SimulatedMouse);
    }

    #[test]
    fn position_and_timestamp_pass_through_unchanged() {
        let p = Point2::new(3.5, -7.25);
        let s = MouseSimulator::sample(p, 42.0, PointerButtons::default());
        assert_eq!(s.position, p);
        assert_eq!(s.timestamp_seconds, 42.0);
    }
}
