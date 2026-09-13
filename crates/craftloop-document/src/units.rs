//! Document unit setting.
//!
//! Execution 01, Phase 07, Task 048 ("Store schema version, metadata,
//! units, ..."). Authority: MCP Article 72 "Unit System".
//!
//! Phase 07 originally defined a small local `DocumentUnits` enum here as
//! a placeholder, explicitly noting that "Phase 09 owns interpreting
//! numbers against it." Phase 09 (`craftloop-units`) has since defined the
//! authoritative unit vocabulary, [`craftloop_units::LengthUnit`]
//! (millimeters/centimeters/meters/inches, plus the conversion logic every
//! numeric parser in that crate uses). Re-exporting it here as
//! `DocumentUnits` avoids keeping two near-identical enums in sync by
//! hand -- exactly the "duplicated truth" the Agent Operating Directive
//! warns against -- while keeping the name document code was already
//! written against.

pub use craftloop_units::LengthUnit as DocumentUnits;

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
            DocumentUnits::Meters,
            DocumentUnits::Inches,
        ] {
            let json = serde_json::to_string(&unit).unwrap();
            let back: DocumentUnits = serde_json::from_str(&json).unwrap();
            assert_eq!(unit, back);
        }
    }
}
