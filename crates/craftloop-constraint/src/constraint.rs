//! Backend-neutral geometric constraint vocabulary.
//!
//! Execution 01, Phase 11, Task 078. Authority: Engine Contract 10.
//!
//! This vocabulary was scoped by evaluating what a real candidate backend
//! (`ezpz`, Task 079) actually offers, not invented speculatively: every
//! variant here corresponds to a relationship `ezpz`'s public `Constraint`
//! enum supports (`Distance`, `Fixed`/pinned value, `Vertical`/`Horizontal`,
//! `Parallel`/`Perpendicular`, `Coincident`, `Radius`, `Equal`), confirmed
//! via `docs.rs/ezpz` before this interface was written (see
//! `execution-evidence/solver-evaluations/`). Keeping this crate's own
//! vocabulary independent of `ezpz`'s actual Rust types is still the point
//! of Task 078's "before choosing implementation": a rejected or replaced
//! backend only needs a new adapter translating this enum into its own
//! types, not a rewrite of every call site across the engine.

use serde::{Deserialize, Serialize};

use crate::variable::{PointVariables, VariableId};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GeometricConstraint {
    /// Pin one scalar variable to an exact value.
    FixedValue { variable: VariableId, value: f64 },
    /// Two scalar variables must be equal.
    EqualValues { a: VariableId, b: VariableId },
    /// Euclidean distance between two points.
    Distance {
        a: PointVariables,
        b: PointVariables,
        value: f64,
    },
    /// Two points occupy the same location.
    Coincident {
        a: PointVariables,
        b: PointVariables,
    },
    /// Two points share the same y coordinate.
    Horizontal {
        a: PointVariables,
        b: PointVariables,
    },
    /// Two points share the same x coordinate.
    Vertical {
        a: PointVariables,
        b: PointVariables,
    },
    /// The segment `a0`-`a1` is parallel to `b0`-`b1`.
    Parallel {
        a0: PointVariables,
        a1: PointVariables,
        b0: PointVariables,
        b1: PointVariables,
    },
    /// The segment `a0`-`a1` is perpendicular to `b0`-`b1`.
    Perpendicular {
        a0: PointVariables,
        a1: PointVariables,
        b0: PointVariables,
        b1: PointVariables,
    },
    /// Distance from `center` to `point_on_circle` equals `value`.
    Radius {
        center: PointVariables,
        point_on_circle: PointVariables,
        value: f64,
    },
    /// Angle (radians) at `vertex` between the rays to `a` and `b`.
    Angle {
        vertex: PointVariables,
        a: PointVariables,
        b: PointVariables,
        value_radians: f64,
    },
}

impl GeometricConstraint {
    /// Every variable this constraint reads or constrains, for callers
    /// building a variable/constraint graph (e.g. to detect which
    /// variables a given constraint touches without matching on the
    /// variant themselves).
    pub fn variable_ids(&self) -> Vec<VariableId> {
        fn pts(points: &[PointVariables]) -> Vec<VariableId> {
            points.iter().flat_map(|p| [p.x, p.y]).collect()
        }
        match self {
            GeometricConstraint::FixedValue { variable, .. } => vec![*variable],
            GeometricConstraint::EqualValues { a, b } => vec![*a, *b],
            GeometricConstraint::Distance { a, b, .. }
            | GeometricConstraint::Coincident { a, b } => pts(&[*a, *b]),
            GeometricConstraint::Horizontal { a, b } | GeometricConstraint::Vertical { a, b } => {
                pts(&[*a, *b])
            }
            GeometricConstraint::Parallel { a0, a1, b0, b1 }
            | GeometricConstraint::Perpendicular { a0, a1, b0, b1 } => pts(&[*a0, *a1, *b0, *b1]),
            GeometricConstraint::Radius {
                center,
                point_on_circle,
                ..
            } => pts(&[*center, *point_on_circle]),
            GeometricConstraint::Angle { vertex, a, b, .. } => pts(&[*vertex, *a, *b]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pv(x: u64, y: u64) -> PointVariables {
        PointVariables::new(VariableId(x), VariableId(y))
    }

    #[test]
    fn distance_reports_all_four_underlying_scalar_variables() {
        let c = GeometricConstraint::Distance {
            a: pv(0, 1),
            b: pv(2, 3),
            value: 10.0,
        };
        assert_eq!(
            c.variable_ids(),
            vec![VariableId(0), VariableId(1), VariableId(2), VariableId(3)]
        );
    }

    #[test]
    fn fixed_value_reports_exactly_one_variable() {
        let c = GeometricConstraint::FixedValue {
            variable: VariableId(5),
            value: 1.0,
        };
        assert_eq!(c.variable_ids(), vec![VariableId(5)]);
    }

    #[test]
    fn parallel_reports_all_eight_underlying_scalar_variables() {
        let c = GeometricConstraint::Parallel {
            a0: pv(0, 1),
            a1: pv(2, 3),
            b0: pv(4, 5),
            b1: pv(6, 7),
        };
        assert_eq!(c.variable_ids().len(), 8);
    }

    #[test]
    fn serialization_round_trips() {
        let c = GeometricConstraint::Radius {
            center: pv(0, 1),
            point_on_circle: pv(2, 3),
            value: 5.0,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: GeometricConstraint = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
    }
}
