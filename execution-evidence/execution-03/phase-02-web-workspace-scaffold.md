# Execution 03, Phase 02 — Web Workspace Scaffold

Recorded 2026-09-15. Tasks 012-017.

## Task 012 — Create apps/web-live

Scaffolded with `npm create vite@latest . -- --template react-ts` in
`apps/web-live/` (React 19.2.8, Vite 8.3.0, TypeScript ~6.0.2). Removed
the template's demo content (`App.css`, `assets/{hero.png,react.svg,
vite.svg}`, `public/icons.svg`, the counter/docs/social sections in
`App.tsx`) — Task 015's "no fake engineering objects" rule applies to
demo boilerplate too. `apps/README.md` updated to list `web-live` as a
member alongside `windows-harness`, with the same "test harness, not a
parallel engineering model" rule Execution 01 Phase 01 Task 007
established for `windows-harness`.

## Task 013 — Configure strict TypeScript

`create-vite`'s current template ships `noUnusedLocals`/
`noUnusedParameters`/etc. but not `strict`. Added explicitly to both
`tsconfig.app.json` and `tsconfig.node.json`: `strict`, `noImplicitAny`,
`noImplicitOverride`, `noImplicitReturns`, `noUncheckedIndexedAccess`,
`exactOptionalPropertyTypes`. `npm run build` (which runs `tsc -b`
first) is green under this configuration — see Task 017 verification.

## Task 014 — Create scripts

`apps/web-live/package.json` scripts: `dev`, `dev:live`, `build`,
`test`, `test:watch`, `test:e2e`, `preview`, `lint`. Added
`scripts/web-live.ps1` at the repo root (Article 64's recommended
location) as the one-command launcher; it installs npm dependencies on
a clean checkout, then runs `npm run dev:live`.

**Honest scope note (Article 65, no-hallucination):** `dev:live` is
currently identical to `dev` — there is no `craftloop-web-bridge` crate
yet for it to watch/rebuild (that's Phase 03), so the Wasm-watch
behavior Article 13 describes does not exist yet. Both the script's doc
comment and `apps/web-live/README.md` say so explicitly. This gets
real Rust-watch behavior in Phase 17 (Article 62).

## Task 015 — Create full-canvas shell

`src/workspace/Workspace.tsx` + `Workspace.module.css`: a `position:
absolute; inset: 0` canvas area filling the viewport, empty of any
content. `App.tsx` now renders only `<Workspace />`. `index.css`
replaced the template's centered-column marketing-page CSS with a
plain full-viewport reset (`html, body, #root { height: 100%; margin:
0 }`).

## Task 016 — Create top-center toolbar host

Same `Workspace.tsx`, a second `div` (`data-testid="toolbar-host"`)
positioned `top: 16px; left: 50%; transform: translateX(-50%)` —
top-center per Creative Precision rule 2. It is empty and
`pointer-events: none` until Phase 07 renders the real toolbar into it;
given `min-height: 48px` so it occupies real, testable screen space as
a reserved anchor rather than a zero-size placeholder.

## Task 017 — Set up tests

Added Vitest (`vitest`, `@vitest/ui`, `jsdom`, `@testing-library/react`,
`@testing-library/jest-dom`) configured in `vite.config.ts`'s `test`
block, with `src/setupTests.ts` registering `@testing-library/jest-dom`
matchers and an explicit `afterEach(cleanup)` (needed because
`test.globals` is left `false` — RTL's own auto-cleanup only
self-registers when a global `afterEach` exists).

Added Playwright (`@playwright/test`, chromium browser installed via
`npx playwright install chromium`), configured in
`playwright.config.ts` to build+serve `dist/` via `npm run preview` on
port 4173 and run `tests/e2e/smoke.spec.ts`.

Smoke coverage, both semantic (not just a screenshot, per the
task-card rule):
- Vitest (`src/workspace/Workspace.test.tsx`): canvas area and toolbar
  host both render and are both empty (no fake engineering objects).
- Playwright (`tests/e2e/smoke.spec.ts`): the real page title contains
  "Craft Loop" and both anchors are visible in an actual browser.

## Verification

Run from `apps/web-live/`:

```powershell
npm run build     # tsc -b (strict) + vite build -> dist/, exit 0
npm run test      # Vitest: 1 file, 2 tests passed
npm run test:e2e  # Playwright/chromium: 1 test passed
npm run lint      # oxlint, no findings
```

All four green. `cargo fmt/clippy/test` at the workspace root untouched
by this phase (no Rust files changed) and still green from Phase 00's
baseline.

## Phase Gate — CLOSED

All six tasks have evidence above. Frontend tests are green (Vitest +
Playwright). Web Live View opens (`npm run preview` + Playwright
proved it) and shows exactly the full-canvas shell plus the reserved
toolbar anchor described in Tasks 015-016 — no more, no less. Shared
semantics remain platform-neutral (no Rust touched). No unsupported
native-device claim made; `dev:live`'s current limitation is stated
plainly rather than implied to work. Continuing automatically to
Phase 03 (Rust/Wasm Compatibility Audit).
