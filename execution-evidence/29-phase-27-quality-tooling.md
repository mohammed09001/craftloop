# Phase 27 — Quality Tooling and Continuous Integration — Evidence

Recorded: 2026-09-13

No new crate this phase: every task adds tooling/configuration around the
existing workspace, not new domain code, except where a real coverage gap
was found and closed (Task 194, below).

## Task 192 — Configure formatting and static analysis

Root `Cargo.toml` gained `[workspace.lints.rust] warnings = "deny"` and
`[workspace.lints.clippy] all = "deny"`; every one of the 21 member
`Cargo.toml` files (20 crates + `windows-harness`) gained
`[lints]\nworkspace = true`. This is a behavior change, not
documentation: before this phase, `cargo clippy --workspace --all-targets`
alone passed even with warnings present -- only the extra `-D warnings`
flag every phase's evidence commands added by hand turned warnings into
failures. Verified by running plain `cargo clippy --workspace
--all-targets` (no `-D warnings` flag) and confirming it still exits
clean with zero findings against the current tree, proving the policy is
now self-enforcing rather than a convention that has to be remembered.

## Task 193 — Configure fast test runner

Installed `cargo-nextest` (0.9.144) and added `.config/nextest.toml`
(`fail-fast = false`, matching `cargo test`'s own default and every
phase's evidence-capture convention of always seeing the full pass/fail
picture). `cargo nextest run --workspace` was run and produced 717/717
passing at the time it was first run this phase (717 = the 710 tests
Phase 26 closed with, plus the 7 new property tests from Task 195, before
Task 194's 6 coverage-closing tests were added). `cargo test --workspace`
remains the one authoritative command in CI and in every phase's evidence
capture; nextest is documented in its own config file as a local
accelerant only, never a hard requirement, per the task's own "do not
make optional tooling a blocker."

## Task 194 — Configure coverage reporting

Installed `cargo-llvm-cov` (0.9.1) plus the `llvm-tools-preview` rustup
component, and ran `cargo llvm-cov --workspace --exclude windows-harness
--summary-only` (full output: `quality-tooling/phase-27-coverage-summary.txt`).
**This surfaced a real gap, not a vanity number**: `craftloop-export`'s
`svg.rs` and `pdf.rs` (Phase 26) had only ever been tested against the
`Line` primitive variant -- `Circle`/`Arc`/`Rectangle` rendering code
existed and shipped in Phase 26 but had zero direct test coverage
(76.02%/82.79% region coverage respectively). Per the task's own
philosophy ("emphasis on critical invariants rather than treating a
percentage as proof of quality"), this specific, real, named gap was
closed with six new tests (three each in `svg.rs`/`pdf.rs`, one per
missing primitive variant) rather than chased as an abstract percentage
target. Re-running coverage afterward confirms the fix: `svg.rs`
76.02% → 95.84%, `pdf.rs` 82.79% → 98.45% region coverage; workspace-wide
TOTAL region coverage 95.86% → 96.66% (97.60% function, 96.73% line).
No CI coverage gate (e.g. "fail under X%") was added -- the task
explicitly warns against treating a percentage as proof of quality, and a
rigid threshold would contradict that on its own terms.

## Task 195 — Configure property tests

`craftloop-geometry` already had real `proptest`-based property tests
from Phase 02 (Task 020, `tests/property_tests.rs`, predating this
phase) -- inspected first, per this execution's standing rule to check
for an existing implementation before adding one, and *not* duplicated.
This phase's real, new contribution is the two other named domains plus
the solver: `craftloop-serialization/tests/canonical_json_properties.rs`
(determinism and round-trip, Article 386), `craftloop-units/tests/length_unit_properties.rs`
(to/from-millimeter round-trip for every `LengthUnit`, with the correct
relative-tolerance check for the inexact inches conversion), and
`craftloop-constraint/tests/residual_properties.rs` (non-negativity and
exact-zero-when-satisfied for `FixedValue`/`EqualValues`/`Coincident`
residuals, generalizing `residual.rs`'s own fixed-example unit tests).

## Task 196 — Configure benchmark suite

Added `criterion` (0.5, `harness = false`) benchmarks covering all five
named categories: recognition (`craftloop-recognition/benches/recognition_bench.rs`),
solving (`craftloop-sketch/benches/solve_bench.rs`, against the real
`ezpz`-backed `EzpzSolver`, not a mock), propagation
(`craftloop-document/benches/propagation_bench.rs`), and
serialization/large-document combined (`craftloop-document/benches/large_document_bench.rs`,
since serializing a large document *is* the large-document operation of
primary concern for this product). All four were run once with real
measurements captured verbatim to `quality-tooling/phase-27-benchmark-results.txt`
(e.g. `recognize/256` ≈ 18.7 µs, `ezpz_solve/chain_20` ≈ 284.9 µs,
`to_canonical_json_large_document/notes_1000` ≈ 3.31 ms) -- real numbers
from a real run on this machine, not estimates. No performance budget/gate
is asserted; these are a baseline for Phase 31 to compare against.

## Task 197 — Configure fuzz targets where valuable

Deferred, with a full written decision record:
`execution-evidence/fuzzing-decision-record.md`. This sandbox has only
the stable MSVC toolchain installed (`rustup toolchain list`); `cargo-fuzz`
needs a nightly toolchain plus libFuzzer, which is not practically
available for an `x86_64-pc-windows-msvc` target regardless. Installing a
full nightly+libFuzzer environment to write harness source that could
never actually be run in this session would produce exactly the
unverified "looks like it works" code the No-Hallucination Contract
forbids claiming. The record names six concrete fuzz-worthy targets
(the four `craftloop-units` parsers, `craftloop_document::load_document`,
and canonical-JSON round-tripping) for whenever that infrastructure is
available, and explicitly distinguishes fuzzing's unstructured
byte/string-level adversarial input from Task 195's proptest coverage of
*valid*-shaped structured input -- a real, distinct gap, not a duplicate
of work already done this phase.

## Task 198 — Configure Windows CI

`.github/workflows/ci.yml`, `windows` job: `actions/checkout`,
`dtolnay/rust-toolchain@stable` with `rustfmt`/`clippy` components, a
cargo registry/target cache, then `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo build --workspace`, `cargo test --workspace` -- covering the
shared core *and* `windows-harness`, exactly Task 198's wording, and
using the identical commands every phase's local evidence has already
been captured with.

## Task 199 — Configure Linux CI

Same file, `linux` job, `ubuntu-latest`: identical steps but scoped to
`--workspace --exclude windows-harness` (a Windows-only native egui/eframe
test harness, not a cross-platform portability target -- excluding it
keeps the Linux job fast and avoids requiring GTK/X11 dev packages for a
component this execution never intends to run on Linux).

## Task 200 — Configure macOS compile gate

`.github/workflows/macos-compile-gate.yml`, separate from `ci.yml`
deliberately (a narrower, forward-looking gate, not full test/lint
parity): `macos-latest`, `cargo build --workspace --exclude windows-harness`
only -- compile-only, matching the task's own framing ("prepare future
iOS-binding build checks... once the binding layer exists"; no iOS
binding layer exists yet, so there is nothing iOS-specific to test today,
only a portability compile check for the shared core).

Both workflow files were validated as syntactically well-formed YAML via
`python -c "import yaml; yaml.safe_load(...)"` against each file (both
`OK`); neither has been executed on an actual GitHub Actions runner in
this session (no `git remote` is configured for this repository -- see
"Deferred, explicitly").

## Commands and results

```
cargo build --workspace --all-targets
cargo test --workspace                                  # 723/723 passing (13 new: 7 property tests + 6 coverage-closing tests)
cargo nextest run --workspace                             # 723/723 passing, matches cargo test exactly
cargo fmt --all -- --check                                 # clean
cargo clippy --workspace --all-targets -- -D warnings       # clean, no findings
cargo clippy --workspace --all-targets                      # clean even WITHOUT -D warnings (Task 192's policy now self-enforcing)
cargo llvm-cov --workspace --exclude windows-harness --summary-only
  # TOTAL: 96.66% region / 97.60% function / 96.73% line
```

Full output: `test-reports/phase-27-cargo-{test,clippy,fmt,nextest}.txt`,
`quality-tooling/phase-27-coverage-summary.txt`,
`quality-tooling/phase-27-benchmark-results.txt`.

## Deferred, explicitly

- Fuzz target execution (Task 197) -- see
  `execution-evidence/fuzzing-decision-record.md`; no nightly/libFuzzer
  toolchain available in this sandbox.
- Actual execution of the new GitHub Actions workflows on a real runner
  -- this repository has no configured `git remote`, so nothing has been
  pushed anywhere a workflow could fire. The workflow files are validated
  for syntax and hand-verified against the exact commands this execution
  already runs locally every phase, but "runs green on a real GitHub
  Actions Windows/Linux/macOS runner" is Implemented-but-not-hardware-validated,
  not Implemented-and-tested, per the No-Hallucination Contract.
- A hard coverage percentage gate -- deliberately not added; see Task 194
  above for why.

## Phase Gate

- All nine tasks (192-200) have an evidence state: seven implemented and
  verified, one (197) formally deferred with a full decision record and
  named future targets, and the CI workflow files (198-200) verified for
  syntax and command-correctness but not yet run on a real remote CI
  provider (no git remote configured).
- `cargo test --workspace`: 723/723 passing, confirmed identically by
  `cargo nextest run --workspace`. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean -- and, per Task 192, clean even
  without the `-D warnings` flag now.
- A real coverage-tooling run found and fixed a genuine test gap in
  Phase 26's SVG/PDF export code (six new tests, all passing), rather
  than merely adding coverage tooling without using it.
- No scope boundary silently crossed: no new crate; `criterion` and
  `proptest` added to `[workspace.dependencies]` and consumed only as
  `dev-dependencies`/bench targets, never a runtime dependency of any
  shipped crate.
- Implemented and tested: warning-policy enforcement, nextest wiring,
  coverage tooling (with a real gap found and closed), property tests
  for geometry/units/serialization/solver, and a full five-category
  benchmark suite with real captured numbers.
- Deferred: fuzzing (toolchain unavailable, documented) and live CI
  execution (no remote configured).
- Blocked with evidence: none -- both deferrals above are documented,
  reasoned scope decisions, not blockers preventing further work.
- Proceeding to Phase 28 automatically.
