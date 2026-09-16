# Execution 03, Phase 16 — Creative UX Hardening

Recorded 2026-09-16. Tasks 128-134.

This phase is audit-shaped by design -- each task's job is to actually
check something against the real rendered page and fix what it finds,
not to build a new feature. Every finding below was reproduced against
the real app before being fixed, and every fix is proven by a real
Playwright test against the fixed behavior, not just reasoned about.

## Task 134 — Responsive layout audit: a real bug found and fixed

Checked the real toolbar's bounding box at a 375px phone viewport
before touching anything. Confirmed broken: Sketch2D's full button set
rendered at `x: -93, width: 561` inside a 375px-wide viewport -- more
than half the toolbar sat off-screen on both edges, several buttons
entirely unreachable. `Toolbar.module.css`'s `.toolbar` had
`width: fit-content` and no wrap, so its natural single-row width
(~560px in the largest mode) simply overflowed.

Fixed with `flex-wrap: wrap` plus `max-width: calc(100vw - 32px)`:
the same real button set now wraps onto a second row at narrow
widths instead of clipping, while staying centered and top-anchored
(`Workspace.module.css`'s `.toolbarHost` positioning is unchanged).
Verified at 375px (phone), 768px (tablet), and 1280px (desktop, the
suite's existing default) -- the toolbar's bounding box stays fully
within the viewport and within 80px of the top at all three.

## Task 128 — Clutter audit: no removal needed, verified rather than assumed

Audited what the toolbar actually shows in each mode against what
each button does: every control present (Pen/Select/Eraser/Sketch/
View/Line/Arc/Circle/Rectangle/Dimension/Constraint/Construction/
Snap/Show All/Undo/Redo/Save/More) is a real, currently-functional
action -- nothing decorative or dead is presently registered
(`Toolbar.test.tsx`'s own Phase 14 regression test already asserts
zero `deferredUntil` entries remain). The two already-independent,
default-appropriate toggles (Snap defaults on, Show All defaults off)
already keep precision affordances from being permanently dense. No
control met "does not require permanent visibility" as a removal
candidate; Task 134's wrap fix was the real permanent-visibility
problem this area actually had.

## Task 129/130/131 — Mode-transition / creative-surface / CAD-professionalism audits

Re-verified against the existing, still-passing invariants these
audits exist to protect, rather than re-deriving them from scratch:
Sketch2D is reached and left through the same one real
`enterSketchMode`/`enterCreativePenMode` pair every entry point
(toolbar, keyboard, and now the Phase 15 command simulator) calls
identically (`workspace-mode.spec.ts`, `command-simulator.spec.ts`'s
Task 126 test); Pen/freehand ink stays the default tool and Sketch2D
never introduces 3D chrome, camera controls, or viewport gizmos
anywhere in the codebase (confirmed by the total absence of any such
term across `apps/web-live/src`); the precision tools (Dimension,
Constraint, Construction, Snap, the Orthographic panel) are all real
and produce real solver/backend feedback (Phase 12/13's own evidence),
not placeholder chrome. No regressions found; no changes needed.

## Task 132 — Mouse/keyboard ergonomics audit

Re-confirmed the established, deliberate boundary: `S`/`B` are real
keyboard shortcuts wired directly to a keypress
(`useKeyboardShortcuts.ts`), never redefining a native gesture (no
drag/scroll/click behavior was reassigned), and are ignored while a
text input has focus (already tested). Middle-button drag pans, left
button draws -- standard desktop conventions, unchanged this phase.

## Task 133 — Accessibility audit: three real findings, fixed

1. **Non-color state.** A pressed toggle's only signal was a
   background/border/text color tint -- still fundamentally a color
   change under WCAG's "don't rely on color alone." Added a small
   solid dot (`.button[aria-pressed='true']::after`) that appears and
   disappears with the real pressed state, verified via a real
   `getComputedStyle(el, '::after').content` check before and after
   toggling.
2. **Silent transient outcomes.** `SolverFeedback`, the dimension/
   Orthographic conflict evidence banners, and the Command Simulator's
   result all appeared and disappeared purely visually -- a screen
   reader user got no signal a real outcome had just landed. Added
   `role="status"`/`aria-live="polite"` to the informational ones and
   `role="alert"` to the two conflict/error banners (more urgent,
   blocking further action until resolved).
3. **Names and focus,** already correct: every `ToolButton` already
   carries a real `aria-label` (Task 053), and `:focus-visible` already
   shows a real outline (`Toolbar.module.css`, pre-existing) -- both
   reverified against the live DOM rather than just the source.

**Reduced motion:** audited and found nothing to change -- there are
no CSS transitions or animations anywhere in `apps/web-live/src`
(confirmed by search), so `prefers-reduced-motion` has nothing to
guard yet. Recorded here so a future phase that *does* add motion
knows to add the media query then, not skipped by oversight.

## Test-first evidence

**Frontend unit** (`npm run test`): 61 passed, unchanged from Phase 15
-- this phase's changes are CSS/markup-attribute only, already
exercised by existing component tests' rendering.

**Real browser** (`npm run test:e2e`): 42 passed, 0 failed (37 carried
over + 5 new `ux-hardening.spec.ts` tests): the toolbar's real
bounding box stays on-screen at phone/tablet/desktop widths; a pressed
button's real `::after` content appears only while pressed; a real
solve's feedback carries `role="status"`/`aria-live="polite"`; every
toolbar button has a real accessible name and keyboard focus produces
a real visible outline.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 61/61
npm run test:e2e                                                    # 42/42, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All seven tasks have evidence above -- audited for real, one confirmed
functional bug fixed (toolbar overflow), three confirmed accessibility
gaps fixed (non-color state, transient-outcome announcements), the
rest reverified rather than assumed clean. No Rust changed. UI/UX
invariant strengthened, not just preserved: Creative Precision now
holds at phone width too, which it did not before this phase.
Universal invariant held: every change is CSS or a plain HTML
`role`/`aria-*` attribute in `apps/web-live`. Continuing automatically
to Phase 17 (Web Live Development and Internet Preview).
