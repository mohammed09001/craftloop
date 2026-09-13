# Phase 19 — Ink Command Language — Evidence

Recorded: 2026-09-13

## New crate: `crates/craftloop-command`

`craftloop-ids` gained `CommandId` (following the existing `define_id!`
pattern); `craftloop-errors` gained `DomainError::Command` +
`CommandErrorKind` (`InvalidForNamespace`, `NotConfirmed`).

## Task 133 — Command object model

`command.rs`: `Command { id, action, source, namespace, parameters, risk,
timestamp_seconds, undo }` -- exactly Article 239's field list. `CommandAction`
is one flat enum across every namespace's vocabulary (Article 237): a
shared action like `Dimension` (valid in both Sketch and Orthographic) is
one variant, not duplicated, per this workspace's no-duplicated-truth
convention.

## Task 134 — Contextual command namespaces

`grammar.rs`'s per-namespace vocabulary tables, exactly Article 237's
three word lists (Notebook: Pen/Eraser/Select/Sketch/Orthographic;
Sketch: Line/Circle/Arc/Rectangle/Dimension/Exit Sketch; Orthographic:
Add View/Label View/Link/Resolve/Dimension), plus the article's own
curated synonym example (`Ortho` -> `Orthographic`) -- no speculative
additions to either the vocabulary or the synonym table.

## Tasks 135-137 — Matching stages

`GrammarMatch` (`Exact`/`UniquePrefix`/`Ambiguous`/`NoMatch`) and
`resolve()`: full-word match checked first (Task 135's "stable
fallback"), then shortest-unique-prefix (Task 136), and any prefix
matching more than one word returns `Ambiguous` naming every candidate
rather than guessing (Task 137) -- there is no code path in `resolve`
that picks arbitrarily among ties.

## Tasks 138-139 — Confirmation gesture and temporal disambiguation

`confirmation.rs`: `ConfirmationEvidence` has exactly Article 238's six
named signals; `evaluate_confirmation` treats four of them (recency,
valid-command, context-would-be-geometry, user-disabled) as hard
disqualifiers rather than scored evidence, matching the article's own
binary framing, before scoring the remaining two (enclosure ratio +
deliberateness) into Execute/Preview/RemainsInk.
`is_recent_enough_to_confirm` (Task 139) is the one function that
separates "confirming a recent command" from "lassoing older content,"
using an honestly-named, uncalibrated 3-second window constant.

## Task 140 — Ephemeral command ink

`ephemeral.rs`: `dispose_of_confirmed_command_ink` names which strokes
(command text + confirmation gesture) may be removed from the page,
paired unconditionally with the `UndoMetadata` record that survives
independent of stroke identity -- Article 236's "documents should not
depend on the current meaning of ephemeral commands after execution."

## Task 141 — Command risk levels

`risk.rs`: `RiskLevel::{Low,Medium,High}`, `requires_confirmation` (only
`Low` skips it), and `may_execute` combining risk with the already-computed
`ConfirmationOutcome` -- `High` (destructive) requires the strongest
signal (`Execute`), `Medium` accepts `Preview` or better, matching Task
141's "destructive commands require stronger confirmation."

## Task 142 — Route every command through the Command Bus

`bus.rs`: `CommandBus::submit` is the single entry point every source
channel goes through (tested with all six `CommandSource` variants
sharing one history in submission order). Validates namespace-grammar
membership and the risk/confirmation policy before recording anything;
rejects leave the history untouched. Deliberately does **not** wire
`CommandAction` variants to the ~15 other engines this execution has
already built -- Task 142's own objective ("must share one semantic
operation") is about proving every source produces the same validated,
recorded object, not about full business-logic dispatch, which no task
in this phase names and which would require this crate to depend on
every domain crate built so far.

## A genuine, honestly-investigated flaky-test finding

`craftloop-sketch`'s Phase 15 test
`near_equal_length_lines_with_no_constraint_are_reported_as_observed_only`
failed once during this phase's full-workspace test run (63/64 passing
in that binary), but passed in isolation. Investigated rather than
re-run-until-green: `Sketch::observed_length_coincidences` (Phase 15)
iterates primitives in `PrimitiveId`'s own order, which is random
per-process (UUIDv4) -- the test asserted the returned pair in a fixed
`(a, b)` position, which only holds when `a`'s UUID happens to sort
before `b`'s. Fixed by asserting the unordered pair (a `BTreeSet`
comparison) instead of a fixed tuple position. Re-ran the fixed test
directly, and the full workspace suite, several times with no further
failures. A real, if minor, test-design bug -- recorded here rather than
silently amended away, per this execution's own evidence standard.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 598/598 passing (37 new), re-run 3x clean
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-19-cargo-test.txt`,
`test-reports/phase-19-cargo-clippy.txt`,
`test-reports/phase-19-cargo-fmt.txt`.

## Deferred, explicitly

- Wiring `CommandAction` variants to real domain operations across the
  engines this execution has built (the actual "Pen tool switches the
  active tool," "Line enters line-sketch mode" effects) -- no task this
  phase names that integration; `Command Bus`'s own doc comment records
  the reasoning.
- Real ink-stroke-to-`ConfirmationEvidence` computation (enclosure ratio
  from actual stroke geometry, deliberateness heuristics) -- `evaluate_confirmation`
  takes already-computed evidence; computing it from raw strokes is a
  future integration between this crate and `craftloop-ink`/
  `craftloop-ink-intent`, not named by any task here.
- Localized/translated command vocabularies (Article 237's own
  "Localization will eventually require translated command vocabularies")
  -- no task in this phase asks for it.

## Phase Gate

- All ten tasks (133-142) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 598/598 passing, confirmed stable across
  repeated runs after fixing the one flaky test found. `cargo fmt --check`
  and `cargo clippy -D warnings`: clean.
- No scope boundary crossed: the new crate depends only on
  `craftloop-errors`/`craftloop-ids`; no coupling to any specific
  domain engine's business logic.
- Implemented and tested: the full command object model, all three
  matching stages, the six-signal confirmation contract, temporal
  disambiguation, ephemeral-ink disposition, risk policy, and the shared
  Command Bus.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 20 automatically.
