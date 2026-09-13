//! Benchmark false-positive behavior.
//!
//! Execution 01, Phase 06, Task 047. Authority: MCP Article 96 "Recognition
//! Confidence"; Quality Measurement item 9 (Benchmarks).
//!
//! Measures two things, as the task requires, not just recognition
//! accuracy in isolation:
//!
//! 1. **True positives**: clean, ordinary shapes are recognized as the
//!    correct primitive kind, ranked first.
//! 2. **Forced-conversion errors (false positives)**: noisy,
//!    handwriting-like scribbles -- never intended as geometry -- do not
//!    get force-classified as a primitive at a confidence a caller would
//!    reasonably act on ("commit").
//!
//! All fixtures are deterministic (fixed formulas / fixed offset tables),
//! not randomly generated, so this benchmark's result is reproducible
//! evidence, not a flaky sample.

use craftloop_geometry::Point2;
use craftloop_recognition::{recognize, CandidateKind};

/// Confidence a caller would reasonably treat as "commit this
/// interpretation" rather than merely "worth listing as an option." Recall
/// `recognize()` already filters out anything below 0.3 entirely; this is a
/// stricter bar for "would this get force-converted."
const COMMIT_THRESHOLD: f64 = 0.6;

fn clean_line() -> Vec<Point2> {
    (0..24)
        .map(|i| Point2::new(i as f64, 1.5 * i as f64 + 4.0))
        .collect()
}

fn clean_circle() -> Vec<Point2> {
    (0..32)
        .map(|i| {
            let a = std::f64::consts::TAU * i as f64 / 32.0;
            Point2::new(12.0 + 6.0 * a.cos(), -3.0 + 6.0 * a.sin())
        })
        .collect()
}

fn clean_rectangle() -> Vec<Point2> {
    let corners = [
        Point2::new(0.0, 0.0),
        Point2::new(30.0, 0.0),
        Point2::new(30.0, 15.0),
        Point2::new(0.0, 15.0),
    ];
    let mut points = Vec::new();
    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        for step in 0..8 {
            points.push(a.lerp(b, step as f64 / 8.0));
        }
    }
    points.push(corners[0]);
    points
}

/// A deterministic "handwriting-like" scribble: a sum of two out-of-phase
/// sine waves, the same qualitative shape a cursive word or a rough
/// sketch-shading scribble produces -- never intended to be read as a
/// single clean primitive.
fn handwriting_scribble(seed: f64) -> Vec<Point2> {
    (0..40)
        .map(|i| {
            let t = i as f64 / 39.0 * 10.0;
            let x = t * 3.0 + seed;
            let y = 3.0 * (t + seed).sin() + 1.2 * (2.3 * t + seed * 0.7).sin();
            Point2::new(x, y)
        })
        .collect()
}

/// A small closed, deliberately irregular scrawl (the kind of loose loop a
/// hand makes while thinking, not a deliberate circle/rectangle attempt).
fn closed_scrawl(seed: f64) -> Vec<Point2> {
    (0..30)
        .map(|i| {
            let t = i as f64 / 29.0 * std::f64::consts::TAU;
            let wobble = 2.5 * (t * 3.3 + seed).sin() + 1.1 * (t * 7.1 - seed).cos();
            let r = 8.0 + wobble;
            Point2::new(r * t.cos(), r * t.sin())
        })
        .collect()
}

struct BenchmarkReport {
    true_positives: usize,
    true_positive_total: usize,
    forced_conversions: usize,
    noisy_total: usize,
}

#[test]
fn ordinary_shapes_are_recognized_as_their_correct_kind() {
    let cases: Vec<(&str, Vec<Point2>, CandidateKind)> = vec![
        ("clean line", clean_line(), CandidateKind::Line),
        ("clean circle", clean_circle(), CandidateKind::Circle),
        (
            "clean rectangle",
            clean_rectangle(),
            CandidateKind::Rectangle,
        ),
    ];

    let mut report = BenchmarkReport {
        true_positives: 0,
        true_positive_total: cases.len(),
        forced_conversions: 0,
        noisy_total: 0,
    };

    for (name, points, expected_kind) in &cases {
        let candidates = recognize(points);
        let top = &candidates[0];
        assert_eq!(
            top.kind(),
            *expected_kind,
            "{name}: expected top candidate {:?}, got {:?} (all: {:?})",
            expected_kind,
            top.kind(),
            candidates
                .iter()
                .map(|c| (c.kind(), c.confidence().get()))
                .collect::<Vec<_>>()
        );
        assert!(
            top.confidence().get() >= COMMIT_THRESHOLD,
            "{name}: expected a commit-worthy confidence, got {}",
            top.confidence().get()
        );
        report.true_positives += 1;
    }

    assert_eq!(report.true_positives, report.true_positive_total);
    assert_eq!(report.forced_conversions, 0);
}

#[test]
fn noisy_handwriting_and_scrawls_are_never_forced_into_a_high_confidence_primitive() {
    let seeds = [0.0, 1.3, 2.7, 4.1, 5.9];
    let mut samples: Vec<(String, Vec<Point2>)> = Vec::new();
    for &seed in &seeds {
        samples.push((format!("handwriting-{seed}"), handwriting_scribble(seed)));
        samples.push((format!("scrawl-{seed}"), closed_scrawl(seed)));
    }

    let mut forced_conversions = 0usize;
    let mut offending: Vec<String> = Vec::new();

    for (name, points) in &samples {
        let candidates = recognize(points);
        for candidate in &candidates {
            if candidate.kind() != CandidateKind::KeepAsInk
                && candidate.confidence().get() >= COMMIT_THRESHOLD
            {
                forced_conversions += 1;
                offending.push(format!(
                    "{name} -> {:?} @ {:.2}",
                    candidate.kind(),
                    candidate.confidence().get()
                ));
            }
        }
    }

    assert_eq!(
        forced_conversions, 0,
        "noisy/handwriting samples must never be offered at commit-level confidence; offenders: {offending:?}"
    );
}

#[test]
fn benchmark_report_covers_both_accuracy_and_forced_conversion_error() {
    // A single combined pass producing the report the task explicitly asks
    // for: not just "how many did we get right" but also "how many did we
    // wrongly force," in one place.
    let clean_cases: Vec<(Vec<Point2>, CandidateKind)> = vec![
        (clean_line(), CandidateKind::Line),
        (clean_circle(), CandidateKind::Circle),
        (clean_rectangle(), CandidateKind::Rectangle),
    ];

    let noisy_cases: Vec<Vec<Point2>> = (0..5)
        .map(|i| handwriting_scribble(i as f64 * 1.1))
        .collect();

    let mut report = BenchmarkReport {
        true_positives: 0,
        true_positive_total: clean_cases.len(),
        forced_conversions: 0,
        noisy_total: noisy_cases.len(),
    };

    for (points, expected_kind) in &clean_cases {
        let candidates = recognize(points);
        if candidates[0].kind() == *expected_kind
            && candidates[0].confidence().get() >= COMMIT_THRESHOLD
        {
            report.true_positives += 1;
        }
    }

    for points in &noisy_cases {
        let candidates = recognize(points);
        if candidates.iter().any(|c| {
            c.kind() != CandidateKind::KeepAsInk && c.confidence().get() >= COMMIT_THRESHOLD
        }) {
            report.forced_conversions += 1;
        }
    }

    assert_eq!(
        report.true_positives, report.true_positive_total,
        "recognition accuracy regressed on ordinary shapes"
    );
    assert_eq!(
        report.forced_conversions, 0,
        "forced-conversion error rate regressed on noisy samples ({}/{})",
        report.forced_conversions, report.noisy_total
    );
}
