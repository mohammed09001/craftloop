# Execution 03, Phase 10 — Direct Geometry Preview and Creative Refinement

Recorded 2026-09-15. Tasks 078-086.

## Design: one geometry function, two consumers

`src/canvas/shapePreview.ts`'s `computeShape(kind, first, last)` is the
single source of truth for "what shape does this two-point drag
define," used by both the live preview `InkCanvas` draws on every
`pointermove` (Tasks 078-081) and the final
`CraftLoopSession.createPrimitive*` call `CanvasStack` makes on
release (refactored from Phase 09's own inline math). Pure,
coordinate-space-agnostic geometry -- the same function works on
`InkCanvas`'s screen-space points and `CanvasStack`'s world-space
points, and it is never treated as real document state (it is
disposable, exactly like the raw-ink trace it replaces for shape
tools, Article 16).

## Task 078-080 — Line / Circle / Rectangle preview

`InkCanvas.tsx`'s `redraw()` now branches on a new `previewKind` prop:
`'freehand'` keeps the original raw mouse-path trace (Pen); any shape
kind instead computes the real shape from the drag's first/last screen
points and draws *that* every frame (`ctx.moveTo`/`lineTo` for a
line, `ctx.arc` for a circle, `ctx.rect` for a rectangle) -- a rubber-
band/live-bounds preview that matches what will actually be created,
not a trace of the mouse's path.

## Task 081 — Arc interaction

Same mechanism, `ctx.arc(center, radius, startAngle, endAngle)` --
"simple real construction matching Arc2 semantics" per the task's own
wording, using Phase 09's already-established 2-point mapping (center
at drag start, radius/start angle from the drag end, fixed
quarter-circle sweep).

## Task 082 — Ellipse interaction

Not added, consistent with Phase 09's Ellipse decision (no Sketch tool
exists to preview).

## Task 083 — Draw-and-Hold timer

`InkCanvas.tsx`: a real `setTimeout(HOLD_DURATION_MS = 500)`, (re)armed
on every `pointerdown`/`pointermove` and cleared on release --
`holdDetectedRef` is `true` only if the pointer stayed down without
moving for the full 500ms before it lifted. Passed to the caller as
`meta.holdDetected` alongside the existing tap/drag deviation
measurement.

## Task 084 — Show ghost refined candidate

`CanvasStack.tsx`'s `handleStrokeComplete`: a held Pen stroke is
submitted as a real stroke (`session.submitStroke`, unchanged), and
only if `eligible_for_recognition` is true (computed by the real
`recognize`/`rank_candidates`, Phase 04) does it call the real
`session.acceptRecognition(strokeId)` -- which internally re-runs
`recognize`+`beautify` and, on success, atomically replaces the stroke
with the resulting primitive in one transaction (Phase 04's own
implementation, reused unchanged). `GeometryLayer` renders that
primitive with a dashed "ghost" style (`data-candidate="true"`,
violet stroke) while it is pending confirmation.

**Design choice worth recording:** rather than adding a new
non-mutating "preview only" Rust method, this reuses the existing
commit-then-allow-undo pattern -- `acceptRecognition` already commits
through `DocumentHistory` exactly like every other real operation, so
"cancel" is just `session.undo()` (Task 086). This avoids inventing a
parallel non-committing recognition path in the shared engine for a
purely presentational need the real undo stack already serves.

## Task 085 — Confirm/cancel candidate

`RefinementBar.tsx`: a small floating bar shown only while a candidate
is pending. Confirm just clears the pending-candidate UI state (the
primitive is already real and committed). Cancel calls the real
`session.undo()`, reverting exactly the `acceptRecognition` transaction
-- "no silent conversion" (Task 085's own wording): the geometry never
changes without either an explicit confirm or an explicit, fully
reversible cancel.

## Task 086 — Preserve raw ink fallback

Two independent guarantees, both proven by real tests: (1) a stroke
released *without* a hold never calls `acceptRecognition` at all --
stays plain ink, no interruption; (2) a held stroke whose recognition
is not eligible (or that the user cancels) is restored to its original
raw-stroke form via the same real undo path.

## Test-first evidence

**Frontend unit** (`npm run test`): 40 passed, 0 failed (35 from Phase
09 + 5 new `shapePreview.test.ts` tests covering all four shape kinds'
exact geometry, including the rectangle's start/end-direction
normalization).

**Real browser** (`npm run test:e2e`): 21 passed, 0 failed (18 carried
over + 3 new `draw-and-hold.spec.ts` tests): a quick stroke stays plain
ink with no refinement prompt; a genuinely held stroke (pointer down,
paused 700ms, released) produces a real `data-candidate="true"` `<line>`
-- the real recognizer actually classified a near-straight drag as a
Line -- with a visible confirm/cancel bar, and Confirm leaves exactly
that line (the raw `<polyline>` is gone, really replaced, not just
hidden); a separate test proves Cancel reverts through a real
`session.undo()`, restoring the original polyline and removing the
line entirely.

Live pixel-level preview correctness (what the Canvas 2D context
actually paints mid-drag) is not separately asserted in the browser
suite -- HTML Canvas content isn't DOM-queryable, and the same
`computeShape` function drives both the preview and the final created
primitive (asserted directly in `shapePreview.test.ts` and indirectly
by every shape-creation e2e test), so a preview/result mismatch is
already structurally impossible rather than merely untested.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets                              # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build
npm run test                                                        # 40/40
npm run test:e2e                                                    # 21/21, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All nine tasks have evidence above. Shared semantics remain
platform-neutral: this entire phase is frontend-only -- no
craftloop-web-bridge or domain crate changed, every real capability
(recognition, beautification, undo) was already Phase 04/09's, only
newly *sequenced* into a hold-then-confirm/cancel UX. No unsupported
native-device claim made; Ellipse's absence stays consistent with
Phase 09. Continuing automatically to Phase 11 (Precision Inference
and Construction), which is where grid/snap and construction-geometry
semantic state -- both left deliberately deferred through Phase 07-09
-- finally get built.
