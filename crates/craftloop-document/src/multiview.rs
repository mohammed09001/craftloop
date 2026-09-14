//! Multiview Constraint Graph.
//!
//! Execution 01, Phase 22, Tasks 157-164. Authority: MCP Article 35
//! ("Front uses width and height. Top uses width and depth. Right uses
//! depth and height. The system does not need a three-dimensional solid
//! to understand that the same width appears in front and top views.");
//! Article 231 "Detailed Specification of the Multiview Constraint
//! Graph".
//!
//! A shared axis variable is not a new value type -- it is a
//! `craftloop_dimension::DimensionId` whose `SemanticDimension` already
//! has `role == Shared` (Phase 10, built for exactly this). Multiple
//! views referencing the *same* `DimensionId` is what makes them share a
//! value: there is nothing to copy or keep in sync, so Article 231's
//! "a change in a shared dimension should update only affected
//! subgraphs" and Article 36's "Dimension anywhere. Resolve everywhere"
//! fall out of the representation itself (Task 161) rather than needing
//! a separate propagation mechanism this module would have to keep
//! correct by hand.

use std::collections::{BTreeMap, BTreeSet};

use craftloop_errors::{DocumentErrorKind, DomainError, DomainResult};
use craftloop_ids::{DimensionId, ViewId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::view::{PrincipalViewIdentity, ViewBlock};

/// Task 157: the shared extent variables Article 35/37 name. "Later
/// feature dimensions" (Task 157's own wording) are explicitly future
/// scope -- no task before this one names a concrete one, so none is
/// invented speculatively here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SharedAxis {
    Width,
    Height,
    Depth,
}

/// Task 158: which axes `identity`'s view consumes -- Article 35's exact
/// mapping, without constructing a 3D solid. `Back` is not in Article
/// 35's own worked example; it is treated as sharing `Front`'s plane
/// (the same width/height, viewed from the opposite face), the most
/// direct reading consistent with the article's logic rather than an
/// invented fourth combination.
pub fn axes_for_identity(identity: PrincipalViewIdentity) -> &'static [SharedAxis] {
    match identity {
        PrincipalViewIdentity::Front => &[SharedAxis::Width, SharedAxis::Height],
        PrincipalViewIdentity::Top => &[SharedAxis::Width, SharedAxis::Depth],
        PrincipalViewIdentity::Right => &[SharedAxis::Depth, SharedAxis::Height],
        PrincipalViewIdentity::Back => &[SharedAxis::Width, SharedAxis::Height],
    }
}

/// Task 157/159: the Multiview Constraint Graph's core state -- which
/// shared `DimensionId` (Task 159's "one semantic value") each view's
/// each consumed axis is currently bound to.
///
/// `Serialize`/`Deserialize` are hand-written rather than derived: a
/// `BTreeMap` keyed by the tuple `(ViewId, SharedAxis)` cannot serialize
/// to JSON at all (object keys must be strings), the exact same failure
/// mode `EntityId` hit in Phase 07 -- caught here the same way, by a real
/// round-trip test (Task 164's own `the_graph_serializes_and_reloads_...`
/// test), not by inspection. The fix here takes the same shape as
/// `EntityId`'s: serialize as a flat list of `(view, axis, dimension)`
/// records instead of a map with a composite key.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiviewGraph {
    bindings: BTreeMap<(ViewId, SharedAxis), DimensionId>,
}

#[derive(Serialize, Deserialize)]
struct BindingRecord {
    view: ViewId,
    axis: SharedAxis,
    dimension: DimensionId,
}

impl Serialize for MultiviewGraph {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let records: Vec<BindingRecord> = self
            .bindings
            .iter()
            .map(|(&(view, axis), &dimension)| BindingRecord {
                view,
                axis,
                dimension,
            })
            .collect();
        records.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MultiviewGraph {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let records = Vec::<BindingRecord>::deserialize(deserializer)?;
        let bindings = records
            .into_iter()
            .map(|record| ((record.view, record.axis), record.dimension))
            .collect();
        Ok(Self { bindings })
    }
}

impl MultiviewGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Task 157/158/159/161: bind `view`'s `axis` to `dimension`,
    /// atomically -- rejected outright (nothing changed) unless `view`
    /// has an established identity that actually consumes `axis`
    /// (Task 158's mapping is the authority on which bindings are even
    /// meaningful).
    pub fn bind_axis(
        &mut self,
        view: &ViewBlock,
        axis: SharedAxis,
        dimension: DimensionId,
    ) -> DomainResult<()> {
        let identity = view.identity().ok_or_else(|| DomainError::Document {
            kind: DocumentErrorKind::UnknownReference,
            detail: format!(
                "view {:?} has no established identity to bind an axis to",
                view.id
            ),
        })?;
        if !axes_for_identity(identity).contains(&axis) {
            return Err(DomainError::Document {
                kind: DocumentErrorKind::UnknownReference,
                detail: format!("{identity:?} views do not consume {axis:?}"),
            });
        }
        self.bindings.insert((view.id, axis), dimension);
        Ok(())
    }

    pub fn axis_of(&self, view: ViewId, axis: SharedAxis) -> Option<DimensionId> {
        self.bindings.get(&(view, axis)).copied()
    }

    /// Execution 02, Phase 04: insert a binding without re-running
    /// `bind_axis`'s identity/axis-compatibility validation. Used only by
    /// `history::DocumentChange::apply`/`invert` to replay a binding that
    /// was already validated once, through `bind_axis` itself, at the
    /// point `CraftLoopSession` first created it -- a `DocumentChange`
    /// only carries the raw `ViewId`/`SharedAxis`/`DimensionId` (not a
    /// `&ViewBlock`), so it has nothing further to validate against, and
    /// re-deriving one would only duplicate a check already performed.
    pub(crate) fn insert_binding_unchecked(
        &mut self,
        view: ViewId,
        axis: SharedAxis,
        dimension: DimensionId,
    ) {
        self.bindings.insert((view, axis), dimension);
    }

    /// Task 160: "Which views depend on this width?" -- the incremental-
    /// propagation query Task 161 relies on: a caller changing
    /// `dimension`'s value only needs to touch exactly these views, never
    /// a global recompute.
    pub fn affected_views(&self, dimension: DimensionId) -> BTreeSet<ViewId> {
        self.bindings
            .iter()
            .filter(|(_, bound_dimension)| **bound_dimension == dimension)
            .map(|((view, _), _)| *view)
            .collect()
    }

    /// Task 160: "Which dimensions control this feature?" -- every axis
    /// binding for one view.
    pub fn bindings_for_view(&self, view: ViewId) -> Vec<(SharedAxis, DimensionId)> {
        self.bindings
            .iter()
            .filter(|((bound_view, _), _)| *bound_view == view)
            .map(|((_, axis), dimension)| (*axis, *dimension))
            .collect()
    }

    /// Task 162: which of `view`'s own consumed axes have no binding at
    /// all yet -- explicit, queryable graph state (Article 38: "An
    /// unresolved dimension is not an error"), not a silently absent
    /// value a caller would have to infer.
    pub fn unresolved_axes(&self, view: &ViewBlock) -> Vec<SharedAxis> {
        let Some(identity) = view.identity() else {
            return Vec::new();
        };
        axes_for_identity(identity)
            .iter()
            .copied()
            .filter(|axis| self.axis_of(view.id, *axis).is_none())
            .collect()
    }

    /// Task 163: remove exactly this one view/axis binding. The bound
    /// `DimensionId`'s own value, and every *other* view's binding to it
    /// (or to anything else), is untouched -- unlinking stops future
    /// propagation to this one view without deleting the shared
    /// dimension or any unrelated geometry. Returns the dimension that
    /// was bound, if any.
    pub fn unlink_axis(&mut self, view: ViewId, axis: SharedAxis) -> Option<DimensionId> {
        self.bindings.remove(&(view, axis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    fn identified_view(identity: PrincipalViewIdentity) -> ViewBlock {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(identity);
        view
    }

    // --- Task 157/158: shared axis variables and view-to-variable mapping --

    #[test]
    fn front_consumes_width_and_height_top_consumes_width_and_depth() {
        assert_eq!(
            axes_for_identity(PrincipalViewIdentity::Front),
            &[SharedAxis::Width, SharedAxis::Height]
        );
        assert_eq!(
            axes_for_identity(PrincipalViewIdentity::Top),
            &[SharedAxis::Width, SharedAxis::Depth]
        );
        assert_eq!(
            axes_for_identity(PrincipalViewIdentity::Right),
            &[SharedAxis::Depth, SharedAxis::Height]
        );
    }

    #[test]
    fn binding_an_axis_the_views_identity_does_not_consume_is_rejected() {
        let mut graph = MultiviewGraph::new();
        let right = identified_view(PrincipalViewIdentity::Right);
        // Right views do not consume Width (Article 35).
        let result = graph.bind_axis(&right, SharedAxis::Width, DimensionId::new());
        assert!(result.is_err());
        assert!(graph.axis_of(right.id, SharedAxis::Width).is_none());
    }

    #[test]
    fn binding_an_axis_on_a_view_with_no_identity_is_rejected() {
        let mut graph = MultiviewGraph::new();
        let unidentified = ViewBlock::new(ViewId::new());
        let result = graph.bind_axis(&unidentified, SharedAxis::Width, DimensionId::new());
        assert!(result.is_err());
    }

    // --- Task 159: shared dimension references ------------------------------

    #[test]
    fn front_and_top_can_share_the_exact_same_width_dimension() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width = DimensionId::new();

        graph.bind_axis(&front, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width).unwrap();

        assert_eq!(graph.axis_of(front.id, SharedAxis::Width), Some(width));
        assert_eq!(graph.axis_of(top.id, SharedAxis::Width), Some(width));
    }

    // --- Task 160: graph dependency queries ----------------------------------

    #[test]
    fn affected_views_finds_every_view_sharing_a_dimension() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let right = identified_view(PrincipalViewIdentity::Right);
        let width = DimensionId::new();
        let depth = DimensionId::new();

        graph.bind_axis(&front, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Depth, depth).unwrap();
        graph.bind_axis(&right, SharedAxis::Depth, depth).unwrap();

        assert_eq!(
            graph.affected_views(width),
            BTreeSet::from([front.id, top.id])
        );
        assert_eq!(
            graph.affected_views(depth),
            BTreeSet::from([top.id, right.id])
        );
    }

    #[test]
    fn bindings_for_view_lists_every_axis_that_view_controls() {
        let mut graph = MultiviewGraph::new();
        let top = identified_view(PrincipalViewIdentity::Top);
        let width = DimensionId::new();
        let depth = DimensionId::new();
        graph.bind_axis(&top, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Depth, depth).unwrap();

        let mut bindings = graph.bindings_for_view(top.id);
        bindings.sort();
        let mut expected = vec![(SharedAxis::Width, width), (SharedAxis::Depth, depth)];
        expected.sort();
        assert_eq!(bindings, expected);
    }

    // --- Task 161: incremental propagation -----------------------------------

    #[test]
    fn binding_a_dimension_to_one_view_never_touches_an_unrelated_views_bindings() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let unrelated_top = identified_view(PrincipalViewIdentity::Top);
        let unrelated_height = DimensionId::new();
        graph
            .bind_axis(&unrelated_top, SharedAxis::Width, unrelated_height)
            .unwrap();

        graph
            .bind_axis(&front, SharedAxis::Height, DimensionId::new())
            .unwrap();

        // The unrelated view's own binding is exactly what it was.
        assert_eq!(
            graph.axis_of(unrelated_top.id, SharedAxis::Width),
            Some(unrelated_height)
        );
    }

    // --- Task 162: unresolved variables ---------------------------------------

    #[test]
    fn a_freshly_identified_view_reports_every_consumed_axis_as_unresolved() {
        let graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let mut unresolved = graph.unresolved_axes(&front);
        unresolved.sort();
        let mut expected = vec![SharedAxis::Width, SharedAxis::Height];
        expected.sort();
        assert_eq!(unresolved, expected);
    }

    #[test]
    fn binding_one_axis_leaves_the_others_reported_as_unresolved() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        graph
            .bind_axis(&front, SharedAxis::Width, DimensionId::new())
            .unwrap();
        assert_eq!(graph.unresolved_axes(&front), vec![SharedAxis::Height]);
    }

    #[test]
    fn an_unidentified_view_has_no_unresolved_axes_reported_identity_is_the_prior_blocker() {
        let graph = MultiviewGraph::new();
        let unidentified = ViewBlock::new(ViewId::new());
        assert!(graph.unresolved_axes(&unidentified).is_empty());
    }

    // --- Task 163: unlink semantics -------------------------------------------

    #[test]
    fn unlinking_one_view_from_a_shared_dimension_leaves_other_views_still_linked() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width = DimensionId::new();
        graph.bind_axis(&front, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width).unwrap();

        let unlinked = graph.unlink_axis(front.id, SharedAxis::Width);

        assert_eq!(unlinked, Some(width));
        assert!(graph.axis_of(front.id, SharedAxis::Width).is_none());
        // Top's own link to the same dimension survives untouched.
        assert_eq!(graph.axis_of(top.id, SharedAxis::Width), Some(width));
    }

    #[test]
    fn unlinking_an_unbound_axis_is_a_harmless_no_op() {
        let mut graph = MultiviewGraph::new();
        assert_eq!(graph.unlink_axis(ViewId::new(), SharedAxis::Width), None);
    }

    // --- Task 164: propagation/reopen ------------------------------------------

    #[test]
    fn the_graph_serializes_and_reloads_with_every_shared_link_intact() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width = DimensionId::new();
        graph.bind_axis(&front, SharedAxis::Width, width).unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width).unwrap();

        let json = serde_json::to_string(&graph).unwrap();
        let reloaded: MultiviewGraph = serde_json::from_str(&json).unwrap();

        assert_eq!(reloaded, graph);
        assert_eq!(reloaded.axis_of(front.id, SharedAxis::Width), Some(width));
        assert_eq!(reloaded.axis_of(top.id, SharedAxis::Width), Some(width));
        assert_eq!(
            reloaded.affected_views(width),
            BTreeSet::from([front.id, top.id])
        );
    }

    #[test]
    fn an_empty_graph_round_trips_too() {
        let graph = MultiviewGraph::new();
        let json = serde_json::to_string(&graph).unwrap();
        let reloaded: MultiviewGraph = serde_json::from_str(&json).unwrap();
        assert_eq!(reloaded, graph);
    }
}
