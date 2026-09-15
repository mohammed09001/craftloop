# Execution 03, Phase 06 — Browser Canvas Stack

Recorded 2026-09-15. Tasks 043-049.

## Wiring the Wasm bridge into apps/web-live (prerequisite)

Phases 03-05 built and proved `craftloop-web-bridge` in isolation
(throwaway HTML harnesses under a scratchpad, cleaned up afterward).
This phase wires it into the real app for the first time.

Added `scripts/build-wasm-bridge.mjs`: `cargo build --target
wasm32-unknown-unknown -p craftloop-web-bridge` then `wasm-bindgen
--target web` into `apps/web-live/src/wasm-bridge/` (gitignored,
generated). Wired via npm's automatic `pre<script>` hooks —
`predev`/`predev:live` run the fast debug build, `prebuild` (and
`pretest:e2e`, which now runs the full `npm run build`) runs
`--release`: the dev-profile `.wasm` is ~12MB; the release one is
1.8MB (468KB gzipped) — the difference between a workable static
internet preview (Article 14) and one that isn't. `wasm-bindgen`'s
generated `web` target loads the `.wasm` via `new URL(...,
import.meta.url)` + `fetch`, which Vite bundles/serves natively with no
extra plugin.

`src/session/craftLoopSession.ts` holds one module-level
`CraftLoopSession` singleton per tab (Article 8: one shared document is
the one source of truth); `useCraftLoopSession.ts` is the React hook
every component uses, exposing the real scene snapshot as state
(refreshed after every mutating call) and never computing or inventing
engineering state itself.

## Task 043 — Implement viewport transform

`src/canvas/viewport.ts`: pure functions only (`worldToScreen`,
`screenToWorld`, `cssTransform`, `panBy`, `zoomAt`, `fitToBounds`), no
DOM/React. `BackgroundGrid`, `GeometryLayer`, and `SelectionOverlay` all
apply the exact same `cssTransform(viewport)` to their root `<g>`
(Article 16: "All geometry layers share one viewport transform").
6 Vitest tests, including a round-trip proof and a "zoom keeps the
point under the cursor fixed" proof.

## Task 044 — Implement transient ink Canvas

`src/canvas/InkCanvas.tsx`: an HTML `<canvas>` capturing every pointer
sample in screen space (cheap, no transform math on the per-move hot
path) and redrawing the in-progress polyline immediately on every
`pointermove`. Real browser proof: a Playwright test drags the mouse
across the canvas and confirms a polyline with real points appears in
the structured layer afterward — not a mock.

## Task 045 — Implement structured SVG layer

`src/canvas/GeometryLayer.tsx`: renders every real primitive
(Line/Circle/Arc/Rectangle, from `WebPrimitiveSummary.geometry`) and
every real stroke (from `WebStrokeSummary.points`) as actual SVG
shapes. Required one more small, honest addition to the shared
snapshot: `WebStrokeSummary` didn't carry point data before this phase
(Phase 05 only added `sample_count`) — without it, a submitted stroke
would visibly vanish the instant `InkCanvas` stopped drawing it, since
nothing would remain to render it. Added `points: Vec<Point2>` to
`WebStrokeSummary` (`types.rs`) and populated it in `scene_snapshot`
(`session.rs`), plus a new native test,
`submitted_stroke_carries_its_real_points_in_the_snapshot` (`cargo test
-p craftloop-web-bridge`: 16/16 including it).

Dimension/constraint *annotation* rendering (leader lines, value
labels, badges) is deliberately out of scope — Phase 12 (Dimensions and
Adaptive Constraints UX) owns that presentation layer; this phase
renders confirmed geometry only.

## Task 046 — Implement Pointer Events adapter

Same `InkCanvas.tsx`. Every sample preserves `pointerType` (mapped to
`SimulatedMouse`/`Stylus`/`Touch`), position, timestamp, and real
pressure/tilt *only* when `pointerType === 'pen'` (a mouse's fixed
`pressure = 0.5`/`tilt = 0` are not real data, so `capability_pressure`/
`capability_tilt` are `false` for mouse input). `button_primary`/
`_secondary`/`_barrel` come from `event.buttons`.
`capability_palm_rejection`/`capability_eraser` are hardcoded `false`
always: the Pointer Events API has no way to actually query either
from JS, so `false` is the honest answer, not a fabricated capability
(Article 65, matching native `FfiPointerSample`'s own doc comment on
this exact point).

## Task 047 — Implement pan/zoom

`src/canvas/usePanZoom.ts`: wheel zooms toward the cursor (real
`zoomAt`, Task 043); a middle-button drag pans. Left-button stays
reserved for drawing so panning never fights the primary "mouse
simulates pen" interaction. Neither ever calls a `CraftLoopSession`
method — viewport-only, proven by inspection (no session reference
exists in `usePanZoom.ts`) and by the real-browser test asserting only
the SVG group's `style` transform changes.

## Task 048 — Implement semantic hit testing

`src/canvas/hitTest.ts`: pure functions testing a world-space point
against real primitive/stroke geometry (point-to-segment distance for
lines/stroke polylines and rectangle edges, ring distance + angle range
for circles/arcs), returning the real stable `id` a hit entity's
`scene_snapshot()` itself uses — never a synthesized id. 5 Vitest
tests. Wired into `CanvasStack.tsx`: a short "tap" gesture (pointer
travel under 4 screen px) hit-tests and calls the real
`session.select()`/`clearSelection()`; a real drag submits a stroke
instead. This distinction is necessary because no explicit tool-mode
switch exists yet (Phase 08's `WorkspaceMode`) — found and fixed a real
bug here: a true zero-movement click produces only one pointer sample
(no `pointermove` fires between down and up), and an early `samples.length
< 2` guard in `InkCanvas` was silently dropping that case entirely,
so a plain click never reached the selection path at all. Caught by
the Playwright selection test, not by review.

## Task 049 — Implement selection handles

`src/canvas/SelectionOverlay.tsx`: a dashed bounding-box outline plus
four corner dots per selected *primitive*, computed from the real
`min_x`/`min_y`/`max_x`/`max_y` the snapshot already carries — no new
geometry computed. Selected *strokes* don't get a handle overlay (the
snapshot carries no bounds for a stroke, only its raw points) but do
get a real visual signal: `GeometryLayer` renders a selected stroke's
polyline in the selection color. Recorded as a deliberate, honest scope
line rather than fabricating stroke bounds.

## Test-first evidence

**Rust** (`cargo test -p craftloop-web-bridge`): 16 passed, 0 failed
(15 from Phase 05 + the new stroke-points test).

**Frontend unit** (`npm run test` in `apps/web-live`): 15 passed, 0
failed — 6 `viewport.ts` tests, 5 `hitTest.ts` tests, 4 `Workspace`
component tests (updated for the real canvas stack: canvas-stack/
background-grid/geometry-layer/ink-canvas all present; toolbar-host
still empty per Task 016; no fake geometry renders before the real
session resolves).

**Real browser** (`npm run test:e2e`, Playwright/Chromium against a
real production build): 4 passed, 0 failed —
- a real mouse drag produces a real `<polyline>` with real points in
  the structured layer;
- a real tap selects the drawn stroke (color changes to the selection
  color) and a tap on empty space clears it, driven entirely through
  `hitTestScene` + real `session.select`/`clearSelection` calls (this
  is the test that caught Task 048's real bug above);
- a real wheel event changes the shared viewport `scale(...)` transform
  and a real middle-button drag changes it again (pan), proving every
  layer really does share one transform.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cargo test -p craftloop-web-bridge                                  # 16/16
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build, includes prebuild --release wasm
npm run test                                                        # 15/15
npm run test:e2e                                                    # 4/4, real Chromium
npm run lint                                                        # oxlint clean (after fixing one real ref-during-render warning)
```

## Phase Gate — CLOSED

All seven tasks have evidence above, including real-browser interactive
proof, not just compiler/test-runner success. Shared semantics remain
platform-neutral: the one shared-crate change (`WebStrokeSummary.points`)
is additive read data, not a new mutation path. No unsupported
native-device claim made; two honest, stated scope limits (no dimension/
constraint annotation rendering yet, no bounding-box handles for
strokes) rather than papered over. Continuing automatically to Phase 07
(Main Toolbar Top-Center Redesign).
