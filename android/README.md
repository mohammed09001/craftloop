# Craft Loop Android Shell (placeholder)

Execution 01, Phase 28, Task 203/206. **Disposable scaffold, never
production UI** — the same status `apps/windows-harness` has for Phase 04
(Engine Contract 27: adapters are test/scaffold surfaces; the shared Rust
core in `../crates/` is the only source of engineering truth).

## What this is

The minimum Gradle project structure needed to prove the shape of the
integration — a single-Activity Compose app that loads the
`craftloop_mobile_ffi` native library and calls one real function
(`resolveCommand`) — not a real drawing surface, not Jetpack Ink
integration, not a design system. Task 206 is explicit that Execution 01
"does not spend its budget on final visual design," and this module does
not: `MainActivity.kt` renders one line of plain text and nothing else.

## What this is not, and why

- **Not build-validated.** This sandbox has no Android SDK, no Gradle,
  and no `adb` (`which gradle`/`which adb` both fail; `$ANDROID_HOME` is
  unset). This is a genuine platform-toolchain blocker in the sense the
  governing execution plan anticipates for this phase, not an oversight
  — every file here is hand-written to be structurally correct Gradle/
  Kotlin/AndroidManifest syntax, but "compiles and runs" has not been,
  and cannot be, verified in this environment. Classify any claim about
  this module as **Scaffolded/stubbed**, never as tested.
- **Does not embed a prebuilt native library.** `app/build.gradle.kts`
  references `jniLibs` the way a real integration would, but no
  `.so` file is checked in: cross-compiling `craftloop-mobile-ffi` to
  Android targets (`aarch64-linux-android`, etc.) needs the Android NDK,
  which is also unavailable here. Wiring a real build step for that is
  future work once real Android CI exists (see Phase 27's
  `.github/workflows/`, which does not yet have an Android job for
  exactly this reason).
- **Not Jetpack Ink.** No `androidx.ink.*` dependency is declared. Task
  204's Jetpack Ink mapping is a design document
  (`../execution-evidence/mobile-ffi/jetpack-ink-mapping.md`), not code,
  precisely because wiring it up for real belongs to whichever future
  phase actually builds the Android drawing surface.

## Files

- `settings.gradle.kts`, `build.gradle.kts` (root) — minimal Gradle
  Kotlin DSL project wiring, Android Gradle Plugin + Kotlin plugin
  versions pinned.
- `app/build.gradle.kts` — single `com.android.application` module,
  Jetpack Compose enabled, `minSdk`/`targetSdk` chosen for a tablet-class
  device (Samsung Galaxy Tab S-series, matching Task 205's validation
  target).
- `app/src/main/AndroidManifest.xml` — one `MainActivity`, no permissions
  beyond the default.
- `app/src/main/java/com/craftloop/shell/MainActivity.kt` — a single
  Compose screen that calls `craftloop_mobile_ffi.resolveCommand` (via
  the UniFFI-generated Kotlin bindings this repository already proved
  generate correctly — see
  `../execution-evidence/mobile-ffi/generated-kotlin/`) and displays the
  result as plain text. Nothing else.
