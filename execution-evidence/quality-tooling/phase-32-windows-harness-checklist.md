# Windows Harness Manual Checklist

Execution 01, Phase 32, Task 230. Recorded 2026-09-14. Authority: Front
End Skill's "Windows Harness Frontend Rules" (the 13-item checklist this
task's own objective refers to as "the documented initial user journey").

## What was verified in this session (no display, no human)

Matches Phase 04's own established, honest pattern:

```
cargo build -p windows-harness              # clean
cargo test -p windows-harness                 # 26/26 passing
timeout 8 cargo run -p windows-harness        # ran the full 8s, killed only
                                               #   by the external timeout
                                               #   (exit 124/143), no panic
```

Full transcript: `test-reports/phase-32-harness-launch.log`.

This is evidence the app **starts and stays alive**, exactly as strong
as Phase 04's own equivalent evidence and no stronger -- it is not
evidence that mouse interaction, drawing, or any panel renders or
behaves correctly on screen. **No screenshot or interactive click-through
was performed.** This sandboxed environment has no display and no tool
equivalent to browser automation for a native win32/eframe window.

## A real, honest finding from actually auditing the harness's current
## scope, not assuming Phase 04's forward-looking plan happened

Phase 04's own evidence file predicted its inspector panels would be
"added alongside their engines (Phases 06, 10, 12, 14, 20-24)." **This
did not happen.** Checked directly: `apps/windows-harness/Cargo.toml`
depends only on `eframe`, `egui`, `serde`, `serde_json`, `thiserror`,
`craftloop-errors`, `craftloop-geometry`, `craftloop-input`, and
`craftloop-serialization` -- exactly Phase 04's original Phase 04 dependency
set, with zero growth. No dependency on `craftloop-recognition`,
`craftloop-dimension`, `craftloop-constraint`/`craftloop-sketch`,
`craftloop-consistency`, `craftloop-document`, or `craftloop-command`
exists, which structurally means the harness cannot display structured
geometry, dimensions, constraints, conflicts, orthographic view blocks,
or real document-level undo/redo/save -- because it has never imported
the types those panels would need to render.

This was never a broken promise inside any single phase: no phase's own
task list (Phases 05 through 31) ever named "wire this into
windows-harness" as one of its tasks -- each phase's scope was its own
crate, per this execution's consistent "shared core first, platform
adapters separately" architecture. It is, however, a real gap against
Front End Skill's checklist that this completion-gate audit should
surface honestly rather than let Phase 04's four-phase-old prediction
stand uncorrected.

## Checklist status (13 items, Front End Skill "Windows Harness Frontend Rules")

| # | Item | Status |
|---|---|---|
| 1 | Canvas | **Present, launch-verified** (Phase 04) |
| 2 | Mouse-as-simulated-pen input | **Present, launch-verified** (Phase 04, `tool.rs`) |
| 3 | Active tool state | **Present, launch-verified** (Phase 04, `tool.rs`) |
| 4 | Raw stroke visibility | **Present, launch-verified** (Phase 04, `state.rs`) |
| 5 | Structured geometry visibility | **Not wired** -- no `craftloop-recognition` dependency |
| 6 | Engine-state inspector | **Not wired** -- no general engine-state panel exists |
| 7 | Dimension inspector | **Not wired** -- no `craftloop-dimension` dependency |
| 8 | Constraint inspector | **Not wired** -- no `craftloop-sketch`/`craftloop-constraint` dependency |
| 9 | Orthographic view blocks | **Not wired** -- no `craftloop-document` dependency |
| 10 | Conflict panel | **Not wired** -- no `craftloop-consistency` dependency |
| 11 | Undo/redo | **Not wired** -- no document-level history in the harness (only stroke capture/clear exist in `state.rs`) |
| 12 | Save/reopen | **Not wired** at the real-`Document` level -- `scenario.rs`'s deterministic trace loader and `diagnostic_export.rs`'s JSON export exist, but neither is `craftloop-document`'s own `save_document_atomically`/`load_document` |
| 13 | Deterministic scenario loader | **Present, launch-verified** (Phase 04, `scenario.rs`) |
| -- | JSON diagnostic export | **Present, launch-verified** (Phase 04, `diagnostic_export.rs`) -- this is the harness's own ad hoc export, not Phase 26's `craftloop-export` crate, which the harness also does not depend on |

**5 of 13 present and launch-verified; 8 of 13 never wired in.** No item
was manually interaction-tested on screen (no display, no human) either
way.

## Deferred, explicitly

- Interactive/visual manual pass -- needs a human with a keyboard/mouse
  and a display, exactly Phase 04's own deferred item, still open.
- Wiring the 8 missing checklist items into the harness -- real,
  substantial UI work (importing 6 more domain crates into `app.rs` and
  building real panels for each) that no phase's task list through 31
  asked for. Named here as a concrete, scoped item for whoever picks up
  Execution 02, rather than left implicit.
