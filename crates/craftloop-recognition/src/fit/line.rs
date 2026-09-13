//! Line candidate fitting.
//!
//! Execution 01, Phase 06, Task 040. Authority: Engine Contract 04
//! (Recognition: "Candidate fitting, confidence, keep-as-ink, rejection
//! state"; "Recognition proposes; it does not silently commit engineering
//! truth"); MCP Article 587 "Requirement Group: Structured Lines".
//!
//! Uses total least squares (orthogonal regression via the 2x2 covariance
//! matrix's principal eigenvector), not a vertical-distance least-squares
//! fit: an ordinary y-on-x regression is undefined/degenerate for a
//! near-vertical stroke, which hand-drawn lines routinely are.

use craftloop_geometry::{Point2, Vector2};

use crate::confidence::{confidence_from_residual, Confidence};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineCandidate {
    pub start: Point2,
    pub end: Point2,
    /// Root-mean-square perpendicular distance from the input points to
    /// the fitted line.
    pub residual: f64,
    pub confidence: Confidence,
}

impl LineCandidate {
    pub fn length(&self) -> f64 {
        self.start.distance_to(self.end)
    }
}

/// Fit the best-fit (orthogonal-regression) line through `points`. Returns
/// `None` if there are fewer than 2 points, or all points coincide (no
/// defined direction).
pub fn fit_line(points: &[Point2]) -> Option<LineCandidate> {
    if points.len() < 2 {
        return None;
    }

    let n = points.len() as f64;
    let mean_x = points.iter().map(|p| p.x).sum::<f64>() / n;
    let mean_y = points.iter().map(|p| p.y).sum::<f64>() / n;

    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    for p in points {
        let dx = p.x - mean_x;
        let dy = p.y - mean_y;
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }

    if sxx.abs() < 1e-18 && syy.abs() < 1e-18 {
        return None; // all points coincide
    }

    // Principal axis angle of the 2x2 covariance matrix [[sxx,sxy],[sxy,syy]].
    let theta = 0.5 * (2.0 * sxy).atan2(sxx - syy);
    let direction = Vector2::new(theta.cos(), theta.sin());

    // Minor eigenvalue (perpendicular spread) via the standard 2x2
    // symmetric-matrix eigenvalue formula.
    let trace = sxx + syy;
    let diff_half = (sxx - syy) / 2.0;
    let radius = (diff_half * diff_half + sxy * sxy).sqrt();
    let lambda_min = (trace / 2.0 - radius).max(0.0);
    let residual = (lambda_min / n).sqrt();

    let mean = Point2::new(mean_x, mean_y);
    let projections: Vec<f64> = points
        .iter()
        .map(|p| mean.vector_to(*p).dot(direction))
        .collect();
    let t_min = projections.iter().cloned().fold(f64::INFINITY, f64::min);
    let t_max = projections
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let start = mean.translated(direction.scaled(t_min));
    let end = mean.translated(direction.scaled(t_max));

    let scale = (t_max - t_min).max(1e-9);
    let confidence = confidence_from_residual(residual, scale * 0.1);

    Some(LineCandidate {
        start,
        end,
        residual,
        confidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fewer_than_two_points_has_no_fit() {
        assert!(fit_line(&[Point2::ORIGIN]).is_none());
        assert!(fit_line(&[]).is_none());
    }

    #[test]
    fn coincident_points_have_no_defined_direction() {
        let points = vec![Point2::new(1.0, 1.0); 5];
        assert!(fit_line(&points).is_none());
    }

    #[test]
    fn a_perfectly_straight_line_fits_with_near_zero_residual_and_high_confidence() {
        let points: Vec<Point2> = (0..20)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64 + 3.0))
            .collect();
        let candidate = fit_line(&points).unwrap();
        assert!(candidate.residual < 1e-9);
        assert!(candidate.confidence.get() > 0.99);
    }

    #[test]
    fn a_vertical_line_is_fit_correctly_unlike_ordinary_y_on_x_regression() {
        let points: Vec<Point2> = (0..10).map(|i| Point2::new(5.0, i as f64)).collect();
        let candidate = fit_line(&points).unwrap();
        assert!(candidate.residual < 1e-9);
        // Endpoints must share the vertical line's x coordinate.
        assert!((candidate.start.x - 5.0).abs() < 1e-9);
        assert!((candidate.end.x - 5.0).abs() < 1e-9);
    }

    #[test]
    fn a_zigzag_path_has_high_residual_and_low_confidence() {
        let points: Vec<Point2> = (0..20)
            .map(|i| Point2::new(i as f64, if i % 2 == 0 { 0.0 } else { 10.0 }))
            .collect();
        let candidate = fit_line(&points).unwrap();
        assert!(candidate.residual > 1.0);
        assert!(candidate.confidence.get() < 0.5);
    }

    #[test]
    fn fitted_endpoints_span_the_extent_of_the_input_along_the_fitted_direction() {
        let points: Vec<Point2> = (0..10).map(|i| Point2::new(i as f64, i as f64)).collect();
        let candidate = fit_line(&points).unwrap();
        assert!(candidate.length() > 12.0); // diag of a 9x9 square ~= 12.7
    }
}
