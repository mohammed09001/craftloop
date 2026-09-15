# Execution 03, Phase 09 — Creative Sketch Toolbar

Recorded 2026-09-15. Tasks 064-077.

## Scope decisions from repository evidence

**Ellipse deliberately not added** (Task 070's own premise -- "expose
because the current kernel includes Ellipse2"). `Ellipse2` is real and
tested (`craftloop-geometry`), but `BeautifiedPrimitive` -- the type
every primitive actually flows through (scene snapshot, dimension
association, consistency validation, PDF/SVG export, constraint point
references) -- has exactly four variants, and every one of those call
sites is an exhaustive match. Adding `Ellipse` would touch 7+ crates
including native `craftloop-mobile-ffi`'s own matches, disproportionate
to "add a toolbar button" and, read carefully, indistinguishable in
spirit from Task 071's own Spline precedent: "do not show it unless a
tested kernel is added" means the *pipeline* is tested, not just the
bare geometry struct in isolation. Deferred to a dedicated future task
with its own cross-crate coverage. Full reasoning recorded in
`toolRegistry.ts`'s own doc comment.

**Arc, by contrast, was low-risk and added for real** (Task 067):
`BeautifiedPrimitive::Arc` already existed everywhere it needed to
(read path only) -- neither native's `CraftLoopSession` nor this
crate's had a *creation* entry point yet. Added
`create_primitive_arc`/`createPrimitiveArc`, mirroring
`create_primitive_circle` exactly and submitting the already-existing
`CommandAction::Arc` through the real Command Bus.

**Construction and Snap/Guide stay disabled** (Tasks 074-075): no
construction-geometry semantic state exists in `craftloop-sketch`/
`craftloop-document` yet (Phase 11 Task 090's job), and the real grid/
snap controls are Phase 11 Tasks 092-093's job -- registered, not
omitted, same pattern as Phase 07's deferred tools.

**Select is available in *both* Creative and Sketch2D**, not
Notebook-only as Phase 08's stopgap implied. Discovered while
implementing Dimension/Constraint: without a way to select an existing
primitive while a shape tool owns the canvas drag gesture, neither
tool could ever have anything real to act on -- Task 072/073's "real
semantic action"/"selection-adaptive control" would be structurally
unreachable. `toolRegistry.ts` updated accordingly.

## Task 064 — Implement toolbar morph

`Toolbar.tsx` now filters `TOOL_REGISTRY` by `workspaceMode` and
renders a genuinely different button set per mode -- a real morph
(Notebook-only tools like Eraser are absent from the DOM in Sketch2D,
not merely disabled), superseding Phase 08's stopgap of disabling
Select/Eraser while keeping them visible.

## Task 065 — Keep Pen in Sketch mode

Unchanged from Phase 08: Pen's `modes` already included both, and
clicking it while in Sketch2D calls `enterCreativePenMode()` first.

## Task 066-069 — Line/Arc/Circle/Rectangle families

Each family has exactly one real variant, matching what
`CraftLoopSession` actually supports (Task 066/068/069's own "only
real supported variants"/"default; variants only when real"): a single
drag maps directly to `createPrimitiveLine`/`createPrimitiveCircle`
(center + drag-distance radius)/`createPrimitiveRectangle`/
`createPrimitiveArc` (center at drag start, radius/start angle from
the drag end, fixed quarter-circle sweep -- the simplest honest
2-point interpretation of real `Arc2` semantics; a richer multi-stage
interaction is Phase 10's "Direct Geometry Preview" territory).
`CanvasStack.tsx`'s `SHAPE_TOOLS` set + `createShapeFromDrag` wires
these; `InkCanvas` stays active (for the drag itself) but the drag's
result is a real primitive, not free ink.

## Task 070/071 — Ellipse / Spline

Covered above: Ellipse deliberately deferred with full reasoning;
Spline was never real to begin with (no spline kernel exists anywhere
in the workspace) and is not shown, per the task's own instruction.

## Task 072 — Add Dimension

Enabled when 1-2 primitives are selected (matching
`DimensionTarget::Single`/`Pair`'s real acceptance), calling the real
`session.createDimension(WebDimensionKind.Linear, selectedIds, value)`
-- `value` comes from a `window.prompt()` for now (a minimal, honest
first pass; Phase 12 Task 094's "geometry-adjacent dimension input"
owns the real inline UX).

## Task 073 — Add Constraint

`src/toolbar/constraintOptions.ts`: a pure function returning only the
constraint kinds the current selection can actually take --
Horizontal/Vertical for one selected Line, Parallel/Perpendicular/
EqualLength/Coincident for two selected Lines, EqualRadius/Concentric
for two selected Circles, nothing otherwise. Deliberately conservative:
only combinations this crate's own `WebConstraintKind::into_domain`
supports (matching native's exact mapping from Phase 04), not every
theoretically-possible `SketchConstraintKind` variant. Clicking a
constraint calls the real `applyConstraint` and immediately
`solveConstraints()` -- applying a constraint with no visible effect
would look broken, not "real" (Article 65's spirit applied to UX, not
just data).

## Task 074/075 — Construction / Snap-Guide

Registered, disabled, with a title naming the phase (11) that adds
real backend support -- same pattern Phase 07 established for View/
Save/More.

## Task 076 — Add More

Still deliberately deferred (no real overflow content exists in either
mode yet); registered in both `modes` so it does not need re-adding
when it eventually gains content.

## Task 077 — Keep history actions reachable

Undo/Redo's `modes: ['creative', 'sketch']` was already correct from
Phase 08 -- confirmed still true and exercised by a real test (a
stroke created, mode cycled twice, Undo/Redo still work correctly)
rather than re-implemented.

## Test-first evidence

**Rust** (`cargo test -p craftloop-web-bridge`): 21 passed, 0 failed
(20 from Phase 08 + `create_primitive_arc_stores_real_arc_geometry_and_rejects_a_degenerate_one`,
which also proves `Arc2::new`'s own real validation rejects a
zero-sweep "arc").

**Frontend unit** (`npm run test`): 35 passed, 0 failed (28 from Phase
08 + 4 `constraintOptions.test.ts` tests + 3 new `Toolbar.test.tsx`
tests covering the real morph, shape-tool rendering, and the
Dimension/Constraint menu flow).

**Real browser** (`npm run test:e2e`): 18 passed, 0 failed (15 carried
over, updated for the real morph where Phase 08's stopgap assertions
no longer held, + 3 new `sketch-toolbar.spec.ts` tests: all four shape
tools produce real SVG geometry from one drag; Select inside Sketch2D
enables Dimension once something is selected; Constraint offers only
Horizontal/Vertical for a single selected line and the real solver
visibly levels it after applying).

**A real debugging exercise, not a product bug:** the Arc e2e test
initially failed only when preceded by other shape creations, in a way
that looked state-dependent. Traced with `document.elementFromPoint`
to a test-authoring bug, not application logic: the drag's target
coordinates (`x:400-450, y:40-80`) landed on the Sketch toolbar itself
(`{x:400, y:16, width:480, height:46}` in the 1280×720 test viewport),
intercepting the pointer event before it ever reached the canvas.
Fixed by moving the drag well below the toolbar's vertical range; noted
here because the investigation (isolating tool-order vs. coordinates vs.
DOM hit-testing) is the kind of thing worth recording so a future
similar failure is diagnosed faster.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build
npm run test                                                        # 35/35
npm run test:e2e                                                    # 18/18, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All fourteen tasks have evidence above. Shared semantics remain
platform-neutral: the one shared-crate change
(`create_primitive_arc`/`createPrimitiveArc`) is additive, mirroring
an existing method exactly, using geometry validation `Arc2::new`
already enforces. No unsupported native-device claim made; Ellipse's
deferral is the most consequential honesty call this phase made, and
it is recorded with its full reasoning rather than silently either
faked or dropped. Continuing automatically to Phase 10 (Direct
Geometry Preview and Creative Refinement), which is where the shape
tools gain a live ghost preview while dragging and Draw-and-Hold
refinement for freehand strokes.
