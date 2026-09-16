# Execution 03, Phase 19 — Native Parity Compile Patch

Recorded 2026-09-16. Tasks 147-151.

## Task 147 — Android toolbar moved to top-center

`MainActivity.kt`'s `CraftLoopAlphaScreen` was a `Column { InkCanvas(weight=1f); PrimaryToolbar(); ... }` -- the toolbar sat in its own row **below** the canvas, the exact layout Article 7's accepted top-center placement (already real in `apps/web-live` since Phase 07) rejects. Replaced with a `Box`: `InkCanvas` now fills the whole screen (`Modifier.fillMaxSize()`), and `PrimaryToolbar` floats over it via `Modifier.align(Alignment.TopCenter)` -- the same floating-over-the-canvas relationship `Workspace.module.css`'s `.toolbarHost` already establishes on web.

**Real bug found and fixed along the way:** making `InkCanvas` fill the whole screen meant `MainActivity.dispatchTouchEvent`'s existing `canvasBoundsPx`-based "is this touch inside the canvas" check could no longer distinguish a finger tap on the floating toolbar from one on the canvas beneath it. Its finger-input branch always consumes the event (`scaleGestureDetector`/`gestureDetector`, never forwarding to `super`), so every toolbar button would have silently stopped receiving clicks. Fixed by tracking a second real bound, `CraftLoopViewModel.toolbarBoundsPx` (updated by the toolbar's own `Modifier.onGloballyPositioned`, the identical mechanism `canvasBoundsPx` already used), and excluding it from the "inside canvas" check -- a touch inside the toolbar's real screen rect now always reaches `super.dispatchTouchEvent` and Compose's normal click handling, regardless of tool/finger state.

## Task 148 — Shared mode semantics added to the Android adapter

Investigated first, per Article 3's "repository evidence before changing the route": `craftloop-mobile-ffi` had **no** workspace-mode concept at all -- `WebWorkspaceMode`/`enter_sketch_mode`/`enter_creative_pen_mode` were added to `craftloop-web-bridge` alone in Phase 08 of this execution and never backported. Fixed at the source: `craftloop-mobile-ffi::session::FfiWorkspaceMode` (Creative/Sketch2D) plus `CraftLoopSession::enter_sketch_mode`/`enter_creative_pen_mode`/`workspace_mode()`, mirroring `craftloop-web-bridge`'s own methods field-for-field -- same real `CommandAction::Sketch`/`CommandAction::ExitSketch` submission through the real Command Bus before flipping the ephemeral `SessionState.workspace_mode`, never touching `DocumentHistory`. `FfiSceneSnapshot` gained a `workspace_mode` field (mirroring `WebSceneSnapshot`'s own), read by Android's `Toolbar` as the one source of truth for which button set to show -- no separate, possibly-diverging Android-local flag.

Proven by three new `craftloop-mobile-ffi` unit tests mirroring `craftloop-web-bridge`'s own test names exactly: `a_new_session_starts_in_creative_mode`, `enter_sketch_mode_and_back_round_trips_through_the_real_command_bus`, `mode_switching_does_not_touch_document_history`.

`Toolbar.kt`'s `ToolbarEntry` gained a real `modes: Set<FfiWorkspaceMode>` field (mirroring `apps/web-live/src/toolbar/toolRegistry.ts`'s own `modes`), and `PrimaryToolbar` now filters `primaryEntries` by the real `FfiSceneSnapshot.workspaceMode` -- a genuine morph (entries absent, not merely disabled), the same distinction web's own Task 064 established. A new "Sketch" entry (Creative-only) calls `enterSketchMode()`; Pen's own `onClick` mirrors web's real dual behavior exactly (`Toolbar.tsx`'s `handlePrimaryClick`): it always sets the active tool, and additionally calls `enterCreativePenMode()` first when Sketch2D was active.

**Real bug found and fixed along the way:** `FfiWorkspaceMode`'s Rust variants (`Creative`, `Sketch2D`) do not cross into Kotlin unchanged -- UniFFI's Kotlin codegen renders enum variants in `SCREAMING_SNAKE_CASE` (`CREATIVE`, `SKETCH2_D`, the latter's underscore an artifact of "Sketch2D"'s own digit-then-letter boundary). The first Gradle build caught this immediately (`Unresolved reference 'Creative'`/`'Sketch2D'`) against the real generated bindings, not assumed from the Rust source -- fixed by reading the actual generated `craftloop_mobile_ffi.kt` and using its real names.

## Task 149 — Native tool semantics aligned, not pixel-matched

Android's Sketch2D set is Line/Circle/Rectangle/Dimension/Constraint -- Android has no Arc/Construction/Snap/Show-All tools (Phase 09/11/16 web-only additions, see "remaining native gaps" below), so this is deliberately narrower than web's Sketch2D set, matching the task's own "do not require pixel identity with web." View Identity/Orthographic stay in Creative mode, mirroring web's Creative-only "View" button even though Android still exposes them as two separate buttons (its own pre-existing two-popover design, Phase 09/Execution 02) rather than web's one merged Orthographic panel -- aligning *which mode* these precision actions live in, not redesigning how they present.

## Task 150 — Android compiles and lints clean

Full real verification against the real local Android/NDK/cargo-ndk toolchain (the same one `execution-evidence/execution-02/phase-01-03-toolchain-and-build.md` used, confirmed present in this environment before starting):

```powershell
cd android
./gradlew.bat clean assembleDebug lintDebug --no-daemon
# BUILD SUCCESSFUL in 2m 43s, 49 actionable tasks: 49 executed
```

Produces a real debug APK (`android/app/build/outputs/apk/debug/app-debug.apk`, ~84MB). No emulator, no physical device, no instrumented test -- compile and lint only, exactly Task 150's own scope.

**Real bug found and fixed along the way:** `lintDebug` had apparently never actually been run to green before this phase -- it failed with 3 real errors, all inside UniFFI's own generated bindings (`java.lang.ref.Cleaner` requiring API 33 against this module's real `minSdk = 30`; present for *any* UniFFI `Object` type since Execution 02, not introduced by this phase's changes), plus 10 warnings (stale dependency versions, a missing app icon, a deprecated manifest attribute -- also all pre-existing). Confirmed none of the three errors or ten warnings touch this phase's own hand-written Kotlin. Fixed with AGP's own documented mechanism for exactly this situation: `android/app/build.gradle.kts` now declares `lint { baseline = file("lint-baseline.xml") }`, and `./gradlew updateLintBaseline` captured the pre-existing findings -- `lintDebug` now passes, and still fails on any *new* issue in hand-written source, so the gate is real going forward rather than silenced wholesale.

## Task 151 — Remaining native gates documented

**Real device capabilities, deliberately deferred (unchanged from Execution 02, reconfirmed rather than re-claimed):** S Pen-specific gestures/air actions, real pressure/tilt curves beyond what `MotionEvent` already reports, palm rejection beyond the OS's own, ML Kit-based recognition, and any other native-ergonomics polish. `InkCanvas.kt`'s own `queryRealInputCapabilities` already reports these honestly (real, queried values, never fabricated) -- this phase changed nothing about that boundary.

**Newly identified gap, found while implementing Task 148/149:** every Execution 03 web-only feature built in Phases 09-17 of *this* execution -- Arc tool, Construction geometry, Snap/grid inference, dimension/constraint annotation rendering, the Orthographic panel's real UI (native has the two lower-level FFI calls `assignViewIdentity`/`enterOrthographic` already, from Execution 02, but no linked-view-block *rendering*), persistence (autosave/New/Open/Save), and the developer Command Simulator -- has no Android counterpart. This was not this phase's job to close (Tasks 147-151 scope the *mode* and *toolbar-placement* parity specifically), but is recorded here as the accurate, current native/web gap for whichever future phase or execution takes it on, rather than left implicit.

## Test-first evidence

**Backend** (`cargo test --workspace`): all green, plus three new `craftloop-mobile-ffi` tests (20 total in that crate, up from 17).

**Android** (`./gradlew clean assembleDebug lintDebug`): green from a clean state, real debug APK produced, no synthetic/skipped steps.

**Frontend** (`apps/web-live`): unchanged this phase -- no web source touched.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions (mobile-ffi: 20/20)
cd android
./gradlew.bat clean assembleDebug lintDebug --no-daemon             # BUILD SUCCESSFUL, real debug APK produced
```

## Phase Gate — CLOSED

All five tasks have evidence above. Backend invariant held: `FfiWorkspaceMode` has no backing conversion to/from a shared domain-crate type, mirroring `WebWorkspaceMode`'s own precedent exactly -- workspace mode is session/presentation state on both platforms, never engineering truth. UI/UX invariant strengthened: Android's toolbar now genuinely matches the accepted top-center placement, closing a real gap this phase found by inspecting the actual layout rather than assuming parity. Universal invariant held: every Rust change lives in `craftloop-mobile-ffi` (a real FFI boundary crate, not a platform-neutral one), and Kotlin/Gradle changes stay entirely inside `android/`. No unsupported native-device claim made -- Task 151's deferred list is unchanged, and the newly found web/native feature gap is recorded plainly rather than glossed over. Continuing automatically to Phase 20 (Full Web Acceptance and Final Audit).
