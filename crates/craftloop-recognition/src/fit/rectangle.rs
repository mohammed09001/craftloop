//! Rectangle/quadrilateral candidate recognition.
//!
//! Execution 01, Phase 06, Task 043. Authority: Engine Contract 04; MCP
//! Article 43 (via the general "recognize conservatively" principle).
//!
//! Forbidden shortcut this module exists to close: forcing an arbitrary
//! closed quadrilateral into a rectangle. This fitter only proposes a
//! rectangle candidate when the stroke is genuinely close to its own
//! axis-aligned bounding box outline; anything else returns `None` rather
//! than a low-confidence guess dressed up as a rectangle.
//!
//! Scope limitation, stated rather than hidden: this only recognizes
//! **axis-aligned** rectangles. Rotated-rectangle recognition would need
//! genuine corner detection (four dominant direction changes), which has no
//! Version 1 consumer yet and is not built speculatively here.

use craftloop_geometry::{Bounds2, Point2, RelationalRectangle};

use crate::confidence::{confidence_from_residual, Confidence};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleCandidate {
    pub rectangle: RelationalRectangle,
    /// Root-mean-square distance from each sample to the nearest edge of
    /// the candidate's bounding box.
    pub residual: f64,
    pub confidence: Confidence,
}

const MIN_CLOSURE_RATIO: f64 = 0.2; // start-to-end gap vs. bbox diagonal
const MIN_BOX_SIZE: f64 = 1e-6;

fn distance_to_bounds_boundary(point: Point2, bounds: Bounds2) -> f64 {
    let dx = (point.x - bounds.min.x).min(bounds.max.x - point.x).abs();
    let dy = (point.y - bounds.min.y).min(bounds.max.y - point.y).abs();
    // The point is closest to whichever edge (vertical or horizontal) it is
    // nearest to, not the corner: take the smaller of the two axis
    // distances, but only after clamping to non-negative (a point outside
    // the box on one axis still has a well-defined distance to that edge).
    dx.min(dy)
}

/// Propose a rectangle candidate for a *closed* stroke, or `None` if the
/// stroke is not closed, is degenerate, or does not hug its own bounding
/// box closely enough to be a conservative rectangle read.
pub fn fit_rectangle(points: &[Point2]) -> Option<RectangleCandidate> {
    if points.len() < 8 {
        return None; // need enough samples to trust a boundary-hugging judgment
    }

    let bounds = Bounds2::from_points(points)?;
    let diagonal = bounds.width().hypot(bounds.height());
    if bounds.width() < MIN_BOX_SIZE || bounds.height() < MIN_BOX_SIZE || diagonal < MIN_BOX_SIZE {
        return None;
    }

    let closure_gap = points.first()?.distance_to(*points.last()?);
    if closure_gap / diagonal > MIN_CLOSURE_RATIO {
        return None; // not a closed stroke
    }

    let residual_sq_sum: f64 = points
        .iter()
        .map(|p| distance_to_bounds_boundary(*p, bounds).powi(2))
        .sum();
    let residual = (residual_sq_sum / points.len() as f64).sqrt();

    let confidence = confidence_from_residual(residual, diagonal * 0.1);
    // Conservative: do not propose a candidate at all below a minimum
    // confidence, rather than returning a near-zero-confidence "rectangle"
    // that a caller might still act on.
    if confidence.get() < 0.3 {
        return None;
    }

    let rectangle = RelationalRectangle::from_axis_aligned(bounds.min, bounds.max).ok()?;

    Some(RectangleCandidate {
        rectangle,
        residual,
        confidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rectangle_outline(min: Point2, max: Point2, per_side: usize) -> Vec<Point2> {
        let corners = [
            min,
            Point2::new(max.x, min.y),
            max,
            Point2::new(min.x, max.y),
        ];
        let mut points = Vec::new();
        for i in 0..4 {
            let a = corners[i];
            let b = corners[(i + 1) % 4];
            for step in 0..per_side {
                let t = step as f64 / per_side as f64;
                points.push(a.lerp(b, t));
            }
        }
        points.push(min); // close the loop back to the start
        points
    }

    #[test]
    fn a_clean_closed_rectangle_outline_is_recognized() {
        let points = rectangle_outline(Point2::new(0.0, 0.0), Point2::new(20.0, 10.0), 6);
        let candidate = fit_rectangle(&points).unwrap();
        assert!(candidate.residual < 0.5);
        assert!(candidate.confidence.get() > 0.9);
    }

    #[test]
    fn an_open_stroke_that_never_closes_is_not_a_rectangle_candidate() {
        // Same outline but stop partway around -- never returns near the
        // start, so this must not be forced into a rectangle.
        let mut points = rectangle_outline(Point2::new(0.0, 0.0), Point2::new(20.0, 10.0), 6);
        points.truncate(points.len() / 2);
        assert!(fit_rectangle(&points).is_none());
    }

    #[test]
    fn a_rough_closed_scribble_is_not_forced_into_a_rectangle() {
        // A closed loop that is nowhere near its own bounding box outline
        // (a rough blob), unlike the arbitrary-quadrilateral case the old
        // aspect-ratio heuristic would have accepted.
        let mut points = Vec::new();
        for i in 0..24 {
            let t = i as f64 / 24.0 * std::f64::consts::TAU;
            let wobble = 3.0 * (t * 5.0).sin();
            points.push(Point2::new(
                10.0 + (10.0 + wobble) * t.cos(),
                10.0 + (10.0 + wobble) * t.sin(),
            ));
        }
        points.push(points[0]);
        assert!(fit_rectangle(&points).is_none());
    }

    #[test]
    fn too_few_points_is_not_a_candidate() {
        let points = vec![Point2::ORIGIN, Point2::new(1.0, 0.0), Point2::new(1.0, 1.0)];
        assert!(fit_rectangle(&points).is_none());
    }

    #[test]
    fn a_degenerate_flat_box_is_not_a_candidate() {
        let points = rectangle_outline(Point2::new(0.0, 0.0), Point2::new(20.0, 0.0), 4);
        assert!(fit_rectangle(&points).is_none());
    }
}
