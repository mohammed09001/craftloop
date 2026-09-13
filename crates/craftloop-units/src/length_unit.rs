//! Canonical internal length unit and conversions.
//!
//! Execution 01, Phase 09, Task 062. Authority: Engine Contract 07
//! (Engineering Parser); MCP Article 72 "Unit System".
//!
//! **Policy: the canonical internal unit is millimeters.** Every `f64`
//! length value stored anywhere in the shared core (`craftloop-geometry`'s
//! `Point2`/`Segment2`/etc., which were built unit-agnostic in Phase 02)
//! is, from this phase onward, a count of millimeters. This crate is where
//! that policy becomes enforceable: anything that turns user-facing text
//! into a stored number must go through [`LengthUnit::to_millimeters`], and
//! anything that turns a stored number into display text must go through
//! [`LengthUnit::from_millimeters`]. Millimeters were chosen (over, say,
//! an abstract "document unit") because it is a single fixed scale with no
//! per-document ambiguity, matches `DocumentUnits`'s existing default
//! (Phase 07), and is the standard base unit for mechanical engineering
//! drawings.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LengthUnit {
    #[default]
    Millimeters,
    Centimeters,
    Meters,
    Inches,
}

impl LengthUnit {
    /// How many millimeters one unit of `self` is.
    const fn millimeters_per_unit(&self) -> f64 {
        match self {
            LengthUnit::Millimeters => 1.0,
            LengthUnit::Centimeters => 10.0,
            LengthUnit::Meters => 1000.0,
            LengthUnit::Inches => 25.4,
        }
    }

    pub fn to_millimeters(&self, value: f64) -> f64 {
        value * self.millimeters_per_unit()
    }

    pub fn from_millimeters(&self, millimeters: f64) -> f64 {
        millimeters / self.millimeters_per_unit()
    }

    /// The canonical (shortest, most common) written suffix for this unit,
    /// used when formatting a value for display.
    pub fn suffix(&self) -> &'static str {
        match self {
            LengthUnit::Millimeters => "mm",
            LengthUnit::Centimeters => "cm",
            LengthUnit::Meters => "m",
            LengthUnit::Inches => "in",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn millimeters_is_the_identity_conversion() {
        assert_eq!(LengthUnit::Millimeters.to_millimeters(42.0), 42.0);
        assert_eq!(LengthUnit::Millimeters.from_millimeters(42.0), 42.0);
    }

    #[test]
    fn centimeters_and_meters_convert_by_powers_of_ten() {
        assert_eq!(LengthUnit::Centimeters.to_millimeters(1.0), 10.0);
        assert_eq!(LengthUnit::Meters.to_millimeters(1.0), 1000.0);
    }

    #[test]
    fn inches_convert_using_the_standard_25_4mm_factor() {
        assert!((LengthUnit::Inches.to_millimeters(1.0) - 25.4).abs() < 1e-9);
    }

    #[test]
    fn to_and_from_millimeters_round_trip_for_every_unit() {
        for unit in [
            LengthUnit::Millimeters,
            LengthUnit::Centimeters,
            LengthUnit::Meters,
            LengthUnit::Inches,
        ] {
            let mm = unit.to_millimeters(7.5);
            let back = unit.from_millimeters(mm);
            assert!((back - 7.5).abs() < 1e-9, "{unit:?} did not round-trip");
        }
    }

    #[test]
    fn default_unit_is_millimeters() {
        assert_eq!(LengthUnit::default(), LengthUnit::Millimeters);
    }
}
