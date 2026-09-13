//! Execution 01, Phase 11 technical spike: evaluate `ezpz` against Craft
//! Loop's required primitive constraints (Task 079), measure small/medium
//! solve latency (Task 081), and evaluate conflict diagnostics (Task 082).
//!
//! This is throwaway evaluation code, not part of the Craft Loop workspace
//! (see this directory's `Cargo.toml`). Run with `cargo run --release`
//! from this directory. Output was captured verbatim into
//! `../ezpz-spike-output.txt`.

use std::time::Instant;

use ezpz::{datatypes::inputs::DatumPoint, solve, Config, Constraint, ConstraintRequest, IdGenerator};

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    values[values.len() / 2]
}

/// Task 079: a small system exercising Distance + Fixed, exactly the
/// crate's own documented example -- confirms the basic integration works
/// at all before anything more elaborate.
fn small_two_point_distance() {
    println!("\n=== Small graph: 2 points, Fixed + Distance (Task 079/081) ===");
    let mut timings = Vec::new();
    let mut last_ok = false;
    for _ in 0..200 {
        let mut ids = IdGenerator::default();
        let p = DatumPoint::new(&mut ids);
        let q = DatumPoint::new(&mut ids);
        let requests = [
            ConstraintRequest::highest_priority(Constraint::Fixed(p.id_x(), 0.0)),
            ConstraintRequest::highest_priority(Constraint::Fixed(p.id_y(), 0.0)),
            ConstraintRequest::highest_priority(Constraint::Distance(p, q, 4.0)),
        ];
        let guesses = vec![(p.id_x(), 0.0), (p.id_y(), -0.02), (q.id_x(), 4.39), (q.id_y(), 4.38)];
        let start = Instant::now();
        let outcome = solve(&requests, guesses, Config::default());
        timings.push(start.elapsed().as_secs_f64() * 1000.0);
        last_ok = matches!(&outcome, Ok(s) if s.is_satisfied());
    }
    println!("last run satisfied: {last_ok}");
    println!(
        "solve latency (ms) over 200 runs: min={:.4} median={:.4} max={:.4}",
        timings.iter().cloned().fold(f64::INFINITY, f64::min),
        median(timings.clone()),
        timings.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    );
}

/// Task 079/081: a medium system -- an axis-aligned rectangle built from 4
/// points, using Horizontal/Vertical/Distance/LinesEqualLength, the same
/// vocabulary Craft Loop's `RelationalRectangle` (Phase 02) needs a
/// constraint layer for. 8 constraints, 8 variables.
fn medium_rectangle() {
    println!("\n=== Medium graph: 4-point rectangle, 8 constraints (Task 079/081) ===");
    let mut timings = Vec::new();
    let mut last_ok = false;
    for _ in 0..200 {
        let mut ids = IdGenerator::default();
        let p0 = DatumPoint::new(&mut ids);
        let p1 = DatumPoint::new(&mut ids);
        let p2 = DatumPoint::new(&mut ids);
        let p3 = DatumPoint::new(&mut ids);

        let edge01 = ezpz::datatypes::inputs::DatumLineSegment { p0, p1 };
        let edge12 = ezpz::datatypes::inputs::DatumLineSegment { p0: p1, p1: p2 };
        let edge23 = ezpz::datatypes::inputs::DatumLineSegment { p0: p2, p1: p3 };
        let edge30 = ezpz::datatypes::inputs::DatumLineSegment { p0: p3, p1: p0 };

        let requests = [
            ConstraintRequest::highest_priority(Constraint::Fixed(p0.id_x(), 0.0)),
            ConstraintRequest::highest_priority(Constraint::Fixed(p0.id_y(), 0.0)),
            ConstraintRequest::highest_priority(Constraint::Horizontal(edge01)),
            ConstraintRequest::highest_priority(Constraint::Vertical(edge12)),
            ConstraintRequest::highest_priority(Constraint::Horizontal(edge23)),
            ConstraintRequest::highest_priority(Constraint::Vertical(edge30)),
            ConstraintRequest::highest_priority(Constraint::HorizontalDistance(p0, p1, 20.0)),
            ConstraintRequest::highest_priority(Constraint::VerticalDistance(p1, p2, 10.0)),
        ];
        let guesses = vec![
            (p0.id_x(), 0.1),
            (p0.id_y(), -0.1),
            (p1.id_x(), 19.7),
            (p1.id_y(), 0.3),
            (p2.id_x(), 20.2),
            (p2.id_y(), 9.8),
            (p3.id_x(), 0.4),
            (p3.id_y(), 10.1),
        ];
        let start = Instant::now();
        let outcome = solve(&requests, guesses, Config::default());
        timings.push(start.elapsed().as_secs_f64() * 1000.0);
        last_ok = matches!(&outcome, Ok(s) if s.is_satisfied());
    }
    println!("last run satisfied: {last_ok}");
    println!(
        "solve latency (ms) over 200 runs: min={:.4} median={:.4} max={:.4}",
        timings.iter().cloned().fold(f64::INFINITY, f64::min),
        median(timings.clone()),
        timings.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    );
}

/// Task 082: deliberately request two contradictory Fixed values on the
/// same variable, and inspect what the solver reports.
fn conflict_diagnostics() {
    println!("\n=== Conflict diagnostics: two contradictory Fixed constraints (Task 082) ===");
    let mut ids = IdGenerator::default();
    let p = DatumPoint::new(&mut ids);
    let requests = [
        ConstraintRequest::highest_priority(Constraint::Fixed(p.id_x(), 0.0)),
        ConstraintRequest::highest_priority(Constraint::Fixed(p.id_x(), 10.0)), // contradicts the above
        ConstraintRequest::highest_priority(Constraint::Fixed(p.id_y(), 0.0)),
    ];
    let guesses = vec![(p.id_x(), 5.0), (p.id_y(), 0.0)];
    match solve(&requests, guesses, Config::default()) {
        Ok(outcome) => {
            println!("converged: {}", outcome.converged());
            println!("is_satisfied: {}", outcome.is_satisfied());
            println!("unsatisfied constraint indices: {:?}", outcome.unsatisfied());
            println!("iterations: {}", outcome.iterations());
            println!("warnings: {:?}", outcome.warnings());
            let solved_p = outcome.final_value_point(&p);
            println!("final P = ({}, {})", solved_p.x, solved_p.y);
            // Craft Loop's own residual() function (craftloop-constraint)
            // can recompute *how far off* an unsatisfied constraint is,
            // since ezpz exposes final_values but not per-constraint
            // residual magnitude on SolveOutcome itself -- see the
            // decision record's "diagnostics gap" note.
            println!(
                "recomputed residual for constraint 0 (Fixed x=0.0): {}",
                (solved_p.x - 0.0f64).abs()
            );
            println!(
                "recomputed residual for constraint 1 (Fixed x=10.0): {}",
                (solved_p.x - 10.0f64).abs()
            );
        }
        Err(failure) => {
            println!("solve failed outright: {}", failure.error());
            println!("num_vars={} num_eqs={}", failure.num_vars(), failure.num_eqs());
        }
    }
}

/// Task 079: an over-constrained (redundant but consistent) system --
/// three Fixed constraints all agreeing, plus a coincident constraint that
/// is automatically implied. Confirms ezpz does not choke on redundancy
/// when the redundant constraints are not actually contradictory.
fn redundant_but_consistent() {
    println!("\n=== Redundant-but-consistent system (Task 079) ===");
    let mut ids = IdGenerator::default();
    let p = DatumPoint::new(&mut ids);
    let q = DatumPoint::new(&mut ids);
    let requests = [
        ConstraintRequest::highest_priority(Constraint::Fixed(p.id_x(), 0.0)),
        ConstraintRequest::highest_priority(Constraint::Fixed(p.id_y(), 0.0)),
        ConstraintRequest::highest_priority(Constraint::Fixed(q.id_x(), 5.0)),
        ConstraintRequest::highest_priority(Constraint::Fixed(q.id_y(), 0.0)),
        // Redundant with the above four, but consistent with them:
        // NOTE: HorizontalDistance(p0, p1, d) is *signed*: residual is
        // (p0.x - p1.x) - d, not |p0.x - p1.x| - d. p.x=0, q.x=5 means
        // (p, q) gives 0-5=-5, so the correct redundant value is -5.0, not
        // +5.0 -- an easy sign mistake to make once, caught by this spike
        // itself (the first attempt used +5.0, ezpz correctly reported it
        // as unsatisfied, which sent us back to check the docs/source
        // rather than assume the solver was wrong).
        ConstraintRequest::highest_priority(Constraint::Distance(p, q, 5.0)),
        ConstraintRequest::highest_priority(Constraint::HorizontalDistance(p, q, -5.0)),
    ];
    let guesses = vec![(p.id_x(), 0.1), (p.id_y(), 0.1), (q.id_x(), 4.9), (q.id_y(), 0.1)];
    let outcome = solve(&requests, guesses, Config::default());
    match outcome {
        Ok(s) => {
            println!("is_satisfied: {}, iterations: {}", s.is_satisfied(), s.iterations());
            println!("unsatisfied: {:?}", s.unsatisfied());
            let solved_p = s.final_value_point(&p);
            let solved_q = s.final_value_point(&q);
            println!("final P=({}, {}) Q=({}, {})", solved_p.x, solved_p.y, solved_q.x, solved_q.y);
        }
        Err(e) => println!("solve failed: {}", e.error()),
    }
}

fn main() {
    small_two_point_distance();
    medium_rectangle();
    conflict_diagnostics();
    redundant_but_consistent();
}
