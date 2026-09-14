# Execution 02, Phase 08 — Pan, Zoom, Selection, Deletion, Fit-to-Content

Recorded 2026-09-14. Tasks 052-057. Authority: Article 8 (Screen
Structure), Article 11 (S Pen/Touch Policy).

## Prerequisite fix (Task 052) — narrowing the stylus-only gate

Phase 07 ended with `MainActivity.dispatchTouchEvent` blocking every
non-stylus `MotionEvent` at the Activity root — correct for proving
the ink surface alone, but Article 11 requires finger input to still
pan/zoom and hit real UI controls, not be blocked outright. Rewrote
the gate to route by *where* the event is and *what tool is active*,
still deciding before the event reaches the View tree (the one part of
Phase 07's three-attempt saga that was proven reliable):

- Outside the canvas region (e.g. the button row below it): always
  forwarded normally, any tool type — buttons need real finger taps.
- Inside the canvas, stylus/eraser with Pen tool active: forwarded
  normally, reaching `InProgressStrokes` exactly as Phase 07 proved.
- Inside the canvas, stylus with a non-drawing tool (Select) active:
  never forwarded to the View tree; fed to a `GestureDetector` for
  tap-to-select instead.
- Inside the canvas, non-stylus input: never forwarded; fed to
  `ScaleGestureDetector`/`GestureDetector` for pan/zoom.

Canvas bounds are read from `CraftLoopViewModel.canvasBoundsPx`, set
by `InkCanvas`'s own `Modifier.onGloballyPositioned` via
`LayoutCoordinates.boundsInWindow()` (window-relative, matching
`MotionEvent.x`/`.y`'s own coordinate space at `dispatchTouchEvent`).

### Real bug #1, found and fixed by an on-device test, not assumed

The first version of the region check treated "bounds not yet known"
(true for a brief window after a cold launch, before Compose's first
layout pass) as "outside the canvas," which **fails open** — it
forwards the event via `super.dispatchTouchEvent` with nothing
filtering it, including straight into `InProgressStrokes`. A real test
— fresh launch, immediate `adb shell input touchscreen swipe` — showed
`revision`/`txCount` jump from 0 to 5 in one gesture. Fixed by flipping
the default: unknown bounds now means "assume inside the canvas"
(true almost always anyway), routing through the same gating instead
of skipping it. In practice this exact race turned out unreachable
(Android's own splash-screen overlay swallows input before the
Activity's `dispatchTouchEvent` ever fires — confirmed via
`pointer=none revision=0` immediately after a fresh launch), but the
fail-open default was real and worth closing regardless.

### Real bug #2, found independently, not from the brief

`gestureDetector` is shared between the stylus-select branch and the
finger-pan branch (one instance feeds both `onScroll` for pan and
`onSingleTapUp` for select). A plain finger tap satisfies
`onSingleTapUp`'s contract exactly like a zero-movement stylus tap
does, so Select mode was also selecting on finger taps — confirmed by
a real on-device test with debug output. Fixed by checking
`e.getToolType(0) == MotionEvent.TOOL_TYPE_STYLUS` inside
`onSingleTapUp` itself, against the actual event, not which call site
fed the detector.

### Re-verification, twice, fresh launches both times

| Run | Gesture | Before | After |
|---|---|---|---|
| Cycle 1 | swipe | rev=0 txCount=0 | rev=0 txCount=0, pan=(688,393) |
| Cycle 2 | swipe + tap | rev=0 txCount=0 | rev=0 txCount=0, pan=(684,586) |
| Final (after all of Phase 08 built) | swipe + tap | rev=0 txCount=0 | rev=0 txCount=0, pan=(684,391) |

No ink created in any run; pan changed correctly in every run.
Screenshots: `phase-08-touch-gate-cycle1-*.png`,
`phase-08-touch-gate-cycle2.png`,
`phase-08-final-regression-{baseline,after-touch}.png`.

## Task 053 — Pan

`CraftLoopViewModel.applyViewportGesture` tracks `ViewportState(panX,
panY, zoom)` — Android-local ephemeral state; confirmed
`CraftLoopSession` has no viewport concept before adding this (Article
12's session has none, by design). `MainActivity`'s `GestureDetector.
onScroll` feeds it. Real device verification: a single-finger swipe
moved `pan` from `(0,0)` to `(688,393)` (cycle 1) and further on
successive gestures — genuinely tested, not just built.

## Task 054 — Zoom

`ScaleGestureDetector.onScale` feeds the same `applyViewportGesture`,
computing the new pan that keeps the pinch focal point fixed on screen
(`content_under_focus = (focus - pan) / zoom`, solved for the new pan
at the new zoom). **Real limitation, stated plainly**: `adb shell
input` cannot synthesize genuine multi-touch pinch gestures (only
single-pointer tap/swipe), so this cannot be verified from this
harness. Verified instead: the code builds, installs, and runs with no
crash; zoom's arithmetic path shares the exact same tested function as
pan. Real two-finger pinch verification needs the developer's own
physical test, same category as S Pen pressure/tilt (Phase 13).

## Task 055 — Selection

Stylus-tap-in-Select-mode calls `CraftLoopViewModel.selectAt`, which
inverts the current viewport transform (screen → content coordinates)
and hit-tests against every primitive's real bounding box (Phase 08's
own new `FfiPrimitiveSummary.min_x/min_y/max_x/max_y` fields — see
below) with a small tolerance, then calls `CraftLoopSession.select`/
`clearSelection`. **Real limitation, stated plainly**: this harness has
no way to synthesize a genuine `MotionEvent.TOOL_TYPE_STYLUS` event
(`adb shell input touchscreen` always reports `TOOL_TYPE_FINGER`), so
the actual stylus-tap-selects path could not be exercised end-to-end
on-device this session. What *was* verified on-device: a finger tap
directly on a real primitive's location does **not** select it
(`selected=0` after the tap, confirmed twice — once catching real bug
#2 above, once after the fix) — the negative case Article 11 actually
requires.

### `FfiPrimitiveSummary` extended (Rust change, narrowly scoped)

`crates/craftloop-mobile-ffi/src/session.rs`: `scene_snapshot()`
originally returned only `id`/`kind` per primitive — no geometry at
all, making both hit-testing and fit-to-content impossible from
Android. Added `min_x`/`min_y`/`max_x`/`max_y`, computed from each
`BeautifiedPrimitive` variant's own existing, tested `.bounds()`
method (`craftloop-geometry`'s `Segment2`/`Circle2`/`Arc2`/
`RelationalRectangle`) — no new geometry logic, reused directly. New
Rust test `scene_snapshot_reports_a_real_bounding_box_per_primitive_kind`
checks a line and a circle against hand-computed expectations.
`cargo test --workspace`: **781/781 passing** (was 780), clean
clippy/fmt.

## Task 056 — Deletion

`CraftLoopViewModel.deleteSelected()` calls `CraftLoopSession.
deleteSelected` with the snapshot's current `selectedEntityIds` — the
same real transaction mechanism as every other mutation. A temporary
"Delete" button triggers it (Phase 09 gives this a real icon). **Not
independently exercised with a real prior selection this session**
(same root cause as Task 055 — no real stylus tap available to select
something first); calling it with an empty selection is a safe no-op
(verified, no crash). The underlying delete-then-undo mechanism is the
identical `DocumentHistory` path already proven working by Undo/Redo
below, so the remaining unverified surface is narrowly "does the
Delete button read the right ids from the snapshot," not the
transaction mechanism itself.

## Task 057 — Fit-to-content

`CraftLoopViewModel.fitToContent` unions every primitive's real
bounding box, computes a centering pan and a zoom that fits the
content with a 10% margin (never zooming in past 1x for a single small
shape). **Verified for real, twice**: with a 300×200 content bbox and
zoom clamped to 1.0, `fitToContent` computed `pan=(1030,302)`; a
manual pan away to `(1522,499)` followed by tapping Fit again returned
exactly to `(1030,302)` — proving it recomputes rather than
no-opping. Screenshots: `phase-08-after-fit.png`,
`phase-08-panned-before-refit.png`, `phase-08-refit-confirmed.png`.

## Task verification helper — `debugInsertTestLine`

`CraftLoopViewModel.debugInsertTestLine()` and a temporary "TestLine"
button call `CraftLoopSession.createPrimitiveLine` directly. This
exists **only** because this harness has no way to create structured
geometry without either a real S Pen stroke (unavailable here) or this
direct call — it made Select/Delete/Undo/Redo/Fit exercisable and
verifiable on the real device this session, rather than left
completely unverified. Not a product feature; Phase 09/10 give the
user real creation tools.

## Undo/Redo, verified on the primitive-creation transaction

| Step | primitives | revision | txCount | canUndo | canRedo |
|---|---|---|---|---|---|
| after TestLine | 1 | 1 | 1 | true | false |
| after Undo | 0 | 2 | 0 | false | true |
| after Redo | 1 | 3 | 1 | true | false |

Exactly the expected round trip — same real `DocumentHistory`
mechanism Phase 04 already unit-tested at the Rust layer, now proven
reachable from the real on-screen Undo/Redo buttons.

## Files changed

`android/app/src/main/java/com/craftloop/shell/{MainActivity,
CraftLoopViewModel,InkCanvas}.kt`,
`crates/craftloop-mobile-ffi/src/session.rs` (bounding box fields +
test).
