# Craft Loop — Web Live View

A browser-based live engineering and UI/UX test harness for Craft
Loop (Execution 03). It is not a new Craft Loop product and not a
replacement for the Android or iPad adapters — it is a fast
interaction laboratory backed by the same shared Rust engineering
core through `crates/craftloop-web-bridge` (Wasm).

See `execution/Craft Loop Execution 03 Web Live View Creative Sketch
Mode.md` for the full spec and
`execution-evidence/execution-03/creative-precision-contract.md` for
the standing UI/UX rules every change here is checked against.

## Scripts

- `npm run dev` — start the Vite dev server (frontend-only HMR). The
  real Rust wasm bridge is built once beforehand; edit
  `craftloop-web-bridge` (or any crate it depends on) and rerun this
  to pick up the change.
- `npm run dev:live` — the one-command live-development loop (Article
  13): builds the wasm bridge, watches `crates/` for Rust changes and
  rebuilds automatically, and runs Vite alongside it. A failed
  rebuild never silently leaves the browser on stale semantics — it
  prints a loud, impossible-to-miss banner in the same terminal, and
  an equally clear "recovered" line once a later save fixes it.
- `npm run build` — strict `tsc -b` typecheck, a release wasm-bridge
  build, then a production Vite build to `dist/` (the static
  internet-preview artifact, Article 14). A failed Rust build aborts
  the whole command (npm's `prebuild` hook), so `dist/` is never
  produced from stale wasm.
- `npm run test` — Vitest unit/component tests (jsdom).
- `npm run test:e2e` — Playwright tests against a real built preview
  (`npm run preview`).
- `npm run preview` — serve the production build locally.
- `npm run lint` — oxlint.

## Deploying a preview

`npm run build`'s `dist/` output is a plain static site — an
`index.html`, JS/CSS bundles, and the release `.wasm` (~1.9MB,
~480KB gzipped). No server-side code, no environment variables, no
provider-specific config committed to the repo — deployable to any
static host. Two concrete providers:

**Vercel**
- Framework preset: *Other* (or *Vite*, if offered).
- Build command: `npm run build`
- Output directory: `apps/web-live/dist` (set the project root to
  `apps/web-live`, or adjust the path if deploying from the repo root).
- No serverless functions, no `vercel.json` required.

**Cloudflare Pages**
- Build command: `npm run build`
- Build output directory: `apps/web-live/dist`
- Root directory: `apps/web-live`
- No Workers/Functions required — static assets only.

Either provider must serve `.wasm` with `Content-Type:
application/wasm` for `instantiateStreaming` to work; both do this
correctly for a file with a `.wasm` extension by default.

**Live preview deployment:** not configured in this repository —
provisioning a real Vercel/Cloudflare Pages project requires an
account and credentials this environment does not have (Execution
03, Task 139, deferred pending real hosting authorization, the same
category as this project's other deferred-native-validation items).
`tests/e2e/preview-smoke.spec.ts` (Task 140) is ready to run against
a real deployed URL once one exists — see that file's own doc comment
for how to point it there.

## Status

Phase 16 (Creative UX Hardening) complete. The full Sketch2D
precision toolchain (dimensions, adaptive constraints, precision
snap/construction geometry, Orthographic linked views), browser
persistence (autosave/New/Open/Save), the developer Command
Simulator, and a responsive/accessible top-center toolbar are all
real and covered by Playwright tests against the real WASM engine —
see `execution-evidence/execution-03/` for the phase-by-phase record.
No fake geometry, no mock engineering objects anywhere in this app.
