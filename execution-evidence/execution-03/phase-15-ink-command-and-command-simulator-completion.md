# Execution 03, Phase 15 — Ink Command and Command-Simulator Completion

Recorded 2026-09-16. Tasks 119-127.

## Task 119 — Command simulator

`workspace/CommandSimulator.tsx` (new): a developer-only panel, opened
from the toolbar's `More` menu (its third real item, alongside Phase
14's New/Open), never presented as a normal end-user input method.
Three real inputs per the task's own wording -- text, source
(implicitly `InkCommand`, Article 15's "simulated ink source" this
whole panel stands in for), and confirmation -- plus a namespace
selector and a developer-only risk override (see Task 124).

## Task 120/121 — S / Sketch enter Sketch2D

Typing the bare letter `s` is treated as the same reserved,
deterministic alias the real `S` keyboard shortcut already is
(`useKeyboardShortcuts.ts`'s own doc comment: "a discrete keypress is
not ambiguous text") -- never run through the grammar. Typing the full
word `sketch` goes through the real `session.resolveCommand`, resolves
`Exact(Sketch)`, and both paths call the exact same
`Workspace`-level `onEnterSketchMode` the toolbar's own Sketch button
calls.

## Task 122/123 — B / Pen enter Creative Pen

Same pattern for the reserved `b` alias. The full-word case is more
interesting than it first looks: `enterCreativePenMode` submits
`CommandAction::ExitSketch` (the real Sketch-namespace word "exit
sketch," Article 237's own vocabulary), not `CommandAction::Pen` --
`Pen` only exists in the Notebook namespace and names a different
real action (selecting the Pen tool while already in Notebook). The
simulator dispatches both `Pen` and `ExitSketch` to the same real
`onEnterCreativePenMode`, since both are genuinely different real
words for reaching the one real semantic outcome this browser harness
can currently produce that way.

## Task 124 — Confirmation policy

New `craftloop-web-bridge` free functions `commandRequiresConfirmation`/
`commandMayExecute` expose `craftloop_command::risk::{requires_confirmation,
may_execute}` unchanged -- not reimplemented. Every action this
harness actually dispatches (`Sketch`, `ExitSketch`) is real `Low`
risk and never requires confirmation (`low_risk_command`'s own
existing, unmodified behavior), so demonstrating Medium/High gating
needed an explicit, clearly-labeled developer risk override in the
simulator UI rather than inventing a risk level for an action that has
none in this codebase (`risk` has always been caller-chosen here, per
`low_risk_command`'s own doc comment, never action-intrinsic). With
the override set to Medium and confirmation set to `RemainsInk`, the
real policy rejects the command and the real workspace mode does not
change; set to `Execute`, the same real policy allows it and the mode
changes for real.

## Task 125 — Ambiguity

Exercised through a real grammar collision outside the two reserved
aliases: `o` matches both real Notebook words `orthographic` and its
curated synonym `ortho` (Article 237's own synonym example) -- two
distinct words, so a real `Ambiguous` result even though both name the
same action. The simulator shows both candidates and executes
nothing; the real workspace mode is provably unchanged afterward.

## Task 126 — Toolbar parity

Not validated by comparing two independent implementations -- there is
only one. `CommandSimulator` calls the identical `onEnterSketchMode`/
`onEnterCreativePenMode` closures `Workspace` passes to `Toolbar`'s own
Sketch/Pen buttons; a resolved action always produces the same real
`session.enterSketchMode()`/`enterCreativePenMode()` call, `activeTool`
reset included, because it is the same function reference, not a
parallel code path that merely intends to match.

## Task 127 — Handwriting deferral, documented

`CommandSimulator.tsx`'s own doc comment and its on-screen copy both
state plainly that this panel starts from already-typed text and
performs no real handwriting recognition -- the same boundary
`resolve_command`'s own Rust doc comment already draws ("already-
recognized, normalized-by-the-caller text"). No claim of native
on-device handwriting is made anywhere in this phase's code or UI.

## Test-first evidence

**Backend** (`cargo test --workspace`): all green, plus one new
`craftloop-web-bridge` test,
`command_confirmation_gate_matches_the_real_risk_policy`, proving the
exposed free functions agree with the real, pre-existing `risk.rs`
policy at Low/Medium/High (24 web-bridge tests total).

**Frontend unit** (`npm run test`): 61 passed (60 at Phase 14's close
+ 1 new `Toolbar.test.tsx` test for the Command Simulator menu item).

**Real browser** (`npm run test:e2e`): 37 passed, 0 failed (32 carried
over + 5 new `command-simulator.spec.ts` tests): reserved `s`/`b` and
the real grammar words `sketch`/`exit sketch` all reach the real
correct mode; a real ambiguous prefix (`o`) is named and executes
nothing; a Medium-risk override with insufficient confirmation is
rejected and Execute clears it; the resolved command's observable
consequence (the toolbar's own real button set for the new mode) is
identical to clicking the toolbar directly.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 61/61
npm run test:e2e                                                    # 37/37, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All nine tasks have evidence above. Backend invariant held: the only
new engine-facing surface (`commandRequiresConfirmation`/
`commandMayExecute`) is a direct, unmodified pass-through to existing,
already-tested pure functions -- no new state, no duplicated policy.
UI/UX invariant held: the simulator is explicitly developer-only,
tucked in the overflow menu rather than the primary toolbar, so it
never competes with Creative Precision's canvas-dominant surface.
Universal invariant held: every new file is plain React in
`apps/web-live`. Continuing automatically to Phase 16 (Creative UX
Hardening).
