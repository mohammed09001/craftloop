# Independent Code Review

Execution 01, Phase 32, Task 231. Recorded 2026-09-14. Run as a genuinely
fresh-context review: a subagent with zero prior conversation history,
given only this repository and pointed at the same four review axes the
task's own objective names (architecture, correctness, dependency
licenses, scope adherence), not primed with this session's own
assumptions about what it would find.

## Scope of the review

A representative cross-section, not an exhaustive pass over all 24
crates or all 33 evidence files (stated explicitly by the review itself
as a limitation): `craftloop-geometry`, `craftloop-document`,
`craftloop-dimension`, `craftloop-mobile-ffi`, `craftloop-recognition`,
`craftloop-consistency`, and `apps/windows-harness`, plus a full
`cargo metadata` sweep across all 541 resolved packages for the license
axis.

## Findings

### Architecture -- no blocking findings

Confirmed `craftloop-document`'s dependency graph is strictly downward
(document -> {ink, recognition, dimension, consistency, command, units,
transactions, serialization}, no circular edges). A workspace-wide grep
for platform tokens (`egui`, `winit`, `jni`, `android`, `uikit`, `objc`,
`pencilkit`, `jetpack`, `winapi`) across `crates/*/src` found only
doc-comment prose *explaining why* those concepts are excluded -- no
actual imports or type leakage. `craftloop-mobile-ffi` confirmed clean:
every `Ffi*` type is a hand-written mirror with explicit conversions,
never a re-export; no Kotlin/Swift/Jetpack Ink/PencilKit vocabulary
anywhere in a type or field name. `apps/windows-harness` confirmed
genuinely thin (`viewport.rs` is pure transform math; no fitting/
solving/consistency logic duplicated locally).

### Correctness -- one real, fixed finding; one noted, lower-priority gap

**Fixed this phase**: `crates/craftloop-recognition/src/fit/circle.rs`'s
`fit_circle` had no guard against non-finite (NaN/Infinite) input
points. Because `det.abs() < 1e-12` is `false` for a NaN determinant
(NaN never compares as less than anything), the existing near-singular
check silently failed to catch NaN input, and a
`Some(CircleCandidate { center: NaN, .. })` would have been fabricated
from garbage input instead of `None`. **Fixed**: added an explicit
`points.iter().any(|p| !p.x.is_finite() || !p.y.is_finite())` guard at
the top of `fit_circle` (rejecting before any arithmetic runs), and
strengthened the final radius check from `radius_sq <= 0.0` to
`!(radius_sq.is_finite() && radius_sq > 0.0)`. Two new tests
(`a_nan_input_point_is_rejected_not_fabricated_into_a_nan_circle`,
`an_infinite_input_point_is_rejected`) confirm the fix; `fit_arc`
(Phase 06) automatically inherits the fix since it calls `fit_circle`
directly. Verified: `cargo test -p craftloop-recognition fit::circle`,
7/7 passing.

**Noted, not fixed this phase** (lower priority than the above -- the
review's own words: "likely not reachable in practice today"):
`crates/craftloop-dimension/src/association.rs` has no direct test for
a candidate whose orientation/bounds computation degenerates (e.g. a
zero-length line candidate), relying only on `orientation_alignment_score`'s
internal `direction_length < 1e-9` guard, untested at the association
call site specifically. Recorded here as a known, minor gap for a future
session rather than silently left unmentioned.

### Dependency Licenses -- one real gap, fixed this phase

**Fixed this phase**: `uniffi` (pinned `=0.32.1`) is MPL-2.0-licensed --
the one non-MIT/Apache-2.0 dependency in the workspace -- and had no
corresponding license review anywhere in `execution-evidence/`, unlike
`ezpz`, which got a full review as part of its own Phase 11 decision
record. **Fixed**: `quality-tooling/phase-32-uniffi-license-review.md`,
concluding MPL-2.0's file-level (not project-wide) copyleft imposes no
obligation on this project's own source for ordinary, unmodified library
usage, and is explicitly designed to combine with a proprietary/
`UNLICENSED` codebase.

The review separately confirmed the `ezpz` decision record's own claims
still hold (`ezpz` 0.2.29 MIT, direct deps `faer`/`indexmap`/`libm`/
`winnow`/`mutants` all MIT or `Apache-2.0 OR MIT`, matching the record's
license table), and found no GPL/AGPL/SSPL anywhere in all 541 resolved
packages -- the only other non-fully-permissive license found across the
entire dependency tree was `r-efi` (a transitive, UEFI-target-only,
inactive-on-this-platform dependency), itself triple-licensed including
a permissive `MIT OR Apache-2.0` option, so not a real concern.

### Scope Adherence -- no findings

Clean on every check performed: no crate under `crates/` imports a GUI
toolkit, Windows API, or mobile SDK; `apps/windows-harness` delegates
every domain operation to a `craftloop-*` crate rather than duplicating
logic locally.

## Disposition

Both real findings ("worth fixing") were fixed in this same phase, not
merely noted for later: the `fit_circle` NaN/Inf guard (with tests) and
the `uniffi` license review. The one "minor" finding
(`association.rs`'s untested degenerate-orientation path) is recorded
above as a known gap, left for a future session -- consistent with this
execution's own convention of naming a deferred item explicitly rather
than silently dropping it.
