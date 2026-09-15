# Craft Loop — Web Live View

A browser-based live engineering and UI/UX test harness for Craft
Loop (Execution 03). It is not a new Craft Loop product and not a
replacement for the Android or iPad adapters — it is a fast
interaction laboratory backed by the same shared Rust engineering
core through `crates/craftloop-web-bridge` (Wasm), once that crate
exists (Phase 03).

See `execution/Craft Loop Execution 03 Web Live View Creative Sketch
Mode.md` for the full spec and
`execution-evidence/execution-03/creative-precision-contract.md` for
the standing UI/UX rules every change here is checked against.

## Scripts

- `npm run dev` — start the Vite dev server (frontend-only HMR).
- `npm run dev:live` — the target one-command live-development loop
  (Article 13). Currently identical to `dev`; gains Rust/Wasm watch
  and rebuild in Phase 17 once `craftloop-web-bridge` exists. Prefer
  `scripts/web-live.ps1` from the repo root, which also installs
  dependencies on a clean checkout.
- `npm run build` — strict `tsc -b` typecheck, then a production
  Vite build to `dist/` (the static internet-preview artifact,
  Article 14).
- `npm run test` — Vitest unit/component tests (jsdom).
- `npm run test:e2e` — Playwright smoke tests against a built
  preview (`npm run preview`).
- `npm run preview` — serve the production build locally.
- `npm run lint` — oxlint.

## Status

Phase 02 (workspace scaffold) only: a full-canvas shell with a
reserved top-center toolbar anchor and no engineering behavior yet.
No fake geometry, no mock engineering objects — the canvas stays
empty until Phase 06 wires it to a real `CraftLoopSession` scene
snapshot.
