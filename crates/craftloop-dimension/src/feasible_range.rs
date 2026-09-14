//! Feasible numeric ranges for dimensions constrained by inequalities.
//!
//! Execution 01, Phase 13, Tasks 097-098. Authority: MCP Article 23
//! ("If a triangle has two sides of 30 and 50 units, the third side
//! cannot be an arbitrary number. The triangle inequality restricts its
//! feasible range.") and Article 27's worked example ("300 cannot form
//! this triangle while the other two sides remain 30 and 50.").
//!
//! Deliberately scoped to exactly the relation MCP names via a concrete
//! worked example -- the triangle-side inequality. A general
//! interval-arithmetic engine over arbitrary constraint graphs is a much
//! larger undertaking no task in this phase, and no other MCP article
//! found while researching this task, asks for.

use craftloop_errors::{DimensionErrorKind, DomainError, DomainResult};

/// A strictly-open feasible interval `(min, max)` for a triangle's third
/// side given the other two fixed side lengths: the endpoints themselves
/// are *not* feasible (they describe a degenerate, zero-area "triangle").
///
/// Deliberately distinct from [`crate::SemanticDimension::feasible_range`],
/// which stores an *inclusive* `[min, max]` (Phase 10, Task 073) -- a
/// caller wiring this into a `Bounded` dimension must account for that
/// difference itself (e.g. by checking [`TriangleSideRange::is_feasible`]
/// directly rather than relying on `SemanticDimension`'s inclusive check
/// alone, which would wrongly accept the two degenerate boundary values).
/// This distinction is recorded here rather than silently papered over by
/// nudging the bounds by an arbitrary epsilon.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriangleSideRange {
    pub min: f64,
    pub max: f64,
}

impl TriangleSideRange {
    /// The feasible range for a triangle's third side, given the other
    /// two side lengths `a` and `b`: `(|a - b|, a + b)`, exclusive.
    pub fn for_two_fixed_sides(a: f64, b: f64) -> DomainResult<Self> {
        if !(a > 0.0 && a.is_finite() && b > 0.0 && b.is_finite()) {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::InvalidValue,
                detail: format!(
                    "triangle side lengths must be positive and finite, got {a} and {b}"
                ),
            });
        }
        Ok(Self {
            min: (a - b).abs(),
            max: a + b,
        })
    }

    /// Is `candidate` a feasible length for the third side? Strict on
    /// both ends: exactly `min` or `max` collapses the triangle to a
    /// straight line (zero area), which is not a valid triangle.
    pub fn is_feasible(&self, candidate: f64) -> bool {
        candidate.is_finite() && candidate > self.min && candidate < self.max
    }
}

/// Convenience wrapper: is `c` a feasible third side given fixed sides
/// `a` and `b`? See [`TriangleSideRange`].
pub fn triangle_third_side_is_feasible(a: f64, b: f64, c: f64) -> DomainResult<bool> {
    Ok(TriangleSideRange::for_two_fixed_sides(a, b)?.is_feasible(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_30_50_300_example_from_mcp_article_27_is_infeasible() {
        // "300 cannot form this triangle while the other two sides remain
        // 30 and 50" -- the exact worked example Article 27 uses to
        // motivate humanized consistency-engine feedback.
        assert!(!triangle_third_side_is_feasible(30.0, 50.0, 300.0).unwrap());
    }

    #[test]
    fn the_30_50_range_is_exactly_20_to_80() {
        let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
        assert_eq!(range.min, 20.0);
        assert_eq!(range.max, 80.0);
    }

    #[test]
    fn a_zero_or_negative_or_non_finite_side_length_is_rejected() {
        // Task 227's mutation-testing pass (Phase 31) found this exact
        // gap: `non_positive_or_non_finite_side_lengths_are_rejected`
        // (above) only ever varies the *first* parameter `a`, always
        // with a valid `b`. A mutant weakening the second parameter's
        // own guard (`b > 0.0` -> `b >= 0.0`, silently accepting a
        // zero-length second side whenever `a` is valid) survived every
        // existing test as a result. Symmetric coverage for `b` closes
        // that gap; `a`'s negative case is included too so both
        // parameters are proven symmetric, not just individually
        // guarded in different ways.
        assert!(TriangleSideRange::for_two_fixed_sides(0.0, 50.0).is_err());
        assert!(TriangleSideRange::for_two_fixed_sides(30.0, 0.0).is_err());
        assert!(TriangleSideRange::for_two_fixed_sides(-30.0, 50.0).is_err());
        assert!(TriangleSideRange::for_two_fixed_sides(30.0, f64::NAN).is_err());
        assert!(TriangleSideRange::for_two_fixed_sides(30.0, f64::INFINITY).is_err());
    }

    #[test]
    fn boundary_values_are_infeasible_not_feasible() {
        // Task 098: exact boundary-value regression coverage. At exactly
        // |a-b| or a+b the "triangle" collapses to a straight line.
        let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
        assert!(
            !range.is_feasible(20.0),
            "lower boundary must be infeasible (degenerate)"
        );
        assert!(
            !range.is_feasible(80.0),
            "upper boundary must be infeasible (degenerate)"
        );
    }

    #[test]
    fn values_just_inside_the_boundary_are_feasible() {
        let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
        assert!(range.is_feasible(20.0001));
        assert!(range.is_feasible(79.9999));
    }

    #[test]
    fn a_side_length_equal_to_one_of_the_fixed_sides_is_ordinarily_feasible() {
        assert!(triangle_third_side_is_feasible(30.0, 50.0, 40.0).unwrap());
    }

    #[test]
    fn equal_fixed_sides_have_a_range_starting_at_zero() {
        // a == b: the degenerate lower bound collapses to zero (any
        // positive third side shorter than a+b is geometrically valid
        // down to, but not including, zero).
        let range = TriangleSideRange::for_two_fixed_sides(10.0, 10.0).unwrap();
        assert_eq!(range.min, 0.0);
        assert_eq!(range.max, 20.0);
    }

    #[test]
    fn non_positive_or_non_finite_side_lengths_are_rejected() {
        for (a, b) in [
            (0.0, 5.0),
            (-3.0, 5.0),
            (f64::NAN, 5.0),
            (f64::INFINITY, 5.0),
        ] {
            let err = TriangleSideRange::for_two_fixed_sides(a, b).unwrap_err();
            assert!(matches!(
                err,
                DomainError::Dimension {
                    kind: DimensionErrorKind::InvalidValue,
                    ..
                }
            ));
        }
    }

    #[test]
    fn a_non_finite_candidate_is_never_feasible() {
        let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
        assert!(!range.is_feasible(f64::NAN));
        assert!(!range.is_feasible(f64::INFINITY));
    }
}
