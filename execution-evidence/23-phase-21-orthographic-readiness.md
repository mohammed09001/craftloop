# Phase 21 — Projection Convention and Orthographic Readiness — Evidence

Recorded: 2026-09-13

Built inside `crates/craftloop-document` (new `orthographic.rs`), which
now also depends on `craftloop-command` (Phase 19, for Task 154's
transition) and reuses `craftloop-consistency::validate_primitive_geometry`
(Phase 14, for Task 153's blocker check) rather than duplicating either.

## Task 150 — Projection convention enum

`ProjectionConvention::{FirstAngle,ThirdAngle}` -- exactly Article 488's
two named conventions.

## Task 151 — Default layout mapping

`default_layout_offset`/`default_orthographic_layout`: a deliberately
simplified, honestly-documented default arrangement (third-angle places
`Top` above and `Right`/`Back` to the right of `Front`; first-angle
mirrors both axes) -- explicitly *not* claimed as certified
standards-compliance (Article 235's own caution; a real profile-driven
arrangement is the Standards Engine's job, Phase 25). Computed fresh on
every call from `(identity, convention, spacing)`, never cached, so page
layout cannot become its own source of truth. MCP Article 666's own
validation scenario (choose first-angle, verify layout, switch to
third-angle, verify the layout changes) is reproduced directly as a test.

## Task 152 — Orthographic Readiness states

`OrthographicReadiness` -- exactly Article 33's five graded levels
(`DraftReady`/`IdentityReady`/`LinkReady`/`Resolved`/`Constrained`), in
declared order so `Ord` supports "at least Link Ready" comparisons.
`evaluate_readiness` computes the real level from a view's actual
membership and geometry -- never requires full dimensioning.

## Task 153 — Blocker classification

`ReadinessIssue::{MissingViewIdentity, DegenerateGeometry,
UnresolvedDimension}` with `is_blocker()` -- exactly Article 234's own
three examples ("a missing View Identity is a blocker," "a local
constraint contradiction may be a blocker," "a missing depth is
incompleteness"). `DegenerateGeometry` is checked for real, via the
already-real `craftloop-consistency` validator; `UnresolvedDimension` is
named and given its `is_blocker() == false` answer now, ready for a
later phase that actually constructs it once a concrete "missing depth"
check exists.

## Task 154 — Ortho command transition

`transition_to_orthographic(command, source, convention, spacing)`:
rejects any `Command` whose `action != Orthographic`; otherwise computes
the same layout regardless of `CommandSource`. Tested directly: a
toolbar-sourced and an ink-command-sourced `Orthographic` command produce
byte-identical output -- "one transaction" made literal, building on
Phase 19's `CommandBus` already unifying the source channel at the
object level.

## Task 155 — Preserve source view and spatial focus

`default_orthographic_layout` never includes `Front` in its output map
at all, and every other view's offset is computed relative to the
source's *actual current* `PageLayoutTransform`, not a fixed origin --
tested with a source already sitting far from the page origin, proving
the new views land relative to it rather than recentering the whole
layout.

## Task 156 — No-dimension orthographic tests

`a_labeled_view_with_real_geometry_and_zero_dimensions_reaches_link_ready_task_156`:
a view with an established identity and one real (non-degenerate)
primitive, and zero dimensions anywhere, reaches `LinkReady` with an
empty issues list -- proving the engine never treats "no dimensions yet"
as a blocker or even as a reportable gap.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 625/625 passing (14 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean (1 finding fixed: if_same_then_else)
```

Full output: `test-reports/phase-21-cargo-test.txt`,
`test-reports/phase-21-cargo-clippy.txt`,
`test-reports/phase-21-cargo-fmt.txt`.

## Deferred, explicitly

- A real, standards-profile-driven layout arrangement (Article 489) --
  Phase 25's Standards Engine job; this phase's mapping is named honestly
  as a simplified default.
- `Resolved`/`Constrained` readiness levels are defined but not yet
  reachable by `evaluate_readiness` (no task before this one gives this
  engine a way to check "critical unknowns supplied" or "sufficient
  constraints for predictable editing" against a live `Sketch`/dimension
  store) -- named for Article 33 completeness, constructed by whichever
  later phase wires readiness evaluation to those live engines.
- Wiring `evaluate_readiness`/`transition_to_orthographic` into a live
  `Document`/`Page` (creating real `ViewBlock`s on transition, persisting
  them) -- no task this phase names that integration.

## Phase Gate

- All seven tasks (150-156) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 625/625 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate; reused
  `craftloop-consistency`/`craftloop-command` rather than duplicating
  either engine's logic.
- Implemented and tested: both projection conventions with a verifiably
  different layout, readiness grading including the zero-dimension case,
  blocker/incompleteness classification, and a source-channel-agnostic
  Ortho transition that never moves the source view.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 22 automatically.
