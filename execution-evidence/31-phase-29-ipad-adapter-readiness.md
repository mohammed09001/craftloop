# Phase 29 — iPad Adapter Readiness — Evidence

Recorded: 2026-09-13

No new crate this phase: `crates/craftloop-mobile-ffi` (Phase 28) needed
zero Rust source changes to serve Swift as well as Kotlin, confirming
Task 207's own premise directly rather than requiring new code to prove
it. This phase's real work is a second real binding-generation smoke
test, three design/decision documents, one CI workflow extension, and
one validation plan -- matching Phase 28's shape (its own expected true
blocker, no macOS/Xcode toolchain in this sandbox, confirmed directly).

## Task 207 — Define Swift binding boundary

Satisfied by Phase 28's `crates/craftloop-mobile-ffi` as-is: every
`Ffi*` type/function (`FfiPointerSample`, `FfiCommandNamespace`,
`FfiGrammarMatch`, `validate_stroke`, `resolve_command`) is UniFFI
proc-macro-derived, which is language-agnostic by construction -- no
SwiftUI/PencilKit concept appears anywhere in the crate, matching the
objective exactly. Proven, not merely asserted: ran
`uniffi-bindgen generate --language swift` against the same compiled
cdylib Phase 28 built and inspected the real output (see Task 208 below).

## Task 208 — Evaluate UniFFI Swift bindings

`mobile-ffi/uniffi-swift-decision-record.md`. Reuses the exact
`uniffi = "=0.32.1"` pin from Phase 28 (one binary, both languages --
no separate version decision needed). Real smoke test:
`mobile-ffi/phase-29-swift-bindgen-smoke-test.txt` documents generating
`craftloop_mobile_ffi.swift` (1328 lines) plus the `.h`/`.modulemap`
pair from the compiled `.dll`, with every exported symbol confirmed
present and correctly Swift-cased by direct grep. Task 208's explicit
"account for the project's pre-1.0 status and current integration
issues" is addressed head-on: UniFFI is genuinely 0.x with known
breaking changes across minor versions (the record names the
proc-macro-only scaffolding mode itself as one such past break), which
is exactly why the pin stays exact rather than a caret range, mirroring
Phase 11's `ezpz` decision record's own reasoning for the same kind of
risk on a different dependency.

## Task 209 — Create PencilKit mapping design

`mobile-ffi/pencilkit-mapping.md`, mirroring Phase 28's Jetpack Ink
document's structure and rigor. Two real, non-obvious findings named
explicitly rather than glossed over: (1) PencilKit's live `PKCanvasView`
does not expose an in-progress stroke handle the way Jetpack Ink's
`InProgressStrokeId` does -- an iPad adapter must choose between
`PKCanvasView`'s built-in capture and a raw `UITouch` path, a decision
explicitly deferred to Task 210's boundary, not resolved here; (2) Apple's
tilt convention (`altitude` measured from the *screen plane*) is the
mirror image of Jetpack Ink's convention (`tiltRadians` from vertical),
so the correct conversion is `90° - altitude`, not a direct reuse of the
Android formula -- getting this backwards would be the iPad-specific
version of the same silent-bug class the Android document names.

## Task 210 — Define PaperKit decision boundary

`mobile-ffi/paperkit-decision-boundary.md`. A scope decision, not code:
PaperKit (if ever adopted -- no phase through 29 asks for it) is
confined to generic markup/annotation presentation on top of already-
exported content, never as a stand-in for `SemanticEntity::Primitive`/
`SemanticDimension`/constraint state, and never as an alternate document
persistence format to `craftloop-document`'s own (Phase 07/08). Directly
enforces Engine Contract 15 ("confirmed semantics survive
recognition-model changes") and Article 128 against the specific,
concrete risk PaperKit's convenience creates, rather than restating the
general document-model rule abstractly.

## Task 211 — Prepare macOS CI build path

`.github/workflows/macos-compile-gate.yml` extended (not replaced) from
Phase 27's compile-only gate: now also builds the
`craftloop-mobile-ffi` cdylib, generates Swift bindings via
`uniffi-bindgen`, and **type-checks them with the real Swift compiler**
(`swiftc -typecheck`) -- the one verification step this Windows sandbox
structurally cannot perform, but a real `macos-latest` GitHub Actions
runner can (Xcode/Swift ship on that image by default). A final step
implements Task 211's own phrase "when source exists": it builds
`ios/` with `xcodebuild` if that directory exists, and prints a clear
skip message (not a failure) if it does not -- accurate today, since no
task through Phase 29 asks for an iOS shell scaffold the way Android's
Task 203 did. The updated YAML was validated for syntax with
`python -c "yaml.safe_load(...)"`, which caught one real bug: an
unquoted step `name:` containing a literal `:` (`"...(Task 211:
\"when source exists\")"`) was parsed as a second YAML mapping key,
breaking the file -- fixed by quoting the whole step name.

## Task 212 — Plan real Apple Pencil validation

`mobile-ffi/apple-pencil-validation-plan.md`, mirroring Phase 28's
Samsung plan's structure. Ten scenarios a Windows-mouse-or-iOS-Simulator
combination cannot validate (per-generation pressure curve differences
across Apple Pencil 1/2/Pro, altitude/azimuth at extreme tilt, hover on
hover-capable models only, palm rejection cost if `PKCanvasView` is
bypassed, Pencil Pro's squeeze/barrel-roll gestures, double-tap tool
switching, magnetic pairing/charging interruption mid-stroke, real
ProMotion-pipeline latency, sustained-session ergonomics, and
accessory-glass sampling-density effects) -- each tied to a concrete
existing type/module or to this phase's own PencilKit mapping document,
not written generically. Status: not yet executed, matching the task's
own "list," not "run," framing -- no iPad, Apple Pencil, or macOS/Xcode
access exists in this sandbox (confirmed directly:
`which swift`/`swiftc`/`xcodebuild` all fail).

## Commands and results

```
cargo build --workspace --all-targets
cargo test --workspace                                  # 729/729 passing (0 new -- documentation/CI phase)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings

# Task 207/208's real smoke test:
cargo build -p craftloop-mobile-ffi
cargo run -p craftloop-mobile-ffi --bin uniffi-bindgen -- generate \
  --library target/debug/craftloop_mobile_ffi.dll \
  --language swift --out-dir execution-evidence/mobile-ffi/generated-swift

# Task 212's honest limitation, confirmed directly:
which swift        # not found
which swiftc        # not found
which xcodebuild    # not found

# Task 211's YAML validated:
python -c "import yaml; yaml.safe_load(open('.github/workflows/macos-compile-gate.yml'))"
```

Full output: `test-reports/phase-29-cargo-{test,clippy,fmt}.txt`,
`mobile-ffi/phase-29-swift-bindgen-smoke-test.txt`.

## Deferred, explicitly

- Actually compiling the generated Swift with a real Swift compiler, and
  running it on a real macOS runner -- no macOS/Xcode toolchain in this
  Windows sandbox; `.github/workflows/macos-compile-gate.yml`'s new
  `swiftc -typecheck` step exists precisely to perform this check on a
  real GitHub Actions macOS runner, but has not itself been executed
  there in this session (no `git remote` configured -- same limitation
  Phase 27 already recorded for `ci.yml`).
- Creating an actual `ios/` Xcode project/Swift package -- no task
  through Phase 29 asks for one (unlike Android's Task 203); the CI step
  that would build it is written and will pick it up automatically once
  one exists.
- Every scenario in the Task 212 Apple Pencil validation plan -- no
  physical device available; the plan itself is this phase's deliverable.
- Real PencilKit/PaperKit integration -- both Task 209 and Task 210 are
  design/decision documents, deliberately, not code.

## Phase Gate

- All six tasks (207-212) have an evidence state: two proven with a real
  cross-compiled-artifact smoke test (207/208), two delivered as
  reasoned design/decision documents (209/210), one delivered as an
  extended, syntax-validated CI workflow that will genuinely execute on
  a real macOS runner (211), and one delivered as a scenario list per
  its own stated objective (212).
- `cargo test --workspace`: 729/729 passing (unchanged from Phase 28 --
  expected, no new Rust code this phase). `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary silently crossed: no new crate, no new Rust
  dependency; the one existing FFI crate was reused exactly as designed,
  and no iOS shell was scaffolded speculatively where no task authorized
  one.
- A real YAML syntax bug was found and fixed (Task 211's workflow file)
  by actually validating rather than assuming correctness -- the same
  pattern Phase 28 already established for its Android manifest.
- Implemented and tested: the Swift binding-generation pipeline (proven
  via real generated output, Rust/codegen layer only). Implemented but
  not hardware/toolchain-validated: actual Swift compilation, the
  `swiftc -typecheck` CI step's real execution, everything iPad/Apple
  Pencil-specific. Deferred: see above, each with a reason.
- Blocked with evidence: macOS/Xcode/Swift toolchain unavailability,
  confirmed directly -- the second and final expected true-blocker phase
  this execution plan anticipated (after Phase 28's Android toolchain
  blocker).
- Proceeding to Phase 30 automatically.
