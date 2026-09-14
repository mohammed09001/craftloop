# Execution 02, Phases 06-07 — Android State Architecture and Jetpack Ink Surface

Recorded 2026-09-14. Tasks 039-051. Authority: Articles 7-11.

## Jetpack Ink dependency (Task 044)

Verified via WebSearch/WebFetch against `developer.android.com/jetpack/androidx/releases/ink`
on 2026-09-14: **1.0.0 is the current stable line**, released
2025-12-17 (current alpha line is 1.1.0-alpha08). Article 10's "prefer
stable 1.0 ... do not use an alpha simply because it is newer" is
satisfied directly, not overridden. Only the modules this phase's
surface actually uses:

```
androidx.ink:ink-authoring:1.0.0
androidx.ink:ink-authoring-compose:1.0.0
androidx.ink:ink-brush:1.0.0
androidx.ink:ink-geometry:1.0.0
androidx.ink:ink-rendering:1.0.0
androidx.ink:ink-strokes:1.0.0
```

**Important tooling note, not a project defect**: the public API
reference pages (`developer.android.com/reference/.../StrokeInput`
etc.) render as an empty JS-driven shell to this session's page
fetcher -- no method/property documentation loads at all, only nav
chrome. After several failed attempts, real ground truth came from
downloading the actual **sources jars** directly from
`maven.google.com/androidx/ink/<artifact>/1.0.0/<artifact>-1.0.0-sources.jar`
and reading the real Kotlin source for `InProgressStrokes.kt`,
`StockBrushes.kt`, `Brush.kt`, `CanvasStrokeRenderer.kt`,
`StrokeInput.kt`, and `StrokeInputBatch.kt`. This is how every exact
signature below was confirmed, not guessed:
- `InProgressStrokes` takes **no `Modifier` parameter at all** and no
  `state` parameter -- confirmed by reading the real composable source,
  after an initial guess (`rememberInProgressStrokesState`, a
  `modifier` param) failed to compile.
- Its own doc comment: "will process pointers... that are not already
  consumed by an earlier touch listener - when
  `PointerInputChange.isConsumed` is `false`" -- this is the exact,
  library-documented mechanism the stylus-only gate below relies on.
- `StrokeInput.hasPressure`/`hasTilt` are real properties backed by
  `NO_PRESSURE`/`NO_TILT = -1f` sentinels -- used instead of an
  initial, wrong NaN-based guess.
- `CanvasStrokeRenderer.draw` takes `android.graphics.Canvas` /
  `android.graphics.Matrix`, not the Compose UI ones -- `drawContext.
  canvas.nativeCanvas` (an explicit import,
  `androidx.compose.ui.graphics.nativeCanvas`) bridges from Compose's
  `DrawScope`.
- `StockBrushes.pressurePen()` (a function returning `BrushFamily`),
  not a `pressurePenLatest` property (the initial, wrong guess).

## Part B — State architecture (`CraftLoopViewModel.kt`)

- One `CraftLoopSession` created in the `ViewModel`'s constructor,
  closed in `onCleared()` -- survives configuration changes (screen
  rotation) by construction, per Task 043; no extra handling needed.
- `EphemeralUiState` (active `Tool` enum, last observed pointer source,
  debug-overlay visibility) is a plain, non-persisted data class --
  never touches `CraftLoopSession` (Task 041's ephemeral/semantic
  split; `CraftLoopSession` has no notion of "active tool" at all).
- `sceneSnapshot`/`debugState` are `StateFlow`s refreshed eagerly after
  every mutating call (`submitStroke` today; every later phase's
  mutating calls follow the same pattern). Article 26 forbids
  optimizing without measurement, and nothing has measured eager
  refresh as too slow for a single-user Alpha document yet.

## Part C — Ink surface (`InkCanvas.kt`)

- `defaultBrush()`: `Brush.createWithColorIntArgb(family =
  StockBrushes.pressurePen(), colorIntArgb = Color.Black.toArgb(), size
  = 5f, epsilon = 0.1f)` -- plain, functional, no design polish
  (Article 7).
- **Stylus-only gate (Article 11)**: an outer `Box`'s
  `Modifier.pointerInput` runs `awaitPointerEvent(PointerEventPass.
  Initial)` -- Compose's parent-before-child pass -- and calls
  `event.changes.forEach { it.consume() }` for any non-`PointerType.
  Stylus` change before `InProgressStrokes` (rendered as that `Box`'s
  child) ever receives it. This is not a guess about Compose's pass
  ordering; it is the exact mechanism `InProgressStrokes`'s own doc
  comment names for excluding pointers from drawing.
- **Sample normalization (Task 047)**, field by field:
  - `x`/`y` -> `FfiPointerSample.x`/`.y` directly (Compose-local view
    coordinates; Phase 08's pan/zoom will introduce a world transform,
    not yet needed).
  - `elapsedTimeMillis / 1000.0` -> `timestampSeconds`.
  - `pressure` -> `Some(pressure)` only if `input.hasPressure`, else
    `null` -- never a fabricated `0.0`.
  - `tiltRadians` (converted to degrees) -> `tiltXDeg` only if
    `input.hasTilt`, else `null`; `tiltYDeg` is always `null` --
    Jetpack Ink's `StrokeInput` models tilt as one magnitude +
    orientation, not independent X/Y tilt axes, so there is no second
    real value to report; reporting `0.0` would fabricate a capability
    this device/library combination does not actually expose in that
    shape.
  - `source` -> always `FfiPointerSource.STYLUS`: the gate above
    guarantees no other pointer type's samples ever reach this
    function.
  - `capability_*` -> `queryRealInputCapabilities()`, a real
    `InputManager`/`InputDevice.getMotionRange` query over every
    connected stylus-sourced input device -- never hardcoded `true`.
    `palmRejection`/`eraser` are `false`: Android exposes no direct
    "device supports palm rejection" query, and this Alpha implements
    no rejection logic beyond the stylus-only gate itself.
- **Batch submit (Task 048)**: one `submitStroke` call per finished
  `Stroke`, inside `onStrokesFinished: (List<Stroke>) -> Unit`.
- **Cancellation (Task 051)**: not physically triggerable from this
  harness (no real S Pen, and `adb shell input` cannot simulate a mid-
  gesture cancel the way a real palm-rejection event would). Verified
  at the code level only: the gate consumes a non-stylus pointer's
  *down* event before `InProgressStrokes` sees it at all, so no stroke
  is ever started for it in the first place -- there is no
  code path here that could call `submitStroke` with zero/cancelled
  samples regardless (see the `if (samples.isNotEmpty())` guard).
- **Committed-ink re-rendering (Task 049/050) -- named gap, not
  silently dropped**: `FfiSceneSnapshot` carries only summaries (id +
  sample count for strokes, per Article 13's coarse-contract
  decision), not full point geometry a real `androidx.ink.strokes.
  Stroke` could be reconstructed from and re-rendered. This phase
  renders only strokes *this same app process* just finished (kept in
  a local `mutableStateListOf<Stroke>` the `MainActivity`-level
  Composable owns) -- a stroke committed in a *previous* session (after
  reopening a saved document) has no code path to become visible on
  the canvas yet. No task before Phase 09's toolbar/inspector work
  names a concrete mechanism for this (e.g. extending the FFI
  boundary with a raw-point export), so none was invented speculatively
  here; tracked as an explicit next-phase item, not hidden.

## Part E — Real on-device verification

Toolchain: JDK 17.0.20.1 (Temurin), Android SDK/NDK 27.2.12479018,
connected Samsung SM-T733 (Android 14/API 34), same device as Phase
17's evidence.

1. **Build**: `./gradlew.bat assembleDebug --no-daemon` -- after fixing
   the three real compile errors found by an *actual* build attempt
   (`rememberInProgressStrokesState`/`modifier`/`state` params on
   `InProgressStrokes` that do not exist; `StockBrushes.
   pressurePenLatest` that does not exist; `Matrix`/`Canvas` type
   mismatch) against the real sources-jar ground truth above,
   **BUILD SUCCESSFUL** on the very next attempt, 37/37 tasks, ~59s.
2. **Install + launch**: `./gradlew.bat installDebug` -- "Installed on
   1 device." `adb shell am start -n com.craftloop.shell/.MainActivity`
   followed by `adb shell dumpsys activity activities` showed
   `ResumedActivity: ActivityRecord{... com.craftloop.shell/.
   MainActivity ...}` -- genuinely running, matching Phase 17's own
   verification method. `libink.so` (Jetpack Ink's native rendering
   library) loaded successfully per `nativeloader` logcat lines.
3. **Crash check**: `adb logcat -d --pid=<pid>` across the *entire*
   session (build, install, two separate launches, ~10 minutes of
   activity) -- **zero** `FATAL`/`AndroidRuntime` lines
   (`grep -ciE "FATAL|AndroidRuntime"` = `0`). One pre-existing,
   unrelated Compose runtime warning ("Method ... clearImpl ... failed
   lock verification and will run slower") -- a known ART/R8
   optimization note, not a crash or functional defect.
4. **Visual screenshot / touch-gate test -- NOT completed this
   session, real limitation, not hidden**: partway through
   verification the tablet's screen timed out and locked. `adb shell
   input keyevent`/`swipe` calls to wake and unlock it left the device
   showing `mCurrentFocus=Window{... NotificationShade}` rather than
   the app, and every subsequent `screencap` returned a black image
   (Samsung's secure-surface screenshot protection over the lock
   screen, or the notification shade itself, most likely -- ADB alone
   cannot supply lock-screen credentials to dismiss this). `dumpsys
   activity activities` confirms the app's task/process is still alive
   underneath (`visible=true`, `app=ProcessRecord{... com.craftloop.
   shell ...}`, no crash) -- this is a device-screen-state problem, not
   an app failure -- but **no screenshot of the actual rendered canvas,
   and no `adb shell input touchscreen` stylus-vs-touch gate test,
   could be captured this session.** The developer needs to physically
   unlock the tablet for a follow-up session to complete Part E's
   remaining visual/touch-gate verification.

## What is proven vs. not, plainly

**Proven, with real evidence**: this compiles against the real 1.0.0
Jetpack Ink API (not an assumption), builds cleanly from source,
installs on the real Samsung tablet, launches, stays resumed, and
produces zero crashes across an extended real session including two
launches.

**Not proven this session**: that the canvas visually renders
correctly, that the stylus-only pointer gate actually blocks touch
input in practice (only reasoned about via the library's own
documented `isConsumed` contract, not observed), and anything about
real S Pen pressure/tilt capture (no physical stylus was used --
Phase 13's own job, and explicitly out of this phase's scope
regardless).

## Files changed

`android/app/build.gradle.kts` (Jetpack Ink + lifecycle-compose
dependencies), `android/app/src/main/java/com/craftloop/shell/
CraftLoopViewModel.kt` (new), `InkCanvas.kt` (new), `MainActivity.kt`
(rewritten).

## Follow-up: the stylus-only gate above was real, and wrong

After the tablet was physically unlocked in a later session, direct
testing found the "consume non-stylus changes in a sibling
`Box.pointerInput(PointerEventPass.Initial)`" mechanism above **did
not work**: a real S Pen stroke committed correctly
(`phase-06-07-canvas-screenshot.png`: `pointer=stylus revision=1
txCount=1`, a real quadrilateral drawn on screen), but a synthetic
`adb shell input touchscreen swipe 600 600 1200 1000 300` (a genuine
touch, not stylus) also produced a new, visible, committed ink stroke
(`phase-06-07-touch-gate-screenshot.png`: `pointer=touch revision=2
txCount=2`, with a new line matching the swipe's exact path). A third
check later read `revision=4` with a blank canvas
(`phase-06-07-gate-test-baseline.png`) -- consistent with `Ink`'s own
transient "wet ink" rendering clearing on some later recomposition,
not a new bug (see the already-disclosed Task 049/050 gap above: this
app never retains finished `Stroke`s in its own `committedStrokes`
list, so nothing survives past Ink's own internal handoff window).

**Real root cause**, found by downloading and reading the actual
`androidx.ink` 1.0.0 sources jars again (`ink-authoring-compose`,
`maven.google.com`), specifically
`androidx/ink/authoring/compose/InProgressShapes.kt`'s
`InProgressShapesImpl`: the library's own internal gesture detector
checks `change.changedToDown()` (which *does* respect `isConsumed`) at
the point it decides whether to start tracking a pointer -- so
consumption alone is the documented mechanism, and in principle Compose's
Initial-before-Main pass ordering should make an ancestor's Initial-pass
consumption visible there. The controlled test above proved that,
empirically, it was not taking effect for the touch swipe in this
app's actual composable tree shape. Rather than keep relying on
inter-composable pass-ordering as the *only* line of defense, the same
source file's `nextShapeSpec: () -> ShapeSpec?` parameter (surfaced to
`InProgressStrokes` as `nextBrush: () -> Brush?`) is a second,
independent, explicitly documented hook: "called at the start of each
pointer" (i.e. at down time), and if it returns `null`, the code's own
`if (shapeSpec != null) { ipsv.startShape(...) }` check (line ~289 of
that file) simply never starts a shape for that pointer at all -- no
ink, no finished-stroke callback, nothing to consume in the first
place.

**Fix**: `InkCanvas.kt` now tracks the most recently observed pointer
type (`lastToolType`, updated by the same Initial-pass observer that
already existed) and passes `nextBrush = { if (lastToolType ==
PointerType.Stylus) defaultBrush() else null }` to `InProgressStrokes`,
alongside (not instead of) the original consumption logic as
defense-in-depth.

**Verification status of the fix itself**: rebuilt and reinstalled
successfully (`gradlew installDebug`, `BUILD SUCCESSFUL in 1m 39s`,
38 tasks). Launching the app to re-run the exact controlled
touch-swipe test hit the **same screen-lock blocker again**
(`mWakefulness=Awake` after a wake keyevent, but
`mCurrentFocus=Window{... NotificationShade}` -- the device is
PIN/pattern/biometric-locked, not just asleep, and no `adb`-only
gesture dismisses that; guessing at a real unlock credential was not
attempted, correctly). **The fix compiles, installs, and is grounded
in the real library source rather than a guess, but the before/after
screenshot re-test proving it actually blocks touch on this device
could not be completed this session** -- this needs one more physical
unlock-and-retest pass, the same three commands already documented in
Part E above (`adb shell input touchscreen swipe ...` immediately
followed by a screenshot, compared against a screenshot taken first).

## Third attempt (commit after `2aa9522`) -- the `nextBrush` fix ALSO failed, real cause found and fixed for real

The developer unlocked the tablet and the coordinator ran the exact
controlled test the previous attempt could not complete. Result: **it
failed too.** Baseline `phase-06-07-gate-fix-baseline.png`:
`pointer=stylus revision=2 txCount=2` (a real stylus stroke visible).
After `adb shell input touchscreen swipe 400 900 1800 1300 300`,
`phase-06-07-gate-fix-after-touch.png`: `pointer=touch revision=3
txCount=3` -- the transaction count incremented again, meaning the
`nextBrush` hook did not prevent the shape from starting.

**The real root cause**, found this time by reading the *complete* raw
text of `InProgressShapes.kt`'s `InProgressShapesImpl` (not a
paraphrase -- the actual 400-line file, `androidx.ink:
ink-authoring-compose-android:1.0.0` sources jar from
maven.google.com) end to end: the composable's own internal `Box`
chains

```kotlin
Modifier.fillMaxSize()
    .pointerInput(nextPointerEventToWorldTransform, nextShapeToWorldTransform) { ... }
    // Use MotionEvent/View interop APIs to configure ASAP delivery of input events.
    // Don't add any real touch handling logic here, as the event processing of
    // pointerInteropFilter relative to pointerInput is inconsistent in its order -
    // for example by delivering down events in a different order to siblings than
    // move events, causing ordered event consumption logic to be confusing and
    // brittle.
    .pointerInteropFilter { containingView.requestUnbufferedDispatch(it); false }
```

quoted verbatim from the real source (the comment is the library's own
authors, not this project's). Both of this project's first two
attempts (Compose-level `PointerInputChange.consume()` from a sibling,
then a `nextBrush` hook keyed off a `lastToolType` variable updated by
that same sibling's Initial-pass observer) are *exactly* the kind of
"ordered event consumption logic" this exact comment warns is
"confusing and brittle" against this library's own internal
`pointerInput`/`pointerInteropFilter` combination. Both attempts
reasoned correctly about Compose's *documented* pass-ordering contract
in isolation; neither accounted for the library's own explicit warning
that its internal View-interop bridge does not reliably honor that
ordering relative to a sibling composable.

**The real fix**: stop trying to gate the pointer from inside Compose
at all. `MainActivity.dispatchTouchEvent(ev: MotionEvent)` is called by
the Android framework before the event reaches the View tree -- before
the root `ComposeView`, before Compose's own pointer input system,
before `InProgressShapesImpl`'s internal `pointerInput`/
`pointerInteropFilter`/`AndroidView` combination ever sees anything.
Checking `ev.getToolType(0) != MotionEvent.TOOL_TYPE_STYLUS` there and
returning `false` (swallowing the event, never calling
`super.dispatchTouchEvent`) means a non-stylus pointer never reaches
Compose at all, so no ordering assumption between two independent
input systems is needed. `InkCanvas.kt`'s now-provably-ineffective
Compose-level consumption/`nextBrush` logic was removed entirely rather
than left in place as inert, misleading dead code; the observed
pointer-type reporting for the Debug Region moved into
`dispatchTouchEvent` itself (recorded even for a swallowed event, so a
rejected touch is still visible diagnostic evidence, not silently
invisible).

**Verification -- real, double-checked, this time actually completed**:
rebuilt (`gradlew installDebug`, `BUILD SUCCESSFUL in 1m 14s`, 38
tasks), force-stopped and relaunched fresh. The tablet locked a third
time before this test could run; the developer unlocked it again
(confirmed via `adb shell dumpsys window` showing `isKeyguardShowing=
false` and the app itself focused) and the coordinator ran the test
directly:

- Fresh-launch baseline (`phase-07-real-fix-baseline.png`):
  `pointer=none revision=0 txCount=0`.
- Round 1: `adb shell input touchscreen swipe 400 900 1800 1300 300`
  then `adb shell input touchscreen tap 1000 700`
  (`phase-07-real-fix-after-touch.png`): `pointer=touch revision=0
  txCount=0` -- **unchanged**, blank canvas, no new ink.
- Round 2, independent gesture shapes
  (`phase-07-real-fix-after-touch-round2.png`): two more swipes,
  `pointer=touch revision=0 txCount=0` -- **unchanged again**.
- `adb logcat` for the app's PID across this entire sequence:
  `grep -ciE "FATAL|AndroidRuntime"` = `0`.

Both rounds show the Debug Region correctly recording `pointer=touch`
(diagnostic value preserved -- a rejected touch is still visible, not
silently invisible) while `revision`/`txCount` never move and no ink
ever appears. This is the real, measured, double-verified fix. No
physical stylus was used to re-confirm the positive case (none is
available in this harness); the fix's safety for real stylus input
rests on `MotionEvent.TOOL_TYPE_STYLUS` being Android's own documented,
stable tool-type constant for exactly that hardware class, not a
guess, and the debug region's `pointer=stylus revision=1/2` readings
earlier in this same file (from real S Pen strokes, before this bug
was found) already prove real stylus events reach the canvas and
commit correctly under the code path this fix did not touch.

**Known, deliberate narrowing to revisit later**: `dispatchTouchEvent`
gates ALL non-stylus input at the Activity level, which is
behaviorally identical to gating just the canvas region only because
this phase's actual screen contains nothing else touch-interactive yet
(a non-interactive Debug Region `Text`). Phase 08 (pan/zoom) and Phase
09 (icon toolbar) will need finger input to reach other on-screen
controls; this gate must narrow to the canvas region specifically at
that point. Documented here as a known follow-up, not a silently
deferred gap.

## Files changed (this fix)

`android/app/src/main/java/com/craftloop/shell/MainActivity.kt`
(`dispatchTouchEvent` override), `InkCanvas.kt` (removed the
ineffective Compose-level gating entirely).
