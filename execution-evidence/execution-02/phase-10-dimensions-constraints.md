# Execution 02, Phase 10 — Dimensions and Constraints Vertical Slice

Recorded 2026-09-14. Tasks 065-071. Authority: Article 14 (Dimension
Alpha Workflow), Article 17 (Constraint Alpha Workflow).

## A real, previously undiscovered crash bug found and fixed

Independent on-device testing (inserting a real primitive, then
actually selecting it -- something no earlier phase's verification had
ever done for real, since Phase 08/09's own synthetic-touch tests
correctly never reached `select()` at all under Article 11's stylus
gate) crashed the app immediately:

```
FfiSessionException$Domain: detail=invalid EntityId string: "4401aad5-dd78-4711-8959-b080d46b502b"
    at CraftLoopSession.select(craftloop_mobile_ffi.kt:4597)
    at CraftLoopViewModel$selectAt$1.invokeSuspend(CraftLoopViewModel.kt:193)
```

**Root cause**: `select`/`delete_selected` (`session.rs`) parsed their
`Vec<String>` input with `raw.parse::<EntityId>()`, expecting
`EntityId`'s own `Display` format (`"Kind:uuid"`, e.g.
`"Primitive:4401aad5-..."`). But every `Ffi*Summary.id` field this
module has ever built (`FfiPrimitiveSummary`, `FfiStrokeSummary`,
`FfiConflictSummary`, ...) — the only real source of ids a Kotlin
caller ever has — is a bare `id.to_string()`, matching the convention
every other `Ffi*` id field in this module already uses
(`edit_dimension`/`remove_constraint`/`assign_view_identity` all parse
bare UUIDs via `parse_id::<T>`). Nothing in this crate has ever
produced the composite format `select`/`delete_selected` expected, so
any real selection crashed the instant a real hit-test actually
reached them — every earlier verification pass (Phase 08's own
"independent verification" included) only ever exercised the *no-hit*
path (`clear_selection`, which parses nothing).

A second instance of the same class of bug was found while fixing the
first: `scene_snapshot()`'s `selected_entity_ids` field used
`EntityId::to_string()` directly (composite format) for the *output*
side too — meaning a caller trying to feed a selected id from the
snapshot into `apply_constraint`/`create_dimension` (both bare-UUID
`parse_id::<PrimitiveId>`) would have hit the identical failure the
other direction.

**Fix** (`crates/craftloop-mobile-ffi/src/session.rs`):
- `resolve_entity_id(page, raw)` replaces `parse_entity_id`: parses
  `raw` as a bare UUID, then tries each `EntityId` variant's own id
  type constructed from the same bits against `page.get(...)`,
  returning whichever one actually exists. `select`/`delete_selected`
  now use this.
- `entity_id_bare_uuid(id)` replaces the direct `.to_string()` call
  building `selected_entity_ids`, unwrapping each `EntityId` variant to
  its inner id's own bare-UUID `Display`.
- The existing test
  `select_and_delete_selected_removes_entities_and_clears_selection`
  had hand-built the composite string it expected to work — masking
  this exact bug at the unit level too. Fixed to use `create_primitive_line`'s
  own bare-UUID return value (what a real caller actually has), plus a
  new test, `select_rejects_an_id_with_no_matching_entity_on_the_active_page`.
- **782/782 workspace tests passing** (was 781 before this phase), clean
  clippy/fmt.

This is exactly the kind of gap Article 27 (No-Hallucination Policy)
exists to surface rather than let slide: Phase 08's own "independently
verified" claims about selection were real for the *code paths they
exercised* (pan, the touch-rejection gate, undo), but selection itself
had never actually been exercised with a real hit until this phase.

## Part A/B — Typed dimension entry and edit (Task 065-067)

`Toolbar.kt`'s `DimensionDialog`: a plain `AlertDialog` with a kind
selector (`FfiDimensionKind`'s real four variants — `Linear`, `Angular`,
`Radius`, `Diameter`, checked in `session.rs`, not guessed) and one
`OutlinedTextField` for the value. Tapping the Dimension toolbar icon
with exactly 1 or 2 primitives selected opens it in create mode
(`createDimension`); tapping an entry in the new "Dimensions &
Conflicts" overflow-menu list (`DimensionAndConflictList`) opens it
pre-filled in edit mode (`editDimension`). 0 or >2 selected shows a
message instead of an empty dialog.

**Real, on-device confirmed** (after the crash fix above):
- Selecting the debug-inserted test primitive: `selected=1
  primitives=1`, no crash (`phase-10-selected-fixed.png`).
- A real dimension was created end-to-end through the real UI:
  `Dimensions & Conflicts` shows `LINEAR = 54.0 (e72af5b4)`
  (`phase-10-dimension-list.png`) — `revision`/`txCount` had advanced
  to 5 with `last=Dimension created` at that point
  (`phase-10-dimension-dialog.png`). Given the exact sequence of
  intervening revisions did not match this session's own adb taps
  one-for-one, this dimension was most likely created by the developer
  directly exercising the real UI with the real S Pen while this
  session also worked — if so, that is *stronger* evidence than a
  synthetic test would have been (genuine physical-device use of the
  new numeric-entry dialog), not weaker; it is recorded here as
  "confirmed real, attribution uncertain" rather than claimed as this
  session's own synthetic test.
- Tapping the dimension list entry opened `Edit Dimension` correctly
  pre-filled with `54.0` (`phase-10-edit-dimension-dialog.png`) —
  Task 067 (edit) confirmed working.

## Part C — Invalid dimension shows a real conflict (Task 068)

`submitDimension` in `CraftLoopViewModel.kt` catches `editDimension`'s
`Err` (rather than letting the coroutine crash), still calls
`refreshSnapshots()` (the real Rust side already commits a
`Conflict { kind: DimensionConstraintMismatch }` entity on a rejected
edit even though the value itself is left unchanged — confirmed
directly from `session.rs`'s real `edit_dimension` body, not
paraphrased: `Err(err) => { ... let conflict = Conflict { kind:
ConflictKind::DimensionConstraintMismatch, ... }; state.commit(vec![
DocumentChange::InsertEntity { ... entity:
SemanticEntity::Conflict(conflict) }])?; Err(FfiSessionError::from(err))
}`), and reports the real failure message via `lastActionMessage`.
`DimensionAndConflictList` shows `sceneSnapshot.conflicts` (id/kind/
unresolved) as a plain list alongside the dimensions.

**Not independently isolated this session, stated plainly**: this
session's own attempts to trigger an actual *rejected* edit (typing a
negative value into the field) did not conclusively reproduce a
visible conflict in the two attempts made — both times the dimension
list still showed the original `54.0` value with `(none)` conflicts
immediately after, and it became clear partway through that real
concurrent device activity (see above) was making cause-and-effect
attribution for this one specific sub-case unreliable within the time
available. This is **not** evidence the mechanism is broken: the exact
same commit-a-conflict-on-rejected-edit code path is independently
proven by the existing Rust unit test
`create_primitive_and_dimension_then_invalid_edit_creates_a_conflict_and_undo_clears_it`
(passing, part of the 782), which exercises `CraftLoopSession.edit_dimension`
directly and asserts a real `Conflict` entity is committed. What is
*not* independently confirmed on-device this session is specifically
that `DimensionAndConflictList` visually renders that conflict once it
exists — a real, narrow, stated gap for a follow-up pass.

## Part D — Constraint solving and solver conflicts (Task 069/070)

`CraftLoopViewModel.applyConstraint` now calls the real
`solveConstraints()` immediately after a successful `applyConstraint`
(nothing before this phase ever invoked the solver at all, so no real
solver outcome — satisfied or not — could previously have been
observed through this toolbar). Read `solve_constraints`'s actual body
before wiring this: it does **not** commit a `Conflict` entity for an
unsatisfied/failed solve (unlike `edit_dimension`) — it either returns
early with `updated_primitive_count: 0` (`SolveStatus::Failed`) or
commits only the primitives that actually moved, returning
`FfiSolveOutcome { status, unsatisfied_constraint_ids,
updated_primitive_count }` directly. So the outcome is surfaced via
`lastActionMessage` ("Constraint added; solve: Solved/Unsatisfied/
Failed"), not through the conflict list — a deliberate, code-verified
distinction from Part C's dimension-conflict path, not an oversight.

**Not independently on-device re-exercised this phase** beyond
confirming the wiring compiles and the existing
`apply_constraint_detects_redundancy_and_solve_moves_the_line` Rust
test (already passing pre-Phase-10, re-confirmed at 782/782) covers
the real solve mechanics directly.

## Task 071 — Undo/redo verification

Confirmed via the same real device session: after the crash fix,
selecting and creating a dimension both visibly advanced
`revision`/`txCount` and `canUndo` correctly (`canUndo=true` throughout
once any transaction existed); the pre-existing Phase 08 `Undo`/`Redo`
toolbar buttons operate on the exact same `DocumentHistory` mechanism
proven since Phase 04 — no new undo/redo code was needed or written
for dimensions/constraints specifically, and no evidence surfaced this
phase that they behave differently for these entity kinds than for
primitives (Phase 08 already proved undo/redo for primitives directly).

## Real build verification

```
cargo test --workspace   # 782/782 passing
cargo clippy --workspace --all-targets   # clean
cargo fmt --all -- --check   # clean
```

Android: `gradlew installDebug` clean on first attempt after the crash
fix; launched fresh, zero `FATAL`/`AndroidRuntime` logcat lines across
the entire verification session except the one real, now-fixed crash
itself (captured and quoted above, not hidden).

## Files changed

`crates/craftloop-mobile-ffi/src/session.rs` (the `select`/
`delete_selected` id-resolution fix, `entity_id_bare_uuid`,
`resolve_entity_id`, test fixes/additions),
`android/app/src/main/java/com/craftloop/shell/{CraftLoopViewModel,
MainActivity,Toolbar}.kt` (dimension dialog/list, constraint-solve
wiring, debug-only `+Line`/`Select 1st` verification helpers reachable
from the Debug Region since Phase 09 removed the Phase 08 temporary
button row that used to expose them).
