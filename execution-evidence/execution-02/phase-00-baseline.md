# Execution 02, Phase 00 — Baseline and Repository Lock

Recorded 2026-09-14. Tasks 001-005.

## Task 001 — Re-audit Android placeholder

Repository evidence, inspected directly:

- `android/` is exactly the placeholder shell Execution 01's final
  report describes: `settings.gradle.kts`, root `build.gradle.kts`
  (AGP 8.6.1 / Kotlin 2.0.21, apply-false plugin pins only),
  `app/build.gradle.kts` (single `com.android.application` module,
  Compose enabled, `compileSdk 35` / `minSdk 30` / `targetSdk 35`,
  Java/Kotlin 17), `app/src/main/AndroidManifest.xml`, one
  `MainActivity.kt` that calls `resolveCommand("pen")` and renders the
  result as plain text. No Jetpack Ink dependency, no `jniLibs`, no
  Gradle Wrapper (`gradlew`/`gradlew.bat`/`gradle/wrapper/` do not
  exist), no real drawing surface, no design system.
- `crates/craftloop-mobile-ffi` exposes exactly two representative
  boundaries: `validate_stroke` (input) and `resolve_command`
  (command grammar). No `CraftLoopSession`, no scene snapshot, no
  document/dimension/constraint/view/conflict/undo operations cross
  the FFI boundary yet. This is Execution 02's Phase 04 target, not a
  regression.
- `.github/workflows/`: `ci.yml` (Rust-only: fmt/clippy/test) and
  `macos-compile-gate.yml`. No Android CI job exists yet (Phase 16
  target).
- Execution 01's final report
  (`execution-evidence/34-phase-32-final-execution-report.md`)
  confirms the same starting point this audit found independently:
  Android is "Scaffolded/stubbed, never tested," blocked on a missing
  Android SDK/Gradle/NDK/`adb` in that session's sandbox.

**Gap map for Execution 02** (matches Articles 5-6, 12-22 exactly):
no Gradle Wrapper; no pinned NDK/cargo-ndk cross-compile pipeline; no
`CraftLoopSession`; no scene snapshot contract; no Jetpack Ink
surface; no icon toolbar; no dimension/constraint/view/orthographic
UI; no persistence UI; no handwriting/ink-command adapters; no
Android CI; no USB dev-loop docs; no physical-device evidence.

## Task 002 — Rust baseline

Run from repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

Results: `fmt` clean (exit 0). `clippy` clean, zero warnings across
all 22 crates + `windows-harness` (workspace `[lints] warnings =
"deny"` policy self-enforces this). `test`: **760 passed, 0 failed**
across 74 test binaries (unit + integration + scenario + doctests) —
matches Execution 01's final report exactly. This is the Execution 02
starting baseline; any regression during this execution is a real
regression, not pre-existing.

## Task 003 — Android/toolchain dependency versions recorded

| Item | Value | Source |
|---|---|---|
| Android Gradle Plugin | 8.6.1 | `android/build.gradle.kts` |
| Kotlin plugin | 2.0.21 | `android/build.gradle.kts` |
| compileSdk / minSdk / targetSdk | 35 / 30 / 35 | `android/app/build.gradle.kts` |
| Java/Kotlin JVM target | 17 | `android/app/build.gradle.kts` |
| Compose BOM | 2024.09.02 | `android/app/build.gradle.kts` |
| JNA | 5.14.0 (`@aar`) | `android/app/build.gradle.kts` |
| UniFFI | `=0.32.1` (pinned exact, `features = ["cli"]`) | `crates/craftloop-mobile-ffi/Cargo.toml` |
| Rust edition / MSRV | 2021 / 1.75 | root `Cargo.toml` |
| Rust toolchain installed (this host) | rustc 1.98.1, cargo 1.98.1, rustup 1.29.1 | `rustc --version` |
| Rust Android targets installed | **none** (`rustup target list --installed` shows only `x86_64-pc-windows-msvc`) | this host |
| Gradle Wrapper | **absent** | `android/` has no `gradlew`, `gradlew.bat`, or `gradle/wrapper/` |
| Java (JDK) | **not found** (`java -version` → command not found) | this host |
| Android SDK | **not found** (no `ANDROID_HOME`/`ANDROID_SDK_ROOT`, no `%LOCALAPPDATA%\Android\Sdk`) | this host |
| Android NDK | **not found** (no SDK means no NDK) | this host |
| `adb` | **not found** | this host |
| Connected Android device | **none observed** (`adb` itself is absent) | this host |

## Task 004 — Evidence directory

Created `execution-evidence/execution-02/`. This file and every
subsequent Execution 02 evidence file lives here; the final report
will be `execution-evidence/execution-02/FINAL-REPORT.md` per Article
40.

## Task 005 — Universal boundary audit

```
grep -rlE "android|jetpack|compose|jna|uniffi" <every platform-neutral crate> --include="*.rs" --include="Cargo.toml" -i
```

Matches found only in doc-comment prose (future-adapter references,
e.g. `craftloop-input/src/sample.rs`, `craftloop-handwriting/src/lib.rs`,
`craftloop-document/src/autosave.rs`) and in `craftloop-mobile-ffi`
itself, which is the one crate allowed to depend on `uniffi`. No
platform-neutral crate imports `uniffi`, Android SDK types, Compose,
Jetpack Ink, or JNA. The universal-core boundary Execution 01
established is intact at the start of Execution 02.

## Phase Gate — CLOSED

All five tasks have evidence above. Rust baseline is green (760/760,
clippy clean, fmt clean). No platform leakage. Continuing
automatically to Phase 01.

**True blocker identified for Phases 01-03 and 16-19**: this
development host has no JDK, no Android SDK, no Android NDK, no
`adb`, and no connected Android device. This matches Article 31's own
definition of a true blocker ("unavailable Android SDK/NDK") and
Execution 01's own prior finding in a different sandbox. Per Article
32, continuing automatically on everything this blocker does not
prevent (Phase 04's `CraftLoopSession` Rust expansion is fully
testable without any Android toolchain), while stating the exact
human action needed to unblock the rest.
