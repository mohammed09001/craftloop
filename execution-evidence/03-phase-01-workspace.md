# Phase 01 — Workspace and Shared Core Skeleton — Evidence

Recorded: 2026-09-13

## Task 007 — Rust workspace

Created root `Cargo.toml` (workspace, `resolver = "2"`) plus `crates/` (domain
crates) and `apps/` (platform adapters/test applications — empty until Phase
04). Domain crates depend only on `serde`, `serde_json`, `thiserror`, `uuid`,
and each other — no UI/platform crate exists yet for them to accidentally
depend on, so the "compiles independently of Android/iOS/Windows UI" boundary
holds trivially today and will be enforced by continued absence of such a
dependency edge as `apps/windows-harness` is added in Phase 04.

## Tasks 008–012 — crates created

| Crate | Task | Purpose |
|---|---|---|
| `crates/craftloop-ids` | 008 | Strongly typed IDs: `StrokeId`, `PrimitiveId`, `DimensionId`, `ConstraintId`, `ViewId`, `OrthographicSetId`, `TransactionId`, `ConflictId`, `SuggestionId`, via a `CraftLoopId` trait + `define_id!` macro. |
| `crates/craftloop-errors` | 009 | `DomainError` enum (Geometry/Parser/Solver/Consistency/Persistence/Transaction variants, each with a typed `kind`), `Severity`, `Diagnostic`. |
| `crates/craftloop-serialization` | 010 | `SchemaVersion`, `OrderedF64` (finite-only, total-ordered), `to_canonical_json` (sorted object keys via `serde_json::Value`, never `preserve_order`). |
| `crates/craftloop-transactions` | 011 | `Transaction<C>` (record/commit/rollback), `CommittedTransaction<C>`, `TransactionLog<C>` (undo/redo over whole transactions). |
| `crates/craftloop-test-support` | 012 | `DeterministicIdSequence<T>`. Geometry/pointer/dimension/view builders are explicitly deferred to Phases 02/03/10/20 (see crate doc comment) rather than stubbed now. |

## Commands run

```
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check      # found 2 files needing reflow, fixed with `cargo fmt --all`
cargo clippy --workspace --all-targets -- -D warnings
```

## Results

- `cargo build --workspace`: succeeded, 31 packages locked/downloaded from
  crates.io (network access confirmed available), 0 errors.
- `cargo test --workspace`: **34/34 tests passed**, 0 failed
  (`craftloop-ids` 7, `craftloop-errors` 7, `craftloop-serialization` 7,
  `craftloop-test-support` 5, `craftloop-transactions` 8). Full output:
  `execution-evidence/test-reports/phase-01-cargo-test.txt`.
- `cargo fmt --all -- --check`: initially found unformatted output in
  `craftloop-ids` and `craftloop-transactions` (macro-call and assert
  wrapping); resolved by running `cargo fmt --all`; re-check is clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: found one real
  issue — `DeterministicIdSequence::next` shadowed
  `std::iter::Iterator::next`'s name while returning a different type
  (`clippy::should_implement_trait`). Renamed the inherent method to
  `next_id`; `Iterator::next` now delegates to it. Re-run is clean (0
  warnings). Full output: `execution-evidence/test-reports/phase-01-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- `apps/windows-harness` does not exist yet — Phase 04.
- Geometry, pointer-sample, dimension, and view test builders in
  `craftloop-test-support` — added alongside their domain types in Phases 02,
  03, 10, 20 respectively, per that crate's module doc.
- No workspace-level CI config yet — Phase 27.

## Phase Gate

- All six tasks (007–012) are represented in repository code with passing
  tests, not merely a compiling stub.
- `cargo test --workspace` and `cargo clippy -- -D warnings` are both green.
- No scope boundary crossed: no geometry/dimension/view logic was added
  early; no platform/UI dependency was introduced.
- Repository remains buildable (`cargo build --workspace` succeeds) and the
  pre-existing Python harness is untouched and still passes (Phase 00
  baseline unaffected).
- Proceeding to Phase 02 automatically.
