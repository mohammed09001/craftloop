# Fuzz Target Decision Record

Execution 01, Phase 27, Task 197. Recorded 2026-09-13.

## Decision

**Defer `cargo-fuzz` target implementation.** No fuzz harness exists in
this repository. This is a documented, deliberate defer per the task's
own text ("target parsers, document decoding, and geometry edge cases in
CI environments that support fuzzing") and the No-Hallucination Contract:
this environment does not support fuzzing, so claiming fuzz coverage
would misrepresent what has actually been verified.

## Why this environment does not support fuzzing

`cargo-fuzz` requires two things this sandbox does not have and that
installing would be disproportionate to add for a "where valuable"
contingent task:

- **A nightly Rust toolchain.** `rustup toolchain list` shows only
  `stable-x86_64-pc-windows-msvc` installed; `cargo-fuzz` requires
  nightly for its `-Z` sanitizer/instrumentation flags.
- **libFuzzer support.** `cargo-fuzz` links against LLVM's libFuzzer,
  which is not packaged for the MSVC toolchain this workspace's
  `rust-toolchain` targets (`x86_64-pc-windows-msvc`) the way it is for
  `-gnu` targets or Linux/macOS -- fuzzing this workspace practically
  needs a Linux or WSL environment regardless of the nightly requirement
  above.

Both are installable in principle (network access was confirmed working
this phase, e.g. for `cargo install cargo-nextest`/`cargo-llvm-cov` and
new crates.io dependencies), but a nightly toolchain plus a
libFuzzer-capable target is a meaningfully larger environment change than
this phase's other tasks, and cannot be verified end-to-end (a fuzz
target that has never actually run cannot be claimed as working, per the
No-Hallucination Contract's "Scaffolded/stubbed" category at best). The
honest choice is a clear defer record naming real targets, not unverified
harness source files.

## Concrete fuzz-worthy targets for when this infrastructure exists

Named now so the defer is actionable later, not just a gap:

| Target | Crate/function | Why it is fuzz-worthy |
|---|---|---|
| Numeric parsing | `craftloop_units::parse_decimal`/`parse_decimal_auto` | Locale-ambiguous decimal/grouping-separator input (Phase 09, Task 062) is exactly the class of adversarial-string input fuzzing finds panics in. |
| Length parsing | `craftloop_units::parse_length` | Combines numeric parsing with unit-suffix tokenizing; a malformed suffix is a second axis of adversarial input the numeric fuzzer alone would not reach. |
| Radius/diameter parsing | `craftloop_units::parse_radial` | Prefix-detection logic (`R`/`⌀`/`D`) over arbitrary strings. |
| Angle parsing | `craftloop_units::parse_angle_degrees` | Same class as length parsing, different unit domain. |
| Document deserialization | `craftloop_document::persistence::load_document` | Task 053's crash-recovery path already handles missing/corrupted files; a fuzzer targeting arbitrary byte input (not just "missing" or "not JSON") would stress-test the same recovery path against inputs a handwritten test suite is unlikely to think of. |
| Canonical JSON round-trip | `craftloop_serialization::to_canonical_json` fed back through `serde_json::from_str` on arbitrary `Document`-shaped JSON | Targets schema-evolution robustness (Article 387) against malformed-but-plausible saved files, not just well-formed ones. |
| Geometry construction | `craftloop_geometry::{Circle2::new, Arc2::new, RelationalRectangle::new}` | Already property-tested (Phase 02, Task 020) against random *valid-shaped* input; a fuzzer additionally explores NaN/infinity/subnormal edge cases property testing's bounded strategies do not generate by default. |

## What is NOT deferred

Proptest-based property testing (Task 195, same phase) covers a
meaningfully overlapping but distinct need -- structured random testing
of *valid* input against known invariants -- and is implemented for real
this phase (`craftloop-geometry` since Phase 02's Task 020;
`craftloop-units`, `craftloop-serialization`, and `craftloop-constraint`
newly this phase). Fuzzing's distinct value is *unstructured*
byte/string-level adversarial input, which is what remains deferred here.
