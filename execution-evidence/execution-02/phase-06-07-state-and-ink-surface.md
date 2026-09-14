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
