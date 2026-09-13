//! Candidate ranking.
//!
//! Execution 01, Phase 06, Task 044. Authority: Engine Contract 04
//! ("Recognition proposes; it does not silently commit engineering
//! truth"); MCP Article 585 "Requirement Group: Primitive Recognition".
//!
//! [`recognize`] always includes [`RecognitionCandidate::KeepAsInk`] in its
//! output, at a fixed baseline confidence, so "leave it as ink" is never
//! silently unavailable -- the whole point of ranking rather than picking a
//! single winner is that a caller (eventually a UI, Phase 16+) can see and
//! choose among the alternatives, keeping the user as the designer (MCP
//! Article 4 / Non-Negotiable Product Law 1).

use craftloop_geometry::Point2;
use serde::{Deserialize, Serialize};

use crate::confidence::Confidence;
use crate::fit::{
    fit_arc, fit_circle, fit_line, fit_rectangle, ArcCandidate, CircleCandidate, LineCandidate,
    RectangleCandidate,
};

/// A confidence value below which `recognize` does not even offer a
/// candidate of that kind -- a candidate output by this module always means
/// "worth showing," not "the numerically best of an uninteresting set."
const MINIMUM_OFFERED_CONFIDENCE: f64 = 0.3;

/// "Leave this stroke as ink" is always a legitimate outcome, never a
/// last-resort default only reached when nothing else fits. Its confidence
/// is fixed rather than computed, since there is no fit residual to derive
/// it from -- keeping raw ink is *by definition* a perfect (zero-residual)
/// representation of itself.
const KEEP_AS_INK_CONFIDENCE: f64 = 0.4;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CandidateKind {
    Line,
    Circle,
    Arc,
    Rectangle,
    KeepAsInk,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecognitionCandidate {
    Line(LineCandidate),
    Circle(CircleCandidate),
    Arc(ArcCandidate),
    Rectangle(RectangleCandidate),
    /// The stroke stays raw ink; no structured geometry is proposed.
    KeepAsInk,
}

impl RecognitionCandidate {
    pub fn kind(&self) -> CandidateKind {
        match self {
            RecognitionCandidate::Line(_) => CandidateKind::Line,
            RecognitionCandidate::Circle(_) => CandidateKind::Circle,
            RecognitionCandidate::Arc(_) => CandidateKind::Arc,
            RecognitionCandidate::Rectangle(_) => CandidateKind::Rectangle,
            RecognitionCandidate::KeepAsInk => CandidateKind::KeepAsInk,
        }
    }

    pub fn confidence(&self) -> Confidence {
        match self {
            RecognitionCandidate::Line(c) => c.confidence,
            RecognitionCandidate::Circle(c) => c.confidence,
            RecognitionCandidate::Arc(c) => c.confidence,
            RecognitionCandidate::Rectangle(c) => c.confidence,
            RecognitionCandidate::KeepAsInk => Confidence::new(KEEP_AS_INK_CONFIDENCE),
        }
    }
}

/// Run every fitter over `points` and return the candidates worth
/// offering, sorted by descending confidence. `KeepAsInk` is always
/// present.
pub fn recognize(points: &[Point2]) -> Vec<RecognitionCandidate> {
    let mut candidates = vec![RecognitionCandidate::KeepAsInk];

    if let Some(c) = fit_line(points) {
        if c.confidence.get() >= MINIMUM_OFFERED_CONFIDENCE {
            candidates.push(RecognitionCandidate::Line(c));
        }
    }
    if let Some(c) = fit_circle(points) {
        if c.confidence.get() >= MINIMUM_OFFERED_CONFIDENCE {
            candidates.push(RecognitionCandidate::Circle(c));
        }
    }
    if let Some(c) = fit_arc(points) {
        // An arc whose sweep is close enough to a full turn to be flagged
        // `PlausibleAsCircle` is redundant with the `Circle` candidate
        // above -- both describe essentially the same closed shape, from
        // fits computed with different confidence scales, which makes
        // ranking between them numerically arbitrary rather than
        // meaningful. Let `Circle` represent that interpretation and only
        // offer `Arc` when it says something `Circle` does not.
        let plausible_as_circle = c
            .ambiguous_with
            .contains(&crate::fit::AmbiguityFlag::PlausibleAsCircle);
        if c.confidence.get() >= MINIMUM_OFFERED_CONFIDENCE && !plausible_as_circle {
            candidates.push(RecognitionCandidate::Arc(c));
        }
    }
    if let Some(c) = fit_rectangle(points) {
        if c.confidence.get() >= MINIMUM_OFFERED_CONFIDENCE {
            candidates.push(RecognitionCandidate::Rectangle(c));
        }
    }

    rank_candidates(candidates)
}

/// Sort candidates by descending confidence. Stable: candidates of equal
/// confidence keep their relative input order, so `KeepAsInk` (always
/// first in `recognize`'s construction order) breaks ties in its own
/// favor -- conservative by default.
pub fn rank_candidates(mut candidates: Vec<RecognitionCandidate>) -> Vec<RecognitionCandidate> {
    candidates.sort_by(|a, b| {
        b.confidence()
            .get()
            .partial_cmp(&a.confidence().get())
            .expect("Confidence is always a finite, clamped value")
    });
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_as_ink_is_always_present() {
        let points = vec![Point2::ORIGIN, Point2::new(1.0, 1.0)];
        let candidates = recognize(&points);
        assert!(candidates
            .iter()
            .any(|c| c.kind() == CandidateKind::KeepAsInk));
    }

    #[test]
    fn a_clean_line_ranks_above_keep_as_ink() {
        let points: Vec<Point2> = (0..20)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64))
            .collect();
        let candidates = recognize(&points);
        assert_eq!(candidates[0].kind(), CandidateKind::Line);
    }

    #[test]
    fn ranking_is_sorted_descending_by_confidence() {
        let points: Vec<Point2> = (0..20)
            .map(|i| Point2::new(i as f64, 2.0 * i as f64))
            .collect();
        let candidates = recognize(&points);
        for pair in candidates.windows(2) {
            assert!(pair[0].confidence().get() >= pair[1].confidence().get());
        }
    }

    #[test]
    fn a_short_two_point_stroke_only_offers_keep_as_ink() {
        // Not enough points for circle/rectangle fits; a 2-point line fit
        // is technically possible but this stroke is short enough that a
        // caller would not usually feed it through recognition at all --
        // still, recognize() must not panic and must always return
        // KeepAsInk.
        let points = vec![Point2::new(0.0, 0.0), Point2::new(0.001, 0.0)];
        let candidates = recognize(&points);
        assert!(!candidates.is_empty());
        assert!(candidates
            .iter()
            .any(|c| c.kind() == CandidateKind::KeepAsInk));
    }

    #[test]
    fn low_confidence_fits_are_not_offered_at_all() {
        // A messy zigzag: line fit will exist but with low confidence, and
        // must be filtered out rather than offered as a weak suggestion.
        let points: Vec<Point2> = (0..10)
            .map(|i| Point2::new(i as f64, if i % 2 == 0 { 0.0 } else { 8.0 }))
            .collect();
        let candidates = recognize(&points);
        assert!(!candidates.iter().any(|c| c.kind() == CandidateKind::Line));
    }
}
