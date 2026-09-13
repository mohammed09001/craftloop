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
    /// The distance from `a0` to `a1` equals the distance from `b0` to
    /// `b1`. Deliberately expressed in terms of point pairs, not a derived
    /// "length" scalar: this lets the same variant serve both equal-length
    /// segments (Task 087) and equal-radius circles (Task 087) by passing
    /// `(center, point_on_circle)` as one or both pairs, matching how
    /// [`GeometricConstraint::Radius`] already represents a circle's radius
    /// as a center-to-point-on-circle distance rather than inventing a
    /// separate scalar-radius variable.
    EqualLength {
        a0: PointVariables,
        a1: PointVariables,
        b0: PointVariables,
        b1: PointVariables,
    },
    /// The line `line_a`-`line_b` is tangent to the circle described by
    /// `center`/`point_on_circle` (the perpendicular distance from `center`
    /// to the line equals the circle's radius).
    LineTangentToCircle {
        line_a: PointVariables,
        line_b: PointVariables,
        center: PointVariables,
        point_on_circle: PointVariables,
    },
    /// Two circles are externally tangent (the distance between their
    /// centers equals the sum of their radii). Internal tangency (one
    /// circle inside the other) is not represented by this variant -- no
    /// Version 1 scenario needs it (Task 089 evidence,
    /// `execution-evidence/solver-evaluations/solver-decision-record.md`),
    /// and adding it speculatively would be exactly the scope creep the
    /// Loop Engineering Contract forbids.
    CircleTangentToCircle {
        a_center: PointVariables,
        a_point_on_circle: PointVariables,
        b_center: PointVariables,
        b_point_on_circle: PointVariables,
    },
    /// Points `a` and `b` are mirror images of each other across the axis
    /// line `axis_a`-`axis_b`.
    Symmetric {
        axis_a: PointVariables,
        axis_b: PointVariables,
        a: PointVariables,
        b: PointVariables,
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
            GeometricConstraint::EqualLength { a0, a1, b0, b1 } => pts(&[*a0, *a1, *b0, *b1]),
            GeometricConstraint::LineTangentToCircle {
                line_a,
                line_b,
                center,
                point_on_circle,
            } => pts(&[*line_a, *line_b, *center, *point_on_circle]),
            GeometricConstraint::CircleTangentToCircle {
                a_center,
                a_point_on_circle,
                b_center,
                b_point_on_circle,
            } => pts(&[*a_center, *a_point_on_circle, *b_center, *b_point_on_circle]),
            GeometricConstraint::Symmetric {
                axis_a,
                axis_b,
                a,
                b,
            } => pts(&[*axis_a, *axis_b, *a, *b]),
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

    #[test]
    fn equal_length_reports_all_eight_underlying_scalar_variables() {
        let c = GeometricConstraint::EqualLength {
            a0: pv(0, 1),
            a1: pv(2, 3),
            b0: pv(4, 5),
            b1: pv(6, 7),
        };
        assert_eq!(c.variable_ids().len(), 8);
    }

    #[test]
    fn line_tangent_to_circle_reports_all_eight_underlying_scalar_variables() {
        let c = GeometricConstraint::LineTangentToCircle {
            line_a: pv(0, 1),
            line_b: pv(2, 3),
            center: pv(4, 5),
            point_on_circle: pv(6, 7),
        };
        assert_eq!(c.variable_ids().len(), 8);
    }

    #[test]
    fn circle_tangent_to_circle_reports_all_eight_underlying_scalar_variables() {
        let c = GeometricConstraint::CircleTangentToCircle {
            a_center: pv(0, 1),
            a_point_on_circle: pv(2, 3),
            b_center: pv(4, 5),
            b_point_on_circle: pv(6, 7),
        };
        assert_eq!(c.variable_ids().len(), 8);
    }

    #[test]
    fn symmetric_reports_all_eight_underlying_scalar_variables() {
        let c = GeometricConstraint::Symmetric {
            axis_a: pv(0, 1),
            axis_b: pv(2, 3),
            a: pv(4, 5),
            b: pv(6, 7),
        };
        assert_eq!(c.variable_ids().len(), 8);
    }

    #[test]
    fn new_variants_serialize_and_round_trip() {
        let cases = [
            GeometricConstraint::EqualLength {
                a0: pv(0, 1),
                a1: pv(2, 3),
                b0: pv(4, 5),
                b1: pv(6, 7),
            },
            GeometricConstraint::LineTangentToCircle {
                line_a: pv(0, 1),
                line_b: pv(2, 3),
                center: pv(4, 5),
                point_on_circle: pv(6, 7),
            },
            GeometricConstraint::CircleTangentToCircle {
                a_center: pv(0, 1),
                a_point_on_circle: pv(2, 3),
                b_center: pv(4, 5),
                b_point_on_circle: pv(6, 7),
            },
            GeometricConstraint::Symmetric {
                axis_a: pv(0, 1),
                axis_b: pv(2, 3),
                a: pv(4, 5),
                b: pv(6, 7),
            },
        ];
        for c in cases {
            let json = serde_json::to_string(&c).unwrap();
            let back: GeometricConstraint = serde_json::from_str(&json).unwrap();
            assert_eq!(c, back);
        }
    }
}
