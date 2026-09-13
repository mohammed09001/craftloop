//! Document unit setting.
//!
//! Execution 01, Phase 07, Task 048 ("Store schema version, metadata,
//! units, ..."). Authority: MCP Article 72 "Unit System".
//!
//! This is intentionally a small presentation-facing enum, not the full
//! numeric parsing/conversion engine -- that is Phase 09's job (Task 062
//! "Define canonical internal units", Task 063 "Implement document unit
//! settings"). What the document root needs *now* is just a stored setting
//! that says which unit the user is working in; Phase 09 owns interpreting
//! numbers against it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DocumentUnits {
    #[default]
    Millimeters,
    Centimeters,
    Inches,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_unit_is_millimeters() {
        assert_eq!(DocumentUnits::default(), DocumentUnits::Millimeters);
    }

    #[test]
    fn serialization_round_trips() {
        for unit in [
            DocumentUnits::Millimeters,
            DocumentUnits::Centimeters,
            DocumentUnits::Inches,
        ] {
            let json = serde_json::to_string(&unit).unwrap();
            let back: DocumentUnits = serde_json::from_str(&json).unwrap();
            assert_eq!(unit, back);
        }
    }
}
