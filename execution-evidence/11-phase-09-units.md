# Phase 09 — Units and Engineering Numeric Parsing — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-units` (Engine Contract 07), depending only on
`craftloop-errors`. No platform/UI, no geometry, no ink dependency — a pure
text-in/number-out parsing layer. `craftloop-document`'s `units.rs` was
refactored to re-export `craftloop_units::LengthUnit as DocumentUnits`
instead of keeping its own near-identical enum, closing the "Phase 09 owns
interpreting numbers against it" note left in the Phase 07 evidence.

## Tasks 062–069

| Task | Module | Summary |
|---|---|---|
| 062 Canonical internal units | `length_unit.rs` | **Policy decision, now enforceable**: the canonical internal length unit is millimeters. `LengthUnit::{to,from}_millimeters` are the only sanctioned conversion path. |
| 063 Document unit settings | `craftloop-document/src/units.rs` | `DocumentUnits` becomes a re-export of `LengthUnit`, not a second enum kept in sync by hand — removes a "duplicated truth" the Phase 07 evidence had flagged as a likely follow-up. |
| 064 Integer/decimal parsing | `numeric.rs::parse_decimal` | One parsing entry point every higher-level parser (length/angle/radial) bottoms out in, explicitly meant to double as the handwriting-recognition-output parser once Phase 17 exists (not built yet, but nothing about this function is keyboard-specific). |
| 065 Explicit unit parsing | `length.rs::parse_length` | Recognizes `mm`/`cm`/`m`/`in`/`"` suffixes (longest-first so `"mm"` isn't misread as `"m"`). Core invariant tested directly: an explicit unit is honored regardless of the caller's default — no silent scale change. |
| 066 Angle parsing | `angle.rs::parse_angle_degrees` | Degrees in, radians out (matching `craftloop-geometry`'s existing radian convention). Actively rejects a length suffix on an angle field as a structured `InvalidUnit` error rather than silently misreading it. |
| 067 Radius/diameter notation | `radius_diameter.rs::parse_radial` | `R`/`D`/`⌀`/`Ø` prefixes → an explicit `RadialKind::{Radius,Diameter}` candidate + millimeter value. Never chooses which geometry the value attaches to (Engine Contract 07's authority boundary) — that is left to a later (Phase 18) consumer. |
| 068 Locale-sensitive decimal tests | `numeric.rs::{parse_decimal, parse_decimal_auto}` | Explicit-locale parsing never guesses. `parse_decimal_auto` implements and tests a documented policy for the no-locale case: unambiguous when zero, two-different, or ≥2-of-one separators are present; genuinely ambiguous (returns `AmbiguousDecimalSeparator`, matching MCP Article 302) only for exactly one separator followed by exactly three digits (e.g. `"1,234"`). |
| 069 Invalid numeric diagnostics | `tests/invalid_numeric_diagnostics.rs` | Cross-cutting proof that every parser in the crate preserves the *exact original raw input* in every `DomainError::Parser` it returns, and that the `kind` field is always something a UI can `match` on without parsing the message string. |

## Two real bugs found and fixed via the Loop Engineering Contract

1. **`parse_decimal_auto` misclassified repeated-single-separator input.**
   An early version routed "multiple commas, no periods at all" (e.g.
   `"1,234,567"`, unambiguous US-style thousands grouping) through the
   *comma-is-decimal* branch, which would have tried to parse a number with
   two decimal points and failed. Caught by re-deriving the case logic by
   hand before writing the implementation (not by a failing test — this one
   was caught in design review), and fixed by handling "≥2 of one
   separator, 0 of the other" as its own unambiguous thousands-grouping
   case, separate from the single-separator ambiguity check. A regression
   test (`auto_parses_repeated_grouping_separator_with_no_decimal_point_as_a_whole_number`)
   locks this in.
2. **Raw input was truncated after unit-suffix stripping.** `parse_length`,
   `parse_angle_degrees`, and `parse_radial` all strip a suffix before
   delegating to `parse_decimal` on the remaining substring; on failure,
   the inner error's `input` field held only that substring (e.g. `"abc"`
   from `"abcmm"`), silently dropping the suffix — a direct violation of
   Task 069. Caught by the Task 069 diagnostic test itself on first run.
   Fixed with a shared `with_original_input` rewrite helper applied at
   every suffix-stripping call site.

## Commands and results

```
cargo build -p craftloop-units
cargo test -p craftloop-units                              # 40/40 unit tests
cargo test -p craftloop-units --test invalid_numeric_diagnostics   # 3/3
                                                             #   (1 real bug found+fixed on first run)
cargo build --workspace                                     # clean after folding
                                                             #   DocumentUnits into LengthUnit
cargo fmt --all -- --check                                  # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
  # 1 finding: derivable_impls on LengthUnit's manual Default; replaced
  #   with #[derive(Default)] + #[default]. Re-run clean.
cargo test --workspace                                      # 385/385 tests passing workspace-wide
```

Full output: `test-reports/phase-09-cargo-test.txt`,
`test-reports/phase-09-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Wiring these parsers into the Windows harness UI (a text-entry field
  calling `parse_length`/etc.) — no task in this phase asked for that; the
  harness's toolbar/inspector work was Phase 04's scope.
- Actually parsing handwriting-recognition candidate text through
  `parse_decimal` — Phase 17 (Engineering Handwriting Adapter) does not
  exist yet; only the shared entry point is ready for it.
- Feet, fractional-inch ("1/2\""), and scientific notation — no Version 1
  requirement named them; adding unused unit/format support would be
  speculative scope.

## Phase Gate

- All eight tasks (062–069) represented in repository code with passing
  tests, including two real bugs (one design-review catch, one test-caught)
  fixed in-loop, and one cross-crate duplication (`DocumentUnits`) removed.
- `cargo test --workspace`: 385/385 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: the parser never chooses a geometry target;
  no dimension/constraint logic added ahead of Phases 10/12.
- Proceeding to Phase 10 automatically.
