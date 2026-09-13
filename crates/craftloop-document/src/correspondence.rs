//! Cross-View Correspondence and Ambiguity.
//!
//! Execution 01, Phase 24, Tasks 173-179. Authority: MCP Article 40
//! "Cross-View Correspondence"; Article 232 "Detailed Specification of
//! the Cross-View Correspondence Engine"; Article 233 "Detailed
//! Specification of the Ambiguity Engine".
//!
//! Task 173 builds the explicit, deterministic manual link/unlink store
//! *before* any inference exists -- `CorrespondenceStore` is usable and
//! fully correct on its own, with zero dependency on the evidence
//! functions below it. Evidence-based suggestion (Tasks 174-178) is
//! strictly additive on top of it.

use std::collections::BTreeSet;

use craftloop_dimension::{DimensionStore, DimensionTarget};
use craftloop_geometry::Point2;
use craftloop_ids::{DimensionId, PrimitiveId};
use craftloop_recognition::{BeautifiedPrimitive, Confidence};

use crate::view::{PrincipalViewIdentity, ViewBlock};

fn normalize(a: PrimitiveId, b: PrimitiveId) -> (PrimitiveId, PrimitiveId) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Task 173/175/176: confirmed (manual) links, kept completely separate
/// from rejected ones. Deliberately has no notion of "suggested" state
/// stored here -- a suggestion (Task 175) is the transient
/// [`CorrespondenceCandidate`] a caller computes on demand via
/// [`evaluate_correspondence`], not persisted state this store owns; what
/// *is* persisted, and must be, is which suggestions were rejected
/// (Task 176), so the same one is not proposed again.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorrespondenceStore {
    confirmed: BTreeSet<(PrimitiveId, PrimitiveId)>,
    rejected: BTreeSet<(PrimitiveId, PrimitiveId)>,
}

impl CorrespondenceStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Task 173: the explicit, deterministic manual link.
    pub fn confirm_link(&mut self, a: PrimitiveId, b: PrimitiveId) {
        self.confirmed.insert(normalize(a, b));
    }

    /// Task 173: the explicit, deterministic manual unlink. Never
    /// touches `rejected` -- unlinking a confirmed correspondence is a
    /// different action from rejecting a suggestion that was never
    /// confirmed.
    pub fn unlink(&mut self, a: PrimitiveId, b: PrimitiveId) -> bool {
        self.confirmed.remove(&normalize(a, b))
    }

    pub fn is_confirmed(&self, a: PrimitiveId, b: PrimitiveId) -> bool {
        self.confirmed.contains(&normalize(a, b))
    }

    /// Task 176: remember a declined suggestion "within the current
    /// document context" -- this store's own lifetime.
    pub fn reject(&mut self, a: PrimitiveId, b: PrimitiveId) {
        self.rejected.insert(normalize(a, b));
    }

    pub fn is_rejected(&self, a: PrimitiveId, b: PrimitiveId) -> bool {
        self.rejected.contains(&normalize(a, b))
    }
}

/// Task 174: which kind of deterministic evidence supports a candidate.
/// Exactly Article 232's deterministic list -- "exact shared coordinates
/// under the current projection... dimensions... established
/// centerlines... existing user links" -- minus `Centerline`, which no
/// task before this phase gives this workspace a concrete type for (no
/// centerline concept exists yet); recorded as a deliberate, named gap
/// rather than a silently missing evidence source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceKind {
    /// The two entities' positions agree, in page space, along the
    /// screen axis their two views' identities are known to share
    /// (Front/Top share the page X axis via `Width`; Front/Right share
    /// page Y via `Height`). Only computed for those two directly-related
    /// pairs -- Top/Right correspondence would need real miter-line
    /// projection transfer geometry, which is out of this phase's scope
    /// (see `shared_screen_axis`'s own doc comment).
    SharedAxisCoordinate,
    /// The two entities have the same characteristic size (a circle's
    /// diameter, a line's length) -- weaker than coordinate agreement,
    /// but real, computable evidence for any view pair.
    SharedExtent,
    /// A confirmed dimension's `DimensionTarget::Pair` already names
    /// exactly these two primitives.
    SharedDimensionTarget(DimensionId),
    /// The correspondence was already manually confirmed
    /// (`CorrespondenceStore::confirm_link`).
    ExistingLink,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub detail: String,
}

/// Task 174/232: one ranked correspondence proposal -- source entity,
/// target entity, confidence, and the evidence behind it, exactly
/// Article 232's four required fields.
#[derive(Debug, Clone, PartialEq)]
pub struct CorrespondenceCandidate {
    pub source: PrimitiveId,
    pub target: PrimitiveId,
    pub confidence: Confidence,
    pub evidence: Vec<Evidence>,
}

enum ScreenAxis {
    X,
    Y,
}

/// Which page-space screen axis two view identities are known to share a
/// direct coordinate correspondence on. Only the two pairs this workspace
/// can already reason about correctly (`Front`/`Top` via `Width` -> X;
/// `Front`/`Right` via `Height` -> Y; `Front`/`Back` treated the same as
/// `Front`/`Top` since `Back` shares `Front`'s plane, Phase 22) return a
/// real axis. Every other pair (notably `Top`/`Right`, which share
/// `Depth`) returns `None` -- real coordinate correspondence there needs
/// miter-line projection transfer, a piece of real descriptive-geometry
/// machinery no earlier phase built and this one does not invent
/// speculatively.
fn shared_screen_axis(a: PrincipalViewIdentity, b: PrincipalViewIdentity) -> Option<ScreenAxis> {
    use PrincipalViewIdentity::{Back, Front, Right, Top};
    match (a, b) {
        (Front, Top) | (Top, Front) | (Front, Back) | (Back, Front) => Some(ScreenAxis::X),
        (Front, Right) | (Right, Front) => Some(ScreenAxis::Y),
        _ => None,
    }
}

fn characteristic_center(primitive: &BeautifiedPrimitive) -> Point2 {
    match primitive {
        BeautifiedPrimitive::Line(segment) => Point2::new(
            (segment.a.x + segment.b.x) / 2.0,
            (segment.a.y + segment.b.y) / 2.0,
        ),
        BeautifiedPrimitive::Circle(circle) => circle.center,
        BeautifiedPrimitive::Arc(arc) => arc.center,
        BeautifiedPrimitive::Rectangle(rectangle) => Point2::new(
            rectangle.corners.iter().map(|p| p.x).sum::<f64>() / 4.0,
            rectangle.corners.iter().map(|p| p.y).sum::<f64>() / 4.0,
        ),
    }
}

fn characteristic_extent(primitive: &BeautifiedPrimitive) -> f64 {
    match primitive {
        BeautifiedPrimitive::Line(segment) => segment.length(),
        BeautifiedPrimitive::Circle(circle) => 2.0 * circle.radius,
        BeautifiedPrimitive::Arc(arc) => 2.0 * arc.radius,
        BeautifiedPrimitive::Rectangle(rectangle) => {
            let xs: Vec<f64> = rectangle.corners.iter().map(|p| p.x).collect();
            let ys: Vec<f64> = rectangle.corners.iter().map(|p| p.y).collect();
            let width = xs.iter().cloned().fold(f64::MIN, f64::max)
                - xs.iter().cloned().fold(f64::MAX, f64::min);
            let height = ys.iter().cloned().fold(f64::MIN, f64::max)
                - ys.iter().cloned().fold(f64::MAX, f64::min);
            width.max(height)
        }
    }
}

fn shared_pair_dimension(
    store: &DimensionStore,
    dimension_ids: &[DimensionId],
    a: PrimitiveId,
    b: PrimitiveId,
) -> Option<DimensionId> {
    dimension_ids.iter().copied().find(|&id| {
        matches!(
            store.dimension(id).map(|dimension| dimension.target),
            Some(DimensionTarget::Pair(x, y)) if (x == a && y == b) || (x == b && y == a)
        )
    })
}

const COORDINATE_TOLERANCE: f64 = 0.5;
const EXTENT_TOLERANCE: f64 = 0.5;

/// Task 174: evaluate real, deterministic evidence for whether `source`
/// (in `source_view`) and `target` (in `target_view`) correspond to the
/// same real-world feature. Returns `None` outright for a rejected pair
/// (Task 176: a rejected suggestion must not reappear) and for a pair
/// with no evidence at all (Task 179: deliberately independent geometry
/// must never be forced into a candidate).
#[allow(clippy::too_many_arguments)]
pub fn evaluate_correspondence(
    source_id: PrimitiveId,
    source_view: &ViewBlock,
    source: &BeautifiedPrimitive,
    target_id: PrimitiveId,
    target_view: &ViewBlock,
    target: &BeautifiedPrimitive,
    store: &CorrespondenceStore,
    dimension_store: &DimensionStore,
    dimension_ids: &[DimensionId],
) -> Option<CorrespondenceCandidate> {
    if store.is_rejected(source_id, target_id) {
        return None;
    }

    let mut evidence = Vec::new();
    let mut confidence = 0.0_f64;

    if store.is_confirmed(source_id, target_id) {
        evidence.push(Evidence {
            kind: EvidenceKind::ExistingLink,
            detail: "already manually confirmed".to_string(),
        });
        confidence = confidence.max(1.0);
    }

    if let (Some(source_identity), Some(target_identity)) =
        (source_view.identity(), target_view.identity())
    {
        if let Some(axis) = shared_screen_axis(source_identity, target_identity) {
            let source_center = source_view.layout.apply(characteristic_center(source));
            let target_center = target_view.layout.apply(characteristic_center(target));
            let (source_coordinate, target_coordinate) = match axis {
                ScreenAxis::X => (source_center.x, target_center.x),
                ScreenAxis::Y => (source_center.y, target_center.y),
            };
            if (source_coordinate - target_coordinate).abs() < COORDINATE_TOLERANCE {
                evidence.push(Evidence {
                    kind: EvidenceKind::SharedAxisCoordinate,
                    detail: format!("page-space coordinates agree within {COORDINATE_TOLERANCE}"),
                });
                confidence = confidence.max(0.75);
            }
        }
    }

    let source_extent = characteristic_extent(source);
    let target_extent = characteristic_extent(target);
    if (source_extent - target_extent).abs() < EXTENT_TOLERANCE {
        evidence.push(Evidence {
            kind: EvidenceKind::SharedExtent,
            detail: format!("characteristic extents agree within {EXTENT_TOLERANCE}"),
        });
        confidence = confidence.max(0.4);
    }

    if let Some(dimension_id) =
        shared_pair_dimension(dimension_store, dimension_ids, source_id, target_id)
    {
        evidence.push(Evidence {
            kind: EvidenceKind::SharedDimensionTarget(dimension_id),
            detail: format!("{dimension_id:?} already targets both entities"),
        });
        confidence = confidence.max(0.9);
    }

    if evidence.is_empty() {
        return None;
    }
    Some(CorrespondenceCandidate {
        source: source_id,
        target: target_id,
        confidence: Confidence::new(confidence),
        evidence,
    })
}

/// Task 177/233: "When the system does not know, ask for the smallest
/// missing fact." For correspondence ambiguity specifically, that is
/// exactly one unconfirmed candidate -- the highest-confidence one, if
/// several remain -- for the interface to ask the user to confirm or
/// reject, rather than presenting every open question at once.
#[derive(Debug, Clone, PartialEq)]
pub struct MissingFact {
    pub source: PrimitiveId,
    pub target: PrimitiveId,
    pub confidence: Confidence,
}

pub fn smallest_missing_fact(candidates: &[CorrespondenceCandidate]) -> Option<MissingFact> {
    candidates
        .iter()
        .filter(|candidate| candidate.confidence.get() < 1.0)
        .max_by(|a, b| {
            a.confidence
                .get()
                .partial_cmp(&b.confidence.get())
                .expect("Confidence is finite")
        })
        .map(|candidate| MissingFact {
            source: candidate.source,
            target: candidate.target,
            confidence: candidate.confidence,
        })
}

/// Task 178: the seam a future learned ranker plugs into. Its signature
/// is itself the guarantee it cannot bypass confirmation: it only ever
/// receives and returns `CorrespondenceCandidate`s, never a
/// `&mut CorrespondenceStore` -- there is no way for an implementation of
/// this trait to confirm a link itself, no matter how confident its
/// ranking is. Only `CorrespondenceStore::confirm_link` (Task 173, an
/// explicit user action) can do that.
pub trait CorrespondenceRanker {
    fn rank(&self, candidates: Vec<CorrespondenceCandidate>) -> Vec<CorrespondenceCandidate>;
}

/// The real, deterministic reference implementation: sorts by the
/// confidence `evaluate_correspondence` already computed, changing
/// nothing else.
#[derive(Debug, Default)]
pub struct DeterministicRanker;

impl CorrespondenceRanker for DeterministicRanker {
    fn rank(&self, mut candidates: Vec<CorrespondenceCandidate>) -> Vec<CorrespondenceCandidate> {
        candidates.sort_by(|a, b| {
            b.confidence
                .get()
                .partial_cmp(&a.confidence.get())
                .expect("Confidence is finite")
        });
        candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_dimension::{DimensionKind, DimensionRole, SemanticDimension};
    use craftloop_geometry::{Circle2, Segment2};
    use craftloop_ids::{CraftLoopId, ViewId};

    fn identified_view(identity: PrincipalViewIdentity) -> ViewBlock {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(identity);
        view
    }

    fn circle(center: Point2, radius: f64) -> BeautifiedPrimitive {
        BeautifiedPrimitive::Circle(Circle2::new(center, radius).unwrap())
    }

    fn line(a: Point2, b: Point2) -> BeautifiedPrimitive {
        BeautifiedPrimitive::Line(Segment2::new(a, b))
    }

    // --- Task 173: manual link/unlink is fully usable standalone -----------

    #[test]
    fn manual_confirm_and_unlink_work_with_zero_evidence_machinery_involved() {
        let mut store = CorrespondenceStore::new();
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        assert!(!store.is_confirmed(a, b));
        store.confirm_link(a, b);
        assert!(store.is_confirmed(a, b));
        // Order-independent.
        assert!(store.is_confirmed(b, a));
        assert!(store.unlink(a, b));
        assert!(!store.is_confirmed(a, b));
    }

    // --- Task 174: deterministic correspondence evidence --------------------

    #[test]
    fn two_circles_aligned_on_the_shared_front_top_axis_get_coordinate_evidence() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let hole_front = circle(Point2::new(10.0, 0.0), 3.0);
        let hole_top = circle(Point2::new(10.0, 5.0), 3.0);
        let store = CorrespondenceStore::new();
        let dimensions = DimensionStore::new();

        let candidate = evaluate_correspondence(
            PrimitiveId::new(),
            &front,
            &hole_front,
            PrimitiveId::new(),
            &top,
            &hole_top,
            &store,
            &dimensions,
            &[],
        )
        .unwrap();

        assert!(candidate
            .evidence
            .iter()
            .any(|e| e.kind == EvidenceKind::SharedAxisCoordinate));
        assert!(candidate.confidence.get() >= 0.75);
    }

    #[test]
    fn a_confirmed_pair_dimension_is_strong_evidence() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let a_id = PrimitiveId::new();
        let b_id = PrimitiveId::new();
        let mut dimensions = DimensionStore::new();
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Reference,
            DimensionTarget::Pair(a_id, b_id),
            1.0,
        )
        .unwrap();
        let dimension_id = dimension.id;
        dimensions.insert_dimension(dimension).unwrap();
        let store = CorrespondenceStore::new();

        let candidate = evaluate_correspondence(
            a_id,
            &front,
            &circle(Point2::new(0.0, 0.0), 3.0),
            b_id,
            &top,
            &circle(Point2::new(999.0, 999.0), 3.0),
            &store,
            &dimensions,
            &[dimension_id],
        )
        .unwrap();

        assert!(candidate.evidence.iter().any(
            |e| matches!(e.kind, EvidenceKind::SharedDimensionTarget(id) if id == dimension_id)
        ));
    }

    #[test]
    fn a_top_right_pair_gets_no_coordinate_evidence_extent_only() {
        // Depth correspondence between Top and Right needs real
        // miter-line projection geometry (deliberately out of scope) --
        // only extent-based evidence is available for this pair.
        let top = identified_view(PrincipalViewIdentity::Top);
        let right = identified_view(PrincipalViewIdentity::Right);
        let store = CorrespondenceStore::new();
        let dimensions = DimensionStore::new();

        let candidate = evaluate_correspondence(
            PrimitiveId::new(),
            &top,
            &circle(Point2::new(0.0, 0.0), 3.0),
            PrimitiveId::new(),
            &right,
            &circle(Point2::new(500.0, 500.0), 3.0),
            &store,
            &dimensions,
            &[],
        )
        .unwrap();

        assert!(!candidate
            .evidence
            .iter()
            .any(|e| e.kind == EvidenceKind::SharedAxisCoordinate));
        assert!(candidate
            .evidence
            .iter()
            .any(|e| e.kind == EvidenceKind::SharedExtent));
    }

    // --- Task 175/176: suggestion vs. confirmed, and rejection memory ------

    #[test]
    fn a_rejected_pair_never_produces_a_candidate_again() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let a_id = PrimitiveId::new();
        let b_id = PrimitiveId::new();
        let mut store = CorrespondenceStore::new();
        store.reject(a_id, b_id);
        let dimensions = DimensionStore::new();

        let candidate = evaluate_correspondence(
            a_id,
            &front,
            &circle(Point2::new(10.0, 0.0), 3.0),
            b_id,
            &top,
            &circle(Point2::new(10.0, 5.0), 3.0),
            &store,
            &dimensions,
            &[],
        );
        assert!(
            candidate.is_none(),
            "a rejected pair must never reappear as a suggestion"
        );
    }

    #[test]
    fn a_confirmed_link_is_distinguishable_from_a_mere_suggestion() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let a_id = PrimitiveId::new();
        let b_id = PrimitiveId::new();
        let mut store = CorrespondenceStore::new();
        store.confirm_link(a_id, b_id);
        let dimensions = DimensionStore::new();

        let candidate = evaluate_correspondence(
            a_id,
            &front,
            &circle(Point2::new(10.0, 0.0), 3.0),
            b_id,
            &top,
            &circle(Point2::new(10.0, 5.0), 3.0),
            &store,
            &dimensions,
            &[],
        )
        .unwrap();
        assert_eq!(candidate.confidence, Confidence::ONE);
        assert!(candidate
            .evidence
            .iter()
            .any(|e| e.kind == EvidenceKind::ExistingLink));
    }

    // --- Task 177: smallest-missing-fact diagnostics -------------------------

    #[test]
    fn smallest_missing_fact_picks_the_single_highest_confidence_unconfirmed_candidate() {
        let low = CorrespondenceCandidate {
            source: PrimitiveId::new(),
            target: PrimitiveId::new(),
            confidence: Confidence::new(0.4),
            evidence: vec![],
        };
        let high = CorrespondenceCandidate {
            source: PrimitiveId::new(),
            target: PrimitiveId::new(),
            confidence: Confidence::new(0.8),
            evidence: vec![],
        };
        let fact = smallest_missing_fact(&[low.clone(), high.clone()]).unwrap();
        assert_eq!(fact.source, high.source);
        assert_eq!(fact.confidence, high.confidence);
    }

    #[test]
    fn an_already_confirmed_candidate_is_not_a_missing_fact() {
        let confirmed = CorrespondenceCandidate {
            source: PrimitiveId::new(),
            target: PrimitiveId::new(),
            confidence: Confidence::ONE,
            evidence: vec![],
        };
        assert!(smallest_missing_fact(&[confirmed]).is_none());
    }

    #[test]
    fn no_candidates_means_no_missing_fact() {
        assert!(smallest_missing_fact(&[]).is_none());
    }

    // --- Task 178: future learned-ranking seam --------------------------------

    #[test]
    fn the_deterministic_ranker_sorts_by_confidence_without_confirming_anything() {
        let low = CorrespondenceCandidate {
            source: PrimitiveId::new(),
            target: PrimitiveId::new(),
            confidence: Confidence::new(0.3),
            evidence: vec![],
        };
        let high = CorrespondenceCandidate {
            source: PrimitiveId::new(),
            target: PrimitiveId::new(),
            confidence: Confidence::new(0.9),
            evidence: vec![],
        };
        let ranked = DeterministicRanker.rank(vec![low.clone(), high.clone()]);
        assert_eq!(ranked[0].confidence, high.confidence);
        assert_eq!(ranked[1].confidence, low.confidence);
        // The ranker's own type has no way to touch a CorrespondenceStore
        // at all -- confirmation stays a separate, explicit action.
    }

    // --- Task 179: ambiguous-feature regression suite -------------------------

    #[test]
    fn repeated_holes_of_identical_size_both_produce_plausible_but_unconfirmed_candidates() {
        // Two identical holes in Front, one hole in Top that could
        // plausibly correspond to either -- genuine ambiguity: neither
        // candidate should reach `ExistingLink`-level confidence.
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let store = CorrespondenceStore::new();
        let dimensions = DimensionStore::new();

        let hole_a = circle(Point2::new(5.0, 0.0), 2.0);
        let hole_b = circle(Point2::new(20.0, 0.0), 2.0);
        let top_hole = circle(Point2::new(5.0, 8.0), 2.0);

        let candidate_a = evaluate_correspondence(
            PrimitiveId::new(),
            &front,
            &hole_a,
            PrimitiveId::new(),
            &top,
            &top_hole,
            &store,
            &dimensions,
            &[],
        )
        .unwrap();
        let candidate_b = evaluate_correspondence(
            PrimitiveId::new(),
            &front,
            &hole_b,
            PrimitiveId::new(),
            &top,
            &top_hole,
            &store,
            &dimensions,
            &[],
        )
        .unwrap();

        // Both are real candidates (same extent); only the aligned one
        // (`hole_a`, matching x=5.0) also gets coordinate evidence.
        assert!(candidate_a.confidence.get() > candidate_b.confidence.get());
        assert!(candidate_a.confidence.get() < 1.0);
    }

    #[test]
    fn similar_length_edges_produce_extent_evidence() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let store = CorrespondenceStore::new();
        let dimensions = DimensionStore::new();

        let edge_a = line(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        let edge_b = line(Point2::new(500.0, 500.0), Point2::new(510.0, 500.0));

        let candidate = evaluate_correspondence(
            PrimitiveId::new(),
            &front,
            &edge_a,
            PrimitiveId::new(),
            &top,
            &edge_b,
            &store,
            &dimensions,
            &[],
        )
        .unwrap();
        assert!(candidate
            .evidence
            .iter()
            .any(|e| e.kind == EvidenceKind::SharedExtent));
    }

    #[test]
    fn deliberately_independent_geometry_produces_no_candidate_at_all() {
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let store = CorrespondenceStore::new();
        let dimensions = DimensionStore::new();

        // Wildly different size, wildly different position, no shared
        // dimension, never confirmed: genuinely nothing in common.
        let small_hole = circle(Point2::new(1.0, 1.0), 1.0);
        let big_rectangle_ish_line = line(Point2::new(900.0, -900.0), Point2::new(1400.0, -900.0));

        let candidate = evaluate_correspondence(
            PrimitiveId::new(),
            &front,
            &small_hole,
            PrimitiveId::new(),
            &top,
            &big_rectangle_ish_line,
            &store,
            &dimensions,
            &[],
        );
        assert!(
            candidate.is_none(),
            "unrelated geometry must never be forced into a candidate"
        );
    }
}
