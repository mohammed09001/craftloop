# Execution 03, Phase 07 — Main Toolbar Top-Center Redesign

Recorded 2026-09-15. Tasks 050-055.

## Scope decision

Task 052's registry names nine tools: Pen, Sketch, Select, Eraser,
View, Undo, Redo, Save, More. Of those, `Sketch` (toolbar morph into
2D Sketch Mode), `View` (Orthographic entry with visible linked-view
rendering), `Save` (browser persistence), and `More` (an overflow menu
with real contents) all depend on infrastructure later phases build
(Sketch Mode: Phase 08-09; Orthographic View Block *rendering*: Phase
13 — the underlying `enterOrthographic`/`assignViewIdentity` session
calls already exist from Phase 04, but calling them today would change
real document state with no visual result to show for it, which is
worse than an honest "not yet"; persistence: Phase 14). Rather than
omit them (an incomplete registry, contradicting Task 052) or fake
their behavior (Article 65), all four are registered and rendered,
disabled, with a tooltip naming the phase that activates them.

Pen, Select, Eraser, Undo, and Redo are fully real this phase: Pen/
Select/Eraser change actual `CanvasStack` interaction behavior (a
natural extension of Phase 06's own drawing/selection code, kept as
local component state rather than reaching into Phase 08's formal
`WorkspaceMode`); Undo/Redo call the real `CraftLoopSession` methods
Phase 04 already exposed.

## Task 050 — Build floating toolbar surface

`src/toolbar/Toolbar.tsx` + `Toolbar.module.css`, rendered inside
`Workspace`'s already-reserved top-center anchor (Phase 02's Task 016).
A compact white pill: `padding: 6px`, buttons `32x32px`, grouped with
thin dividers. The anchor itself stays `pointer-events: none` (Phase
02); the toolbar surface re-enables `pointer-events: auto` for itself
only, so nothing outside its own bounds intercepts canvas input.

## Task 051 — Eliminate bottom-primary-toolbar assumption

There is exactly one toolbar in `apps/web-live` and it renders inside
the top-center anchor — no bottom or side counterpart exists anywhere
in this codebase, unlike the native Android layout Phase 00's audit
found (`MainActivity`'s `Column { InkCanvas; PrimaryToolbar }`, toolbar
below the canvas). Checked with a real-browser test
(`toolbar.spec.ts`): the toolbar's bounding box sits within the top
quarter of the viewport and is horizontally centered.

## Task 052 — Create main tool registry

`src/toolbar/toolRegistry.ts`: a typed, data-only array of all nine
tools with `id`/`label`/`group`/`kind`/optional `deferredUntil`. A
Vitest test (`Toolbar.test.tsx`) asserts every registry entry actually
renders a button.

## Task 053 — Create icon-first buttons

`src/toolbar/icons.tsx`: one minimal inline SVG line icon per tool (no
icon library dependency). Every button's accessible name comes from
`aria-label`/`title`, not visible text — proven directly:
`button.textContent === ''` for every tool, while
`toHaveAccessibleName(tool.label)` still passes (jsdom + testing-library
computing the same accessible-name algorithm a screen reader would).

## Task 054 — Add active/disabled states

Real `aria-pressed` (Pen/Select/Eraser, mutually exclusive) and real
`disabled`/`aria-disabled` (Undo/Redo driven by
`session.snapshot.can_undo`/`can_redo`; the four deferred tools always
disabled) — native button semantics plus a background/border color
change, never color alone. Proven in both Vitest (`aria-pressed`
toggling, `toBeEnabled`/`toBeDisabled` tracking real
`canUndo`/`canRedo` props) and Playwright (Undo starts disabled with no
history, becomes enabled after a real stroke, and clicking it actually
removes the stroke from the structured layer).

## Task 055 — Run canvas-dominance review

Measured, not eyeballed: a real-browser test asserts the toolbar's
height stays under 60px and its width under 60% of the viewport —
nowhere near ribbon territory (a typical desktop-app ribbon runs
80-120px tall and spans the full window width). The canvas fills
everything else; nothing was added that competes with it for space.

## Test-first evidence

**Frontend unit** (`npm run test`): 21 passed, 0 failed (15 from Phase
06 + 6 new `Toolbar.test.tsx` tests: registry completeness, icon-first/
no-visible-text, `aria-pressed` state, click wiring, Undo/Redo
enablement, deferred-tool disabling with an explanatory title).
`Workspace.test.tsx` updated: the toolbar host now contains the real
`main-toolbar`, not an empty placeholder.

**Real browser** (`npm run test:e2e`): 10 passed, 0 failed (4 from
Phase 06 + 6 new `toolbar.spec.ts` tests) —
- top-center positioning and compactness, measured against the real
  viewport;
- Select mode really disables drawing (a drag produces zero strokes)
  and click-to-select still works;
- Eraser really deletes the entity under a click;
- Undo/Redo really gate on real history and really mutate the
  document (a stroke disappears/reappears through real
  `session.undo()`/`redo()`);
- the four deferred tools are really `disabled`, not merely styled to
  look inactive.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build
npm run test                                                        # 21/21
npm run test:e2e                                                    # 10/10, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All six tasks have evidence above, including measured real-browser
proof for the two purely visual requirements (Task 051/055) rather than
a screenshot alone. Shared semantics remain platform-neutral: no
Rust/domain crate changed this phase; every real tool routes through
`CraftLoopSession` methods Phase 04/06 already proved. No unsupported
native-device claim made; the four deferred tools are honestly
disabled with a stated reason rather than faked. Continuing
automatically to Phase 08 (Shared Interaction Mode and Commands),
which is where `Sketch`'s real toggle behavior — and the formal
`WorkspaceMode` this phase's local `activeTool` state anticipates —
gets built.
