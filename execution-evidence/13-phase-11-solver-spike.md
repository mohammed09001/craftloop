# Phase 11 — Constraint Solver Technical Spike — Evidence

Recorded: 2026-09-13

## Crate created (production, permanent)

`crates/craftloop-constraint` (Engine Contract 10): the backend-neutral
interface only — `VariableId`/`Variable`/`PointVariables`,
`GeometricConstraint` (11 variants covering Craft Loop's V1 constraint
vocabulary), `residual()`, `SolveResult`/`SolveStatus`/`ConstraintDiagnostic`,
and the `ConstraintSolver` trait plus a non-iterating reference
implementation (`ResidualChecker`). No real numeric solver is chosen or
wired in — that is explicitly out of scope for this phase (Task 078's own
title: "before choosing implementation") and lands in Phase 12.

## Spike/evaluation artifacts (not part of the workspace)

- `execution-evidence/solver-evaluations/ezpz-spike/` — a real, standalone
  Cargo project (its own `[workspace]`, excluded from the main Craft Loop
  workspace) depending on `ezpz = "0.2.29"`, built and run against four
  real scenarios.
- `crates/craftloop-constraint/examples/naive_gradient_descent_baseline.rs`
  — a minimal in-house solver prototype (Task 080), part of the real
  workspace since it only uses `craftloop-constraint`'s own code, no new
  dependency.
- `execution-evidence/solver-evaluations/solver-decision-record.md` — the
  Task 083 decision record.
- `execution-evidence/solver-evaluations/ezpz-spike-output.txt`,
  `naive-baseline-output.txt` — raw captured command output.

## Tasks 078–083

| Task | Evidence |
|---|---|
| 078 Solver interface | `crates/craftloop-constraint` (23 unit tests, all passing). |
| 079 Evaluate `ezpz` | Real crate source read directly from the cargo registry cache (not documentation summaries alone); 24-variant `Constraint` enum covers every V1 relationship needed; real spike program built, compiled, and run against small/medium/conflict/redundant-consistent scenarios. |
| 080 Alternative/in-house prototype | `naive_gradient_descent_baseline.rs`: ~9x slower than `ezpz` and needs 338 iterations vs. a handful, on the simplest possible test case — real, measured comparison, not assumed. |
| 081 Interactive solve latency | Small graph: 0.0126 ms median (200 runs). Medium graph (4-point rectangle, 8 constraints): 0.0171 ms median. Measured on this Windows workstation only; no CI hardware was available to this session, recorded as a gap rather than assumed equal. |
| 082 Conflict diagnostics | `SolveOutcome::unsatisfied()`/`converged()`/`warnings()`/`priority_solved()` inspected from source; a real gap found (no public per-constraint residual magnitude) and a real, demonstrated mitigation built (`craftloop-constraint`'s own `residual()` re-applied to `ezpz`'s `final_values()`). |
| 083 Decision record | `solver-decision-record.md`: **adopt `ezpz`, pinned at exactly `0.2.29`**, with license notes, a supply-chain observation (the crate name was previously used by an unrelated project), and an explicit fallback plan. |

## A genuine, honestly-reported spike finding

The first `redundant_but_consistent` scenario in the `ezpz` spike reported
`is_satisfied: false` — initially looking like a solver limitation on
exactly-redundant (rank-deficient) systems, a real category of Craft Loop
scenario (MCP Article 307). Investigation (reading `ezpz`'s
`HorizontalDistance` residual formula in its source, `residual0 = (p0.x -
p1.x) - expected_distance`) showed the constraint was **signed**, and the
spike's own test data had the wrong sign for the redundant constraint —
i.e., the "unsatisfied" result was `ezpz` correctly detecting a genuinely
contradictory system the spike had accidentally constructed. Fixed the
sign in the test; the corrected, truly-redundant-and-consistent system
then solved with `is_satisfied: true` in a single iteration. This is
recorded in the decision record as a data point *in favor* of `ezpz`
(it did not silently paper over bad input) rather than erased from the
evidence trail.

## Commands and results

```
cargo build -p craftloop-constraint
cargo test -p craftloop-constraint                       # 23/23 unit tests
cd execution-evidence/solver-evaluations/ezpz-spike
cargo run --release                                       # real ezpz integration, 4 scenarios
cd -; cargo run -p craftloop-constraint \
  --example naive_gradient_descent_baseline --release     # in-house baseline
cargo fmt --all -- --check                                 # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
cargo test --workspace                                     # 401/401 tests passing workspace-wide
```

Full output: `test-reports/phase-11-cargo-test.txt`,
`test-reports/phase-11-cargo-clippy.txt`,
`solver-evaluations/ezpz-spike-output.txt`,
`solver-evaluations/naive-baseline-output.txt`.

## Deferred (explicitly, not silently)

- Real `ezpz` adapter implementing `craftloop_constraint::ConstraintSolver`
  — Phase 12.
- CI-hardware latency numbers — no CI environment available to this
  session.
- Android/iOS cross-compilation of `ezpz` — no toolchain available on this
  Windows workstation (a true blocker for *that specific verification*
  only, per the True Blocker Policy); dependency list looks favorable but
  is not verified.
- Large-sketch performance scaling — no Version 1 scenario currently
  requires it.

## Phase Gate

- All six tasks (078–083) represented in repository code or evidence
  files, backed by real, executed spike code and directly-read crate
  source, not summaries taken on faith.
- `cargo test --workspace`: 401/401 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no real solver backend wired into the
  permanent workspace this phase; the interface stays backend-neutral as
  Task 078 requires.
- Proceeding to Phase 12 automatically.
