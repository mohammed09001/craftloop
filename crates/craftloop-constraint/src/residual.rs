//! Constraint residual evaluation.
//!
//! Execution 01, Phase 11, Task 078. Shared by the reference
//! [`crate::solver::ResidualChecker`] and, per Task 080, by any in-house
//! solver prototype that needs "how far is this constraint from
//! satisfied" as a building block (gradient-descent, Newton's method, and
//! simple satisfaction checks all start from the same residual function).

use std::collections::BTreeMap;

use crate::constraint::GeometricConstraint;
use crate::variable::{PointVariables, VariableId};

fn value_of(values: &BTreeMap<VariableId, f64>, id: VariableId) -> f64 {
    values.get(&id).copied().unwrap_or(0.0)
}

fn point_of(values: &BTreeMap<VariableId, f64>, p: PointVariables) -> (f64, f64) {
    (value_of(values, p.x), value_of(values, p.y))
}

fn distance((ax, ay): (f64, f64), (bx, by): (f64, f64)) -> f64 {
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

/// How far `constraint` is from satisfied at `values`, in the constraint's
/// own unit (the same convention as `ConstraintDiagnostic::residual`).
/// Always `>= 0.0`.
pub fn residual(constraint: &GeometricConstraint, values: &BTreeMap<VariableId, f64>) -> f64 {
    match constraint {
        GeometricConstraint::FixedValue { variable, value } => {
            (value_of(values, *variable) - value).abs()
        }
        GeometricConstraint::EqualValues { a, b } => {
            (value_of(values, *a) - value_of(values, *b)).abs()
        }
        GeometricConstraint::Distance { a, b, value } => {
            (distance(point_of(values, *a), point_of(values, *b)) - value).abs()
        }
        GeometricConstraint::Coincident { a, b } => {
            distance(point_of(values, *a), point_of(values, *b))
        }
        GeometricConstraint::Horizontal { a, b } => {
            (point_of(values, *a).1 - point_of(values, *b).1).abs()
        }
        GeometricConstraint::Vertical { a, b } => {
            (point_of(values, *a).0 - point_of(values, *b).0).abs()
        }
        GeometricConstraint::Parallel { a0, a1, b0, b1 } => {
            let (ax0, ay0) = point_of(values, *a0);
            let (ax1, ay1) = point_of(values, *a1);
            let (bx0, by0) = point_of(values, *b0);
            let (bx1, by1) = point_of(values, *b1);
            // Cross product of the two direction vectors; zero iff parallel.
            ((ax1 - ax0) * (by1 - by0) - (ay1 - ay0) * (bx1 - bx0)).abs()
        }
        GeometricConstraint::Perpendicular { a0, a1, b0, b1 } => {
            let (ax0, ay0) = point_of(values, *a0);
            let (ax1, ay1) = point_of(values, *a1);
            let (bx0, by0) = point_of(values, *b0);
            let (bx1, by1) = point_of(values, *b1);
            // Dot product of the two direction vectors; zero iff perpendicular.
            ((ax1 - ax0) * (bx1 - bx0) + (ay1 - ay0) * (by1 - by0)).abs()
        }
        GeometricConstraint::Radius {
            center,
            point_on_circle,
            value,
        } => (distance(
            point_of(values, *center),
            point_of(values, *point_on_circle),
        ) - value)
            .abs(),
        GeometricConstraint::Angle {
            vertex,
            a,
            b,
            value_radians,
        } => {
            let (vx, vy) = point_of(values, *vertex);
            let (ax, ay) = point_of(values, *a);
            let (bx, by) = point_of(values, *b);
            let angle_a = (ay - vy).atan2(ax - vx);
            let angle_b = (by - vy).atan2(bx - vx);
            let mut diff = angle_b - angle_a - value_radians;
            // Normalize into (-PI, PI] so e.g. a 2*PI-off angle reads as ~0.
            while diff > std::f64::consts::PI {
                diff -= std::f64::consts::TAU;
            }
            while diff < -std::f64::consts::PI {
                diff += std::f64::consts::TAU;
            }
            diff.abs()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variable::PointVariables;

    fn values(pairs: &[(u64, f64)]) -> BTreeMap<VariableId, f64> {
        pairs.iter().map(|(id, v)| (VariableId(*id), *v)).collect()
    }

    #[test]
    fn fixed_value_residual_is_zero_when_satisfied() {
        let c = GeometricConstraint::FixedValue {
            variable: VariableId(0),
            value: 5.0,
        };
        assert_eq!(residual(&c, &values(&[(0, 5.0)])), 0.0);
    }

    #[test]
    fn fixed_value_residual_is_positive_when_unsatisfied() {
        let c = GeometricConstraint::FixedValue {
            variable: VariableId(0),
            value: 5.0,
        };
        assert_eq!(residual(&c, &values(&[(0, 8.0)])), 3.0);
    }

    #[test]
    fn distance_residual_matches_actual_euclidean_distance_minus_target() {
        let a = PointVariables::new(VariableId(0), VariableId(1));
        let b = PointVariables::new(VariableId(2), VariableId(3));
        let c = GeometricConstraint::Distance { a, b, value: 5.0 };
        // (0,0) to (3,4) is distance 5 -- should be exactly satisfied.
        let r = residual(&c, &values(&[(0, 0.0), (1, 0.0), (2, 3.0), (3, 4.0)]));
        assert!(r < 1e-9);
    }

    #[test]
    fn coincident_residual_is_the_distance_between_the_two_points() {
        let a = PointVariables::new(VariableId(0), VariableId(1));
        let b = PointVariables::new(VariableId(2), VariableId(3));
        let c = GeometricConstraint::Coincident { a, b };
        let r = residual(&c, &values(&[(0, 0.0), (1, 0.0), (2, 3.0), (3, 4.0)]));
        assert!((r - 5.0).abs() < 1e-9);
    }

    #[test]
    fn parallel_residual_is_zero_for_two_parallel_segments() {
        let a0 = PointVariables::new(VariableId(0), VariableId(1));
        let a1 = PointVariables::new(VariableId(2), VariableId(3));
        let b0 = PointVariables::new(VariableId(4), VariableId(5));
        let b1 = PointVariables::new(VariableId(6), VariableId(7));
        let c = GeometricConstraint::Parallel { a0, a1, b0, b1 };
        // a: (0,0)->(1,0); b: (0,5)->(1,5) -- both horizontal, parallel.
        let r = residual(
            &c,
            &values(&[
                (0, 0.0),
                (1, 0.0),
                (2, 1.0),
                (3, 0.0),
                (4, 0.0),
                (5, 5.0),
                (6, 1.0),
                (7, 5.0),
            ]),
        );
        assert!(r < 1e-9);
    }

    #[test]
    fn perpendicular_residual_is_zero_for_a_right_angle() {
        let a0 = PointVariables::new(VariableId(0), VariableId(1));
        let a1 = PointVariables::new(VariableId(2), VariableId(3));
        let b0 = PointVariables::new(VariableId(4), VariableId(5));
        let b1 = PointVariables::new(VariableId(6), VariableId(7));
        let c = GeometricConstraint::Perpendicular { a0, a1, b0, b1 };
        // a: (0,0)->(1,0) horizontal; b: (0,0)->(0,1) vertical.
        let r = residual(
            &c,
            &values(&[
                (0, 0.0),
                (1, 0.0),
                (2, 1.0),
                (3, 0.0),
                (4, 0.0),
                (5, 0.0),
                (6, 0.0),
                (7, 1.0),
            ]),
        );
        assert!(r < 1e-9);
    }

    #[test]
    fn angle_residual_is_zero_for_a_matching_right_angle() {
        let vertex = PointVariables::new(VariableId(0), VariableId(1));
        let a = PointVariables::new(VariableId(2), VariableId(3));
        let b = PointVariables::new(VariableId(4), VariableId(5));
        let c = GeometricConstraint::Angle {
            vertex,
            a,
            b,
            value_radians: std::f64::consts::FRAC_PI_2,
        };
        // vertex (0,0), a at (1,0) [angle 0], b at (0,1) [angle PI/2] -> diff = PI/2.
        let r = residual(
            &c,
            &values(&[(0, 0.0), (1, 0.0), (2, 1.0), (3, 0.0), (4, 0.0), (5, 1.0)]),
        );
        assert!(r < 1e-9);
    }

    #[test]
    fn all_residuals_are_nonnegative() {
        let c = GeometricConstraint::EqualValues {
            a: VariableId(0),
            b: VariableId(1),
        };
        assert!(residual(&c, &values(&[(0, -5.0), (1, 5.0)])) >= 0.0);
    }
}
