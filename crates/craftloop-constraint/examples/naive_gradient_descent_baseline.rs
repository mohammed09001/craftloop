//! Execution 01, Phase 11, Task 080: a minimal in-house solver prototype,
//! compared against `ezpz` (see `execution-evidence/solver-evaluations/`)
//! so the eventual backend choice is evidence-based rather than
//! popularity-based.
//!
//! Deliberately crude: finite-difference numerical gradient descent on the
//! sum of squared residuals (using this crate's own `residual()`
//! function), no analytic Jacobian, no line search, fixed step size. This
//! is *not* a candidate for production use -- it exists to answer "is
//! writing our own solver obviously easier/comparable to integrating
//! ezpz," and the honest answer, demonstrated by the numbers this prints,
//! is no: it is both slower and far less robust (see the decision
//! record).
//!
//! Run with `cargo run -p craftloop-constraint --example naive_gradient_descent_baseline --release`.

use std::collections::BTreeMap;
use std::time::Instant;

use craftloop_constraint::{residual, GeometricConstraint, PointVariables, VariableId};

fn total_cost(constraints: &[GeometricConstraint], values: &BTreeMap<VariableId, f64>) -> f64 {
    constraints
        .iter()
        .map(|c| residual(c, values).powi(2))
        .sum()
}

/// Extremely simple finite-difference gradient descent. Returns
/// (converged, iterations_used, final_values).
fn naive_solve(
    variable_ids: &[VariableId],
    constraints: &[GeometricConstraint],
    mut values: BTreeMap<VariableId, f64>,
    max_iterations: usize,
    tolerance: f64,
) -> (bool, usize, BTreeMap<VariableId, f64>) {
    const H: f64 = 1e-6;
    const STEP: f64 = 0.05;

    for iteration in 0..max_iterations {
        let cost = total_cost(constraints, &values);
        if cost.sqrt() < tolerance {
            return (true, iteration, values);
        }

        let mut gradient = BTreeMap::new();
        for &id in variable_ids {
            let original = *values.get(&id).unwrap_or(&0.0);
            values.insert(id, original + H);
            let cost_plus = total_cost(constraints, &values);
            values.insert(id, original);
            let d = (cost_plus - cost) / H;
            gradient.insert(id, d);
        }

        for &id in variable_ids {
            let g = gradient[&id];
            let current = *values.get(&id).unwrap_or(&0.0);
            values.insert(id, current - STEP * g);
        }
    }

    let final_cost = total_cost(constraints, &values).sqrt();
    (final_cost < tolerance, max_iterations, values)
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    values[values.len() / 2]
}

fn small_two_point_distance() {
    println!("\n=== In-house baseline: 2 points, Fixed + Distance ===");
    let px = VariableId(0);
    let py = VariableId(1);
    let qx = VariableId(2);
    let qy = VariableId(3);
    let p = PointVariables::new(px, py);
    let q = PointVariables::new(qx, qy);

    let constraints = vec![
        GeometricConstraint::FixedValue {
            variable: px,
            value: 0.0,
        },
        GeometricConstraint::FixedValue {
            variable: py,
            value: 0.0,
        },
        GeometricConstraint::Distance {
            a: p,
            b: q,
            value: 4.0,
        },
    ];

    let mut timings = Vec::new();
    let mut converged_count = 0;
    for _ in 0..50 {
        let values: BTreeMap<VariableId, f64> = [(px, 0.0), (py, -0.02), (qx, 4.39), (qy, 4.38)]
            .into_iter()
            .collect();
        let start = Instant::now();
        let (converged, iterations, _) =
            naive_solve(&[px, py, qx, qy], &constraints, values, 5000, 1e-6);
        timings.push(start.elapsed().as_secs_f64() * 1000.0);
        if converged {
            converged_count += 1;
        }
        if converged_count == 1 {
            println!("first run: converged={converged} iterations_used={iterations}");
        }
    }
    println!("converged {converged_count}/50 runs");
    println!(
        "solve latency (ms) over 50 runs: min={:.4} median={:.4} max={:.4}",
        timings.iter().cloned().fold(f64::INFINITY, f64::min),
        median(timings.clone()),
        timings.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    );
}

fn main() {
    small_two_point_distance();
    println!(
        "\nSee execution-evidence/solver-evaluations/solver-decision-record.md \
         for the side-by-side comparison against ezpz."
    );
}
