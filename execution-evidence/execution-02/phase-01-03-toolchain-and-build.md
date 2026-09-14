# Execution 02, Phases 01-03 — Windows Toolchain, Gradle Reproducibility, Rust-to-Android Native Build

Recorded 2026-09-14. Tasks 006-021. Authority: Articles 19-22, 34 (Gate A).

## What changed since Phase 00

Phase 00 (`phase-00-baseline.md`) found this host had no JDK, Android
SDK, NDK, `adb`, or Gradle Wrapper -- a true blocker for these phases
per Article 31. With the developer's explicit approval, this session
installed the full toolchain via command-line downloads (no Android
Studio GUI) rather than stopping:

| Tool | Source | Version installed |
|---|---|---|
| JDK | `winget install EclipseAdoptium.Temurin.17.JDK` | Temurin 17.0.20.1+1 |
| Android SDK / Platform Tools | Google's official `commandlinetools-win-15859902_latest.zip` (SHA-256 verified against the published checksum before extracting) | Platform-tools 37.0.1, platform android-35, build-tools 35.0.0 |
| Android NDK | `sdkmanager "ndk;27.2.12479018"` | 27.2.12479018 (pinned; r27 line, not the bleeding-edge r29/r30 already available, matching Article 20's "avoid fragile dependency magic" and Article 37's dependency-pinning policy) |
| Rust Android target | `rustup target add aarch64-linux-android` | installed |
| Gradle (used once, to generate the wrapper) | `services.gradle.org/distributions/gradle-8.9-bin.zip` (SHA-256 verified) | 8.9 |
| `cargo-ndk` | `cargo install cargo-ndk` | 4.1.2 |

`ANDROID_HOME`/`ANDROID_SDK_ROOT` and `PATH` (platform-tools,
cmdline-tools/latest/bin) were persisted at User scope via
`[System.Environment]::SetEnvironmentVariable`, so a fresh terminal on
this machine sees them without re-running any of the above.

Disk space note: this host was at 4.3GB free (222GB drive, matching
Execution 01's own prior report of tight resource constraints on this
project). `cargo clean` reclaimed 9.9GB before the first Android build
was attempted, bringing free space to 12GB. Worth monitoring before any
future clean-build run.

## Task 011 — Environment doctor script

`scripts/environment-doctor.ps1`: checks Git, rustup, cargo, the
`aarch64-linux-android` Rust target, Java, the Android SDK, `adb`, the
Android NDK, this repository's Gradle Wrapper, and a connected/
authorized device. Detects and reports only -- never installs, never
fabricates a pass. Two real bugs were found and fixed while validating
it against this now-fully-installed toolchain, before it was trusted
for anything else:
1. `(adb devices) 2>&1` wrapped stderr lines (`* daemon not running...`)
   in `ErrorRecord` objects under Windows PowerShell 5.1, and calling
   `.Trim()` on one crashed the script. Fixed by capturing stdout only
   and locating the real `List of devices attached` line instead of
   assuming line 1 is always the header.
2. `($results | Where-Object {...}).Count` returns `$null` (not `0`)
   when zero results match in Windows PowerShell 5.1, which silently
   broke the final blocker/fail count and caused the script to print
   `Result: READY.` once even though a real `BLOCKER` (missing Gradle
   Wrapper) was present in the same run. Fixed with `@(...)` to force
   array context. This is exactly the class of bug the No-Hallucination
   Policy exists to catch before it reaches a human as a false "ready"
   signal -- found and fixed in this same session, not shipped.

Current real output on this host (Samsung tablet not yet connected):
8 PASS, 1 FAIL (no device attached), 1 BLOCKER at the time of writing
(Gradle Wrapper, since fixed below) -- see the script's own output for
the live state.

## Phase 02 — Gradle Reproducibility (Tasks 012-016)

- **Task 012 (Gradle Wrapper)**: generated for real via
  `gradle wrapper --gradle-version 8.9` (AGP 8.6.1's documented minimum
  is Gradle 8.7; 8.9 is a stable minor release inside that range, not
  the newest available). `android/gradlew`, `gradlew.bat`,
  `gradle/wrapper/gradle-wrapper.{jar,properties}` are committed;
  `local.properties`, `.gradle/`, and `build/` are not (new
  `android/.gitignore`).
- **Task 014 (real compile error found)**: the very first real Gradle
  invocation failed project configuration outright:
  `Starting in Kotlin 2.0, the Compose Compiler Gradle plugin is
  required when compose is enabled.` Execution 01's scaffold predated
  this Kotlin 2.0 requirement and was never actually build-validated
  (its own README said exactly that), so this was invisible until a
  real build ran. Fixed by adding
  `org.jetbrains.kotlin.plugin.compose` (version `2.0.21`, matching
  the already-pinned Kotlin version) to both `android/build.gradle.kts`
  and `android/app/build.gradle.kts`.
- **Task 016 (clean build proof)**: `./gradlew.bat clean` then
  `./gradlew.bat assembleDebug` from a fully clean state, twice, both
  green -- see Phase 03 below for the full log; this is Gate A.

## Phase 03 — Rust-to-Android Native Build (Tasks 017-021)

Chose `cargo-ndk` (Task 017) over hand-rolled `.cargo/config.toml`
linker plumbing -- it is the standard, maintained tool for this exact
job and keeps the Android-specific cross-compile logic out of the
Rust workspace's own configuration.

**Automated, not manually copied** (Article 19's explicit prohibition
on "manually copying stale `.so` files or generated bindings from
evidence folders"): `android/app/build.gradle.kts` now defines two
real Gradle `Exec` tasks, both wired as a `preBuild` dependency so
plain `assembleDebug` triggers them with no extra developer step:

- `cargoBuildAndroid` -- runs
  `cargo ndk --target arm64-v8a --platform 30 -o <build>/rustJniLibs
  build -p craftloop-mobile-ffi`, with `ANDROID_NDK_HOME` set from
  `android.ndkDirectory` (AGP's own resolved NDK, pinned via
  `ndkVersion = "27.2.12479018"` in the same file) so the Rust
  cross-compile and the rest of the Android build always agree on
  which NDK they used.
- `generateUniffiBindings` -- depends on the above, runs the
  `craftloop-mobile-ffi` crate's own `uniffi-bindgen` binary
  (`cargo run --bin uniffi-bindgen -- generate --library
  <compiled .so> --language kotlin --out-dir <build>/generated/uniffi`)
  against the just-built native library.

Both outputs are registered as real Android/Kotlin source
directories (`android.sourceSets["main"].jniLibs.srcDir(...)`,
`kotlin.sourceSets["main"].kotlin.srcDir(...)`) under `build/`, so
neither the `.so` nor the generated `.kt` is ever committed --
regenerated by every build instead.

### Gate A -- proven, twice, from a clean checkout

```powershell
cd android
.\gradlew.bat clean
.\gradlew.bat assembleDebug
```

Second (fully clean) run's real task list:
`cargoBuildAndroid` -> `generateUniffiBindings` -> `preBuild` -> ...
-> `compileDebugKotlin` -> ... -> `packageDebug` -> `assembleDebug`.
**BUILD SUCCESSFUL in 2m 39s, 37/37 tasks executed.**

Output: `android/app/build/outputs/apk/debug/app-debug.apk`, 42.4MB,
verified with `aapt dump badging` (package `com.craftloop.shell`,
`minSdkVersion 30`, `targetSdkVersion 35`) and `unzip -l` (contains
`lib/arm64-v8a/libcraftloop_mobile_ffi.so`, 18.7MB, produced by this
session's own cross-compile, not a stale copy).

The placeholder `MainActivity.kt`'s existing `resolveCommand`/
`FfiCommandNamespace` calls compiled and linked correctly against the
freshly generated bindings, which now also include the entire
`CraftLoopSession` API from Phase 04 (4176 generated lines) -- proving
the FFI boundary end to end, from real Rust source to a real linkable
Android artifact, for the first time in this project's history.

## What Gate A does not prove

No Samsung tablet is connected to this host (Gate B onward). This
build has not been installed on, or launched on, any physical or
emulated Android device. Compiling and packaging is not the same as
running -- per Article 27, that claim is not made here.

## Files changed

`android/build.gradle.kts`, `android/app/build.gradle.kts` (new
`cargoBuildAndroid`/`generateUniffiBindings` tasks, `ndkVersion`,
Compose Compiler plugin), `android/.gitignore` (new),
`android/gradlew`, `android/gradlew.bat`, `android/gradle/wrapper/*`
(new), `scripts/environment-doctor.ps1` (new, two bugs fixed in this
same session).
