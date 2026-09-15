# Execution 03, Phase 04 — Web CraftLoopSession

Recorded 2026-09-15. Tasks 023-034.

## Design decision this whole phase rests on

`craftloop-mobile-ffi/src/session.rs`'s native `CraftLoopSession` (2234
lines) is UniFFI-shaped: `#[uniffi::export]`, `Mutex`-guarded interior
state (a mobile client may call in from more than one thread), and
`Ffi*` `Record`/`Enum` types UniFFI can cross the FFI boundary with
directly, including enums that carry data.

`wasm-bindgen` cannot cross an enum that carries data, and has no
ergonomic `Vec<CustomStruct>` parameter/return support. Rather than
inventing a parallel, thinner "session-ish" API, this phase ports the
**same** methods, over the **same** real engines, with the **same**
mutation discipline (scratch clone → real engine call → commit through
`DocumentHistory::commit`), adapted only at the boundary:

- No `Mutex`: a browser tab's JS is single-threaded; wasm-bindgen
  already gives each exported method exclusive `&mut self`.
- No `open(path)`/`save(path)`: replaced by `toJson()`/`fromJson()`
  (Task 034), since there is no filesystem on `wasm32-unknown-unknown`
  (Phase 03's finding).
- Every UniFFI `Record`/data-carrying `Enum` (`FfiSceneSnapshot`,
  `FfiConstraintKind`, `FfiSolveOutcome`, ...) becomes a JSON string in
  and out, built from a plain-data DTO in the new `types` module
  (`WebSceneSnapshot`, `WebConstraintKind`, ...) rather than a
  `#[wasm_bindgen]` struct wasm-bindgen cannot actually express for
  these shapes. Four *fieldless* enums (`WebDimensionKind`,
  `WebPrincipalViewIdentity`, `WebSharedAxis`, `WebResolutionChoice`)
  cross as real `#[wasm_bindgen]` TS enums instead, since wasm-bindgen
  supports that natively and it is more ergonomic than JSON for a
  bare-word parameter.

New files: `crates/craftloop-web-bridge/src/types.rs` (DTOs),
`crates/craftloop-web-bridge/src/session.rs` (the session itself, ~970
lines including tests).

## Task-by-task

- **023 Create browser session** — `#[wasm_bindgen] pub struct
  CraftLoopSession` with a `#[wasm_bindgen(constructor)] pub fn new()`.
- **024 Expose scene snapshot** — `sceneSnapshot() -> String` (JSON
  `WebSceneSnapshot`), built from the live `Document`/`DocumentHistory`
  exactly like native's `scene_snapshot`, including its real
  `.bounds()`-derived per-primitive bounding boxes.
- **025 Expose stroke submission** — `submitStroke(samples_json) ->
  Result<String, WebSessionError>`, routes through the same real
  `craftloop_recognition::{recognize, rank_candidates}` native uses,
  and `acceptRecognition(stroke_id)` through the same real `beautify`.
- **026 Expose primitive creation** — `createPrimitiveLine/Circle/
  Rectangle`. Arc/Ellipse are not exposed: native `CraftLoopSession`
  itself has no `create_primitive_arc`/`_ellipse` method to mirror
  (repository-first context, Article 81's own instruction: mirror what
  exists, don't invent ahead of it) — Phase 09/10's toolbar work is
  where those get a real decision.
- **027 Expose dimensions** — `createDimension`/`editDimension`,
  including native's exact conflict-on-invalid-edit behavior (commits a
  real `Conflict` entity and returns `Err`, never a silent no-op).
- **028 Expose constraints** — `applyConstraint`/`removeConstraint`/
  `solveConstraints`, against the same real `EzpzSolver`.
- **029 Expose View Identity** — `assignViewIdentity`.
- **030 Expose Orthographic transition** — `enterOrthographic`, same
  `evaluate_readiness`/`transition_to_orthographic` gate as native (a
  view that is not `LinkReady` is rejected with the same blocker list).
- **031 Expose shared-axis operations** — `propagateSharedValue`,
  including the same reuse-an-existing-binding-in-the-same-
  `OrthographicSet` logic and the same "propose, only commit a
  `Conflict` on a real mismatch" behavior.
- **032 Expose conflicts** — `resolveConflict`, including the
  `ReplaceAndPropagate` choice's replay of a pending shared-value
  proposal.
- **033 Expose undo/redo** — `undo`/`redo`/`canUndo`/`canRedo` over the
  same `DocumentHistory`.
- **034 Expose serialization** — `toJson()`/`fromJson()` using
  `craftloop_serialization::to_canonical_json`/`serde_json::from_str` +
  `Document::validate()` directly (Phase 03's documented replacement
  for the fs-based `save_document_atomically`/`load_document`).

## Test-first evidence

`cargo test -p craftloop-web-bridge`: **9 passed, 0 failed** (up from
Phase 03's 1). Ported from native `session.rs`'s own test module,
adapted to `&mut self` and JSON-string assertions via `serde_json::Value`
rather than typed `Ffi*` equality:

- a new session starts empty, no undo/redo;
- create-dimension → invalid edit creates a real conflict → undo clears
  it → a valid edit does take effect;
- per-primitive-kind bounding boxes match hand-computed expectations;
- `applyConstraint` detects redundancy (Article 312: not an error) and
  `solveConstraints` actually moves the primitive;
- select/delete-selected round-trips through the exact bare-UUID id
  format `sceneSnapshot()` itself emits (native's own real-bug
  regression test, ported unchanged in spirit);
- `select` on a UUID matching no real entity fails rather than silently
  selecting nothing;
- `toJson()`/`fromJson()` restores an equal scene with a fresh (empty)
  undo history;
- **the full Golden Alpha Journey** (Article 35), end to end: draw a
  line → dimension it → constrain it Horizontal → an incompatible edit
  creates and is undone past a real conflict → assign FRONT → enter
  Orthographic (Top/Right/Back created) → depth entered on TOP
  propagates to RIGHT without a second manual action → a contradictory
  value on RIGHT creates a real cross-view conflict → resolve
  `ReplaceAndPropagate` → undo/redo the resolution → `toJson`/`fromJson`
  round-trip preserves every semantic field except (correctly) undo
  history.

## Real browser evidence (Task 020's precedent, not compile-success alone)

Built release Wasm, generated JS glue with the same pinned
`wasm-bindgen-cli` 0.2.128, and drove the generated `CraftLoopSession`
class inside real Chromium (same throwaway-harness technique as Phase
03, cleaned up afterward):

```json
{
  "lineId": "d3ce0083-5319-424f-9414-b888478a8968",
  "dimId": "62d1b369-a7c9-438f-bbcc-2f5c989e8c60",
  "applyResult": { "Added": { "constraint_id": "e2d9cf4a-14af-4def-bccc-ef815d8f9a09" } },
  "solveResult": { "status": "Solved", "unsatisfied_constraint_ids": [], "updated_primitive_count": 1 },
  "solvedLineY": { "id": "d3ce0083-...", "kind": "Line", "max_x": 4, "max_y": 1.5000000000000002, "min_x": 0, "min_y": 1.4999999985 },
  "canUndoAfter": true,
  "reopenedPrimitiveCount": 1,
  "reopenedRevision": 5
}
```

A line from `(0,0)-(4,3)` constrained `Horizontal` and solved to
`y≈1.5` on both ends, undo left `canUndo() == true`, and a
`toJson()`/`fromJson()` round trip inside the same browser preserved
the primitive and the exact revision count (`5`). This exercised
`createPrimitiveLine`, `createDimension`, `applyConstraint`,
`solveConstraints`, `sceneSnapshot`, `undo`, `canUndo`, `toJson`, and
the static `fromJson` factory — real Chromium, real WebAssembly, real
solver.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cargo test -p craftloop-web-bridge                                  # 9/9
cargo build --target wasm32-unknown-unknown -p craftloop-web-bridge # exit 0
```

## Phase Gate — CLOSED

All twelve tasks have evidence above; 8 new native tests plus a real
Chromium run cover the ported API. Shared semantics remain
platform-neutral -- every method routes to the same domain crates
native `CraftLoopSession` uses; no logic was reimplemented at this
boundary, only the crossing shape changed (JSON DTOs / real
wasm-bindgen enums instead of UniFFI records). No unsupported
native-device claim made. `apps/web-live` itself is not yet wired to
this session (that is Phase 06's Browser Canvas Stack, which needs
Phase 05's snapshot enrichment first) -- this phase proves the API
works, deliberately not duplicating that wiring ahead of the phase that
owns it. Continuing automatically to Phase 05 (Scene Snapshot
Enrichment).
