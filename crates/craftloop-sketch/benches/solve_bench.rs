//! Constraint solving benchmark.
//!
//! Execution 01, Phase 27, Task 196. Authority: Engine Contract 10
//! (Constraint Solver). Benchmarks `EzpzSolver::solve` (Phase 12's real
//! numeric backend, Task 082's spike outcome) on a small chained system
//! of distance/fixed-value constraints -- representative of a sketch a
//! user might actually draw (a handful of dimensioned points), not a
//! synthetic worst case.

use craftloop_constraint::{ConstraintRequest, ConstraintSolver, GeometricConstraint, Variable};
use craftloop_ids::{ConstraintId, CraftLoopId};
use craftloop_sketch::EzpzSolver;
use criterion::{criterion_group, criterion_main, Criterion};

/// A chain of `point_count` points, each a fixed distance from the last,
/// starting from a pinned origin -- solvable, well-determined, and large
/// enough (at the higher point count) to show real solve cost.
fn chain_system(point_count: usize) -> (Vec<Variable>, Vec<ConstraintRequest>) {
    let mut variables = Vec::new();
    let mut constraints = Vec::new();

    let origin_x = craftloop_constraint::VariableId(0);
    let origin_y = craftloop_constraint::VariableId(1);
    variables.push(Variable::new(origin_x, 0.1)); // slightly off so the
    variables.push(Variable::new(origin_y, -0.1)); // solver has real work to do
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

    let mut previous = craftloop_constraint::PointVariables::new(origin_x, origin_y);
    for i in 0..point_count {
        let x = craftloop_constraint::VariableId((2 + i * 2) as u64);
        let y = craftloop_constraint::VariableId((3 + i * 2) as u64);
        variables.push(Variable::new(x, (i as f64) + 0.9));
        variables.push(Variable::new(y, 0.1));
        let current = craftloop_constraint::PointVariables::new(x, y);
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

fn bench_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("ezpz_solve");
    for &point_count in &[5usize, 20usize] {
        let (variables, constraints) = chain_system(point_count);
        group.bench_function(format!("chain_{point_count}"), |b| {
            b.iter(|| {
                let mut solver = EzpzSolver;
                solver.solve(
                    std::hint::black_box(&variables),
                    std::hint::black_box(&constraints),
                )
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
