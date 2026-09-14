//! Task 223 — Measure solver latency (p50/p95).
//!
//! Execution 01, Phase 31, Task 223. Records p50/p95 solve times for
//! representative small (5-point chain) and medium (20-point chain)
//! constraint graphs, against the real `ezpz`-backed `EzpzSolver`
//! (the same chain-of-distance-constraints system Phase 27's
//! `benches/solve_bench.rs` uses for criterion's own statistics) --
//! this example instead computes literal p50/p95 percentiles from many
//! repeated real solves, printed for evidence capture, per the task's
//! own explicit wording ("record p50/p95").
//!
//! Run with: `cargo run --release --example solver_latency_percentiles -p craftloop-sketch`

use std::time::Instant;

use craftloop_constraint::{
    ConstraintRequest, ConstraintSolver, GeometricConstraint, PointVariables, Variable, VariableId,
};
use craftloop_ids::{ConstraintId, CraftLoopId};
use craftloop_sketch::EzpzSolver;

fn chain_system(point_count: usize) -> (Vec<Variable>, Vec<ConstraintRequest>) {
    let mut variables = Vec::new();
    let mut constraints = Vec::new();

    let origin_x = VariableId(0);
    let origin_y = VariableId(1);
    variables.push(Variable::new(origin_x, 0.1));
    variables.push(Variable::new(origin_y, -0.1));
    constraints.push(ConstraintRequest {
        id: ConstraintId::new(),
        constraint: GeometricConstraint::FixedValue {
            variable: origin_x,
            value: 0.0,
        },
    });
    constraints.push(ConstraintRequest {
        id: ConstraintId::new(),
        constraint: GeometricConstraint::FixedValue {
            variable: origin_y,
            value: 0.0,
        },
    });

    let mut previous = PointVariables::new(origin_x, origin_y);
    for i in 0..point_count {
        let x = VariableId((2 + i * 2) as u64);
        let y = VariableId((3 + i * 2) as u64);
        variables.push(Variable::new(x, (i as f64) + 0.9));
        variables.push(Variable::new(y, 0.1));
        let current = PointVariables::new(x, y);
        constraints.push(ConstraintRequest {
            id: ConstraintId::new(),
            constraint: GeometricConstraint::Distance {
                a: previous,
                b: current,
                value: 1.0,
            },
        });
        previous = current;
    }

    (variables, constraints)
}

fn percentile(mut samples: Vec<f64>, p: f64) -> f64 {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let rank = ((p / 100.0) * (samples.len() as f64 - 1.0)).round() as usize;
    samples[rank]
}

fn measure(point_count: usize, iterations: usize) -> (f64, f64) {
    let (variables, constraints) = chain_system(point_count);
    let mut samples_micros = Vec::with_capacity(iterations);
    // Warm-up, excluded from measurement (first-call allocator/cache effects).
    for _ in 0..20 {
        let mut solver = EzpzSolver;
        let _ = solver.solve(&variables, &constraints);
    }
    for _ in 0..iterations {
        let mut solver = EzpzSolver;
        let start = Instant::now();
        let result = solver.solve(&variables, &constraints);
        let elapsed = start.elapsed();
        assert_eq!(result.status, craftloop_constraint::SolveStatus::Solved);
        samples_micros.push(elapsed.as_secs_f64() * 1e6);
    }
    let p50 = percentile(samples_micros.clone(), 50.0);
    let p95 = percentile(samples_micros, 95.0);
    (p50, p95)
}

fn main() {
    const ITERATIONS: usize = 500;
    for &point_count in &[5usize, 20usize] {
        let (p50, p95) = measure(point_count, ITERATIONS);
        println!("chain_{point_count} (n={ITERATIONS}): p50 = {p50:.1} us, p95 = {p95:.1} us");
    }
}
