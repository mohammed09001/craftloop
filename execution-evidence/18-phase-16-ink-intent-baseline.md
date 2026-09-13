# Phase 16 — Ink Intent Classification Baseline — Evidence

Recorded: 2026-09-13

## New crate: `crates/craftloop-ink-intent`

Deliberately precedes any real handwriting-content reading: Phase 17,
the very next phase, is "Engineering Handwriting Adapter Boundary." This
crate defines the category vocabulary and a real, conservative baseline
classifier built only from signals already available in this workspace
(tool context, Phase 06 geometry confidence, timing, position) -- it
never claims to read actual word/number content, because nothing in this
workspace can yet.

## Task 113 — Intent categories

`category.rs`: `IntentCategory`, exactly the nine variants named
(RawInk, GeometryCandidate, Text, NumericDimensionCandidate, ViewLabel,
Command, SelectionGesture, EraseGesture, Annotation) -- no speculative
extras.

## Task 114 — Deterministic context features

`context.rs`: `ContextFeatures::compute` -- tool context (caller-supplied,
never inferred/upgraded from `Unknown`), bounds, nearby-entity count,
duration (from the stroke's own timestamps), selection-active flag,
near-labeling-guide flag, and a computed `path_length_to_diagonal_ratio`
(a real geometric signal distinguishing a deliberate closed shape from a
scribble, used by the erase-gesture heuristic). All real computation, no
placeholders.

## Task 115 — Confidence/result contract

`classifier.rs`: `IntentCandidate { category, confidence }`, reusing
`craftloop_recognition::Confidence` (Phase 06) rather than a duplicate
scale (Article 96: "confidence" means the same thing everywhere).
`RawInk` is always present in the returned list, mirroring
`RecognitionCandidate::KeepAsInk`'s own "always offered" precedent.

## Task 116 — Conservative thresholds

`COMMIT_THRESHOLD = 0.6` (matching `craftloop-recognition`'s own
established `false_positive_benchmark.rs` constant -- same value, same
crate-external convention, not reinvented) and `CONFIRM_THRESHOLD = 0.3`
(Article 22's three-tier apply/confirm/stay-as-ink framing).
`RawInk`'s own confidence (0.5) sits deliberately below commit so real
evidence for another category can outrank it, but nothing defaults to an
unprompted commit. Every content-dependent category
(ViewLabel/NumericDimensionCandidate/Command/Text/Annotation) that this
baseline cannot actually verify is either never offered at all, or offered
only below `CONFIRM_THRESHOLD`.

## Task 117 — Mixed-content tests

`tests/mixed_content.rs`, the four named cases:
- **Numbers in sentences / command words in notes**: proven as restraint
  -- a generic handwriting-shaped stroke never yields a
  `NumericDimensionCandidate` or `Command` entry at all (this baseline
  has no way to read the content), and `RawInk` stays top-ranked.
- **Letter O versus circle**: the identical clean closed-loop shape,
  fed through real `craftloop_recognition::recognize`, is offered as
  `GeometryCandidate` with `ToolContext::Pen` and never as
  `SelectionGesture`.
- **Circles used for selection**: the same shape with
  `ToolContext::Selector` is a committable `SelectionGesture` and never
  offered as `GeometryCandidate` -- resolved entirely by tool context
  (Task 114's first-listed feature), not by re-analyzing the shape.

## Task 118 — Future ML adapter interface

`IntentClassifier` trait (mirrors `craftloop_constraint::ConstraintSolver`'s
own "define the interface, ship a real reference implementation" pattern
from Phase 11); `DeterministicBaselineClassifier` is the real, tested,
non-learned implementation. `IntentCategory`/`ContextFeatures`/
`IntentCandidate` do not depend on which implementation is behind the
trait -- a future learned classifier plugs in without touching domain
semantics.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 525/525 passing (11 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-16-cargo-test.txt`,
`test-reports/phase-16-cargo-clippy.txt`,
`test-reports/phase-16-cargo-fmt.txt`.

## Deferred, explicitly

- Actual text/word/number/command content reading -- Phase 17's job by
  this execution document's own phase title and ordering; implementing it
  here would be exactly the fabricated capability the No-Hallucination
  Contract forbids.
- A learned/ML `IntentClassifier` implementation -- Task 118 asks only
  for the seam, not a model; no task in this phase names training data,
  a model architecture, or an inference runtime.
- Wiring `IntentClassifier` into `craftloop-document`/a live editing
  session -- no task this phase asks for that integration; this crate is
  intentionally usable standalone, the same layering choice
  `craftloop-sketch` (Phase 12) and `craftloop-consistency` (Phase 14)
  both made.

## Phase Gate

- All six tasks (113-118) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 525/525 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: new crate depends only on already-existing
  crates (`craftloop-ink`, `craftloop-recognition`, `craftloop-geometry`,
  `craftloop-errors`); no platform/UI dependency.
- Implemented and tested: category vocabulary, deterministic feature
  extraction, ranked confidence contract, conservative thresholds
  matching the established workspace convention, all four Task 117
  scenarios, and the ML adapter seam.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 17 automatically.
