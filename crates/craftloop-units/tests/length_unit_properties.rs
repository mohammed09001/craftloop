//! Property-based tests for `LengthUnit` conversion.
//!
//! Execution 01, Phase 27, Task 195. Authority: MCP Article 72 "Unit
//! System" -- a value converted to the canonical millimeter storage unit
//! and back must recover the original number for every unit, not just
//! the fixed examples (1.0 inch, 1.0 meter, ...) the Phase 09 unit tests
//! happen to check.

use craftloop_units::LengthUnit;
use proptest::prelude::*;

fn finite_value() -> impl Strategy<Value = f64> {
    -1.0e9..1.0e9
}

fn any_unit() -> impl Strategy<Value = LengthUnit> {
    prop_oneof![
        Just(LengthUnit::Millimeters),
        Just(LengthUnit::Centimeters),
        Just(LengthUnit::Meters),
        Just(LengthUnit::Inches),
    ]
}

proptest! {
    #[test]
    fn to_millimeters_and_back_recovers_the_original_value(
        value in finite_value(), unit in any_unit(),
    ) {
        let millimeters = unit.to_millimeters(value);
        let recovered = unit.from_millimeters(millimeters);
        // Inches divides by 25.4, which is not exactly representable in
        // binary floating point, so this is a relative-tolerance check,
        // not bit-for-bit equality.
        let tolerance = 1e-9 * value.abs().max(1.0);
        prop_assert!(
            (recovered - value).abs() <= tolerance,
            "unit={unit:?} value={value} recovered={recovered}"
        );
    }

    #[test]
    fn millimeters_to_millimeters_is_always_the_exact_identity(value in finite_value()) {
        prop_assert_eq!(LengthUnit::Millimeters.to_millimeters(value), value);
        prop_assert_eq!(LengthUnit::Millimeters.from_millimeters(value), value);
    }
}
