# Handwriting Adapter Platform Implementation Plan

Execution 01, Phase 17, Task 124. Recorded 2026-09-13.

## What is being planned

`craftloop-handwriting::HandwritingRecognizer` (Task 119) is the entire
contract a platform adapter must satisfy:

```rust
pub trait HandwritingRecognizer {
    fn recognize(&mut self, strokes: &[Stroke]) -> Vec<TextCandidate>;
}
```

`Stroke` (from `craftloop-ink`, Phase 05) and `TextCandidate`
(`{ text: String, confidence: Confidence }`, this phase) are both
already platform-independent types with no OS dependency. Any real
recognizer -- on-device or a network service -- implements this trait and
nothing upstream of it (routing, provenance, the future dimension
association engine, Phase 18) needs to change.

## Windows (this execution's only currently available environment)

No production on-device handwriting recognizer is reachable from this
Windows development workstation, and building one from scratch is
explicitly out of this execution's scope. `FixtureHandwritingRecognizer`
(Task 120) is the only Windows-available implementation: a deterministic,
pre-registered-results stub. It exists to test everything *downstream* of
recognition (routing, ambiguity handling, provenance), not to claim any
recognition capability. This is recorded plainly, not glossed over: the
No-Hallucination Contract's "Implemented but not hardware-validated"
category applies to real handwriting recognition on every platform this
execution has touched so far, Windows included -- Windows in fact has
*no* real implementation at all, stub only.

## Android (planned, not built this execution)

The best available on-device recognizer is Android's ML Kit **Digital
Ink Recognition** API, which supports both general handwriting and a
math/formula-oriented model closer to engineering notation. A
`MlKitHandwritingRecognizer` (Kotlin, called from the shared Rust core
via the UniFFI binding layer the Architecture Decision already commits
to for all Android integration) would:
1. Convert `craftloop-ink::Stroke` sample points into ML Kit's `Ink`/
   `Stroke`/`StrokePoint` builder types (a straightforward coordinate
   remap, no semantic translation).
2. Call ML Kit's recognizer, which itself already returns ranked
   candidate strings with scores.
3. Map ML Kit's score scale into `craftloop_recognition::Confidence`'s
   `[0, 1]` range (ML Kit's own scores are not documented as already
   `[0, 1]`-normalized across all models, so this mapping needs its own
   real calibration pass once real device testing is possible -- not
   assumed here).
4. Return `Vec<TextCandidate>`, nothing else -- the trait boundary means
   no Android-specific type ever needs to cross into the shared core.

**Not yet verified**: no Android toolchain is available in this Windows
development session (a legitimate True Blocker Policy exception for
*that specific verification only*, per the execution document's own
policy). This plan is therefore itself "implemented but not
hardware-validated" in the sense that it names a real, existing API and a
real integration point, but the integration has not been built or run.

## iPad (planned, not built this execution)

The best available on-device recognizer is PencilKit's built-in
handwriting-to-text support (`PKRecognizer`/the system's Scribble
infrastructure on modern iPadOS) or, if finer control over candidate
ranking is needed, direct use of the Vision framework's
`VNRecognizeTextRequest` against a rendered image of the stroke group. A
`PencilKitHandwritingRecognizer` (Swift, called from the shared Rust core
the same UniFFI way) would follow the identical three-step shape as the
Android plan: convert `Stroke` samples into the platform's own ink
representation, call the platform recognizer, map its result into
`Vec<TextCandidate>`. Also not yet built or verified this execution, for
the same toolchain-availability reason.

## What stays true regardless of which adapter is behind the trait

- `routing.rs`'s functions (Task 121) never see a platform-specific type
  at all -- they operate purely on `TextCandidate` and
  `craftloop-units`' locale-explicit parsers. A platform adapter cannot
  accidentally couple geometry mutation to recognition, because the
  types available to it do not include geometry.
- `HandwritingRecognitionResult` (Task 122) ties every result back to its
  source strokes regardless of which recognizer produced it.
- Ambiguity handling (Task 123) is entirely a property of `routing.rs`
  and `craftloop-units`, not of any specific recognizer -- a real ML Kit
  or Vision-backed recognizer returning multiple ranked candidates for an
  ambiguous "1" vs "7" is handled by the exact same code path already
  tested against the fixture stub.
