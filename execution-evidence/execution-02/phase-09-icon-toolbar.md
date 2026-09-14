# Execution 02, Phase 09 — Icon-First Toolbar

Recorded 2026-09-14. Tasks 058-064. Authority: Article 7 (UI Contract).

## What was built

**`android/app/src/main/java/com/craftloop/shell/Toolbar.kt`** (new):
`PrimaryToolbar` -- Article 7's exact required icon list, icon-only,
each wrapped in a Material3 `TooltipBox` (long-press name, Task 061)
and a real `contentDescription` (Task 059) equal to that same name.
The active tool gets a `primaryContainer`-tinted background (Task 060)
-- no other visual design, per Article 7. Icon choices (Material
Icons Extended, added as a new dependency in `app/build.gradle.kts`):

| Tool | Icon | Note |
|---|---|---|
| Pen | `Icons.Filled.Edit` | |
| Eraser | `Icons.AutoMirrored.Filled.Backspace` | |
| Select | `Icons.Filled.NearMe` | |
| Line | `Icons.Filled.Timeline` | no literal "line" icon exists; closest reasonable fit |
| Circle | `Icons.Filled.Circle` | |
| Rectangle | `Icons.Filled.CropSquare` | |
| Dimension | `Icons.Filled.Straighten` | a ruler -- genuinely apt |
| Constraint | `Icons.Filled.Link` | constraints "link" geometry together |
| View Identity | `Icons.Filled.Visibility` | |
| Orthographic | `Icons.Filled.GridOn` | suggests a multi-view grid; no closer standard icon exists |
| Undo/Redo | `Icons.AutoMirrored.Filled.Undo`/`Redo` | |
| Save | `Icons.Filled.Save` | |

`ConstraintPopover` (Task 062), `ViewIdentityPopover` (Task 063),
`InputCapabilityDialog`/`AlphaSettingsDialog` and the overflow
`DropdownMenu` (Task 064) are all real `AlertDialog`/`DropdownMenu`
instances wired to real `CraftLoopSession` calls, not stubs -- see
"Real API wiring" below.

**`CraftLoopViewModel.kt`**: `Tool` enum extended with `DIMENSION`,
`CONSTRAINT`, `VIEW_IDENTITY`, `ORTHOGRAPHIC`. New methods:
`eraseAt`, `createShapeFromDrag`, `applyConstraint`,
`assignViewIdentity`, `enterOrthographic`,
`dimensionToolNotYetAvailable`, `save`/`open`/`newDocument`,
`toggleDebugOverlay`, and the five popover/dialog visibility setters.
`session` changed from `val` to `var` (Task 064's New/Open replace the
whole session object -- `CraftLoopSession` has no in-place "reset"
operation, and none should exist per Article 12's object lifecycle).

**`MainActivity.kt`**: the Phase 08 temporary text-button row is gone,
replaced by `PrimaryToolbar(viewModel)`. `dispatchTouchEvent` gained a
third real branch: while Line/Circle/Rectangle is the active tool, a
stylus down/up pair is captured directly (never reaching
`InProgressStrokes` -- these are direct two-point primitives, not
freeform ink) and calls `createShapeFromDrag`. The existing Select-tap
branch now checks `activeTool == Tool.ERASER` and calls `eraseAt`
instead of `selectAt` when appropriate -- same real stylus-tap
mechanism Phase 08 already proved, just two different real outcomes.

## Real API wiring (not stubs)

- **Constraint popover**: reads `sceneSnapshot.selectedEntityIds`
  directly; offers Horizontal/Vertical only when exactly one primitive
  is selected, Coincident/Parallel/Perpendicular/EqualLength/
  EqualRadius/Concentric only when exactly two are -- matching
  `FfiConstraintKind`'s real arity in `session.rs` exactly (checked
  there directly, not assumed). No plain "Equal": `SketchConstraintKind`
  only has `EqualLength`/`EqualRadius` since Phase 04, so both are
  offered as distinct options rather than inventing a merged one.
- **View Identity popover**: calls the real `assignViewIdentity`,
  creating the Alpha's one view on first use and reusing it afterward
  (`EphemeralUiState.currentViewId`) -- also feeds the current
  selection into `addGeometryToView` so the view is not left
  permanently `LinkReady`-ineligible with no way to populate it before
  Phase 11 exists.
- **Orthographic icon**: calls the real `enterOrthographic` against the
  current view, reporting the real derived-view count (or the real
  error) via a one-shot Debug Region message -- proves Phase 04's
  already-built call is reachable end to end from a real toolbar
  action. Does **not** render the resulting Top/Right blocks anywhere
  -- that is Phase 11's real scope.
- **Dimension icon**: highlights like every other tool but its
  `onClick` only sets a "Phase 10 -- not built yet" status message,
  stated as such rather than silently doing nothing.
- **Save/Open/New**: `save`/`open` act on one fixed path
  (`context.filesDir/craftloop-alpha-document.json`) -- no file picker
  exists, stated plainly; a real per-document picker has no task
  driving it before Phase 12.

## Real build errors found and fixed (not guessed around)

The real Gradle build (not assumption) caught two errors on the first
attempt:
1. `TooltipBox`/`PlainTooltip`/`rememberTooltipState` require
   `@OptIn(ExperimentalMaterial3Api::class)` in `material3:1.3.0`
   (the version this project's BOM actually resolves to, confirmed by
   inspecting the real cached `.aar`, not the BOM string alone).
2. `TooltipDefaults.rememberTooltipPositionProvider()` does not exist;
   the real name, found by extracting `material3-release.aar`'s
   `classes.jar` and grepping the actual class file list for `Tooltip`
   (no sources jar was cached locally, so this was the closest ground
   truth available), is
   `TooltipDefaults.rememberPlainTooltipPositionProvider()`.

Both fixed; the second `assembleDebug` was clean except one harmless
deprecation warning (`Icons.Filled.Backspace` -> the AutoMirrored
version), also fixed.

## Real on-device verification

- **Build**: `assembleDebug`, clean after the two fixes above, no
  warnings. **PASS.**
- **Install**: `installDebug` -- "Installed on 1 device." One real,
  in-session complication: the connected Samsung tablet went from
  `device` -> `offline` -> absent from `adb devices` entirely partway
  through this phase (cause not diagnosed -- USB enumeration drop, not
  triggered by anything this session did to the device), then
  reappeared on its own after `adb kill-server`/`start-server`.
  Documented here as a real, observed hardware/USB flakiness, not
  hidden. **PASS**, once the device came back.
- **Launch**: `am start` + `dumpsys activity activities` confirmed
  `ResumedActivity` is `MainActivity`; logcat for the full session's
  PID has **zero** `FATAL`/`AndroidRuntime` lines. **PASS.**
- **Screen lock, hit and then resolved mid-session**: immediately after
  the device reappeared and the app was confirmed resumed, a
  `screencap` returned a fully black image. `dumpsys power`/`dumpsys
  window` showed `mWakefulness=Dozing`, `mCurrentFocus=
  NotificationShade` -- the screen had locked again (as before in this
  project). `adb shell input keyevent KEYCODE_WAKEUP` + a swipe-up did
  **not** dismiss it (`mCurrentFocus` stayed `NotificationShade`),
  confirming a genuine PIN/pattern/biometric lock -- correctly not
  bypassed, no credential guessed. The device was unlocked (by the
  developer, outside this session's control) a short time later,
  confirmed by `mCurrentFocus` returning to `MainActivity`, and full
  visual/interactive verification below completed once that happened.
- **Toolbar renders correctly** (`phase-09-toolbar.png`): all fourteen
  primary icons visible in the documented order, Pen shown active
  (tinted background) as the default tool, Redo correctly greyed out
  (`canRedo=false`), Debug Region shows `tool=PEN`.
- **Tool switching, real** (`phase-09-select-test.png`): tapping the
  Select icon changed `tool=SELECT` in the Debug Region and visibly
  moved the active-tool tint to that icon.
- **Article 11 holds for the new Eraser/Select routing too**: a
  synthetic `adb shell input tap` on the canvas while Select was active
  registered as `pointer=touch` and left `selected=0` -- a finger tap
  correctly does not select (the same real tool-type check Phase 08's
  `onSingleTapUp` already had, now also gating `eraseAt`).
- **Overflow menu renders correctly** (`phase-09-overflow-menu2.png`):
  all seven real items (New, Open, Delete selected, Fit View, Debug
  Inspector, Input Capability Inspector, Alpha Settings) appear as a
  real `DropdownMenu`.
- **Input Capability Inspector shows real, queried device data**
  (`phase-09-capability-dialog.png`): `pressure=true tilt=true
  hover=true palmRejection=false eraser=false` -- genuine
  `InputManager`/`MotionRange` output from this Samsung tablet's real
  S Pen digitizer, not a placeholder (incidentally, this is real Phase
  13-relevant physical capability evidence, captured here for free).
- **View Identity popover, fully real end to end**
  (`phase-09-view-identity-popover.png` ->
  `phase-09-view-identity-front-assigned.png`): opening it shows
  Front/Top/Right/Back; tapping "Front" produced `revision`/`txCount`
  advancing by exactly 1 (5 -> 6), the popover closing itself, and the
  View Identity icon becoming the active-tinted icon -- a real
  `CraftLoopSession.assignViewIdentity` transaction, not a UI-only
  state change.

## What is proven vs. not, plainly

**Proven, with real evidence, on the real Samsung tablet**: the
toolbar compiles against the real Material3 1.3.0 API (two real
errors found and fixed, not guessed around), builds, installs,
launches with zero crashes, renders every primary icon correctly with
working active-tool highlighting, the overflow menu and its Input
Capability Inspector both work and the latter surfaces genuine device
capability data, and the View Identity popover drives a real,
committed `CraftLoopSession` transaction end to end.

**Not independently re-exercised this session**: the Constraint
popover's real `applyConstraint` call (its arity-gating UI logic was
read and reasoned through, and it reuses the exact same real
transaction pattern View Identity just proved works, but no primitive
was actually selected via a real stylus tap to drive it through, since
this harness has no physical stylus), and the Line/Circle/Rectangle
drag-capture path (same reason -- creating a shape via stylus drag
needs a real stylus event this harness cannot synthesize; only a
synthetic finger tap was tested, which correctly did nothing). Both
are code-reviewed and structurally consistent with the View Identity
path that was proven, but a physical stylus test is the honest
remaining step, per Article 27.

## Files changed

`android/app/build.gradle.kts` (material-icons-extended dependency),
`android/app/src/main/java/com/craftloop/shell/{MainActivity,
CraftLoopViewModel}.kt`, `Toolbar.kt` (new).
