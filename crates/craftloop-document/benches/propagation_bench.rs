//! Multiview propagation benchmark.
//!
//! Execution 01, Phase 27, Task 196. Authority: Execution 01, Phase 23
//! (Orthographic Relationship Engine). Benchmarks
//! `propagate_confirmed_value` -- editing one shared dimension and
//! discovering every view it affects -- against a document with many
//! bound views, representative of a drawing with several orthographic
//! sheets all sharing the same handful of driving dimensions.

use craftloop_dimension::{
    DimensionKind, DimensionRole, DimensionStore, DimensionTarget, SemanticDimension,
};
use craftloop_document::{
    propagate_confirmed_value, MultiviewGraph, PrincipalViewIdentity, SharedAxis, ViewBlock,
};
use craftloop_ids::{CraftLoopId, DimensionId, PrimitiveId, ViewId};
use criterion::{criterion_group, criterion_main, Criterion};

fn setup(view_count: usize) -> (DimensionStore, MultiviewGraph, DimensionId) {
    let mut store = DimensionStore::new();
    let dimension = SemanticDimension::new(
        DimensionId::new(),
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        50.0,
    )
    .unwrap();
    let dimension_id = dimension.id;
    store.insert_dimension(dimension).unwrap();

    let mut graph = MultiviewGraph::new();
    for _ in 0..view_count {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(PrincipalViewIdentity::Front);
        graph
            .bind_axis(&view, SharedAxis::Width, dimension_id)
            .unwrap();
    }

    (store, graph, dimension_id)
}

fn bench_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("propagate_confirmed_value");
    for &view_count in &[4usize, 64usize] {
        let (mut store, graph, dimension_id) = setup(view_count);
        let mut value = 50.0;
        group.bench_function(format!("views_{view_count}"), |b| {
            b.iter(|| {
                value += 1.0;
                propagate_confirmed_value(
                    std::hint::black_box(&mut store),
                    std::hint::black_box(&graph),
                    dimension_id,
                    value,
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_propagation);
criterion_main!(benches);
