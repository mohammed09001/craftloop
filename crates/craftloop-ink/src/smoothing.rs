//! Smoothing pipeline: raw path vs. smoothed presentation path.
//!
//! Execution 01, Phase 05, Task 036. Authority: Engine Contract 02; MCP
//! Article 243 "Detailed Specification of Stroke Smoothing".
//!
//! Three representations of a stroke's shape are kept distinct, per Task
//! 036's own wording:
//!
//! 1. **Raw path** -- `Stroke::samples()` (Task 034). Never modified.
//! 2. **Smoothed presentation path** -- [`smoothed_positions`], produced
//!    here. A rendering-quality curve derived from the raw path; still just
//!    points, not committed geometry.
//! 3. **Structured geometry** -- not this crate. Produced by recognition
//!    (Phase 06) from either the raw or smoothed path, and is the only one
//!    of the three that can become a committed engineering primitive.
//!
//! Conflating (2) and (3) would violate Article 15 ("Raw Ink and
//! Structured Geometry Must Coexist"): a smoothed *presentation* curve is
//! still ink, not a semantic line/circle/arc.

use craftloop_geometry::Point2;

/// Centered moving-average smoothing over `window` samples on each side
/// (so an interior point averages `2 * window + 1` samples). Endpoints are
/// preserved exactly and the window narrows near them, so the smoothed
/// path has the same length and the same start/end points as the input --
/// required so downstream code (e.g. bounds, hit-testing) can keep treating
/// "first/last point" as meaningful.
///
/// `window = 0` returns the input unchanged.
pub fn smoothed_positions(positions: &[Point2], window: usize) -> Vec<Point2> {
    if window == 0 || positions.len() <= 2 {
        return positions.to_vec();
    }

    let last = positions.len() - 1;
    (0..positions.len())
        .map(|i| {
            // Endpoints are preserved exactly, not just approximately: an
            // asymmetric window at the boundary would otherwise pull them
            // toward the interior, which is exactly what the doc comment
            // above promises not to happen.
            if i == 0 || i == last {
                return positions[i];
            }
            // Shrink the window *symmetrically* near the boundaries rather
            // than clamping each side independently: an asymmetric window
            // averages toward whichever side has more points in range,
            // which would visibly bow even a perfectly straight input path
            // near its ends.
            let radius = window.min(i).min(last - i);
            let lo = i - radius;
            let hi = i + radius;
            let count = (hi - lo + 1) as f64;
            let (sum_x, sum_y) = positions[lo..=hi]
                .iter()
                .fold((0.0, 0.0), |(sx, sy), p| (sx + p.x, sy + p.y));
            Point2::new(sum_x / count, sum_y / count)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_window_returns_the_input_unchanged() {
        let path = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 5.0),
            Point2::new(2.0, 0.0),
        ];
        assert_eq!(smoothed_positions(&path, 0), path);
    }

    #[test]
    fn smoothing_preserves_endpoints_exactly() {
        let path = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 10.0),
            Point2::new(2.0, -10.0),
            Point2::new(3.0, 10.0),
            Point2::new(4.0, 0.0),
        ];
        let smoothed = smoothed_positions(&path, 1);
        assert_eq!(smoothed.first(), path.first());
        assert_eq!(smoothed.last(), path.last());
    }

    #[test]
    fn smoothing_reduces_a_single_spike() {
        let path = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 100.0), // spike
            Point2::new(3.0, 0.0),
            Point2::new(4.0, 0.0),
        ];
        let smoothed = smoothed_positions(&path, 1);
        assert!(
            smoothed[2].y < 100.0,
            "the spike must be attenuated toward its neighbors"
        );
        assert!(smoothed[2].y > 0.0);
    }

    #[test]
    fn smoothing_preserves_the_number_of_points() {
        let path: Vec<Point2> = (0..20)
            .map(|i| Point2::new(i as f64, (i as f64).sin()))
            .collect();
        let smoothed = smoothed_positions(&path, 2);
        assert_eq!(smoothed.len(), path.len());
    }

    #[test]
    fn a_straight_line_is_unchanged_by_smoothing() {
        let path: Vec<Point2> = (0..10)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64))
            .collect();
        let smoothed = smoothed_positions(&path, 2);
        for (raw, smooth) in path.iter().zip(smoothed.iter()) {
            assert!((raw.x - smooth.x).abs() < 1e-9);
            assert!((raw.y - smooth.y).abs() < 1e-9);
        }
    }

    #[test]
    fn very_short_paths_are_returned_unchanged() {
        let path = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)];
        assert_eq!(smoothed_positions(&path, 3), path);
    }
}
