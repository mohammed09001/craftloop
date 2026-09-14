# Phase 30 — End-to-End Engine Integration — Evidence

Recorded: 2026-09-14

New crate: `crates/craftloop-scenarios`. Test-only (no production code of
its own, only `tests/*.rs` integration tests) -- each of the nine tasks
below gets its own file, wiring together the real domain crates the way
a real editing session would, using each crate's own real public API.
No scenario mocks a domain function or duplicates domain logic; where a
scenario needed a stand-in (Task 219's toy harness-mode struct, since
this crate cannot import `apps/windows-harness`'s private `state`
module), that stand-in is named and justified in the test file's own
doc comment.

## Task 213 — Blank-page-to-structured-line scenario

`scenario_213_blank_page_to_structured_line.rs`. A near-straight
synthetic pointer-sample sequence becomes a real `Stroke` (input/stroke
model), is recognized as a `Line` candidate and beautified (recognition/
beautification), both the raw stroke and the beautified primitive are
committed as one atomic `DocumentHistory` transaction, undone and redone
as one coherent action, then the document is saved and reopened with
both entities confirmed present by real ID lookup -- every named engine
boundary (input, stroke model, recognition, beautification, transaction,
undo, persistence) crossed for real, in one continuous pipeline.

## Task 214 — Handwritten-dimension stub scenario

`scenario_214_handwritten_dimension.rs`. `FixtureHandwritingRecognizer`
returns `"25mm"` for a deterministic stroke fixture; `route_as_length`
parses it to `25.0` canonical mm; `associate` binds it to the one nearby
line primitive with a commit-worthy confidence; the real `ezpz`-backed
`EzpzSolver` corrects that line's endpoints to exactly 25mm (a genuine
`GeometricConstraint::Distance` solve, not a hand-computed replacement);
the solved value becomes a `Driving` `SemanticDimension` with a visible
`DimensionAnnotation`; the dimension is committed/undone/redone through
`DocumentHistory`; and the document round-trips through a real save/
reopen on this Windows development machine. One real floating-point
precision fact surfaced by the test itself, not assumed: the solver
converges to `24.999999999`, not bit-for-bit `25.0` -- both the
solved-length and post-reopen assertions use a `1e-6` tolerance rather
than exact equality, honestly reflecting what a numeric solver actually
guarantees.

## Task 215 — Invalid-dimension scenario

`scenario_215_invalid_dimension.rs`. Reproduces MCP Article 27's own
worked example verbatim (`TriangleSideRange::for_two_fixed_sides(30.0,
50.0)` -> feasible range `(20.0, 80.0)`; `300.0` confirmed infeasible).
Wires that range directly into a real `Driving` `SemanticDimension` via
`with_feasible_range`, and shows `DimensionStore::edit_driving_value`
itself rejects the impossible `300.0` edit, leaving the dimension's
value at its last valid `40.0` -- then confirms a genuinely feasible
edit (`60.0`) still succeeds normally, proving the rejection is about
feasibility specifically, not a general inability to edit. A second test
confirms the type's own documented degenerate-boundary distinction
(`20.0`/`80.0` themselves are infeasible; the same values ±0.0001 are
feasible).

## Task 216 — Front-to-Orthographic scenario

`scenario_216_front_to_orthographic.rs`. A labeled Front view with one
real primitive and *zero* dimensions reaches `LinkReady` with no
blockers (Article 234's explicit "should not require full
dimensioning" rule, proven not asserted), then a real `Orthographic`
`Command` transitions it into a full set of derived-view layouts via
`transition_to_orthographic`, and a real `OrthographicSet` accepts both
the Front view and a newly-identified Top view derived from those
layouts without any identity collision.

## Task 217 — Resolve-depth-in-Top scenario

`scenario_217_resolve_depth_in_top.rs`. A Depth dimension bound first to
a Top view, then to a Right view (Article 35's exact Top/Right Depth
pairing), propagates via `propagate_confirmed_value` to both views at
once -- and never creates a 3D solid, true by construction: every
function this scenario calls returns only `ViewId`/`DimensionId`
collections, and `craftloop-geometry` defines no 3D geometry type
anywhere a 3D result could even be returned. A second test confirms the
negative case directly: binding Depth onto a *Front*-identity view is
refused (Article 35's Front/Top/Right pairing is asymmetric on purpose).

## Task 218 — Cross-view conflict scenario

`scenario_218_cross_view_conflict.rs`. Reproduces Article 37's exact
100mm-vs-130mm example: a contradictory shared-Width proposal from a Top
view against an established Front value produces a real
`CrossViewMismatch` `Conflict`. Both real resolution paths are exercised
and shown to actually change behavior, not just record a status:
`ReplaceAndPropagate` followed by the same real `propagate_confirmed_value`
Task 217 uses genuinely updates the shared value to 130mm everywhere;
`KeepExisting` genuinely leaves it at 100mm with no propagation call
made at all. A fourth test confirms `resolve` itself refuses a
double-resolution attempt.

## Task 219 — Ink-command scenario

`scenario_219_ink_command.rs`. A deterministic `"sketch"` handwriting
fixture resolves against the real Article 237 grammar to
`CommandAction::Sketch`, passes the real Article 238 confirmation policy
with every signal deliberately positive (`ConfirmationOutcome::Execute`),
routes through a real `CommandBus`, and a small local stand-in for
harness tool/mode state (this crate cannot import
`apps/windows-harness`'s own private `state` module) updates from
`CommandBus::history()` -- confirmed to reach `Sketch` mode. A second
test confirms the negative path: the same command with confirmation
evidence saying the ink remains ordinary geometry is rejected by the
bus outright, and the stand-in harness state never updates.

## Task 220 — Mixed-note scenario

`scenario_220_mixed_note.rs`. A note sentence containing both numbers
and a real command-vocabulary word as a *substring* of a longer word
(`"pens"` contains `"pen"`) resolves as `GrammarMatch::NoMatch` against
the real grammar, then is stored and read back as an ordinary
`SemanticEntity::Note` with its exact text intact. A second, systematic
test repeats the check for every Notebook vocabulary word embedded in a
longer real word (pencil/selection/sketchy/erasers), confirming
`resolve`'s whole-word/prefix matching is genuinely substring-safe, not
merely assumed to be from reading `grammar.rs`'s own implementation.

## Task 221 — Save/reopen all scenarios

`scenario_221_save_reopen_all_scenarios.rs`. One combined document
carrying one instance of every entity kind the other eight scenarios
exercise (stroke, beautified primitive, driving dimension, note, and a
*resolved* `CrossViewMismatch` conflict) is saved and reopened, and
checked with `Document`'s own derived `PartialEq` for full structural
equality -- not a hand-picked field subset that could hide a real
regression, the same equality check every other persistence test in this
workspace already trusts.

## Commands and results

```
cargo build --workspace --all-targets
cargo test --workspace                                  # 745/745 passing (16 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-30-cargo-{test,clippy,fmt}.txt`.

## Errors and fixes

- One real floating-point precision fact, not a bug: Task 214's `EzpzSolver`
  solve converges to `24.999999999`, not exactly `25.0`. The test's final
  post-reopen assertion originally used `assert_eq!(..., 25.0)` and
  failed on first run; fixed to the same `1e-6` tolerance already used
  for the solved-length check earlier in the same test, honestly
  reflecting what a numeric solver guarantees rather than asserting
  false bit-for-bit precision.
- One environment incident, unrelated to any code defect: mid-phase, a
  `cargo test` build failed with `os error 112` ("not enough space on
  the disk") -- `df -h` confirmed the whole `C:` drive was at 20MB free
  out of 222GB (not specific to this project; the disk is otherwise
  nearly full from unrelated system/user data). `cargo clean` reclaimed
  15.2GiB of fully-regenerable build cache, after which every build in
  this phase succeeded normally. The disk remains tight (11GB free as of
  this evidence capture) and worth the user's attention independent of
  this execution.

## Deferred, explicitly

Nothing named in Tasks 213-221 was deferred -- every task's own scope
(a specific end-to-end scenario) was fully realized against real domain
code. The one honest scope note: Task 219's "harness state" is a small
local stand-in, not `apps/windows-harness`'s actual private `state`
module, since a test-only crate cannot import a different binary
crate's private types -- the wiring pattern (react to
`CommandBus::history()`) is proven identically either way.

## Phase Gate

- All nine tasks (213-221) represented in repository code, each with a
  focused, passing integration test (16 total, several tasks covering
  both a positive and a negative/edge case).
- `cargo test --workspace`: 745/745 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary silently crossed: one new test-only crate, zero new
  external dependencies, every scenario built entirely from existing
  domain crates' real public APIs.
- Implemented and tested: every one of the nine named scenarios, each
  exercising the specific engine boundaries its own task names, verified
  by real assertions against real computed/persisted state (not
  compilation alone).
- Deferred: none for this phase's own scope; see the one honest stand-in
  note above.
- Blocked with evidence: none from the domain work itself; one transient
  environment (disk space) issue hit and resolved mid-phase, documented
  above.
- Proceeding to Phase 31 automatically.
