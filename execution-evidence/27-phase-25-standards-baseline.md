# Phase 25 — Standards-Aware Representation Baseline — Evidence

Recorded: 2026-09-13

New crate: `crates/craftloop-standards`. A genuinely new crate rather than
an extension of `craftloop-document` (unlike Phases 20-24), since line
role/style, dimension typography, and standards-profile metadata are
presentation/export-adjacent concerns, not part of the document's own
entity model. Depends on `craftloop-document` (for `ProjectionConvention`)
and `craftloop-dimension` (for `SemanticDimension`/`DimensionAnnotation`)
rather than duplicating either.

## Task 180 — Line role vocabulary

`LineRole` (`line_role.rs`): six semantic roles -- `Visible`,
`Construction`, `Centerline`, `Hidden`, `DimensionLine`,
`ExtensionLine`. A plain, exhaustively-matched enum; distinctness and
serialization round-trip each proven by a direct test.

## Task 181 — Standards profile metadata

`StandardsProfile` (`profile.rs`): a name plus a
`craftloop_document::ProjectionConvention` (Phase 21), reused rather than
duplicated. Article 235's fuller profile shape (line-weight
relationships beyond Task 182's flat mapping, leader/centerline/hidden-
line conventions, section-view rules) is named as future capability, not
built speculatively -- see `UNSUPPORTED_CONVENTIONS` (Task 185).

## Task 182 — Deterministic line role → style mapping

`style_for_role` (`line_style.rs`): one `LineStyle` (dash pattern in
document millimeters, relative weight) per role, exhaustive match, no
"unmapped role" case possible. Hidden and centerline are proven
distinguishable from each other and from solid visible geometry by
direct test, not just by inspection of the dash patterns.

## Task 183 — Dimension typography, kept separate from semantic value

`DimensionTypography`/`PresentedDimension`/`present()`
(`dimension_presentation.rs`): typography (text height, line styles) is
purely presentational and holds no dimension value of its own.
`present()` always reads `SemanticDimension::value()` fresh at call time
-- proven by a test that edits the dimension between two `present()`
calls and asserts the second reflects the new value, so there is no code
path by which a cached presentation-layer copy could disagree with the
real value.

## Task 184 — Certified-compliance guardrail, enforced at construction

`contains_forbidden_compliance_claim`/`reject_forbidden_claim`
(`guardrail.rs`), per Article 235's explicit instruction that Version 1
must not claim full standards compliance. Not a linting convention:
`StandardsProfile::new` calls `reject_forbidden_claim` on the profile
name and refuses construction outright, proven by
`a_compliance_claiming_name_is_rejected_at_construction`. Plain
"standard(s)" wording (including this crate's own name) is explicitly
allowed; only claims of having *met* a standard (`compliant`,
`certified`, `conforms to`, etc.) are rejected, case-insensitively.

## Task 185 — Honest gap list plus end-to-end export exercise

`UNSUPPORTED_CONVENTIONS` (`lib.rs`): leader conventions, per-profile-
configurable styling beyond the flat baseline mapping, section-view
rules, and licensed ISO/ASME rule text, each named explicitly rather
than silently missing. `tests/standards_export.rs` exercises the whole
pipeline the way an export pass would: resolve a style for every
implemented role, confirm visible/hidden are never visually identical,
build a named profile and read back its projection convention, and
assert the gap list documents (not omits) the Leader and Section-view
gaps.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 686/686 passing (19 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-25-cargo-test.txt`,
`test-reports/phase-25-cargo-clippy.txt`,
`test-reports/phase-25-cargo-fmt.txt`.

## Errors and fixes

- `LineStyle.dash_pattern` was first written as `&'static [f64]` with a
  derived `Deserialize`; borrowed static-lifetime slices cannot
  implement `Deserialize<'de>`, so the crate failed to compile. Fixed by
  changing the field to an owned `Vec<f64>` and updating every
  `style_for_role` match arm from slice literals to `vec![]` literals;
  `LineStyle`'s derive list dropped `Copy` accordingly, since `Vec` is
  not `Copy`. Re-verified with a full `cargo test -p craftloop-standards`
  pass (19 unit + 4 integration tests) before proceeding.

## Deferred, explicitly

- Leader-line conventions -- no leader concept exists anywhere in this
  workspace yet.
- Per-profile-configurable line styling -- this phase's mapping is a
  single deterministic baseline (one style per role), not a stylesheet a
  `StandardsProfile` can override.
- Section-view rules -- no section-view concept exists yet.
- Licensed ISO/ASME normative rule text -- out of scope for this product
  regardless of phase (licensing, not an engineering gap).
- Wiring this crate into a live export/render pipeline -- no task this
  phase names that integration.

## Phase Gate

- All six tasks (180-185) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 686/686 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- New crate `craftloop-standards` added to workspace members; depends
  only on existing crates (`craftloop-document`, `craftloop-dimension`,
  `craftloop-geometry`, `craftloop-ids`, `craftloop-errors`), no new
  external dependency beyond `serde`/`serde_json` (already used
  workspace-wide).
- Implemented and tested: the six-role vocabulary, deterministic
  role-to-style mapping, typography kept structurally separate from
  semantic value and always read fresh, the construction-time
  compliance-claim guardrail, and the honest unsupported-conventions
  list backed by an end-to-end export-shaped test.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 26 automatically.
