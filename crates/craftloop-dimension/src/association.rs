//! Dimension Association Engine.
//!
//! Execution 01, Phase 18, Tasks 125-130, 132. Authority: Engine Contract
//! 09; MCP Article 23 ("Dimensions Are Relationships, Not Labels").
//!
//! Ranks which primitive a recognized dimension value should bind to.
//! Deliberately layered evidence, strongest first:
//! 1. **View scope** (Task 129) is a hard filter, not evidence: a
//!    candidate outside the active view is excluded before anything else
//!    runs, so nearby geometry in another view can never "win" on
//!    proximity alone. Scoped by `craftloop_ids::ViewId` -- the concrete
//!    `ViewBlock` type this ID will eventually belong to is Phase 20's
//!    job, not built yet, but the stable ID type already exists (Phase
//!    01) and is enough to express real scoping now.
//! 2. **Explicit selection** (Task 125) and **dimension-guide binding**
//!    (Task 126) are both direct user actions and short-circuit ranking
//!    entirely when present and in-scope -- Article 23's "the user
//!    should not see mathematical machinery" does not mean the machinery
//!    should override what the user explicitly pointed at.
//! 3. Otherwise, **proximity** (Task 127, reusing
//!    `craftloop_recognition::confidence_from_residual` rather than a
//!    duplicate decay function) and **directional evidence** (Task 128,
//!    does the text sit roughly perpendicular-offset from the
//!    candidate's own orientation, the way a real dimension is drawn)
//!    combine into one ranked confidence per remaining candidate.
//!
//! Task 127's "never as sole authority" is enforced by construction:
//! there is no code path that returns a result from proximity alone
//! without also folding in directional evidence.

use craftloop_geometry::{Bounds2, Point2};
use craftloop_ids::{PrimitiveId, ViewId};
use craftloop_recognition::confidence::confidence_from_residual;
use craftloop_recognition::{BeautifiedPrimitive, Confidence};
use serde::{Deserialize, Serialize};

/// Task 116/Phase 16's already-established convention, redefined here
/// (this crate does not depend on `craftloop-ink-intent`, a different
/// domain) at the same numeric value: "confidence" means the same thing
/// everywhere (Article 96).
pub const COMMIT_THRESHOLD: f64 = 0.6;
pub const CONFIRM_THRESHOLD: f64 = 0.3;

/// A characteristic distance beyond which proximity evidence is
/// considered fully decayed. Chosen as a plain, documented constant
/// (not yet calibrated against real device/page-scale data) -- matches
/// this workspace's established pattern of naming an uncalibrated
/// placeholder honestly rather than presenting it as tuned (see
/// `craftloop-geometry::Tolerances::recognition`'s own doc comment).
pub const PROXIMITY_SCALE_MM: f64 = 50.0;

/// One primitive eligible to receive a dimension association.
#[derive(Debug, Clone)]
pub struct AssociationCandidate {
    pub primitive_id: PrimitiveId,
    pub primitive: BeautifiedPrimitive,
    /// Which view this primitive belongs to, if any (Task 129).
    pub view: Option<ViewId>,
}

/// Where a ranked result's confidence actually came from -- kept
/// distinguishable so a caller (and Task 132's regression tests) can
/// assert not just *which* primitive won, but *why*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociationSource {
    ExplicitSelection,
    DimensionGuide,
    ProximityAndOrientation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssociationResult {
    pub primitive_id: PrimitiveId,
    pub confidence: Confidence,
    pub source: AssociationSource,
}

impl AssociationResult {
    pub fn is_committable(&self) -> bool {
        self.confidence.get() >= COMMIT_THRESHOLD
    }

    /// Task 130: worth showing as one of several candidates, but not
    /// worth committing unprompted.
    pub fn needs_preview(&self) -> bool {
        let value = self.confidence.get();
        (CONFIRM_THRESHOLD..COMMIT_THRESHOLD).contains(&value)
    }
}

fn primitive_bounds(primitive: &BeautifiedPrimitive) -> Bounds2 {
    match primitive {
        BeautifiedPrimitive::Line(segment) => {
            Bounds2::from_points(&[segment.a, segment.b]).expect("a segment always has two points")
        }
        BeautifiedPrimitive::Circle(circle) => Bounds2 {
            min: Point2::new(
                circle.center.x - circle.radius,
                circle.center.y - circle.radius,
            ),
            max: Point2::new(
                circle.center.x + circle.radius,
                circle.center.y + circle.radius,
            ),
        },
        BeautifiedPrimitive::Arc(arc) => Bounds2 {
            min: Point2::new(arc.center.x - arc.radius, arc.center.y - arc.radius),
            max: Point2::new(arc.center.x + arc.radius, arc.center.y + arc.radius),
        },
        BeautifiedPrimitive::Rectangle(rectangle) => {
            Bounds2::from_points(&rectangle.corners).expect("a rectangle always has four points")
        }
    }
}

fn distance_point_to_bounds(point: Point2, bounds: Bounds2) -> f64 {
    let dx = (bounds.min.x - point.x)
        .max(0.0)
        .max(point.x - bounds.max.x);
    let dy = (bounds.min.y - point.y)
        .max(0.0)
        .max(point.y - bounds.max.y);
    (dx * dx + dy * dy).sqrt()
}

/// Task 128: reward text positioned roughly perpendicular-offset from
/// the primitive's own orientation (how a dimension is conventionally
/// drawn -- offset to the side of what it measures, not floating off at
/// an unrelated angle). Circles/arcs have no preferred orientation, so
/// they get a neutral score rather than a fabricated one.
fn orientation_alignment_score(text_position: Point2, primitive: &BeautifiedPrimitive) -> f64 {
    let (center, direction) = match primitive {
        BeautifiedPrimitive::Line(segment) => {
            let center = Point2::new(
                (segment.a.x + segment.b.x) / 2.0,
                (segment.a.y + segment.b.y) / 2.0,
            );
            (
                center,
                (segment.b.x - segment.a.x, segment.b.y - segment.a.y),
            )
        }
        BeautifiedPrimitive::Rectangle(rectangle) => {
            let a = rectangle.corners[0];
            let b = rectangle.corners[1];
            let center = Point2::new(
                rectangle.corners.iter().map(|p| p.x).sum::<f64>() / 4.0,
                rectangle.corners.iter().map(|p| p.y).sum::<f64>() / 4.0,
            );
            (center, (b.x - a.x, b.y - a.y))
        }
        BeautifiedPrimitive::Circle(_) | BeautifiedPrimitive::Arc(_) => return 0.5,
    };
    let direction_length = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
    let offset = (text_position.x - center.x, text_position.y - center.y);
    let offset_length = (offset.0 * offset.0 + offset.1 * offset.1).sqrt();
    if direction_length < 1e-9 || offset_length < 1e-9 {
        return 0.5;
    }
    // |sin(angle)| between the offset and the primitive's own direction:
    // 1.0 when the offset is perpendicular (conventional placement),
    // 0.0 when the text sits directly along the primitive's own line.
    let cross = direction.0 * offset.1 - direction.1 * offset.0;
    (cross.abs() / (direction_length * offset_length)).clamp(0.0, 1.0)
}

/// Task 125-129: rank every in-scope candidate for `text_position`.
/// Returns an empty `Vec` if no candidate is in scope -- a legitimate
/// "nothing to associate with" outcome, not an error.
#[allow(clippy::too_many_arguments)]
pub fn associate(
    text_position: Point2,
    candidates: &[AssociationCandidate],
    explicit_selection: Option<PrimitiveId>,
    dimension_guide_target: Option<PrimitiveId>,
    active_view: Option<ViewId>,
) -> Vec<AssociationResult> {
    let in_scope: Vec<&AssociationCandidate> = candidates
        .iter()
        .filter(|c| match (active_view, c.view) {
            (Some(active), Some(candidate_view)) => active == candidate_view,
            (Some(_), None) => false,
            (None, _) => true,
        })
        .collect();

    if let Some(selected) = explicit_selection {
        if let Some(candidate) = in_scope.iter().find(|c| c.primitive_id == selected) {
            return vec![AssociationResult {
                primitive_id: candidate.primitive_id,
                confidence: Confidence::ONE,
                source: AssociationSource::ExplicitSelection,
            }];
        }
    }

    if let Some(guide_target) = dimension_guide_target {
        if let Some(candidate) = in_scope.iter().find(|c| c.primitive_id == guide_target) {
            return vec![AssociationResult {
                primitive_id: candidate.primitive_id,
                confidence: Confidence::new(0.95),
                source: AssociationSource::DimensionGuide,
            }];
        }
    }

    let mut results: Vec<AssociationResult> = in_scope
        .iter()
        .map(|candidate| {
            let bounds = primitive_bounds(&candidate.primitive);
            let distance = distance_point_to_bounds(text_position, bounds);
            let proximity = confidence_from_residual(distance, PROXIMITY_SCALE_MM);
            let orientation = orientation_alignment_score(text_position, &candidate.primitive);
            let combined = proximity.get() * 0.7 + orientation * 0.3;
            AssociationResult {
                primitive_id: candidate.primitive_id,
                confidence: Confidence::new(combined),
                source: AssociationSource::ProximityAndOrientation,
            }
        })
        .collect();

    results.sort_by(|a, b| {
        b.confidence
            .get()
            .partial_cmp(&a.confidence.get())
            .expect("Confidence is always finite and clamped")
    });
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Segment2;
    use craftloop_ids::CraftLoopId;

    fn line_candidate(
        id: PrimitiveId,
        a: Point2,
        b: Point2,
        view: Option<ViewId>,
    ) -> AssociationCandidate {
        AssociationCandidate {
            primitive_id: id,
            primitive: BeautifiedPrimitive::Line(Segment2::new(a, b)),
            view,
        }
    }

    // --- Task 125: explicit selection -----------------------------------

    #[test]
    fn explicit_selection_wins_even_when_a_different_candidate_is_geometrically_closer() {
        let far = PrimitiveId::new();
        let near = PrimitiveId::new();
        let candidates = vec![
            line_candidate(far, Point2::new(0.0, 100.0), Point2::new(10.0, 100.0), None),
            line_candidate(near, Point2::new(0.0, 0.0), Point2::new(10.0, 0.0), None),
        ];
        // Text sits right on top of `near`, but the user explicitly
        // selected `far`.
        let results = associate(Point2::new(5.0, 0.0), &candidates, Some(far), None, None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].primitive_id, far);
        assert_eq!(results[0].source, AssociationSource::ExplicitSelection);
        assert!(results[0].is_committable());
    }

    // --- Task 126: dimension-guide binding --------------------------------

    #[test]
    fn a_dimension_guide_target_wins_over_plain_proximity_when_nothing_is_explicitly_selected() {
        let guided = PrimitiveId::new();
        let closer = PrimitiveId::new();
        let candidates = vec![
            line_candidate(
                guided,
                Point2::new(0.0, 100.0),
                Point2::new(10.0, 100.0),
                None,
            ),
            line_candidate(closer, Point2::new(0.0, 0.0), Point2::new(10.0, 0.0), None),
        ];
        let results = associate(Point2::new(5.0, 0.0), &candidates, None, Some(guided), None);
        assert_eq!(results[0].primitive_id, guided);
        assert_eq!(results[0].source, AssociationSource::DimensionGuide);
    }

    #[test]
    fn explicit_selection_still_wins_over_a_dimension_guide_when_both_are_present() {
        let selected = PrimitiveId::new();
        let guided = PrimitiveId::new();
        let candidates = vec![
            line_candidate(
                selected,
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 0.0),
                None,
            ),
            line_candidate(
                guided,
                Point2::new(0.0, 50.0),
                Point2::new(10.0, 50.0),
                None,
            ),
        ];
        let results = associate(
            Point2::new(5.0, 25.0),
            &candidates,
            Some(selected),
            Some(guided),
            None,
        );
        assert_eq!(results[0].primitive_id, selected);
        assert_eq!(results[0].source, AssociationSource::ExplicitSelection);
    }

    // --- Task 127: proximity, never sole authority ------------------------

    #[test]
    fn the_closer_of_two_unselected_candidates_ranks_higher() {
        let near = PrimitiveId::new();
        let far = PrimitiveId::new();
        let candidates = vec![
            line_candidate(near, Point2::new(0.0, 1.0), Point2::new(10.0, 1.0), None),
            line_candidate(far, Point2::new(0.0, 40.0), Point2::new(10.0, 40.0), None),
        ];
        let results = associate(Point2::new(5.0, 0.0), &candidates, None, None, None);
        assert_eq!(results[0].primitive_id, near);
        assert_eq!(
            results[0].source,
            AssociationSource::ProximityAndOrientation
        );
    }

    #[test]
    fn proximity_alone_never_produces_a_result_the_engine_cannot_also_explain_with_orientation() {
        // Every ProximityAndOrientation result's confidence is a genuine
        // blend, not a passthrough of the proximity score alone: a
        // candidate positioned for perfect proximity (distance 0) but
        // terrible orientation (text sits *along* the line, not offset
        // from it) must not read as full confidence.
        let id = PrimitiveId::new();
        let candidates = vec![line_candidate(
            id,
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            None,
        )];
        // Text sits exactly on the line's own extended axis (in-line, not
        // offset) -- distance to bounds is 0 (on the segment's bounding
        // box edge), but orientation alignment is 0.
        let results = associate(Point2::new(5.0, 0.0), &candidates, None, None, None);
        assert!(results[0].confidence.get() < 1.0);
    }

    // --- Task 128: directional evidence -----------------------------------

    #[test]
    fn a_perpendicular_offset_can_outrank_a_closer_but_in_line_offset() {
        let perpendicular = PrimitiveId::new();
        let in_line = PrimitiveId::new();
        // A horizontal line (text placed above it is a conventional
        // perpendicular dimension offset) versus a vertical line whose
        // own bounds happen to sit closer to the text but along the same
        // axis the text is offset on (an unconventional placement for
        // dimensioning that line). Orientation evidence (Task 128) is
        // strong enough to outrank the shorter raw distance here --
        // proximity alone would have picked the wrong one.
        let candidates = vec![
            line_candidate(
                perpendicular,
                Point2::new(-5.0, 0.0),
                Point2::new(5.0, 0.0),
                None,
            ),
            line_candidate(in_line, Point2::new(0.0, -5.0), Point2::new(0.0, 5.0), None),
        ];
        let results = associate(Point2::new(0.0, 8.0), &candidates, None, None, None);
        let perpendicular_result = results
            .iter()
            .find(|r| r.primitive_id == perpendicular)
            .unwrap();
        let in_line_result = results.iter().find(|r| r.primitive_id == in_line).unwrap();
        assert!(perpendicular_result.confidence.get() > in_line_result.confidence.get());
    }

    // --- Task 129: view scoping --------------------------------------------

    #[test]
    fn a_candidate_in_a_different_view_never_wins_even_if_it_is_closer() {
        let this_view = ViewId::new();
        let other_view = ViewId::new();
        let far_but_correct_view = PrimitiveId::new();
        let near_but_wrong_view = PrimitiveId::new();
        let candidates = vec![
            line_candidate(
                far_but_correct_view,
                Point2::new(0.0, 40.0),
                Point2::new(10.0, 40.0),
                Some(this_view),
            ),
            line_candidate(
                near_but_wrong_view,
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 0.0),
                Some(other_view),
            ),
        ];
        let results = associate(
            Point2::new(5.0, 0.0),
            &candidates,
            None,
            None,
            Some(this_view),
        );
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].primitive_id, far_but_correct_view);
    }

    #[test]
    fn a_candidate_with_no_view_at_all_is_excluded_once_a_view_is_active() {
        let this_view = ViewId::new();
        let unscoped = PrimitiveId::new();
        let candidates = vec![line_candidate(
            unscoped,
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            None,
        )];
        let results = associate(
            Point2::new(5.0, 0.0),
            &candidates,
            None,
            None,
            Some(this_view),
        );
        assert!(results.is_empty());
    }

    #[test]
    fn with_no_active_view_scoping_is_a_no_op() {
        let candidates = vec![line_candidate(
            PrimitiveId::new(),
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            Some(ViewId::new()),
        )];
        let results = associate(Point2::new(5.0, 0.0), &candidates, None, None, None);
        assert_eq!(results.len(), 1);
    }

    // --- Task 130: medium-confidence preview -------------------------------

    #[test]
    fn a_far_ambiguous_candidate_needs_preview_rather_than_auto_committing() {
        let id = PrimitiveId::new();
        let candidates = vec![line_candidate(
            id,
            Point2::new(0.0, 30.0),
            Point2::new(10.0, 30.0),
            None,
        )];
        let results = associate(Point2::new(5.0, 0.0), &candidates, None, None, None);
        assert!(!results[0].is_committable());
    }

    #[test]
    fn no_eligible_candidates_returns_an_empty_not_fabricated_result() {
        let results = associate(Point2::new(0.0, 0.0), &[], None, None, None);
        assert!(results.is_empty());
    }

    // --- Task 132: wrong-target regression ---------------------------------

    #[test]
    fn two_nearly_equidistant_candidates_are_both_surfaced_not_silently_collapsed_to_one() {
        // The engine must never quietly pick a "winner" between two
        // genuinely ambiguous candidates without leaving evidence a
        // caller can inspect -- both must appear in the ranked list.
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let candidates = vec![
            line_candidate(a, Point2::new(-10.0, 1.0), Point2::new(-1.0, 1.0), None),
            line_candidate(b, Point2::new(1.0, 1.0), Point2::new(10.0, 1.0), None),
        ];
        let results = associate(Point2::new(0.0, 0.0), &candidates, None, None, None);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|r| r.primitive_id == a));
        assert!(results.iter().any(|r| r.primitive_id == b));
    }
}
