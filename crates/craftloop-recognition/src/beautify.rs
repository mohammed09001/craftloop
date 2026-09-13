//! Beautification transforms.
//!
//! Execution 01, Phase 06, Task 045. Authority: Engine Contract 05
//! (Beautification: "Accepted candidate refinement and morph targets";
//! "Minimize displacement and preserve provenance"); MCP Article 19
//! "Beautification and Ink Morphing".
//!
//! Beautification only runs on a candidate the caller has already decided
//! to accept -- it never runs implicitly inside `recognize()`. Converting
//! an accepted [`RecognitionCandidate`] into a clean `craftloop-geometry`
//! primitive is close to free: the fitted parameters (least-squares line
//! endpoints, Kåsa circle center/radius, ...) already minimize displacement
//! from the raw ink in an L2 sense, so "clean" and "close to what was drawn"
//! are the same computation, not two competing goals to trade off.

use craftloop_geometry::{Arc2, Circle2, RelationalRectangle, Segment2};

use crate::candidate::RecognitionCandidate;

#[derive(Debug, Clone, PartialEq)]
pub enum BeautifiedPrimitive {
    Line(Segment2),
    Circle(Circle2),
    Arc(Arc2),
    Rectangle(RelationalRectangle),
}

/// A beautified primitive paired with the residual (displacement) of the
/// candidate it was built from -- how far, on average, the raw ink sat
/// from this clean shape, so callers (and tests) can verify "minimizes
/// displacement" is more than a claim.
#[derive(Debug, Clone, PartialEq)]
pub struct Beautified {
    pub primitive: BeautifiedPrimitive,
    pub displacement: f64,
}

/// Convert an accepted candidate into a clean geometric primitive.
/// `RecognitionCandidate::KeepAsInk` has nothing to beautify into, so this
/// returns `None` for it; a candidate whose fitted parameters happen to be
/// geometrically invalid (e.g. a circle radius that rounds to zero, which
/// the fitter itself already guards against) also yields `None` rather than
/// panicking.
pub fn beautify(candidate: &RecognitionCandidate) -> Option<Beautified> {
    match candidate {
        RecognitionCandidate::Line(line) => Some(Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(line.start, line.end)),
            displacement: line.residual,
        }),
        RecognitionCandidate::Circle(circle) => {
            let geometry = Circle2::new(circle.center, circle.radius).ok()?;
            Some(Beautified {
                primitive: BeautifiedPrimitive::Circle(geometry),
                displacement: circle.residual,
            })
        }
        RecognitionCandidate::Arc(arc) => {
            let geometry =
                Arc2::new(arc.center, arc.radius, arc.start_angle, arc.sweep_angle).ok()?;
            Some(Beautified {
                primitive: BeautifiedPrimitive::Arc(geometry),
                displacement: arc.residual,
            })
        }
        RecognitionCandidate::Rectangle(rect) => Some(Beautified {
            primitive: BeautifiedPrimitive::Rectangle(rect.rectangle),
            displacement: rect.residual,
        }),
        RecognitionCandidate::KeepAsInk => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fit::{fit_circle, fit_line};
    use craftloop_geometry::Point2;

    #[test]
    fn keep_as_ink_has_nothing_to_beautify() {
        assert!(beautify(&RecognitionCandidate::KeepAsInk).is_none());
    }

    #[test]
    fn a_line_candidate_beautifies_to_a_segment_using_the_fitted_endpoints() {
        let points: Vec<Point2> = (0..10)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64))
            .collect();
        let line = fit_line(&points).unwrap();
        let candidate = RecognitionCandidate::Line(line);
        let beautified = beautify(&candidate).unwrap();
        match beautified.primitive {
            BeautifiedPrimitive::Line(segment) => {
                assert!((segment.a.distance_to(line.start)) < 1e-9);
                assert!((segment.b.distance_to(line.end)) < 1e-9);
            }
            other => panic!("expected a line, got {other:?}"),
        }
        assert_eq!(beautified.displacement, line.residual);
    }

    #[test]
    fn a_circle_candidate_beautifies_to_a_valid_circle2() {
        let points: Vec<Point2> = (0..16)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / 16.0;
                Point2::new(5.0 * a.cos(), 5.0 * a.sin())
            })
            .collect();
        let circle = fit_circle(&points).unwrap();
        let beautified = beautify(&RecognitionCandidate::Circle(circle)).unwrap();
        match beautified.primitive {
            BeautifiedPrimitive::Circle(c) => assert!((c.radius - 5.0).abs() < 1e-6),
            other => panic!("expected a circle, got {other:?}"),
        }
    }

    #[test]
    fn beautified_displacement_equals_the_original_candidates_residual() {
        let points: Vec<Point2> = (0..10).map(|i| Point2::new(i as f64, i as f64)).collect();
        let line = fit_line(&points).unwrap();
        let residual = line.residual;
        let beautified = beautify(&RecognitionCandidate::Line(line)).unwrap();
        assert_eq!(beautified.displacement, residual);
    }
}
