//! Task 217 — Resolve-depth-in-Top scenario.
//!
//! Execution 01, Phase 30, Task 217. Adds a Depth value in a Top view
//! and confirms it propagates to a Right view (Article 35's exact
//! Front/Top/Right sharing example, Phase 22) without ever creating a
//! 3D solid -- true by construction, not merely claimed: every function
//! this scenario calls (`MultiviewGraph::bind_axis`,
//! `propagate_confirmed_value`) touches only `DimensionId`s and
//! `ViewId`s; no 3D geometry type (mesh, solid, B-rep) is defined
//! anywhere in `craftloop-geometry` for any such function to return.

use craftloop_dimension::{
    DimensionKind, DimensionRole, DimensionStore, DimensionTarget, SemanticDimension,
};
use craftloop_document::{
    propagate_confirmed_value, MultiviewGraph, PrincipalViewIdentity, SharedAxis, ViewBlock,
};
use craftloop_ids::{CraftLoopId, DimensionId, PrimitiveId, ViewId};

#[test]
fn depth_added_in_top_propagates_to_right_via_the_shared_dimension_id() {
    let mut top = ViewBlock::new(ViewId::new());
    top.set_identity(PrincipalViewIdentity::Top);
    let mut right = ViewBlock::new(ViewId::new());
    right.set_identity(PrincipalViewIdentity::Right);

    let mut store = DimensionStore::new();
    let dimension_id = DimensionId::new();
    let depth = SemanticDimension::new(
        dimension_id,
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        15.0,
    )
    .unwrap();
    store.insert_dimension(depth).unwrap();

    let mut graph = MultiviewGraph::new();
    // Depth is added in Top first...
    graph
        .bind_axis(&top, SharedAxis::Depth, dimension_id)
        .unwrap();
    // ...then Right consumes the *same* semantic value (Article 35: Top
    // and Right both share Depth with each other, not with Front).
    graph
        .bind_axis(&right, SharedAxis::Depth, dimension_id)
        .unwrap();

    let affected = propagate_confirmed_value(&mut store, &graph, dimension_id, 20.0).unwrap();
    assert!(affected.contains(&top.id));
    assert!(affected.contains(&right.id));
    assert_eq!(affected.len(), 2);

    // Both views read the one real value -- there is nothing to
    // "propagate" beyond the edit itself, since sharing already means
    // "the same DimensionId," not a copy.
    assert_eq!(store.dimension(dimension_id).unwrap().value(), 20.0);
    assert_eq!(graph.axis_of(top.id, SharedAxis::Depth), Some(dimension_id));
    assert_eq!(
        graph.axis_of(right.id, SharedAxis::Depth),
        Some(dimension_id)
    );
}

#[test]
fn front_does_not_consume_depth_at_all() {
    // Article 35's own pairing: Front shares Width/Height, never Depth.
    // Binding Depth onto a Front-identity view must be refused, which is
    // itself part of why no 3D solid can accidentally form here -- Depth
    // never reaches a Front view's own axis set.
    let mut front = ViewBlock::new(ViewId::new());
    front.set_identity(PrincipalViewIdentity::Front);
    let mut graph = MultiviewGraph::new();
    let dimension_id = DimensionId::new();
    assert!(graph
        .bind_axis(&front, SharedAxis::Depth, dimension_id)
        .is_err());
}
