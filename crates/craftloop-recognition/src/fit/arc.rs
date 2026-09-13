//! Arc candidate fitting.
//!
//! Execution 01, Phase 06, Task 042. Authority: Engine Contract 04; MCP
//! Article 589 "Requirement Group: Arcs".
//!
//! Builds on the circle fit (Task 041): an arc is a circle plus an angular
//! sweep. Task 042 explicitly requires "explicit ambiguity when a circle or
//! freehand curve is also plausible" -- rather than picking one
//! interpretation silently, [`ArcCandidate::ambiguous_with`] names the
//! competing interpretations so a later ranking/UI layer (Task 044, Phase
//! 16+) can surface them instead of the recognizer guessing on the user's
//! behalf (Article 4: never fabricate engineering certainty).

use std::f64::consts::{PI, TAU};

use craftloop_geometry::{Bounds2, Point2};

use crate::confidence::{confidence_from_residual, Confidence};
use crate::fit::circle::fit_circle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguityFlag {
    /// The swept angle is close enough to a full turn that "closed circle"
    /// is an equally plausible reading of this stroke.
    PlausibleAsCircle,
    /// The fit is not clean enough to rule out "this was never meant to be
    /// circular at all" -- see `fit_arc`'s comment on why fit quality, not
    /// the measured sweep, is what actually signals this.
    PlausibleAsFreehand,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArcCandidate {
    pub center: Point2,
    pub radius: f64,
    pub start_angle: f64,
    /// Signed total angular sweep from the first sample to the last,
    /// accumulated sample-to-sample so a stroke that winds more than once
    /// is not silently wrapped into `[-2*PI, 2*PI]`.
    pub sweep_angle: f64,
    pub residual: f64,
    pub confidence: Confidence,
    pub ambiguous_with: Vec<AmbiguityFlag>,
}

const CIRCLE_AMBIGUITY_SWEEP_FRACTION: f64 = 0.85;
const FREEHAND_AMBIGUITY_CONFIDENCE_CEILING: f64 = 0.6;

/// Fit an arc through `points`. Returns `None` under the same conditions
/// [`fit_circle`] does (fewer than 3 points, or collinear input).
pub fn fit_arc(points: &[Point2]) -> Option<ArcCandidate> {
    let circle = fit_circle(points)?;

    let angles: Vec<f64> = points
        .iter()
        .map(|p| (p.y - circle.center.y).atan2(p.x - circle.center.x))
        .collect();

    let start_angle = angles[0];
    let mut sweep_angle = 0.0;
    for pair in angles.windows(2) {
        let mut delta = pair[1] - pair[0];
        if delta > PI {
            delta -= TAU;
        } else if delta < -PI {
            delta += TAU;
        }
        sweep_angle += delta;
    }

    // A short arc's fitted radius can be extrapolated wildly large by the
    // Kåsa method (a short, near-straight sample of a huge circle fits a
    // small noisy sample of ink almost as well as a small one) -- scaling
    // confidence by that radius would then rate a noisy short scribble as
    // "confident" simply because the scale it is judged against exploded
    // along with the radius. Scaling by the sampled points' own bounding
    // box instead keeps confidence tied to something the caller can see:
    // how tightly the ink itself sits on the fitted arc.
    let bounds =
        Bounds2::from_points(points).expect("fit_circle already confirmed points is nonempty");
    let local_scale = bounds.width().hypot(bounds.height()).max(1e-9);
    let confidence = confidence_from_residual(circle.residual, local_scale * 0.1);

    let sweep_fraction = sweep_angle.abs() / TAU;
    let mut ambiguous_with = Vec::new();
    if sweep_fraction >= CIRCLE_AMBIGUITY_SWEEP_FRACTION {
        ambiguous_with.push(AmbiguityFlag::PlausibleAsCircle);
    }
    // Freehand ambiguity is judged purely on fit quality, not on the
    // measured sweep: noisy ink over a short *intended* arc does not
    // reliably fit with a small *measured* sweep around whatever center
    // the algebraic fit lands on (Kåsa fits are known to be poorly
    // conditioned for near-straight/short-arc input, and can swing to a
    // small, unrelated-looking circle under even modest noise -- with a
    // correspondingly large measured sweep around *that* circle). A low
    // confidence fit is inherently uncertain about being circular at all,
    // regardless of what sweep it happens to report, so that is the
    // signal used here.
    if confidence.get() < FREEHAND_AMBIGUITY_CONFIDENCE_CEILING {
        ambiguous_with.push(AmbiguityFlag::PlausibleAsFreehand);
    }

    Some(ArcCandidate {
        center: circle.center,
        radius: circle.radius,
        start_angle,
        sweep_angle,
        residual: circle.residual,
        confidence,
        ambiguous_with,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arc_points(
        center: Point2,
        radius: f64,
        start_deg: f64,
        sweep_deg: f64,
        count: usize,
        noise: f64,
    ) -> Vec<Point2> {
        (0..count)
            .map(|i| {
                let t = i as f64 / (count - 1) as f64;
                let angle = (start_deg + sweep_deg * t).to_radians();
                // Deterministic pseudo-noise: alternate the radius slightly
                // instead of a true RNG, to keep the fixture reproducible.
                let r = radius + if i % 2 == 0 { noise } else { -noise };
                Point2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect()
    }

    #[test]
    fn a_half_circle_arc_is_unambiguous() {
        let points = arc_points(Point2::ORIGIN, 5.0, 0.0, 180.0, 20, 0.0);
        let candidate = fit_arc(&points).unwrap();
        assert!((candidate.sweep_angle.abs().to_degrees() - 180.0).abs() < 1.0);
        assert!(candidate.ambiguous_with.is_empty());
    }

    #[test]
    fn a_near_full_sweep_is_flagged_ambiguous_with_circle() {
        let points = arc_points(Point2::ORIGIN, 5.0, 0.0, 350.0, 40, 0.0);
        let candidate = fit_arc(&points).unwrap();
        assert!(candidate
            .ambiguous_with
            .contains(&AmbiguityFlag::PlausibleAsCircle));
    }

    #[test]
    fn a_short_noisy_sweep_is_flagged_ambiguous_with_freehand() {
        // Small 2D jitter (not purely radial -- see module docs on why a
        // short arc's algebraic fit is sensitive to noise direction)
        // applied to an otherwise clean 10-degree arc of radius 5.
        let dx = [
            0.05, -0.1, 0.08, -0.03, 0.12, -0.07, 0.02, -0.11, 0.09, -0.04_f64,
        ];
        let dy = [
            -0.06, 0.09, -0.02, 0.11, -0.08, 0.04, -0.1, 0.03, -0.05, 0.1_f64,
        ];
        let points: Vec<Point2> = (0..10)
            .map(|i| {
                let t = i as f64 / 9.0;
                let angle = (10.0 * t).to_radians();
                Point2::new(5.0 * angle.cos() + dx[i], 5.0 * angle.sin() + dy[i])
            })
            .collect();
        let candidate = fit_arc(&points).unwrap();
        assert!(candidate.confidence.get() < FREEHAND_AMBIGUITY_CONFIDENCE_CEILING);
        assert!(candidate
            .ambiguous_with
            .contains(&AmbiguityFlag::PlausibleAsFreehand));
    }

    #[test]
    fn a_short_but_clean_sweep_is_not_flagged_ambiguous_with_freehand() {
        let points = arc_points(Point2::ORIGIN, 5.0, 0.0, 20.0, 12, 0.0);
        let candidate = fit_arc(&points).unwrap();
        assert!(candidate.confidence.get() > FREEHAND_AMBIGUITY_CONFIDENCE_CEILING);
        assert!(!candidate
            .ambiguous_with
            .contains(&AmbiguityFlag::PlausibleAsFreehand));
    }

    #[test]
    fn sweep_direction_sign_matches_traversal_direction() {
        let ccw = arc_points(Point2::ORIGIN, 5.0, 0.0, 90.0, 10, 0.0);
        let cw = arc_points(Point2::ORIGIN, 5.0, 0.0, -90.0, 10, 0.0);
        assert!(fit_arc(&ccw).unwrap().sweep_angle > 0.0);
        assert!(fit_arc(&cw).unwrap().sweep_angle < 0.0);
    }

    #[test]
    fn too_few_points_has_no_fit() {
        assert!(fit_arc(&[Point2::ORIGIN, Point2::new(1.0, 0.0)]).is_none());
    }
}
