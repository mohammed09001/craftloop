# Phase 17 — Engineering Handwriting Adapter Boundary — Evidence

Recorded: 2026-09-13

## New crate: `crates/craftloop-handwriting`

## Task 119 — Handwriting recognizer interface

`recognizer.rs`: `HandwritingRecognizer` trait (`recognize(&mut self,
strokes: &[Stroke]) -> Vec<TextCandidate>`) and `TextCandidate { text,
confidence }`, reusing `craftloop_recognition::Confidence` (same
convention Phase 16 already followed). No Apple/Android type anywhere in
the signature.

## Task 120 — Windows test stub

`stub.rs`: `FixtureHandwritingRecognizer`, keyed by the exact ordered
`StrokeId` sequence a fixture was registered for -- exact match only,
never fuzzy, so it can never "helpfully" guess an unregistered input. Its
own doc comment states plainly that no real recognizer is reachable from
this Windows workstation and that this stub proves nothing about
recognition accuracy, only about the correctness of everything
downstream of it.

## Task 121 — Route recognized text to the semantic parser

`routing.rs`: `route_as_length`/`route_as_angle`/`route_as_radial`, all
built directly on `craftloop-units`' already-existing, already-tested
parsers (Phase 09) -- `parse_length`, `parse_angle_degrees`,
`parse_radial`. The enforcement mechanism behind "do not modify geometry
directly from recognizer output" is structural: no function or type in
this crate accepts or returns a `PrimitiveId`, `BeautifiedPrimitive`, or
any other geometry type, so there is no path by which it could mutate
geometry even by accident.

## Task 122 — Preserve raw handwriting provenance

`provenance.rs`: `HandwritingRecognitionResult { source_strokes,
candidates }`, plus `is_traceable_to` making the invariant directly
checkable. Every function in this crate takes strokes by shared
reference; nothing here ever deletes or consumes a `Stroke`.

## Task 123 — Ambiguity tests

`tests/ambiguity.rs`, all six named cases, each against the real parsers:

| Case | Proof |
|---|---|
| 1 vs 7 | both candidates route to distinct, correct lengths (1.0mm, 7.0mm), neither dropped |
| 3 vs 8 | same, for 3.0mm/8.0mm |
| Decimal separator | `parse_decimal_auto("1,234")` (Phase 09's own genuinely-ambiguous case) surfaces `AmbiguousDecimalSeparator`, not a guess; once a locale is explicit, `route_as_length` resolves the same text two different, individually-unambiguous ways depending on locale, never silently picking one |
| Degree symbol | `"45°"` and `"45"` both route to the identical angle |
| R | `"R5"` routes to `Radius, 5mm`; a bare `"R"` with no number is a real parser error, never a fabricated value |
| Diameter notation | `"D10"`, `"⌀10"`, `"Ø10"` all route to the identical `Diameter, 10mm` reading; radius and diameter prefixes are never confused with each other |

Plus a provenance-under-ambiguity test proving a multi-candidate result
still names its exact source stroke.

## Task 124 — Platform implementation plan

`execution-evidence/handwriting-adapter-platform-plan.md`: names the real
candidate APIs (Android ML Kit Digital Ink Recognition, iPadOS
PencilKit/Vision `VNRecognizeTextRequest`), the concrete integration
shape (convert `Stroke` samples to the platform's ink type, call the
recognizer, map its scores into `Confidence`, return `Vec<TextCandidate>`
-- nothing platform-specific crosses the trait boundary), and states
plainly that neither adapter is built or verified this execution (no
Android/iPad toolchain available, a legitimate True Blocker Policy
exception for *that specific verification*).

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 544/544 passing (19 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean (2 findings fixed: cloned_ref_to_slice_refs)
```

Full output: `test-reports/phase-17-cargo-test.txt`,
`test-reports/phase-17-cargo-clippy.txt`,
`test-reports/phase-17-cargo-fmt.txt`.

## No-Hallucination Contract summary

**Implemented and tested**: the recognizer interface, the fixture stub,
routing into real numeric/angle/radial parsers, provenance tracking, all
six ambiguity classes.

**Implemented but not hardware-validated**: none -- there is no partial
Android/iPad implementation this phase to even partially validate (see
below).

**Deferred**: real Android (ML Kit) and iPad (PencilKit/Vision) adapter
implementations -- planned in detail (Task 124) but not built, since no
Android/iPad toolchain is available in this Windows development session.
Windows itself has no real handwriting recognizer at all, stub only --
recorded plainly rather than left ambiguous.

## Phase Gate

- All six tasks (119-124) represented in repository code or (Task 124) a
  documented plan, each with a focused, passing test where code applies.
- `cargo test --workspace`: 544/544 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: `routing.rs` structurally cannot touch
  geometry (no geometry types in its signatures); no platform dependency
  anywhere in the new crate.
- Deferred: real platform adapters, named above with reasons.
- Blocked with evidence: none (the missing Android/iPad toolchain is a
  documented, legitimate True Blocker Policy exception for platform-only
  verification, not a blocker on this phase's own deliverables).
- Proceeding to Phase 18 automatically.
