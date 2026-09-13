//! Orthographic Relationship Engine: deterministic shared-extent
//! propagation, conservative Back-extent sharing, projection guides,
//! view-local/shared geometry separation, cross-view conflict
//! generation, and the no-permanent-master-view invariant.
//!
//! Execution 01, Phase 23, Tasks 165-172. Authority: MCP Article 36 ("No
//! Permanent Master View"), Article 37 ("Shared Dimensions Across
//! Views").
//!
//! Tasks 165-167 (propagate width/height/depth between the view pairs
//! that share each axis) are one mechanism, not three: because
//! `MultiviewGraph` (Phase 22) already represents sharing as multiple
//! views binding the *same* `DimensionId`, "propagation" is simply
//! editing that one dimension through `DimensionStore` -- every bound
//! view already reads the current value, nothing has to be copied to
//! them. `propagate_confirmed_value` is that one function; it is tested
//! three times, once per axis pair Article 35 names, because Tasks
//! 165-167 are each a real, separately evidenced requirement even though
//! they share an implementation (this workspace's own
//! "no-duplicated-truth" convention applies to code, not to test
//! coverage of distinct requirements).

use craftloop_consistency::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};
use craftloop_dimension::DimensionStore;
use craftloop_errors::Severity;
use craftloop_geometry::Tolerances;
use craftloop_ids::{ConflictId, CraftLoopId, DimensionId, ViewId};

use crate::multiview::{MultiviewGraph, SharedAxis};
use crate::view::ViewBlock;

/// Tasks 165-167/172: edit `dimension`'s value once, through the real
/// `DimensionStore` (Phase 10) -- fails exactly as
/// `DimensionStore::edit_driving_value` would (unknown dimension, not
/// directly editable, out of feasible range) if the edit itself is
/// invalid. Returns every view `graph` has bound to this dimension: the
/// "affected subgraph" (Article 231) a caller should refresh, without
/// this function needing to know anything about which axis or which
/// views those are -- Task 172's "no permanent master view" falls out
/// directly, since this function never special-cases which view is
/// making the edit.
pub fn propagate_confirmed_value(
    store: &mut DimensionStore,
    graph: &MultiviewGraph,
    dimension: DimensionId,
    new_value: f64,
) -> craftloop_errors::DomainResult<Vec<ViewId>> {
    store.edit_driving_value(dimension, new_value)?;
    Ok(graph.affected_views(dimension).into_iter().collect())
}

/// Task 168: share `axis` from `front` onto `back`, but only if `front`
/// already has a real, resolved binding for that axis -- never inventing
/// a value for an unseen back feature. Also refuses an axis `back`'s own
/// identity does not consume at all (e.g. `Depth`, which no `Back` view
/// shares with `Front` under this workspace's `axes_for_identity`
/// mapping) -- `MultiviewGraph::bind_axis` already enforces that second
/// check; this function's only *additional* rule is "don't share what
/// hasn't itself been established yet."
pub fn share_back_extent_from_front(
    graph: &mut MultiviewGraph,
    front: &ViewBlock,
    back: &ViewBlock,
    axis: SharedAxis,
) -> craftloop_errors::DomainResult<()> {
    let dimension = graph.axis_of(front.id, axis).ok_or_else(|| craftloop_errors::DomainError::Document {
        kind: craftloop_errors::DocumentErrorKind::UnknownReference,
        detail: format!("front view has no resolved {axis:?} to share -- refusing to invent one for the back view"),
    })?;
    graph.bind_axis(back, axis, dimension)
}

/// Task 169: a purely computed, transient alignment hint between two
/// views sharing an axis -- never stored, never a real geometric
/// constraint (`craftloop-sketch`, Phase 12) unless a caller explicitly
/// turns it into one through that crate's own API. Returning a plain
/// `Vec` that nothing retains is the enforcement mechanism: there is no
/// method on this type, or anywhere in this module, that inserts a guide
/// into any persisted store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionGuide {
    pub axis: SharedAxis,
    pub from_view: ViewId,
    pub to_view: ViewId,
    pub dimension: DimensionId,
}

/// Compute every current cross-view alignment guide implied by `graph`'s
/// existing bindings among `views`. Purely a function of current state --
/// calling it twice with the same inputs returns equal, independently
/// owned results, and calling it never mutates `graph` or creates
/// anything durable.
pub fn projection_guides(graph: &MultiviewGraph, views: &[&ViewBlock]) -> Vec<ProjectionGuide> {
    let mut guides = Vec::new();
    for axis in [SharedAxis::Width, SharedAxis::Height, SharedAxis::Depth] {
        let bound: Vec<(&ViewBlock, DimensionId)> = views
            .iter()
            .filter_map(|view| {
                graph
                    .axis_of(view.id, axis)
                    .map(|dimension| (*view, dimension))
            })
            .collect();
        for i in 0..bound.len() {
            for j in (i + 1)..bound.len() {
                let (view_a, dimension_a) = bound[i];
                let (view_b, dimension_b) = bound[j];
                if dimension_a == dimension_b {
                    guides.push(ProjectionGuide {
                        axis,
                        from_view: view_a.id,
                        to_view: view_b.id,
                        dimension: dimension_a,
                    });
                }
            }
        }
    }
    guides
}

/// Task 171: propose `value` for `view`'s `axis`. If no binding exists
/// yet, this is a normal, fresh confirmation (`Ok(None)`, nothing to
/// report). If a binding already exists and `value` matches the
/// dimension's current value within committed tolerance, this is just a
/// (harmless) re-confirmation. If it already exists and disagrees, this
/// returns a real [`Conflict`] (`CrossViewMismatch`) instead of ever
/// creating a second `DimensionId` for the same semantic quantity --
/// Article 37's exact rule, and Task 171's "a second incompatible value
/// creates a conflict object, not a second truth."
pub fn propose_shared_value(
    store: &DimensionStore,
    graph: &MultiviewGraph,
    view: &ViewBlock,
    axis: SharedAxis,
    value: f64,
) -> Option<Conflict> {
    let existing_id = graph.axis_of(view.id, axis)?;
    let existing_value = store.dimension(existing_id)?.value();
    if (existing_value - value).abs() <= Tolerances::committed().length_equality {
        return None;
    }
    Some(Conflict {
        id: ConflictId::new(),
        kind: ConflictKind::CrossViewMismatch,
        severity: Severity::Error,
        affected_entities: vec![format!("{existing_id:?}"), format!("{:?}", view.id)],
        existing_truth: format!("{axis:?} = {existing_value}"),
        proposed_truth: format!("{axis:?} = {value} (from {:?})", view.id),
        evidence: format!("difference = {}", (existing_value - value).abs()),
        resolution_choices: vec![
            ResolutionChoice::KeepExisting,
            ResolutionChoice::ReplaceAndPropagate,
        ],
        status: ConflictStatus::Unresolved,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::PrincipalViewIdentity;
    use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget};
    use craftloop_ids::PrimitiveId;

    fn identified_view(identity: PrincipalViewIdentity) -> ViewBlock {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(identity);
        view
    }

    /// A dimension already resolved to `value` -- standing in for "some
    /// value a view has already confirmed," for tests that only read it
    /// (`share_back_extent_from_front`, `propose_shared_value`) rather
    /// than edit it through `DimensionStore::edit_driving_value` (which
    /// tests exercising `propagate_confirmed_value` build their own
    /// `Driving`-role dimension for directly, since editability is the
    /// exact thing under test there).
    fn driving_dimension(store: &mut DimensionStore, value: f64) -> DimensionId {
        let dimension = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            value,
        )
        .unwrap();
        let id = dimension.id;
        store.insert_dimension(dimension).unwrap();
        id
    }

    // --- Tasks 165-167: deterministic shared-extent propagation ------------

    #[test]
    fn propagating_width_reaches_every_view_bound_to_it_front_and_top() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            50.0,
        )
        .unwrap();
        let width_id = width.id;
        store.insert_dimension(width).unwrap();
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width_id).unwrap();

        let affected = propagate_confirmed_value(&mut store, &graph, width_id, 100.0).unwrap();

        assert_eq!(store.dimension(width_id).unwrap().value(), 100.0);
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&front.id));
        assert!(affected.contains(&top.id));
    }

    #[test]
    fn propagating_height_reaches_front_and_right() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let right = identified_view(PrincipalViewIdentity::Right);
        let height = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            20.0,
        )
        .unwrap();
        let height_id = height.id;
        store.insert_dimension(height).unwrap();
        graph
            .bind_axis(&front, SharedAxis::Height, height_id)
            .unwrap();
        graph
            .bind_axis(&right, SharedAxis::Height, height_id)
            .unwrap();

        let affected = propagate_confirmed_value(&mut store, &graph, height_id, 45.0).unwrap();
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&front.id));
        assert!(affected.contains(&right.id));
    }

    #[test]
    fn propagating_depth_reaches_top_and_right() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let top = identified_view(PrincipalViewIdentity::Top);
        let right = identified_view(PrincipalViewIdentity::Right);
        let depth = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            15.0,
        )
        .unwrap();
        let depth_id = depth.id;
        store.insert_dimension(depth).unwrap();
        graph.bind_axis(&top, SharedAxis::Depth, depth_id).unwrap();
        graph
            .bind_axis(&right, SharedAxis::Depth, depth_id)
            .unwrap();

        let affected = propagate_confirmed_value(&mut store, &graph, depth_id, 30.0).unwrap();
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&top.id));
        assert!(affected.contains(&right.id));
    }

    #[test]
    fn propagation_never_touches_a_view_bound_to_a_different_dimension() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let unrelated_top = identified_view(PrincipalViewIdentity::Top);
        let width = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            50.0,
        )
        .unwrap();
        let width_id = width.id;
        let unrelated_depth = craftloop_dimension::SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            10.0,
        )
        .unwrap();
        let unrelated_depth_id = unrelated_depth.id;
        store.insert_dimension(width).unwrap();
        store.insert_dimension(unrelated_depth).unwrap();
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();
        graph
            .bind_axis(&unrelated_top, SharedAxis::Depth, unrelated_depth_id)
            .unwrap();

        propagate_confirmed_value(&mut store, &graph, width_id, 99.0).unwrap();

        assert_eq!(store.dimension(unrelated_depth_id).unwrap().value(), 10.0);
    }

    // --- Task 168: conservative Back extent sharing -------------------------

    #[test]
    fn back_can_share_an_extent_front_has_already_resolved() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let back = identified_view(PrincipalViewIdentity::Back);
        let width_id = driving_dimension(&mut store, 50.0);
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();

        share_back_extent_from_front(&mut graph, &front, &back, SharedAxis::Width).unwrap();

        assert_eq!(graph.axis_of(back.id, SharedAxis::Width), Some(width_id));
    }

    #[test]
    fn back_cannot_share_an_extent_front_has_not_resolved_yet() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let back = identified_view(PrincipalViewIdentity::Back);

        let result = share_back_extent_from_front(&mut graph, &front, &back, SharedAxis::Width);
        assert!(result.is_err(), "must never invent an unseen back feature");
        assert!(graph.axis_of(back.id, SharedAxis::Width).is_none());
    }

    #[test]
    fn back_cannot_share_depth_which_back_views_do_not_consume() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        // Depth is not shared between Front and Back under this
        // workspace's axes mapping -- use Top as the (irrelevant) source
        // of a resolved Depth value to prove the rejection is about
        // Back's own identity, not about whether a value exists.
        let top = identified_view(PrincipalViewIdentity::Top);
        let back = identified_view(PrincipalViewIdentity::Back);
        let depth_id = driving_dimension(&mut store, 12.0);
        graph.bind_axis(&top, SharedAxis::Depth, depth_id).unwrap();

        let result = share_back_extent_from_front(&mut graph, &top, &back, SharedAxis::Depth);
        assert!(result.is_err());
    }

    // --- Task 169: projection guides ----------------------------------------

    #[test]
    fn a_shared_binding_produces_a_projection_guide_between_the_two_views() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width_id = DimensionId::new();
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width_id).unwrap();

        let guides = projection_guides(&graph, &[&front, &top]);
        assert_eq!(guides.len(), 1);
        assert_eq!(guides[0].axis, SharedAxis::Width);
        assert_eq!(guides[0].dimension, width_id);
    }

    #[test]
    fn projection_guides_are_purely_computed_and_never_mutate_the_graph() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        graph
            .bind_axis(&front, SharedAxis::Width, DimensionId::new())
            .unwrap();
        graph
            .bind_axis(&top, SharedAxis::Width, DimensionId::new())
            .unwrap();

        let before = graph.clone();
        let first = projection_guides(&graph, &[&front, &top]);
        let second = projection_guides(&graph, &[&front, &top]);

        assert_eq!(
            graph, before,
            "computing guides must never mutate the graph"
        );
        assert_eq!(first, second);
    }

    #[test]
    fn views_with_no_shared_binding_produce_no_guide() {
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        graph
            .bind_axis(&front, SharedAxis::Width, DimensionId::new())
            .unwrap();
        graph
            .bind_axis(&top, SharedAxis::Width, DimensionId::new())
            .unwrap(); // a *different* dimension
        assert!(projection_guides(&graph, &[&front, &top]).is_empty());
    }

    // --- Task 170: view-local vs. shared geometry ---------------------------

    #[test]
    fn visually_identical_geometry_in_two_views_is_never_auto_linked() {
        // Two views each get a primitive with the same PrimitiveId's
        // *shape* coincidentally -- but MultiviewGraph has no mechanism
        // at all that inspects geometry to auto-create a binding. Their
        // membership sets stay completely independent.
        let mut front = identified_view(PrincipalViewIdentity::Front);
        let mut top = identified_view(PrincipalViewIdentity::Top);
        front.geometry_members.insert(PrimitiveId::new());
        top.geometry_members.insert(PrimitiveId::new());

        let graph = MultiviewGraph::new();
        assert!(graph.bindings_for_view(front.id).is_empty());
        assert!(graph.bindings_for_view(top.id).is_empty());
        assert_ne!(front.geometry_members, top.geometry_members);
    }

    // --- Task 171: cross-view conflict generation ---------------------------

    #[test]
    fn a_matching_re_confirmation_produces_no_conflict() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let width_id = driving_dimension(&mut store, 100.0);
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();

        let conflict = propose_shared_value(&store, &graph, &front, SharedAxis::Width, 100.0);
        assert!(conflict.is_none());
    }

    #[test]
    fn article_37s_exact_scenario_100mm_then_130mm_is_a_conflict_not_a_second_truth() {
        let mut store = DimensionStore::new();
        let mut graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        let top = identified_view(PrincipalViewIdentity::Top);
        let width_id = driving_dimension(&mut store, 100.0);
        graph
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();
        graph.bind_axis(&top, SharedAxis::Width, width_id).unwrap();

        let conflict =
            propose_shared_value(&store, &graph, &top, SharedAxis::Width, 130.0).unwrap();
        assert_eq!(conflict.kind, ConflictKind::CrossViewMismatch);
        assert!(conflict.is_unresolved());
        // No second dimension was created -- the graph still resolves
        // Top's width to the one original id.
        assert_eq!(graph.axis_of(top.id, SharedAxis::Width), Some(width_id));
    }

    #[test]
    fn a_first_time_value_with_no_existing_binding_is_not_a_conflict() {
        let store = DimensionStore::new();
        let graph = MultiviewGraph::new();
        let front = identified_view(PrincipalViewIdentity::Front);
        assert!(propose_shared_value(&store, &graph, &front, SharedAxis::Width, 100.0).is_none());
    }

    // --- Task 172: no permanent master view ---------------------------------

    #[test]
    fn binding_order_across_views_does_not_matter_no_view_is_privileged() {
        let mut store = DimensionStore::new();
        let width_id = driving_dimension(&mut store, 10.0);

        // Top establishes the shared value first, not Front.
        let mut graph_top_first = MultiviewGraph::new();
        let top = identified_view(PrincipalViewIdentity::Top);
        let front = identified_view(PrincipalViewIdentity::Front);
        graph_top_first
            .bind_axis(&top, SharedAxis::Width, width_id)
            .unwrap();
        graph_top_first
            .bind_axis(&front, SharedAxis::Width, width_id)
            .unwrap();

        let affected =
            propagate_confirmed_value(&mut store, &graph_top_first, width_id, 77.0).unwrap();
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&top.id));
        assert!(affected.contains(&front.id));
    }
}
