//! Recognition confidence.
//!
//! Execution 01, Phase 06. Authority: MCP Article 96 "Recognition
//! Confidence".
//!
//! A single clamped `[0, 1]` type shared by every candidate fitter, so
//! "confidence" always means the same thing and is always comparable
//! across primitive kinds when ranking (Task 044).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(f64);

impl Confidence {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub const ZERO: Confidence = Confidence(0.0);
    pub const ONE: Confidence = Confidence(1.0);
}

/// Map a nonnegative fit residual to a confidence value: `0` residual maps
/// to confidence `1.0`, and confidence decays smoothly to `0.0` as the
/// residual grows relative to `scale` (a characteristic size of the input,
/// e.g. the stroke's bounding-box diagonal, so the same absolute residual
/// is judged more harshly on a small stroke than a large one).
pub fn confidence_from_residual(residual: f64, scale: f64) -> Confidence {
    if scale <= 0.0 {
        return Confidence::ZERO;
    }
    let normalized = residual / scale;
    Confidence::new((1.0 - normalized).max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_clamps_out_of_range_inputs() {
        assert_eq!(Confidence::new(-1.0).get(), 0.0);
        assert_eq!(Confidence::new(2.0).get(), 1.0);
        assert_eq!(Confidence::new(0.5).get(), 0.5);
    }

    #[test]
    fn zero_residual_is_full_confidence() {
        assert_eq!(confidence_from_residual(0.0, 10.0), Confidence::ONE);
    }

    #[test]
    fn residual_equal_to_scale_is_zero_confidence() {
        assert_eq!(confidence_from_residual(10.0, 10.0), Confidence::ZERO);
    }

    #[test]
    fn residual_beyond_scale_stays_clamped_at_zero() {
        assert_eq!(confidence_from_residual(1000.0, 10.0), Confidence::ZERO);
    }

    #[test]
    fn nonpositive_scale_is_zero_confidence_not_a_division_panic() {
        assert_eq!(confidence_from_residual(1.0, 0.0), Confidence::ZERO);
    }
}
