# Phase 05 — Raw Ink Model and Stroke Processing — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-ink` (Engine Contract 02), depending on `craftloop-errors`,
`craftloop-ids`, `craftloop-geometry`, `craftloop-input`,
`craftloop-serialization`. Dev-depends on `craftloop-test-support`. No
platform/UI dependency. `craftloop-errors` gained a new `DomainError::Ink`
variant (`InkErrorKind::{EmptyStroke, MixedSource}`), following the same
per-subsystem structured-error pattern as Input/Transaction/etc.

## Tasks 034–039

| Task | Module | Summary |
|---|---|---|
| 034 Raw stroke storage | `stroke.rs` | `Stroke { id: StrokeId, samples }`. Rejects empty and mixed-`PointerSource` construction as structured errors. `from_events()` builds directly from a completed lifecycle's `PointerEvent`s (skipping `CaptureLost`, which carries no sample). |
| 035 Stroke grouping | `grouping.rs` | `group_by_proximity()`: pure function, reversible (returns `Vec<Vec<StrokeId>>`, never mutates or merges strokes). Groups chronologically-sorted strokes by a time-gap **and** spatial-gap tolerance. |
| 036 Smoothing pipeline | `smoothing.rs` | `smoothed_positions()`: symmetric moving-average smoother. Keeps raw path (`Stroke::samples`), smoothed presentation path (this function's output), and structured geometry (Phase 06, not here) as three distinct representations, per the task's own wording. |
| 037 Bounds and spatial index hooks | `spatial.rs` | `Stroke::bounds()`, `StrokeSpatialIndex` (naive O(n) bounding-box scan behind a query interface, explicitly documented as a placeholder for a real spatial structure once profiling justifies one — not fabricated "efficiency"). |
| 038 Stroke provenance | `provenance.rs` | `StrokeProvenance`/`ProvenanceEvent`, append-only. Only the `Created` variant exists; later interpretation-event variants (recognition, acceptance, rejection) are added alongside the engines that produce them (Phase 06+), not stubbed now. |
| 039 Replay/serialization tests | `tests/replay_and_serialization.rs` | 6 integration tests: byte-identical repeated JSON serialization, reload-then-resmooth equality, a `DeterministicIdSequence`-driven reproducible-bug workflow, provenance round trip, rejection of an empty-events stroke, and a check that replayed-simulated-stroke capabilities never claim real hardware. |

## Bugs found and fixed via the Loop Engineering Contract

Two real production bugs in `smoothing.rs`, both caught by unit tests on
first run, not found by inspection:

1. **Endpoints were not preserved.** The initial window calculation applied
   an asymmetric clamped window to every index including 0 and the last
   index, so `smoothed_positions` visibly moved the stroke's start/end
   points — directly violating the function's own doc comment. Fixed by
   special-casing `i == 0 || i == last` to return the original point
   unchanged.
2. **A straight line was distorted near its ends even after fix 1.**
   Clamping `lo`/`hi` independently (rather than shrinking the window
   *symmetrically*) meant an interior point one step from either endpoint
   still averaged an asymmetric neighborhood, which is mathematically
   guaranteed to shift off a linear sequence's true value. Fixed by
   computing `radius = window.min(i).min(last - i)` and using a always-
   symmetric `[i - radius, i + radius]` window.

A third failure was a genuine test bug, not a production bug: an early
version of `grouping_does_not_depend_on_input_order` constructed two
independent sets of `Stroke`s (via `stroke_at(...)` called twice per
logical stroke), which get distinct random `StrokeId`s — the test was
comparing groups of different strokes, not the same strokes in different
order. Fixed by constructing each stroke once and reusing/cloning it into
both orderings.

## Commands and results

```
cargo build -p craftloop-ink            # clean
cargo test -p craftloop-ink             # 27/27 unit + 6/6 integration
                                         #   (3 real failures found and fixed on first run,
                                         #   see "Bugs found" above)
cargo fmt --all -- --check              # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
  # 1 finding: len_without_is_empty on Stroke; added Stroke::is_empty()
  #   (documented as always false, since construction rejects empty samples).
  # Re-run clean.
cargo test --workspace                  # 198/198 tests passing workspace-wide
```

Full output: `test-reports/phase-05-cargo-test.txt`,
`test-reports/phase-05-cargo-clippy.txt`.

## Architecture note: windows-harness was not refactored to use `Stroke`

`apps/windows-harness/src/state.rs`'s `RecordedStroke` remains a thin
`{ events: Vec<PointerEvent> }` rather than being migrated to wrap
`craftloop_ink::Stroke`. It does not duplicate any of `Stroke`'s validation
logic (the harness's own `StrokeLifecycleValidator` already guarantees
well-formed sequences before a `RecordedStroke` is created), so this is not
duplicated business logic in the sense Engine Contract 16 warns against —
just two independently-valid representations. Per the Agent Operating
Directive ("do not rewrite working code merely because another
implementation is aesthetically preferred" / "prefer the smallest coherent
change"), this migration is deferred rather than done speculatively; it
remains a reasonable follow-up once a phase actually needs `Stroke`'s
`bounds()`/grouping/smoothing from inside the harness.

## Deferred (explicitly, not silently)

- `StrokeSpatialIndex` stays a naive O(n) scan until real performance data
  (Article 91) justifies a tree/grid structure.
- `ProvenanceEvent` variants beyond `Created` — added with Phase 06+
  recognition/beautification/command engines.
- Migrating `windows-harness` to use `craftloop_ink::Stroke` — see above.

## Phase Gate

- All six tasks (034–039) represented in repository code with passing
  tests, including three real bugs caught and fixed by the tests
  themselves.
- `cargo test --workspace`: 198/198 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no recognition logic, no UI dependency, no
  speculative spatial-index optimization.
- Proceeding to Phase 06 automatically.
