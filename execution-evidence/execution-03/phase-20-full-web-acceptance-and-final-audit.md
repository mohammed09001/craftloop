# Execution 03, Phase 20 — Full Web Acceptance and Final Audit

Recorded 2026-09-16. Tasks 152-162. This is Execution 03's final phase
and this document is Task 162's final report.

## Tasks 152-157 — The Golden Web Journey, run as one continuous session

New `tests/e2e/golden-journey.spec.ts` runs Article 83's entire 32-step
"Golden Web Journey" (steps 1-30; 31-32 are Task 158's own job) as
**one continuous real Playwright session** -- every other e2e spec in
this project proves one feature against a fresh page
(`beforeEach`); this is the one test proving every phase's real
capability composes correctly with every other in the same document,
the way an actual user's session would unfold, not just in isolation.

Concretely, in one session: opens the app; confirms the toolbar is
real top-center; draws real freehand ink; presses `S`; creates a real
Line, Circle, Rectangle, and Arc (confirms Ellipse is genuinely absent,
not silently skipped); creates and selects toward a real dimension and
a real constraint (with real solver feedback); presses `B` back to
Creative with Pen active; triggers `Sketch` through the real Command
Simulator and confirms the identical real state; assigns Front and
enters Orthographic from real selected geometry; confirms Top and
Right appear with Depth genuinely unresolved; enters Depth 40 in Top
and confirms Right receives it; enters a contradictory 999 in Right
and confirms a real conflict; resolves it; Undoes (Top/Right revert to
40) and Redoes (back to 999) through the real shared transaction
history; Saves; reloads the actual browser page; and confirms every
one of the above -- the freehand stroke, all four Sketch2D primitives,
and the full real Orthographic/shared-axis state -- survives.

**Real bug found and fixed while writing this test:** the test
originally clicked Save and reloaded immediately. `saveNow` is async
(a real IndexedDB write); reloading before it resolved raced a real,
if narrow, data-loss window. Fixed the test to wait for the Save
button's own real `lastSavedAt`-derived title update before reloading
-- the same signal a real user would see -- rather than assuming an
instant write. This is a genuine characteristic of explicit Save
worth knowing (autosave, proven separately in
`persistence.spec.ts`, remains the primary safety net for the "close
without ever clicking Save" case), not a regression introduced this
phase.

## Task 158 — Preview journey: `BLOCKED_BY_HUMAN_AUTHORIZATION`

Per Article 82 Gate P's own instruction: "If provider authorization is
the only missing step, record `BLOCKED_BY_HUMAN_AUTHORIZATION`." That
is exactly this repository's state -- `npm run build` produces a real,
deployable static site (Phase 17, reverified below), and
`tests/e2e/preview-smoke.spec.ts` is real, ready infrastructure that
repeats the same core smoke flow against a real deployed URL the
moment one exists (`PREVIEW_URL=<url> npx playwright test
tests/e2e/preview-smoke.spec.ts`) -- but provisioning that real URL
needs Vercel/Cloudflare Pages account credentials this environment
does not have. **Status: `BLOCKED_BY_HUMAN_AUTHORIZATION`.**

## Task 159 — No-hallucination audit

Searched every `execution-evidence/execution-03/*.md` file for
native-device claims not clearly framed as deferred/absent
(`grep -rniE "physical (device|tablet)|real S Pen|on-device|real
device"`, then excluded the honest-disclaimer hits by hand). Two
matches, both confirmed to be explicit *non*-claims on inspection
("No claim of native on-device handwriting is made anywhere in this
phase's code or UI" -- Phase 15; "the manual validation this
compile-only gate deliberately does not attempt itself" -- Phase 18).
A second pass for confident-sounding confirmation phrasing
(`"confirmed on"`, `"verified on"`, `"tested on a real"`, `"works on a
real"`) found zero matches anywhere in this execution's evidence.
**No hallucinated native-device claim found.**

## Task 160 — Architecture audit

Two real, scriptable checks against every crate *except*
`craftloop-web-bridge`/`craftloop-mobile-ffi` (the two deliberately
platform-specific bridge crates):

1. `grep` every `.rs` file in the other 20 crates for
   `wasm_bindgen|web_sys|js_sys|JsValue|HtmlCanvasElement|PointerEvent|winit|glutin`
   -- the only hits were `craftloop_input::PointerEvent`, a real,
   platform-neutral domain enum (`Down`/`Move`/`Up`/`Cancel`/`Hover`/
   `CaptureLost`) defined *inside* that shared crate itself, deliberately
   named to mirror the browser's own `PointerEvent` concept (Article
   15) without being it -- not a real leak.
2. `grep` every shared crate's `Cargo.toml` for a
   `wasm-bindgen`/`web-sys`/`js-sys`/`winit`/`egui`/`react`/`uniffi`
   dependency -- zero matches. No shared crate depends on a browser,
   native-UI, or FFI-binding-generation crate at all.

**No React/DOM/browser type leakage into any platform-neutral crate.**

## Task 161 — Design audit

Phase 16 already ran the substantive audit (clutter, mode-transition,
creative-surface, CAD-professionalism, ergonomics, accessibility,
responsive layout) and fixed the two real findings it turned up (a
phone-width toolbar overflow, two accessibility gaps). This phase
reverifies rather than re-litigates: no web UI changed between Phase
16 and this one (Phases 17-19 touched dev tooling, CI, and Android
only), and the Golden Web Journey's own step 2 independently
re-confirms the toolbar's real bounding box is centered and near the
top at the suite's default viewport, on top of `ux-hardening.spec.ts`'s
dedicated phone/tablet/desktop coverage. **Creative Precision holds.**

## Task 162 — Final report: Article 82 Gate A-Q

| Gate | What it requires | Status | Evidence |
|---|---|---|---|
| A — Local Live View | One command starts frontend+Wasm; Rust rebuilds without stale semantics | **PASS** | Phase 17: `npm run dev:live`, real break/recover cycle verified |
| B — Real Shared Core | No fake TS geometry model; browser calls the real Rust core | **PASS** | Every phase's e2e suite; Golden Web Journey |
| C — Top-Center Main Toolbar | Visibly top-center, icon-first, canvas dominates | **PASS** | Phase 07/16; `toolbar.spec.ts`, `ux-hardening.spec.ts` |
| D — Sketch Entry | Icon/S/Sketch (incl. command simulator) all enter | **PASS** | Golden Web Journey steps 4-5, 15-16; `command-simulator.spec.ts` |
| E — Creative Return | Pen/B/Pen(sim) exit; document unchanged; Pen active | **PASS** | Golden Web Journey steps 13-14; `workspace-mode.spec.ts` |
| F — Sketch Geometry | All exposed core-supported entities create real domain state | **PASS** | Golden Web Journey steps 6-9 (Ellipse confirmed absent, deliberate) |
| G — Pen + Refinement | Freehand; Draw-and-Hold; Cancel keeps ink; Confirm commits geometry | **PASS** | Golden Web Journey step 3; `draw-and-hold.spec.ts` (Phase 10) |
| H — Dimensions | Create/edit; invalid input -> real conflict/rejection | **PASS** | Golden Web Journey step 11; `dimensions-and-constraints.spec.ts` (Phase 12) |
| I — Constraints | Apply+solve through real backend; no silent corruption | **PASS** | Golden Web Journey step 12; Phase 12 (redundancy detection, real conflicts) |
| J — Orthographic | Assign FRONT, enter Orthographic, real linked TOP/RIGHT | **PASS** | Golden Web Journey steps 17-19; `orthographic-views.spec.ts` (Phase 13) |
| K — Unresolved Depth | Stays unresolved until the user supplies it | **PASS** | Golden Web Journey step 20 |
| L — Shared Propagation | TOP->RIGHT; no permanent master view | **PASS** | Golden Web Journey steps 21-22; Phase 13 (entered from Top, not Front) |
| M — Cross-View Conflict | Contradiction -> real conflict, valid resolutions offered | **PASS** | Golden Web Journey steps 23-25 |
| N — Undo/Redo | Shared transaction history, across geometry/dimensions/constraints/Orthographic | **PASS** | Golden Web Journey steps 26-27; `toolbar.spec.ts` |
| O — Web Persistence | Save, reload, recover same semantic document | **PASS** | Golden Web Journey steps 28-30; `persistence.spec.ts` (Phase 14) |
| P — Internet Preview | Static build + real deployed preview + smoke journey there | **`BLOCKED_BY_HUMAN_AUTHORIZATION`** | Phase 17/20: build real, deploy needs real hosting credentials |
| Q — Android Compile Regression | Debug build stays compile-verified; no physical-device claim | **PASS** | Phase 19: clean `assembleDebug`+`lintDebug`, real APK, no device |

**16 of 17 gates PASS. One (P) is `BLOCKED_BY_HUMAN_AUTHORIZATION`,**
**exactly as Article 82 itself defines that outcome** -- every
prerequisite this repository controls (a real, tested static build) is
done; the one remaining step is a human providing real hosting
credentials this environment cannot have.

### Known, deliberate limitations (not gate failures)

- **Ellipse**: not exposed. `craftloop-geometry::Ellipse2` exists and
  is tested, but `BeautifiedPrimitive` (the type every primitive
  actually flows through) has exactly four variants; adding a fifth is
  a real, invasive change across seven-plus crates including native
  `craftloop-mobile-ffi` -- disproportionate to "add a toolbar button"
  (Phase 09's own reasoning, reconfirmed here, still current).
- **Native device capabilities** (S Pen gestures, real pressure/tilt
  curves, palm rejection beyond the OS's own, ML Kit recognition):
  deliberately deferred on Android, unchanged and reconfirmed this
  phase (Phase 19, Task 151).
- **Native/web feature gap**: every Execution 03 web-only capability
  from Phases 09-17 of *this* execution (Arc, Construction, Snap,
  annotation rendering, the Orthographic panel's real UI, persistence,
  the Command Simulator) has no Android counterpart yet -- found and
  recorded honestly in Phase 19 rather than left implicit.

## Test-first evidence

**Frontend unit** (`npm run test`): 61 passed, unchanged.

**Real browser** (`npm run test:e2e`): 44 tests, 43 passed + 1
correctly-skipped (`preview-smoke.spec.ts`, gated on `PREVIEW_URL`) --
0 failed. New this phase: the full 30-step Golden Web Journey.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 61/61
npm run test:e2e                                                    # 43/44 passed, 1 correctly skipped
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED. Execution 03 — COMPLETE.

All eleven tasks (152-162) have evidence above. All twenty phases of
Execution 03 (Web Live View / Creative Sketch Mode) are now real,
tested, and committed. The one open item, Gate P's internet preview,
is correctly and honestly recorded as `BLOCKED_BY_HUMAN_AUTHORIZATION`
per Article 82's own defined outcome for that exact situation -- not a
failure of this execution's own work, and not fabricated. Backend/UI-
UX/Universal invariants held throughout this phase (no source changed
beyond the one new e2e spec) and are reconfirmed, not merely restated,
by this phase's two independent audits (Tasks 159/160) and the Golden
Web Journey's own real, passing, end-to-end proof.
