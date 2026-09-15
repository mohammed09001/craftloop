# Execution 03, Phase 08 — Shared Interaction Mode and Commands

Recorded 2026-09-15. Tasks 056-063.

## Repository-first discovery this phase rests on

`craftloop-command` (Execution 01, Phase 19) already models almost
everything this phase needs: `CommandAction::{Pen, Sketch, ExitSketch,
...}`, `CommandNamespace::{Notebook, Sketch, Orthographic}`, and a real,
tested grammar resolver (`craftloop_command::grammar::resolve`) with
exact/prefix/ambiguous/no-match resolution. Critically, that resolver's
own existing test
(`an_ambiguous_prefix_never_guesses_and_names_every_candidate`)
confirms `"s"` is a genuine, deliberate collision between "select" and
"sketch" in the Notebook vocabulary — a real prior product decision
("never guess"), not an oversight.

This creates a real tension with this execution's own contract ("S or
Sketch enters Sketch Mode"). Resolved by treating the two as
legitimately different problems: a literal keyboard keypress is not
ambiguous text (it is a discrete, named key), so `S`/`B` are wired as
direct keyboard shortcuts calling dedicated session methods, never
routed through the text-grammar resolver. The grammar resolver itself
was not touched, so Execution 01's own ambiguity test and its intent
stay correct and unmodified.

## Task 056 — Add WorkspaceMode

`WebWorkspaceMode` (`Creative` | `Sketch2D`) added to
`craftloop-web-bridge`'s `types.rs`, and a `workspace_mode` field added
to `CraftLoopSession`, ephemeral exactly like `selection` already was
(never a `DocumentChange`, never touches `DocumentHistory`) — same
established pattern from Phase 04, not a new mechanism. Kept as its own
small type rather than reusing `CommandNamespace` directly: the two are
correlated (each `WorkspaceMode` maps onto one real `CommandNamespace`
when submitting through the bus) but are different concepts —
`CommandNamespace` is the grammar's three-value concept (including
`Orthographic`, not a toolbar drawing mode); `WorkspaceMode` is the
narrower "what does the canvas/toolbar look like" concept. Exposed in
`scene_snapshot()`'s `workspace_mode` field, same access pattern as
`selection`.

## Task 057/058 — EnterSketchMode / EnterCreativePenMode

`CraftLoopSession::enter_sketch_mode()`/`enter_creative_pen_mode()`
(wasm: `enterSketchMode`/`enterCreativePenMode`): each submits the real
`CommandAction::Sketch`/`ExitSketch` through the real `CommandBus`
(same validation every other command goes through) before flipping
`workspace_mode`, and neither ever calls `self.commit(...)`. One
semantic action per transition — the toolbar's Sketch/Pen buttons and
the `S`/`B` keyboard shortcuts (Task 061) call these exact methods,
never a separate code path.

## Task 059/060 — Reserve aliases / wire keyboard

`src/workspace/useKeyboardShortcuts.ts`: a `window` `keydown` listener
mapping the literal keys `s`/`b` (case-insensitive, no modifier keys)
to `enterSketchMode`/`enterCreativePenMode`, ignored entirely when
`isTextInputFocused(event.target)` — checked via the element's tag
(`INPUT`/`TEXTAREA`/`SELECT`) or a real `contenteditable` attribute
(not `element.isContentEditable`, which jsdom's test environment does
not implement — reading the attribute directly is portable across a
real browser and jsdom and means the same thing). 4 Vitest tests for
this pure check plus 3 Playwright tests: `S`/`B` really toggle mode, a
focused `<input>` really suppresses them, and the toolbar buttons
really dispatch the identical transitions.

## Task 061 — Wire toolbar actions

`Toolbar.tsx`'s Sketch button now calls `onEnterSketchMode` (no longer
`deferredUntil`); Pen calls `onEnterCreativePenMode` first whenever
clicked while already in `Sketch2D`, then selects the Pen tool. Article
26 ("Pen stays in Sketch Mode"): the Pen button stays enabled in
Sketch2D; Select/Eraser — Notebook-only concepts — visibly pause
(`disabled`, with a title explaining how to get them back) rather than
silently doing something wrong if clicked.

## Task 062 — Wire command simulator (infrastructure)

New free wasm function `resolveCommand(input, namespace) -> JSON
WebGrammarMatch`, calling the real
`craftloop_command::grammar::resolve` directly and mirroring its
`Exact`/`UniquePrefix`/`Ambiguous`/`NoMatch` result exactly (including
every real `CommandAction` variant via `WebCommandAction`, and the
Notebook `"s"` case genuinely returning `Ambiguous{candidates:
["select","sketch"]}` — proven by a real test, not asserted from
memory). Exposed on `useCraftLoopSession` as a read-only `resolveCommand`
action. This is the infrastructure Phase 15's dedicated command
simulator UI (Article 45, Task 119) will build on; no simulator widget
exists yet — that polish is that phase's own job, not duplicated here.

## Task 063 — Test history separation

Proven directly, not inferred: `mode_switching_does_not_touch_document_history`
(native/Rust) creates one real primitive, records `revision`, cycles
Sketch2D → Creative → Sketch2D three times, and asserts `revision` and
`primitives` are byte-for-byte unchanged, `can_undo` still reflects
only the one real edit, and `undo()` still removes exactly that one
primitive — never a phantom mode-switch entry. Mirrored in a real
browser (`workspace-mode.spec.ts`): draw one stroke, cycle S/B/S three
times, confirm the stroke is still the only geometry and Undo still
removes exactly it.

## Test-first evidence

**Rust** (`cargo test -p craftloop-web-bridge`): 20 passed, 0 failed
(16 from Phase 05/06 + 4 new: mode starts Creative, mode round-trips
through the real Command Bus, history separation, and
`resolveCommand`'s real-grammar/never-guess proof).

**Frontend unit** (`npm run test`): 28 passed, 0 failed (21 from Phase
07 + 4 `isTextInputFocused` tests + 3 new `Toolbar.test.tsx` tests:
Sketch dispatches and shows pressed, Select/Eraser pause in Sketch2D
with Pen staying enabled, Pen click in Sketch2D calls
`onEnterCreativePenMode`).

**Real browser** (`npm run test:e2e`): 15 passed, 0 failed (10 from
Phase 06/07, `sketch` removed from Phase 07's stale "deferred tools"
list since it is real now, + 5 new `workspace-mode.spec.ts` tests). One
real flakiness bug found and fixed by these tests, not by review: fast
tests that pressed a key or clicked a tool immediately after
`main-toolbar` became visible could race the async Wasm module
init/`getCraftLoopSession()` promise under parallel-worker load
(`data-workspace-mode` still showing the pre-init default). Fixed by
exposing a real `data-session-ready` attribute
(`Workspace.tsx`, driven by `useCraftLoopSession`'s own `ready` state)
and waiting for it in every spec's `beforeEach`.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build
npm run test                                                        # 28/28
npm run test:e2e                                                    # 15/15, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All eight tasks have evidence above. Shared semantics remain
platform-neutral: `craftloop-command`'s existing grammar/vocabulary was
read, not modified — the one behavior change (S/B as keyboard aliases)
lives entirely in the web-only `useKeyboardShortcuts` hook, and
`WorkspaceMode` is ephemeral session state following the exact
precedent `selection` already established. No unsupported
native-device claim made; the command simulator's real UI is honestly
left to Phase 15 rather than duplicated early. Continuing automatically
to Phase 09 (Creative Sketch Toolbar), which is where Sketch2D mode
finally gets its own dedicated tool set instead of just pausing
Select/Eraser.
