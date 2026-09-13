//! Circle candidate fitting.
//!
//! Execution 01, Phase 06, Task 041. Authority: Engine Contract 04; MCP
//! Article 588 "Requirement Group: Structured Circles".
//!
//! Forbidden shortcut this module exists to close: the reference
//! `windows-simulator` heuristic (`sim_core.circle_candidate`) only checks
//! that a closed stroke's bounding box is roughly square. This is a real
//! geometric fit -- the Kåsa algebraic circle fit, a linear least-squares
//! method that minimizes algebraic (not geometric) residual of
//! `x^2 + y^2 = D*x + E*y + F`, which is the standard closed-form approach
//! for fitting a circle to noisy points without iterative optimization.

use craftloop_geometry::Point2;

use crate::confidence::{confidence_from_residual, Confidence};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleCandidate {
    pub center: Point2,
    pub radius: f64,
    /// Root-mean-square of `|distance(point, center) - radius|`.
    pub residual: f64,
    pub confidence: Confidence,
}

/// Solve the 3x3 linear system `a * x = b` via Cramer's rule. Returns
/// `None` if `a` is (near-)singular.
fn solve_3x3(a: [[f64; 3]; 3], b: [f64; 3]) -> Option<[f64; 3]> {
    fn det3(m: [[f64; 3]; 3]) -> f64 {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    let det = det3(a);
    if det.abs() < 1e-12 {
        return None;
    }

    let mut result = [0.0; 3];
    for col in 0..3 {
        let mut replaced = a;
        for row in 0..3 {
            replaced[row][col] = b[row];
        }
        result[col] = det3(replaced) / det;
    }
    Some(result)
}

/// Fit the best-fit circle through `points` via the Kåsa method. Returns
/// `None` for fewer than 3 points or a (near-)degenerate (collinear) point
/// set, for which no circle is well defined.
pub fn fit_circle(points: &[Point2]) -> Option<CircleCandidate> {
    if points.len() < 3 {
        return None;
    }

    let n = points.len() as f64;
    let (mut sx, mut sy, mut sxx, mut syy, mut sxy, mut sxz, mut syz, mut sz) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    for p in points {
        let z = p.x * p.x + p.y * p.y;
        sx += p.x;
        sy += p.y;
        sxx += p.x * p.x;
        syy += p.y * p.y;
        sxy += p.x * p.y;
        sxz += p.x * z;
        syz += p.y * z;
        sz += z;
    }

    let matrix = [[sxx, sxy, sx], [sxy, syy, sy], [sx, sy, n]];
    let rhs = [sxz, syz, sz];
    let [d, e, f] = solve_3x3(matrix, rhs)?;

    let center = Point2::new(d / 2.0, e / 2.0);
    let radius_sq = f + center.x * center.x + center.y * center.y;
    if radius_sq <= 0.0 {
        return None;
    }
    let radius = radius_sq.sqrt();

    let residual_sq_sum: f64 = points
        .iter()
        .map(|p| (p.distance_to(center) - radius).powi(2))
        .sum();
    let residual = (residual_sq_sum / n).sqrt();

    let confidence = confidence_from_residual(residual, radius.max(1e-9) * 0.1);

    Some(CircleCandidate {
        center,
        radius,
        residual,
        confidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    fn circle_points(center: Point2, radius: f64, count: usize) -> Vec<Point2> {
        (0..count)
            .map(|i| {
                let angle = TAU * (i as f64) / (count as f64);
                Point2::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
            })
            .collect()
    }

    #[test]
    fn fewer_than_three_points_has_no_fit() {
        assert!(fit_circle(&[Point2::ORIGIN, Point2::new(1.0, 0.0)]).is_none());
    }

    #[test]
    fn collinear_points_have_no_defined_circle() {
        let points: Vec<Point2> = (0..10)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64))
            .collect();
        assert!(fit_circle(&points).is_none());
    }

    #[test]
    fn a_perfect_circle_fits_with_near_zero_residual_and_high_confidence() {
        let points = circle_points(Point2::new(10.0, -5.0), 7.0, 24);
        let candidate = fit_circle(&points).unwrap();
        assert!((candidate.center.x - 10.0).abs() < 1e-6);
        assert!((candidate.center.y - (-5.0)).abs() < 1e-6);
        assert!((candidate.radius - 7.0).abs() < 1e-6);
        assert!(candidate.residual < 1e-6);
        assert!(candidate.confidence.get() > 0.99);
    }

    #[test]
    fn a_square_shaped_stroke_is_a_poor_circle_fit() {
        // A closed square path -- the kind of shape the old aspect-ratio
        // heuristic would have accepted as "roughly square, call it a
        // circle." A real geometric fit must show a large residual instead.
        let square = vec![
            Point2::new(0.0, 0.0),
            Point2::new(5.0, 0.0),
            Point2::new(10.0, 0.0),
            Point2::new(10.0, 5.0),
            Point2::new(10.0, 10.0),
            Point2::new(5.0, 10.0),
            Point2::new(0.0, 10.0),
            Point2::new(0.0, 5.0),
        ];
        let candidate = fit_circle(&square).unwrap();
        assert!(
            candidate.residual > 1.0,
            "square corners must not fit a circle tightly"
        );
        assert!(candidate.confidence.get() < 0.5);
    }

    #[test]
    fn residual_is_nonnegative() {
        let points = circle_points(Point2::ORIGIN, 3.0, 12);
        let candidate = fit_circle(&points).unwrap();
        assert!(candidate.residual >= 0.0);
    }
}
