# Execution 02, Phase 04 — CraftLoopSession Mobile API

Recorded 2026-09-14. Tasks 022-033. Authority: Article 12
(CraftLoopSession Mobile API), Article 13 (Scene Snapshot Contract),
Article 35 (Golden Alpha Journey). Pure Rust work — no Android
toolchain touched or required; this closes the shared-core integration
gap identified in `phase-00-baseline.md` (Execution 01 built ~20
independent domain engines but never wired them into one persisted,
undoable session).

## What was found before writing any code

`craftloop_document::Document` held only pages of `SemanticEntity` plus
provenance. `ViewBlock`/`OrthographicSet` (Phase 20), `MultiviewGraph`
(Phase 22), and `DimensionStore` (Phase 10) were each fully built and
unit-tested in isolation, but no scenario test that used more than one
of them (e.g. `scenario_216_front_to_orthographic.rs`) ever went through
a real `Document` — each constructed its own bare local instances.
`craftloop-sketch::Sketch` (constraint-bearing geometry, Phase 12) had
the same problem, plus it did not even derive `Serialize`. This made
Article 35's "save, kill the app, reopen it" step have nothing real to
round-trip. Closing this was necessarily Phase 04's first sub-task,
ahead of `CraftLoopSession` itself.

## Part A/B — `Document` extension (`craftloop-document`)

Added four fields to `Document`: `view_blocks: BTreeMap<ViewId,
ViewBlock>`, `orthographic_sets: BTreeMap<OrthographicSetId,
OrthographicSet>`, `multiview_graph: MultiviewGraph`, `dimension_store:
DimensionStore`, all part of the existing `#[derive(... Serialize,
Deserialize)]`. Read-only accessors are `pub`; mutation is
`pub(crate)`-only, reachable exclusively through four new
`DocumentChange` variants in `history.rs`, following the file's own
documented "state deltas, not inverse functions" convention exactly
(`previous`/`new` `Option` pairs, `invert()` swaps them):
`SetViewBlock`, `SetOrthographicSet`, `SetMultiviewBinding`,
`SetDimension`. Every variant has a `DocumentHistory`-level test
(insert via commit, undo restores the prior state including the empty
case, redo restores forward).

`dimension_store` is **not** a replacement for `Page`'s
`SemanticEntity::Dimension` copies — see `document.rs`'s own doc
comment. The page copy is what Phase 07 already renders/tests;
`dimension_store` is the authoritative copy the multiview/propagation
engine (`propagation.rs`) actually mutates, since reusing its real
validation (`DimensionStore::edit_driving_value`) is required rather
than reimplemented. `CraftLoopSession` keeps the two mirrored, always
inside one `DocumentHistory::commit` transaction.

Two small, justified additions to existing crates (documented in their
own doc comments, each with its own test):
- `MultiviewGraph::insert_binding_unchecked` (craftloop-document) —
  `DocumentChange` only carries raw ids, not a `&ViewBlock`, so
  `bind_axis`'s own validation cannot be re-run on replay; this
  replays an already-validated binding, exactly as `bind_axis` itself
  produced it.
- `DimensionStore::set_dimension` (craftloop-dimension) — an
  unconditional replace-or-insert raw setter, since neither
  `insert_dimension` (rejects duplicates) nor `edit_driving_value`
  (only touches `.value`) could apply a whole-value snapshot. Kept
  infallible (`Option<SemanticDimension>` return, not
  `DomainResult<...>`) since a `BTreeMap::insert` cannot fail — a
  deliberate deviation from a stricter signature drafted before
  implementation.
- `DimensionStore::dimensions()` — an iterator over every stored
  dimension, needed so a scene snapshot can render shared/propagated
  dimensions that have no page placement at all (see Part D below).

## Part C — `Sketch` persistence (`craftloop-sketch`)

Every field `Sketch`/`ConstraintEntry` needed (`PrimitiveMap` =
`BTreeMap<PrimitiveId, Beautified>`, `SketchConstraintKind`,
`ConstraintProvenance`) already derived `Serialize`/`Deserialize` for
its own Phase 12 persistence. Added `#[derive(Debug, Clone, PartialEq,
Serialize, Deserialize)]` to both `Sketch` and `ConstraintEntry`
directly — no field type needed changing. Two round-trip tests added
(populated and empty). Also added `Sketch::remove_primitive`,
`Sketch::primitive_ids()`, `Sketch::set_constraint_unchecked`,
`Sketch::remove_constraint_unchecked` — each a minimal, undo-replay or
enumeration primitive `CraftLoopSession`/`DocumentChange` genuinely
needed and none of these crates exposed yet (each has its own doc
comment explaining why re-running `add_constraint`'s redundancy check
on redo would be actively wrong, not just redundant work).

**No deferral was needed** — constraints are fully wired into
`CraftLoopSession` (Article 17), unlike the brief's contingency for
this part.

## Part D/E/F — `CraftLoopSession` (`crates/craftloop-mobile-ffi/src/session.rs`, ~850 new lines)

A `#[derive(uniffi::Object)]` type wrapping `Mutex<SessionState>`
(`document`, `history`, `command_bus`, ephemeral `selection`, and
`pending_shared_value_proposals` — see below). Every method that
changes persisted state builds a scratch clone of whatever engine
state it needs (`Document::dimension_store()`/`multiview_graph()`/
`sketch()` all now `Clone`), calls the real engine function against
the clone, and commits the resulting delta through
`DocumentHistory::commit` — never mutating `state.document` directly.
This is what makes every operation undoable/redoable.

**Public API surface** (all under `#[uniffi::export] impl
CraftLoopSession`):

```
new() / open(path) -> Result<Self, _>          // constructors
save(path) -> Result<(), _>
scene_snapshot() -> FfiSceneSnapshot
debug_state() -> FfiDebugState
submit_stroke(samples) -> Result<FfiStrokeOutcome, _>
accept_recognition(stroke_id) -> Result<Option<String>, _>
create_primitive_line/circle/rectangle(...) -> Result<String, _>
select(ids) / clear_selection() / delete_selected(ids)
create_dimension(kind, target_ids, value) -> Result<String, _>
edit_dimension(id, new_value) -> Result<(), _>
apply_constraint(kind) -> Result<FfiConstraintOutcome, _>
remove_constraint(id) -> Result<(), _>
solve_constraints() -> Result<FfiSolveOutcome, _>
assign_view_identity(view_id?, identity) -> Result<String, _>
add_geometry_to_view(view_id, primitive_ids) -> Result<(), _>
enter_orthographic(view_id) -> Result<Vec<String>, _>
propagate_shared_value(view_id, axis, value) -> Result<FfiPropagateOutcome, _>
resolve_conflict(id, choice) -> Result<(), _>
undo() / redo() / can_undo() / can_redo()
```

Plus the FFI record/enum types Article 13 asks for:
`FfiSceneSnapshot`, `FfiDebugState`, `FfiStrokeOutcome`,
`FfiConstraintOutcome`, `FfiSolveOutcome`, `FfiPropagateOutcome`, and
mirror enums for every domain enum crossed (`FfiPrincipalViewIdentity`,
`FfiSharedAxis`, `FfiDimensionKind/Role`, `FfiResolutionChoice`,
`FfiConflictKind`, `FfiConstraintKind`, `FfiPrimitiveKind`,
`FfiSolveStatus`).

### Deviations from the brief, and why

- **`resolve_command` was not added as a session method.** The
  existing free function in `lib.rs` (unchanged since Phase 28) is
  left as the sole entry point — adding a session wrapper would only
  duplicate a one-line delegation for no real caller benefit yet.
- **Command Bus mapping uses `RiskLevel::Low` uniformly.** Every
  Execution 02 Alpha action is an explicit, already-decided user
  action (a toolbar tap, a typed value), never risky/ambiguous
  ink-derived input needing gathered confirmation evidence (that is
  Phase 15's Ink Command Adapter). `requires_confirmation(Low) ==
  false`, so `submit(command, None)` always succeeds once the
  namespace/action check passes.
- **`undo`/`redo`/`delete_selected` are not routed through the
  Command Bus.** Article 237's vocabulary has no "Undo", "Redo", or
  "Delete" word in any namespace. Inventing one was explicitly
  forbidden by the brief unless truly needed; it is not — these are
  session-level operations with an obvious, unambiguous meaning
  without a grammar word.
- **`apply_constraint`/`remove_constraint`/`solve_constraints` bypass
  the Command Bus entirely** for the same reason — Article 237 never
  names a "Constraint" command word, and Article 17 explicitly warns
  against expanding solver/toolbar scope to fill a gap that doesn't
  exist yet.
- **`add_geometry_to_view` was added** (not in Article 12's literal
  list) because nothing else could ever populate a `ViewBlock`'s
  `geometry_members`, which `evaluate_readiness` requires to reach
  `LinkReady` — without it, `enter_orthographic` could never succeed
  through this API at all.
- **`resolve_conflict`'s `ReplaceAndPropagate` uses a small
  session-local side table** (`pending_shared_value_proposals:
  BTreeMap<ConflictId, (ViewId, SharedAxis, f64)>`) rather than parsing
  the proposed value back out of `Conflict::affected_entities`'s
  free-form diagnostic strings (Article 27: those are for a human, not
  a machine-parseable reference). This table is deliberately **not
  persisted** — an unresolved `CrossViewMismatch` conflict from a
  *previous* session still survives save/reopen (Gate M/O) and is
  still resolvable, but `ReplaceAndPropagate` on one from a prior
  session falls back to only recording the resolution choice without
  re-applying the propagation (no crash, an explicit, narrow,
  documented limitation, not a silent gap).
- **`propagate_shared_value` auto-binds every other view in the same
  `OrthographicSet` whose identity consumes the same axis and has no
  binding yet** — this is the actual mechanism behind Article 18/35's
  "enter depth in TOP, RIGHT sees it too" without a second manual
  action; nothing in `craftloop-document` did this automatically
  before, since no caller had ever driven the propagation engine
  through a real multi-view scenario end to end.

## Verification

```
cargo fmt --all -- --check      # clean
cargo clippy --workspace --all-targets   # clean, zero warnings
cargo test --workspace          # 780/780 passing (was 760/760 at Phase 00)
```

20 new tests: 6 in `craftloop-document` (view/orthographic/multiview/
dimension/sketch-primitive/sketch-constraint undo-redo), 3 in
`craftloop-sketch` (round-trip x2, `primitive_ids`), 2 in
`craftloop-dimension` (`set_dimension` insert/replace), and 15 in
`craftloop-mobile-ffi::session::tests`, including one end-to-end test
that replays Article 35's entire Golden Alpha Journey against
`CraftLoopSession` alone: line → dimension → constraint → invalid edit
→ conflict → undo → FRONT → Orthographic → propagate depth into TOP →
confirm RIGHT sees it → contradictory value → cross-view conflict →
resolve with propagation → undo/redo → save → drop the session
(simulating a killed app) → reopen a *new* session from disk → assert
every persisted field (geometry, dimensions, conflicts, view identity,
orthographic relationships, revision) matches exactly. See
`crates/craftloop-mobile-ffi/src/session.rs`,
`golden_alpha_journey_end_to_end_through_save_and_reopen`.

One real finding from writing this test, not a bug: `DocumentHistory`
(undo/redo) is session-local and deliberately not persisted —
`CraftLoopSession::open` always starts a fresh one, so `can_undo` is
`false` immediately after reopening even though every other field
matches exactly. This matches ordinary application behavior (undo
history resetting after a process restart) and is asserted explicitly
in both persistence tests rather than silently excluded.

## What is still Rust-only / deferred to later phases

- Handwriting recognition, Ink Commands, Kotlin bindings, Compose UI,
  Jetpack Ink, Android CI, USB/physical device work — all untouched,
  per this phase's explicit scope (Rust-only, Phase 04).
- `FfiSceneSnapshot` is a coarse, full snapshot (Article 13 explicitly
  permits this for the first Alpha); no diffing.
- No timestamp/clock is threaded in from a caller — `Command.
  timestamp_seconds` uses `SystemTime::now()` directly inside this
  crate, since no platform clock exists to pass in yet at this layer.

## Files changed

`crates/craftloop-document/src/{document,history,multiview}.rs`,
`crates/craftloop-document/Cargo.toml`,
`crates/craftloop-dimension/src/store.rs`,
`crates/craftloop-sketch/src/sketch.rs`,
`crates/craftloop-mobile-ffi/src/{lib,session}.rs` (session.rs new),
`crates/craftloop-mobile-ffi/Cargo.toml`.
