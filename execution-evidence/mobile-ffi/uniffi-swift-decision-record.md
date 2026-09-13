# UniFFI Swift Bindings Decision Record

Execution 01, Phase 29, Task 208. Recorded 2026-09-13.

## Decision

**Reuse the exact same `uniffi = "=0.32.1"` pin Phase 28 (Task 202)
already made for Kotlin -- no separate version choice for Swift.** UniFFI
generates every target language from one compiled cdylib's embedded
metadata; there is no per-language version to evaluate independently,
and pinning two different UniFFI versions for the same crate's two
binding targets would be exactly the kind of unreviewed inconsistency
Phase 11's `ezpz` decision record warned against for a different
dependency.

## Evidence

Real Swift bindings were generated from the actually-compiled
`craftloop_mobile_ffi` cdylib and inspected -- see
`mobile-ffi/phase-29-swift-bindgen-smoke-test.txt` for the full
transcript. Every exported function/type from Phase 28's Task 201 is
present in correct Swift form (`public struct`/`enum`/`func`, real
`Equatable`/`Hashable` conformances). This confirms Task 207's premise
directly rather than asserting it: the Android FFI boundary crate needed
**zero** source changes to also serve Swift.

## Accounting for UniFFI's pre-1.0 status and known integration issues (Task 208's explicit ask)

UniFFI is versioned 0.x (`0.32.1` at the time of this record) --
genuinely pre-1.0, and the project's own release history includes
breaking changes between minor versions (e.g., the proc-macro-only
scaffolding mode this crate uses, `uniffi::setup_scaffolding!()`, was
itself a breaking addition around the 0.25 line; async/callback-interface
support and Swift Package Manager packaging conventions have also
changed across minor versions in ways this record cannot exhaustively
verify without live access to UniFFI's current changelog). Concretely,
for this project:

- **Pin stays exact (`=0.32.1`), never a caret range**, for the same
  reason Phase 11 pinned `ezpz` exactly: an unreviewed minor-version bump
  on a pre-1.0 dependency is a real risk, not a formality, and this
  workspace's `[workspace.lints]` policy (Phase 27) already treats any
  resulting warning as a hard build failure rather than something that
  could slip through unnoticed.
- **Xcode/Swift toolchain version compatibility cannot be verified in
  this sandbox** (no Swift/Xcode toolchain exists here at all -- see the
  smoke test transcript). The generated `.modulemap`/`.h`/`.swift` triple
  is standard UniFFI output shape and should work with the Swift Package
  Manager "binary XCFramework + Swift wrapper" pattern UniFFI's own docs
  recommend, but the *exact* Xcode version this will be validated against
  is a decision for whoever first runs Task 211's macOS CI job for real
  (this record does not invent an Xcode version pin no evidence backs).
- **Known community-reported friction points** (accurate to this
  assistant's training knowledge, not independently re-verified against
  live issue trackers in this session — flagged per the same confidence
  caveat `jetpack-ink-mapping.md` uses): async Rust function export to
  Swift has historically lagged sync export in maturity; XCFramework
  packaging for multi-architecture (device + simulator, arm64 + x86_64)
  builds requires an explicit `lipo`/`xcodebuild -create-xcframework`
  step UniFFI itself does not automate; and Swift's strict concurrency
  checking (Swift 6 mode) has required adjustments in some UniFFI
  consumer projects. None of this project's current exported functions
  (`resolveCommand`, `validateStroke`) are `async`, so the first friction
  point does not apply yet; the XCFramework packaging step is real future
  work for whoever builds Task 211's CI job against a real macOS runner.

## What is NOT decided here

Which exact Xcode version, iOS deployment target, or XCFramework
packaging approach the real iPad build will use -- none of that has
evidence behind it yet in this sandbox and is left for the phase/session
that has real macOS CI access to decide and record.
