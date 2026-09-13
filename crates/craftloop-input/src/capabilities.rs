//! Input capability descriptors.
//!
//! Execution 01, Phase 03, Task 022. Authority: MCP Article 127 "Device
//! Capability Matrix".
//!
//! Forbidden shortcut this module exists to close: fabricating a capability
//! a device does not have (e.g. reporting `tilt: true` on Windows because
//! "most styluses support tilt"). Every field here must reflect what the
//! concrete adapter genuinely queried/knows, not an assumption.

use serde::{Deserialize, Serialize};

/// What a pointer input source genuinely supports. Every adapter (mouse
/// simulator, future Android/iPad adapters) must construct this from real
/// platform capability queries, never from a guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputCapabilities {
    /// The device reports genuine, hardware-sensed pressure.
    pub pressure: bool,
    /// The device reports genuine stylus tilt angles.
    pub tilt: bool,
    /// The device can report pointer position before contact (hover).
    pub hover: bool,
    /// The device/OS can distinguish an incidental palm touch from
    /// intentional pointer input.
    pub palm_rejection: bool,
    /// The device has a distinct eraser end that can be sensed separately
    /// from the writing tip.
    pub eraser: bool,
}

impl InputCapabilities {
    /// No capability present. The honest baseline for an unknown or
    /// minimal input source; a real adapter should set fields to `true`
    /// only for capabilities it has actually confirmed.
    pub const NONE: InputCapabilities = InputCapabilities {
        pressure: false,
        tilt: false,
        hover: false,
        palm_rejection: false,
        eraser: false,
    };

    pub fn has_any(&self) -> bool {
        self.pressure || self.tilt || self.hover || self.palm_rejection || self.eraser
    }
}

impl Default for InputCapabilities {
    fn default() -> Self {
        Self::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_reports_no_capabilities() {
        assert!(!InputCapabilities::NONE.has_any());
        assert_eq!(InputCapabilities::default(), InputCapabilities::NONE);
    }

    #[test]
    fn has_any_is_true_if_a_single_capability_is_set() {
        let mostly_none = InputCapabilities {
            hover: true,
            ..InputCapabilities::NONE
        };
        assert!(mostly_none.has_any());
    }
}
