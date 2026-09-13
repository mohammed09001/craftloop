//! Recognition benchmark.
//!
//! Execution 01, Phase 27, Task 196. Authority: Engine Contract 04
//! (Recognition). Benchmarks `recognize()` -- candidate fitting/ranking
//! across line, circle, arc, and rectangle candidates at once -- against
//! a synthetic noisy-line stroke at two sizes representative of a real
//! pen stroke (dozens of points) and a much longer one (hundreds), so a
//! future regression in fitting cost shows up here rather than only as a
//! vague "the app feels slower."

use craftloop_geometry::Point2;
use craftloop_recognition::recognize;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

/// A deterministic, slightly noisy near-straight line -- not perfectly
/// straight, so every fitter in `recognize()` (line, circle, arc,
/// rectangle) actually has to do real least-squares work rather than
/// hitting a degenerate shortcut.
fn synthetic_stroke(point_count: usize) -> Vec<Point2> {
    (0..point_count)
        .map(|i| {
            let t = i as f64;
            let noise = 0.05 * (t * 0.7).sin();
            Point2::new(t, 0.02 * t + noise)
        })
        .collect()
}

fn bench_recognize(c: &mut Criterion) {
    let mut group = c.benchmark_group("recognize");
    for &point_count in &[32usize, 256usize] {
        let points = synthetic_stroke(point_count);
        group.bench_with_input(
            BenchmarkId::from_parameter(point_count),
            &points,
            |b, points| {
                b.iter(|| recognize(std::hint::black_box(points)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_recognize);
criterion_main!(benches);
