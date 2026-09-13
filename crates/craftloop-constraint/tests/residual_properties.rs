//! Property-based tests for `residual`.
//!
//! Execution 01, Phase 27, Task 195. Authority: Execution 01, Phase 11,
//! Task 078 ("how far is this constraint from satisfied"); every residual
//! function a solver leans on must never report a negative distance from
//! satisfied, and must report exactly zero when the values it is handed
//! already satisfy the constraint -- checked here against hundreds of
//! randomly generated values per constraint kind, not the fixed examples
//! `residual.rs`'s own unit tests already cover.

use std::collections::BTreeMap;

use craftloop_constraint::{residual, GeometricConstraint};
use craftloop_constraint::{PointVariables, VariableId};
use proptest::prelude::*;

fn finite_value() -> impl Strategy<Value = f64> {
    -1.0e6..1.0e6
}

proptest! {
    #[test]
    fn fixed_value_residual_is_nonnegative_and_zero_when_satisfied(target in finite_value(), actual in finite_value()) {
        let variable = VariableId(0);
        let constraint = GeometricConstraint::FixedValue { variable, value: target };
        let mut values = BTreeMap::new();
        values.insert(variable, actual);

        let r = residual(&constraint, &values);
        prop_assert!(r >= 0.0);

        values.insert(variable, target);
        prop_assert_eq!(residual(&constraint, &values), 0.0);
    }

    #[test]
    fn equal_values_residual_is_nonnegative_and_zero_when_equal(a_value in finite_value(), b_value in finite_value()) {
        let a = VariableId(0);
        let b = VariableId(1);
        let constraint = GeometricConstraint::EqualValues { a, b };
        let mut values = BTreeMap::new();
        values.insert(a, a_value);
        values.insert(b, b_value);

        prop_assert!(residual(&constraint, &values) >= 0.0);

        values.insert(b, a_value);
        prop_assert_eq!(residual(&constraint, &values), 0.0);
    }

    #[test]
    fn coincident_residual_is_nonnegative_and_zero_for_the_same_point(
        ax in finite_value(), ay in finite_value(),
        bx in finite_value(), by in finite_value(),
    ) {
        let a = PointVariables::new(VariableId(0), VariableId(1));
        let b = PointVariables::new(VariableId(2), VariableId(3));
        let constraint = GeometricConstraint::Coincident { a, b };
        let mut values = BTreeMap::new();
        values.insert(VariableId(0), ax);
        values.insert(VariableId(1), ay);
        values.insert(VariableId(2), bx);
        values.insert(VariableId(3), by);

        prop_assert!(residual(&constraint, &values) >= 0.0);

        values.insert(VariableId(2), ax);
        values.insert(VariableId(3), ay);
        prop_assert_eq!(residual(&constraint, &values), 0.0);
    }
}
