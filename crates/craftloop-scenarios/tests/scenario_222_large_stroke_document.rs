//! Task 222 — Measure large-stroke document behavior.
//!
//! Execution 01, Phase 31, Task 222. A stress fixture with thousands of
//! strokes, checking two real, measurable proxies for "memory/rendering
//! remains bounded" that this sandbox can actually verify without a
//! heap profiler: wall-clock time stays within a generous bound (no
//! accidental O(n^2)/O(n^3) blowup as stroke count grows), and
//! serialized size scales linearly with stroke count (no per-entity
//! overhead that grows with document size, which would indicate an
//! accidental quadratic data structure somewhere in the page/entity
//! storage or canonical-JSON path).

use std::time::Instant;

use craftloop_document::{Document, Page, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, PageId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{MouseSimulator, PointerButtons};

fn stroke_with_n_samples(n: usize, offset: f64) -> Stroke {
    let samples = (0..n)
        .map(|i| {
            MouseSimulator::sample(
                Point2::new(offset + i as f64 * 0.5, offset),
                i as f64 * 0.01,
                PointerButtons::default(),
            )
        })
        .collect();
    Stroke::new(StrokeId::new(), samples).unwrap()
}

fn page_with_n_strokes(n: usize) -> Page {
    let mut page = Page::new(PageId::new(), "Stress page");
    for i in 0..n {
        let stroke = stroke_with_n_samples(5, i as f64);
        page.insert(SemanticEntity::Stroke(stroke)).unwrap();
    }
    page
}

#[test]
fn inserting_and_serializing_thousands_of_strokes_stays_within_a_generous_time_bound() {
    const STROKE_COUNT: usize = 3000;

    let start = Instant::now();
    let mut document = Document::new("Scenario 222", 0.0);
    let page_id = document.active_page().unwrap();
    // Replace the default empty page's contents in place: insert
    // STROKE_COUNT real strokes through the same `Page::insert` path a
    // real editing session uses, not a bulk shortcut.
    for i in 0..STROKE_COUNT {
        let stroke = stroke_with_n_samples(5, i as f64);
        document
            .page_mut(page_id)
            .unwrap()
            .insert(SemanticEntity::Stroke(stroke))
            .unwrap();
    }
    let insert_elapsed = start.elapsed();
    assert_eq!(document.page(page_id).unwrap().len(), STROKE_COUNT);

    let serialize_start = Instant::now();
    let json = craftloop_serialization::to_canonical_json(&document).unwrap();
    let serialize_elapsed = serialize_start.elapsed();
    eprintln!(
        "Task 222: inserting {STROKE_COUNT} strokes took {insert_elapsed:?}; serializing them took {serialize_elapsed:?} ({} bytes)",
        json.len()
    );

    // Generous bounds: this is a correctness/scaling gate, not a
    // performance benchmark (Task 223/224 own that) -- a real O(n^2)
    // regression on 3000 strokes would blow past these by orders of
    // magnitude, while a healthy O(n) or O(n log n) implementation
    // finishes in well under a second on any real development machine.
    assert!(
        insert_elapsed.as_secs() < 10,
        "inserting {STROKE_COUNT} strokes took {insert_elapsed:?}, suggesting non-linear blowup"
    );
    assert!(
        serialize_elapsed.as_secs() < 10,
        "serializing {STROKE_COUNT} strokes took {serialize_elapsed:?}, suggesting non-linear blowup"
    );
    assert!(!json.is_empty());
}

#[test]
fn serialized_size_scales_linearly_with_stroke_count_not_quadratically() {
    let small = page_with_n_strokes(200);
    let large = page_with_n_strokes(2000);

    let small_json = serde_json::to_string(&small).unwrap();
    let large_json = serde_json::to_string(&large).unwrap();

    let bytes_per_stroke_small = small_json.len() as f64 / 200.0;
    let bytes_per_stroke_large = large_json.len() as f64 / 2000.0;

    // A healthy, linear-in-entity-count storage/serialization path keeps
    // per-stroke overhead essentially constant as the document grows 10x
    // larger. A generous 2x tolerance catches a real accidental
    // quadratic blowup (which would show a much larger ratio) without
    // being a brittle exact-byte-count assertion.
    let ratio = bytes_per_stroke_large / bytes_per_stroke_small;
    eprintln!(
        "Task 222: {:.2} bytes/stroke at 200 strokes, {:.2} bytes/stroke at 2000 strokes (ratio {ratio:.3})",
        bytes_per_stroke_small, bytes_per_stroke_large
    );
    assert!(
        ratio < 2.0,
        "bytes-per-stroke grew by {ratio:.2}x from 200 to 2000 strokes -- expected roughly constant, suggesting non-linear serialized size"
    );
}
