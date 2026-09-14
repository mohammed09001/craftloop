//! Task 224 — Measure propagation latency (p50/p95).
//!
//! Execution 01, Phase 31, Task 224. Records p50/p95 shared-dimension
//! update time across multiple bound views, using the same real
//! `propagate_confirmed_value` path Phase 27's `benches/propagation_bench.rs`
//! uses for criterion's own statistics -- this example instead computes
//! literal p50/p95 percentiles from many repeated real calls, printed
//! for evidence capture, per the task's own explicit wording.
//!
//! Run with: `cargo run --release --example propagation_latency_percentiles -p craftloop-document`

use std::time::Instant;

use craftloop_dimension::{
    DimensionKind, DimensionRole, DimensionStore, DimensionTarget, SemanticDimension,
};
use craftloop_document::{
    propagate_confirmed_value, MultiviewGraph, PrincipalViewIdentity, SharedAxis, ViewBlock,
};
use craftloop_ids::{CraftLoopId, DimensionId, PrimitiveId, ViewId};

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

fn percentile(mut samples: Vec<f64>, p: f64) -> f64 {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let rank = ((p / 100.0) * (samples.len() as f64 - 1.0)).round() as usize;
    samples[rank]
}

fn measure(view_count: usize, iterations: usize) -> (f64, f64) {
    let (mut store, graph, dimension_id) = setup(view_count);
    let mut value = 50.0;
    // Warm-up.
    for _ in 0..20 {
        value += 1.0;
        propagate_confirmed_value(&mut store, &graph, dimension_id, value).unwrap();
    }
    let mut samples_micros = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        value += 1.0;
        let start = Instant::now();
        propagate_confirmed_value(&mut store, &graph, dimension_id, value).unwrap();
        samples_micros.push(start.elapsed().as_secs_f64() * 1e6);
    }
    let p50 = percentile(samples_micros.clone(), 50.0);
    let p95 = percentile(samples_micros, 95.0);
    (p50, p95)
}

fn main() {
    const ITERATIONS: usize = 2000;
    for &view_count in &[4usize, 64usize] {
        let (p50, p95) = measure(view_count, ITERATIONS);
        println!("views_{view_count} (n={ITERATIONS}): p50 = {p50:.2} us, p95 = {p95:.2} us");
    }
}
