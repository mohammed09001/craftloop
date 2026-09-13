//! Tool selection and simulated capability controls.
//!
//! Execution 01, Phase 04, Task 030. Authority: MCP Article 244 "Detailed
//! Specification of Pen Tools".
//!
//! `SimulatedControls` exists because a mouse genuinely cannot produce
//! pressure/tilt variation, but exercising engine behavior that depends on
//! those values (once later phases consume them) still needs *some* way to
//! vary them from the harness. It never edits `InputCapabilities` — the
//! capability descriptor stays honestly `NONE` regardless of what a
//! developer dials in here, per Task 023/026's "never claim real hardware."

use craftloop_errors::{DomainError, DomainResult, InputErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessTool {
    Pen,
    Line,
    Circle,
    Rectangle,
    Eraser,
    Select,
}

impl HarnessTool {
    pub const ALL: [HarnessTool; 6] = [
        HarnessTool::Pen,
        HarnessTool::Line,
        HarnessTool::Circle,
        HarnessTool::Rectangle,
        HarnessTool::Eraser,
        HarnessTool::Select,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            HarnessTool::Pen => "Pen",
            HarnessTool::Line => "Line",
            HarnessTool::Circle => "Circle",
            HarnessTool::Rectangle => "Rectangle",
            HarnessTool::Eraser => "Eraser",
            HarnessTool::Select => "Select",
        }
    }
}

/// Developer-adjustable stand-ins for capabilities a mouse cannot provide.
/// Purely a harness testing aid -- see module docs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatedControls {
    pressure_override: f64,
    pub barrel_button_down: bool,
}

impl SimulatedControls {
    pub fn new() -> Self {
        Self {
            pressure_override: 0.5,
            barrel_button_down: false,
        }
    }

    pub fn pressure_override(&self) -> f64 {
        self.pressure_override
    }

    pub fn set_pressure_override(&mut self, value: f64) -> DomainResult<()> {
        if !(0.0..=1.0).contains(&value) {
            return Err(DomainError::Input {
                kind: InputErrorKind::OutOfRange,
                detail: format!("simulated pressure override must be within [0, 1], got {value}"),
            });
        }
        self.pressure_override = value;
        Ok(())
    }
}

impl Default for SimulatedControls {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tool_has_a_distinct_nonempty_label() {
        let labels: Vec<&str> = HarnessTool::ALL.iter().map(HarnessTool::label).collect();
        for label in &labels {
            assert!(!label.is_empty());
        }
        let mut sorted = labels.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), labels.len(), "tool labels must be distinct");
    }

    #[test]
    fn default_pressure_override_matches_the_mouse_simulator_constant() {
        assert_eq!(
            SimulatedControls::new().pressure_override(),
            craftloop_input::MouseSimulator::CONSTANT_PRESSURE
        );
    }

    #[test]
    fn pressure_override_rejects_out_of_range_values() {
        let mut controls = SimulatedControls::new();
        assert!(controls.set_pressure_override(1.5).is_err());
        assert!(controls.set_pressure_override(-0.1).is_err());
        // A rejected update must not partially apply.
        assert_eq!(controls.pressure_override(), 0.5);
    }

    #[test]
    fn pressure_override_accepts_and_applies_valid_values() {
        let mut controls = SimulatedControls::new();
        controls.set_pressure_override(0.9).unwrap();
        assert_eq!(controls.pressure_override(), 0.9);
    }
}
