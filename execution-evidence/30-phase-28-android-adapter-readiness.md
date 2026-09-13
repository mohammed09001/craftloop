# Phase 28 — Android Adapter Readiness — Evidence

Recorded: 2026-09-13

New crate: `crates/craftloop-mobile-ffi`. New non-Rust scaffold:
`android/` (placeholder Gradle/Kotlin shell, per `apps/README.md`'s own
pre-existing note that Android/iPad adapters live in their own top-level
directories, not under `apps/`).

This phase is the first genuinely expected true blocker named by the
governing execution plan: no Android SDK, Gradle, `adb`, or NDK exists in
this sandbox (`which gradle`/`which adb` both fail; `$ANDROID_HOME` is
unset). Every task below is completed to the full extent possible without
that toolchain -- real Rust code, a real generated-and-inspected Kotlin
binding smoke test, real scaffolding -- with the toolchain-dependent
remainder named explicitly rather than silently skipped or claimed done.

## Task 201 — Define Android FFI boundary

`crates/craftloop-mobile-ffi` (new). Two representative boundaries, not
an exhaustive mirror of every domain type (deliberately -- see the
crate's own module doc): `FfiPointerSample`/`validate_stroke` (input) and
`FfiCommandNamespace`/`FfiGrammarMatch`/`resolve_command` (command). Every
`Ffi*` type is a plain, lifetime-free, FFI-safe mirror of a real domain
type with an explicit `From`/`TryFrom` conversion -- never the domain
type re-exported directly, and no domain crate (`craftloop-input`,
`craftloop-ink`, `craftloop-command`) has any dependency on this crate or
knowledge a mobile platform exists. `validate_stroke` calls a real
domain function (`craftloop_ink::Stroke::new`) and returns its real
result, proving the boundary is a genuine two-way bridge: 6 tests cover a
valid stroke, an empty stroke (rejected with the real domain error
message), a mixed-`PointerSource` stroke (rejected), an exact grammar
match, a no-match, and an ambiguous-prefix match carrying every candidate
word as owned `String`s (the exact reason `GrammarMatch::Ambiguous`'s
`Vec<&'static str>` cannot cross an FFI boundary unmodified).

## Task 202 — Evaluate UniFFI Kotlin bindings

Pinned `uniffi = "=0.32.1"` (exact version, matching the same convention
Phase 11's `ezpz` decision record established), proc-macro-only
scaffolding mode (no separate `.udl` file). **Real smoke test, not a
hand-written expectation**: built the crate's cdylib, then actually ran
`uniffi-bindgen generate --language kotlin` against the compiled
`.dll` and inspected the output -- full transcript and grep-verified
symbol list in `mobile-ffi/phase-28-kotlin-bindgen-smoke-test.txt`; the
1613-line generated Kotlin file itself is committed at
`mobile-ffi/generated-kotlin/uniffi/craftloop_mobile_ffi/craftloop_mobile_ffi.kt`.
Every exported function and FFI type is present with correct Kotlin
naming. Explicitly **not** claimed: that the generated Kotlin compiles
under a real Kotlin compiler or runs on a JVM/Android device -- no JDK or
Android toolchain exists in this sandbox to verify that.

## Task 203 — Create Android shell placeholder

`android/` (Gradle Kotlin DSL, root `settings.gradle.kts`/`build.gradle.kts`,
one `app` module: `AndroidManifest.xml`, `MainActivity.kt`, `strings.xml`).
Deliberately minimal, matching `apps/windows-harness`'s own "disposable
test adapter, never production authority" precedent (Engine Contract 27):
`MainActivity` renders one line of plain text showing the real result of
one call into the generated UniFFI Kotlin bindings
(`resolveCommand("pen", FfiCommandNamespace.NOTEBOOK)`) -- no navigation,
no drawing surface, no design system. `compileSdk`/`minSdk` chosen for a
tablet-class device (Galaxy Tab S-series, matching Task 205's target), not
phone-first. **Not build-validated**: no Gradle/Android SDK/NDK in this
sandbox, so this module has never actually been compiled -- classified
honestly as Scaffolded/stubbed, documented explicitly in `android/README.md`'s
own "What this is not, and why" section. One real bug was caught and fixed
by actually validating rather than assuming: `AndroidManifest.xml`'s and
(checked, clean) `strings.xml`'s XML was verified well-formed via
`python -c "xml.etree.ElementTree.parse(...)"`, which caught an invalid
`--` sequence inside an XML comment (XML forbids `--` inside comment
bodies) in the manifest's own header comment -- fixed immediately.

## Task 204 — Document Jetpack Ink mapping

`mobile-ffi/jetpack-ink-mapping.md`. Maps in-progress/final stroke
lifecycle, pressure, tilt/orientation, storage, and capability flags onto
this workspace's existing normalized types (`FfiPointerSample`,
`craftloop_ink::Stroke`, `InputCapabilities`). The one genuinely
non-trivial finding, named explicitly rather than glossed over: Jetpack
Ink models tilt in **polar** form (`tiltRadians` + `orientationRadians`),
while this workspace's `PointerSample` models it in **Cartesian** form
(`tilt_x_deg`/`tilt_y_deg`, Phase 03) -- a real adapter needs an explicit
trigonometric conversion, not a field rename; getting the axes backwards
would silently produce plausible-but-wrong numbers. Carries an explicit
confidence caveat: the `androidx.ink` API names reflect training
knowledge, not a live docs fetch (no Android docs access in this
sandbox), and must be re-verified against the actual API version at real
integration time.

## Task 205 — Plan Samsung real-device validation

`mobile-ffi/samsung-device-validation-plan.md`. Ten named scenarios a
Windows mouse simulator is structurally incapable of exercising (full
pressure-curve range, tilt/orientation at extreme angles, hover
detection, two-handed palm rejection, S Pen button/air-action gestures,
BLE disconnect/reconnect mid-stroke, real rendering-pipeline latency, DeX/
multi-window behavior, thermal throttling under sustained load, and
real-glass surface friction effects on sampling density) -- each tied to
a specific existing type/module this workspace already has
(`PointerButtons.barrel`, `InputCapabilities.hover`,
`StrokeLifecycleValidator`, Phase 27's benchmark baselines) rather than
written generically. Status: not yet executed, matching Task 205's own
objective ("list the scenarios," not "run them") -- no Samsung device
available in this sandbox.

## Task 206 — Keep Android UI production work deferred

Enforced by construction, not merely stated: `MainActivity.kt` is one
`Text` composable inside a bare `MaterialTheme`/`Surface` -- no
navigation graph, no custom design system, no drawing canvas, no
Jetpack Ink dependency declared in `app/build.gradle.kts` at all.
`android/README.md`'s own "What this is not, and why" section documents
the boundary explicitly, and `jetpack-ink-mapping.md` is a *design
document*, not code, for exactly this reason -- wiring up a real drawing
surface is future work, not this phase's budget.

## Commands and results

```
cargo build --workspace --all-targets
cargo test --workspace                                  # 729/729 passing (6 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings

# Task 202's real smoke test:
cargo build -p craftloop-mobile-ffi
cargo run -p craftloop-mobile-ffi --bin uniffi-bindgen -- generate \
  --library target/debug/craftloop_mobile_ffi.dll \
  --language kotlin --out-dir execution-evidence/mobile-ffi/generated-kotlin

# Task 203's honest limitation, confirmed directly:
which gradle    # not found
which adb       # not found
echo $ANDROID_HOME    # unset
```

Full output: `test-reports/phase-28-cargo-{test,clippy,fmt}.txt`,
`mobile-ffi/phase-28-kotlin-bindgen-smoke-test.txt`.

## Deferred, explicitly

- Gradle/Android SDK build validation of `android/` (Task 203) -- no
  Android toolchain in this sandbox; true blocker per the governing
  execution plan's own anticipated category.
- Actually compiling/running the generated Kotlin bindings (Task 202) --
  no JDK/Kotlin compiler available.
- Cross-compiling `craftloop-mobile-ffi` to an Android target
  (`aarch64-linux-android`, etc.) and embedding a real `.so` in
  `android/app` -- needs the Android NDK, also unavailable.
- Every scenario in the Task 205 Samsung validation plan -- no physical
  device/S Pen available; the plan itself, not its execution, is this
  phase's deliverable.
- Full Jetpack Ink integration (beyond the Task 204 mapping document) --
  explicitly out of this phase's budget per Task 206.

## Phase Gate

- All six tasks (201-206) have an evidence state: three implemented and
  tested at the Rust/codegen layer (201, 202's binding-generation
  pipeline, 206 enforced by construction), three genuinely blocked on an
  unavailable platform toolchain and documented as such (203's build
  validation, 202's Kotlin/JVM execution, all of 205's scenario
  execution) rather than silently skipped or falsely claimed complete.
- `cargo test --workspace`: 729/729 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary silently crossed: `craftloop-mobile-ffi` depends only
  on existing domain crates plus `uniffi` (pinned exact version); no
  domain crate gained a dependency on it or on anything mobile-specific.
- A real XML-validity bug was found and fixed (Task 203's manifest
  comment) by actually validating output rather than assuming
  correctness -- consistent with this execution's established pattern of
  trusting tests/verification over inspection.
- Implemented and tested: the FFI boundary crate and its unit tests
  (Rust-only, hardware-independent). Implemented but not
  hardware/toolchain-validated: the generated Kotlin bindings' actual
  compilation, and the entire `android/` shell. Scaffolded/stubbed: the
  Android shell's structure. Deferred: see above, each with a reason.
- Blocked with evidence: Android SDK/Gradle/NDK/`adb` unavailability,
  confirmed directly (`which`/`$ANDROID_HOME` checks captured above) --
  exactly the true-blocker category the governing execution plan
  anticipated for this phase.
- Proceeding to Phase 29 automatically.
