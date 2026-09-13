//! Large-document serialization benchmark.
//!
//! Execution 01, Phase 27, Task 196. Authority: MCP Article 386
//! "Architecture Principle: Deterministic Serialization"; Engine Contract
//! 15 (Document). Benchmarks `to_canonical_json` against a document with
//! hundreds of entities -- a proxy for "large document operations,"
//! since save/autosave/diagnostic-export cost scales with entity count
//! and this is the one operation every one of those paths shares.

use craftloop_document::{Document, Note, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, NoteId};
use craftloop_serialization::to_canonical_json;
use criterion::{criterion_group, criterion_main, Criterion};

fn document_with_notes(note_count: usize) -> Document {
    let mut document = Document::new("Large Bench Document", 0.0);
    let page_id = document.active_page().unwrap();
    let page = document.page_mut(page_id).unwrap();
    for i in 0..note_count {
        let note = Note::new(
            NoteId::new(),
            Point2::new(i as f64, (i as f64) * 0.5),
            format!("Note {i}: dimension callout text of realistic length"),
        );
        page.insert(SemanticEntity::Note(note)).unwrap();
    }
    document
}

fn bench_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_canonical_json_large_document");
    for &note_count in &[100usize, 1000usize] {
        let document = document_with_notes(note_count);
        group.bench_function(format!("notes_{note_count}"), |b| {
            b.iter(|| to_canonical_json(std::hint::black_box(&document)).unwrap());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_serialize);
criterion_main!(benches);
