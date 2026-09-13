# Constraint Solver Decision Record

Execution 01, Phase 11, Task 083. Recorded 2026-09-13.

## Decision

**Adopt `ezpz` (crates.io, `github.com/KittyCAD/ezpz`) as the constraint
solver backend, pinned at exactly `0.2.29`, behind the
`craftloop-constraint` interface (Task 078).** Integration itself
(implementing `craftloop_constraint::ConstraintSolver` for an `ezpz`
adapter) is Phase 12's job, not this phase's — this record authorizes that
work with evidence, it does not do it.

## Evidence

### Task 079 — Evaluate `ezpz` against Craft Loop's required primitive constraints

Confirmed via direct inspection of the crate source (downloaded to the
local cargo registry cache, not taken on faith from documentation) at
`~/.cargo/registry/src/index.crates.io-*/ezpz-0.2.29/src/constraints.rs`.
The public `Constraint` enum has 24 variants. Every relationship Craft
Loop's V1 scope needs is present, several with more precision than
initially assumed:

| Craft Loop need | `ezpz` variant |
|---|---|
| Fix a value | `Fixed` |
| Distance between two points | `Distance`, `DistanceVar` (distance driven by another variable) |
| Coincident points | `PointsCoincident` |
| Horizontal / vertical alignment | `Horizontal`, `Vertical`, `HorizontalDistance`, `VerticalDistance` |
| Angle between lines | `LinesAtAngle`, `PointsAtAngle` |
| Circle radius | `CircleRadius` |
| Equal values / equal-length lines | `ScalarEqual`, `LinesEqualLength` |
| Tangency | `LineTangentToCircle`, `CircleTangentToCircle` |
| Arc-specific | `ArcRadius`, `Arc`, `ArcLength`, `ArcAngle`, `PointArcCoincident` |
| Extras with no immediate Craft Loop use | `Midpoint`, `PointLineDistance` (+ H/V variants), `Symmetric` |

`craftloop-constraint`'s own `GeometricConstraint` vocabulary (Task 078)
was scoped to match this list's V1-relevant subset — confirmed compatible
*before* writing the interface, not the other way around.

**Real, working integration was built and run**, not just read about: see
`ezpz-spike/` (its own standalone Cargo workspace, `ezpz = "0.2.29"` as its
only dependency; excluded from the main Craft Loop workspace since Phase 11
is evaluation, not adoption). Output captured verbatim in
`ezpz-spike-output.txt`.

A **redundant-but-consistent** system (Article 307's named edge case:
6 constraints over 4 degrees of freedom, all mutually consistent) solved
correctly (`is_satisfied: true`, converged in 1 iteration since the guess
was already close, final residuals ~1e-11). The first attempt at this test
used the wrong sign for `HorizontalDistance` (it is `p0.x - p1.x`, signed,
not `|p0.x - p1.x|` — easy to get wrong once); `ezpz` correctly reported
the resulting genuinely-contradictory system as unsatisfied rather than
silently producing a garbage compromise, which is itself a small positive
data point about the library not papering over bad input.

### Task 080 — In-house alternative

A minimal finite-difference gradient-descent prototype was built at
`crates/craftloop-constraint/examples/naive_gradient_descent_baseline.rs`
(part of the real workspace, using `craftloop-constraint`'s own
`residual()` function — no throwaway duplicate math). On the *simplest*
possible test case (2 points, 3 constraints):

| | `ezpz` (Newton/Levenberg-Marquardt) | in-house (gradient descent) |
|---|---|---|
| Median solve time | 0.0126 ms | 0.1170 ms (**~9x slower**) |
| Iterations to converge | not exposed for this tiny case, but `medium` case converges in single digits | 338 |
| Convergence guarantee | analytic Jacobian, damped Newton — well-conditioned near a solution | none; step size and convergence are hand-tuned and would need re-tuning per problem shape |

The gap is expected to widen, not narrow, on larger/more constrained
sketches: gradient descent's iteration count scales poorly with problem
conditioning, while Newton-family methods (what `ezpz` uses) converge
quadratically near a solution. Writing and maintaining a competitive
in-house solver — with analytic Jacobians for 24 constraint types, robust
handling of degenerate/rank-deficient cases, and a priority system — is a
substantial, ongoing engineering investment with no demonstrated advantage
over the evaluated alternative.

### Task 081 — Interactive solve latency

Measured on this Windows development workstation (200 runs each, release
build); **no CI hardware was available to this session to also measure
there** — recorded as a real gap, not silently assumed equivalent:

| Graph size | Median | Max (of 200 runs) |
|---|---|---|
| Small (2 points, 3 constraints) | 0.0126 ms | 0.1793 ms |
| Medium (4 points, 8 constraints — an axis-aligned rectangle) | 0.0171 ms | 0.0605 ms |

Both are far below any interactive-latency budget (single-digit
milliseconds would already be imperceptible for a per-edit solve). No
incremental-solve API is exposed by `ezpz` directly (confirmed by
inspecting `solver.rs`/`lib.rs`'s public surface: `solve()` always solves
from scratch, given initial guesses) — but at these absolute latencies for
small/medium sketches, "solve from scratch with the previous solution as
the initial guess" (exactly what
`craftloop_constraint::ConstraintSolver::resolve_incremental`'s interface
contract already allows a backend to do, per its own doc comment) is not a
performance concern at Version 1's scale. Large-sketch scaling was not
measured this phase (no Version 1 scenario requires it yet); flagged as a
Phase 12 follow-up if a real large document ever motivates it.

### Task 082 — Conflict diagnostics

`SolveOutcome` exposes `unsatisfied()` (indices of unsatisfied constraint
requests), `converged()` (numerical convergence, distinct from constraint
satisfaction), `iterations()`, `warnings()` (a structured `Warning` enum,
including a genuinely nice touch: it lints "you wrote a 90°/180° angle
constraint, consider `Perpendicular`/`Parallel` instead"), and
`priority_solved()` (useful for the priority-tiered constraint model
`ezpz` supports natively — 0 is highest priority, and lower-priority
constraints can be dropped if the higher-priority ones are already
infeasible).

**Gap found, with a working mitigation**: `SolveOutcome` does not expose a
per-constraint residual *magnitude* directly (only *which* constraint
indices are unsatisfied). This was confirmed by reading `solve_outcome.rs`
— the underlying `Constraint::residual` method exists internally but is
`pub(crate)`, not part of the public API. The spike's conflict-diagnostics
test demonstrates the mitigation: `SolveOutcome::final_values()` /
`final_value_point()` are public, so `craftloop-constraint`'s own
`residual()` function (Task 078, built independently of `ezpz`) can be
re-applied to `ezpz`'s solved values to recover the "how far off" number a
humanized conflict message needs (Article 99). Demonstrated live: for two
contradictory `Fixed` constraints (x=0 vs x=10), the solver settles at the
midpoint (x=5) and both constraints' recomputed residuals correctly read
5.0. This closes the gap without needing an unreleased/forked version of
`ezpz`, so Task 082's rejection condition ("reject a solver integration
that cannot provide enough information to humanize common conflicts
unless a reliable diagnostic layer can be built") is satisfied: a reliable
diagnostic layer *can* be built, and was demonstrated, not just assumed.

## License and supply-chain notes

- `ezpz` itself: **MIT**, confirmed from its own `Cargo.toml` (not just a
  marketing page).
- Direct dependencies, each checked individually via `cargo info`, all
  permissive: `faer` (MIT), `indexmap` (Apache-2.0 OR MIT), `libm` (MIT),
  `mutants` (MIT), `thiserror` (MIT OR Apache-2.0), `winnow` (MIT). No
  copyleft, no unclear licensing.
- **Supply-chain observation worth recording plainly**: the crate name
  `ezpz` on crates.io was previously used (version `0.0.1`) by an unrelated
  project — "a smart Rust project bootstrapper" by a different author
  (`github.com/fnabinash/ezpz`) — before KittyCAD's constraint solver
  occupies the name from version range up to `0.2.29`
  (`github.com/KittyCAD/ezpz`). This is a legitimate crates.io name
  reassignment between two unrelated projects, not a typosquat (KittyCAD's
  `ezpz` is a real, actively maintained, 246-commit, 48-star repository
  with its own `ezpz-cli`/`ezpz-wasm` siblings) — but it is exactly the
  kind of history that makes **exact version pinning non-optional** here.
  `craftloop-constraint`'s eventual adapter (Phase 12) must pin `ezpz =
  "=0.2.29"` (or whatever version is actually integrated), not a caret
  range, and any version bump must be a deliberate, reviewed change.
- Repository activity at evaluation time: 246 commits, 48 stars, 8 forks,
  10 open issues (via GitHub) — actively maintained, not abandoned.
- `edition = "2024"` in `ezpz`'s manifest requires a reasonably current
  Rust toolchain; this workstation's `rustc 1.98.1` handles it without
  issue.

## Mobile portability

Not directly tested this phase (no Android/iOS toolchain available on this
Windows workstation — see the True Blocker Policy: unavailable platform
toolchain for platform-only verification is a legitimate blocker for *that
specific verification*, not for the rest of this evaluation). `ezpz`'s
dependency list (`faer`, `indexmap`, `libm`, `thiserror`, `winnow`) is
pure-Rust with no OS-specific bindings visible in its `Cargo.toml`, which
is a good sign for cross-compilation via UniFFI (the Architecture
Decision's chosen Rust-to-mobile binding path), but this is *not* the same
as a verified Android/iOS build. Recorded here as **implemented but not
hardware/toolchain-validated**, per the No-Hallucination Contract's
required category, and left as an explicit Phase 28/29 follow-up.

## Fallback plan

If Phase 12 integration surfaces a blocking problem not visible in this
spike (e.g. a real Version 1 sketch size where solve latency or robustness
degrades unacceptably, or the mobile cross-compile genuinely fails), the
fallback is **not** "write a full in-house solver" (Task 080's evidence
argues against that as a first resort) but:
1. Re-evaluate at the specific failure with `ezpz`'s own `Config` tuning
   (iteration count, tolerances, initial `Config::default()` values were
   not tuned this phase).
2. Check for an alternative MIT/Apache-licensed Rust 2D solver crate with
   the same evaluation rigor applied here (none was identified as a strong
   candidate during this phase's search — `ezpz` was the only real,
   actively-maintained match).
3. Only as a last resort, scope a purpose-built in-house solver for
   exactly the constraint subset actually blocking, informed by which
   specific limitation was hit — not a speculative general-purpose rewrite.

## What was and was not verified (No-Hallucination Contract summary)

**Implemented and tested**: `ezpz` integration compiles and runs standalone
against real V1-shaped constraint systems (small/medium graphs, a
deliberate conflict, a redundant-but-consistent system); latency measured
on real hardware; diagnostics gap identified and a working mitigation
demonstrated; licenses checked from source manifests, not summaries.

**Implemented but not hardware-validated**: mobile (Android/iOS)
cross-compilation of `ezpz` — dependency list looks favorable, nothing
Android/iOS-specific was actually built or run.

**Deferred**: real Phase 12 integration (an adapter implementing
`craftloop_constraint::ConstraintSolver` for `ezpz`), large-sketch
performance scaling, CI-hardware latency numbers (no CI environment
available to this session).

**Blocked with evidence**: none for this phase — every task (078–083)
reached a real, evidence-backed conclusion.
