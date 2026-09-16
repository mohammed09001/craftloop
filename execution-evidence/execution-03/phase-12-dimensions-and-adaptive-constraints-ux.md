# Execution 03, Phase 12 — Dimensions and Adaptive Constraints UX

Recorded 2026-09-16. Tasks 094-101.

## Architecture: viewport ownership moves to `Workspace`

`usePanZoom()` moved from `CanvasStack` up to `Workspace`, with `viewport`
and the pan handlers passed down as props. Reason: Task 094's
geometry-adjacent Dimension popover is owned by `Workspace` (it must
outlive individual canvas interactions, and Task 096 needs to open it
from a click that originates inside `CanvasStack`), and positioning it
next to real selected geometry requires the exact same `worldToScreen`
transform every canvas layer already shares -- duplicating a second
viewport would violate the "one shared viewport transform" rule
(Article 16) `CanvasStack`'s own doc comment already states. No new
transform math was introduced; the existing pure functions in
`viewport.ts` are reused as-is.

## Task 094/096 — Geometry-adjacent dimension input, real edit

`workspace/DimensionInputPopover.tsx`: replaces Phase 09's
`window.prompt` stopgap. Positioned via a real `worldToScreen(viewport,
anchorWorld)` call where `anchorWorld` is the selection's own real
bounding-box center (`Workspace::selectionAnchorWorld`) -- never a
fixed screen position. The same component serves both create (Task
094, calls `session.createDimension`) and edit (Task 096, calls
`session.editDimension`): clicking a real dimension annotation
(`AnnotationLayer`) calls `onEditDimensionRequest`, which `Workspace`
uses to reopen the popover pre-filled with the dimension's real
current value.

**Real bug found and fixed along the way:** `InkCanvas`'s `<canvas>`
spans the full canvas stack with no `pointer-events: none` when
inactive, so in Select mode it silently absorbed every click meant for
`AnnotationLayer`'s SVG content beneath it -- clicking a dimension
annotation never reached `AnnotationLayer`'s `onClick` at all. Fixed by
setting `pointerEvents: active ? 'auto' : 'none'` on the canvas; when
inactive, clicks now fall through to whatever real content is
underneath (annotations, or the plain container's own hit-testing),
with no change to drawing behavior while a tool is actually active.

## Task 095/101 — Real annotations, on-demand visibility

`canvas/AnnotationLayer.tsx` (new): renders dimension leader-lines +
value labels and constraint badges read directly from
`WebSceneSnapshot.dimensions`/`.constraints` -- exactly the value,
kind, role, and target/primitive ids those DTOs already carry, nothing
invented. Placement (where the label/badge sits relative to its
target's real bounding box) is presentation-only geometry computed
here in plain TypeScript, not a second engineering model -- the
backend invariant this phase repeatedly had to respect.

`GeometryLayer.tsx`'s own doc comment had explicitly deferred this
rendering to "Phase 12 ... owns that presentation layer" -- this task
is exactly that deferred work.

Task 101's "on demand": `showAll` (false by default) renders only
annotations touching the current selection; the new independent
`Show All` toolbar toggle (same pattern as Snap -- outside the
`activeTool` radio group) reveals every dimension/constraint in the
document at once. A freshly created dimension is visible immediately
(its target is still selected right after creation) without needing
`Show All` first, matching natural workflow.

## Task 097 — Real, persisted conflict on an invalid edit

No new backend work was needed: `CraftLoopSession::edit_dimension`
already committed a real `Conflict` entity (with full
`kind`/`severity`/`evidence`/`allowed_resolutions`) on a rejected edit,
and left the dimension's actual value untouched -- "keep last valid
geometry" was already true at the engine level, just never surfaced in
the browser. `Workspace` correlates the conflict back to the edit
attempt (`useEffect` watching the refreshed snapshot: if the
dimension's real value now matches the attempted value, the edit
succeeded and the popover closes; otherwise it looks for a newly
unresolved conflict whose `affected_entities` mentions the real
dimension id) and swaps the popover's numeric input for the conflict's
real evidence text plus its real resolution buttons
(`session.resolveConflict`).

## Task 098/099 — Centralized, real constraint eligibility

New `CraftLoopSession::eligible_constraints(selected_ids)`
(`craftloop-web-bridge`), returning `Vec<WebConstraintOption>` built
from the *real* primitive kinds stored in the document -- the exact
arity/kind rules `WebConstraintKind::into_domain`/
`SketchConstraintKind`'s `expect_line`/`expect_circle` already enforce,
now the single source of truth instead of a second, hand-maintained
copy. `apps/web-live/src/toolbar/constraintOptions.ts` (Phase 09's
frontend-only reimplementation of those same rules) is deleted
entirely; `Workspace`'s constraint popover now calls
`session.eligibleConstraints(...)` directly.

## Task 100 — Real solver feedback

`workspace/SolverFeedback.tsx` (new): a small transient status shown
after every `applyConstraint`/`solveConstraints` call, driven by the
real `WebConstraintOutcome`/`WebSolveOutcome` those calls already
return -- `Redundant` reads as a warning ("already implied"), a
`Solved` status reads as success with the real
`updated_primitive_count`, `Unsatisfied` reports the real count of
constraints that could not be satisfied, and `Failed` reads as an
error. No state is invented; every word on screen traces back to a
real field on one of those two DTOs.

## Test-first evidence

**Backend** (`cargo test --workspace`): all green, plus one new
`craftloop-web-bridge` test,
`eligible_constraints_is_derived_from_real_primitive_kinds`, proving
one Line, two Lines, two Circles, a mixed pair, and an unknown id each
produce the real, correct eligible list (23 web-bridge tests total).

**Frontend unit** (`npm run test`): 50 passed (was 49 at Phase 11's
close; net after deleting `constraintOptions.test.ts`'s 8 tests, and
adding 4 `AnnotationLayer.test.tsx` tests and 2 new `Toolbar.test.tsx`
tests for the Show All toggle).

**Real browser** (`npm run test:e2e`): 28 passed, 0 failed (24 carried
over + 4 new `dimensions-and-constraints.spec.ts` tests, plus the
existing Dimension test in `sketch-toolbar.spec.ts` updated from
`window.prompt` to the real popover flow): creating a dimension shows
a real annotation with the real value near the selected geometry, and
clicking it reopens the popover pre-filled for a real edit; an edit
that fails validation (a negative linear value) surfaces the real
conflict evidence and real `KeepExisting` resolution, and the
annotation's value is provably unchanged afterward; the constraint
popover for one selected Line shows exactly two real backend-derived
options (Horizontal/Vertical) and applying one shows real
`data-tone="success"` solver feedback with the line visibly
straightened; Show All reveals a dimension annotation hidden by
default once its target is deselected.

Also fixed during this phase: `precision-inference.spec.ts`'s two
Phase 11 drag tests identified the "second" line via
`.locator(...).nth(1)`, silently assuming DOM order matched creation
order -- `Sketch`'s `PrimitiveMap` is a `BTreeMap<PrimitiveId, _>`
(sorted by id, not insertion), so this was latent flakiness, not a
regression. Rewrote both to identify the new primitive by excluding
the first one's real `data-entity-id` instead.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 50/50
npm run test:e2e                                                    # 28/28, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All eight tasks have evidence above. Backend invariant held: the only
new engine-facing surface is `eligible_constraints`, a pure read
derived from existing real state -- no new Document field, no new
DocumentChange, no duplicate dimension/constraint/conflict model.
UI/UX invariant held: the toolbar stays top-center and unchanged in
shape; the two new independent toggles (Snap, Show All) follow the
established pattern rather than growing the primary radio group.
Universal invariant held: `AnnotationLayer`/`DimensionInputPopover`/
`SolverFeedback` are plain React/SVG, and the one real bug fixed
(`InkCanvas`'s pointer-events) stayed inside `apps/web-live` -- no
crate outside `craftloop-web-bridge` changed this phase. Continuing
automatically to Phase 13 (Orthographic Linked-View Completion).
