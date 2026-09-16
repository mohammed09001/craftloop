# Execution 03, Phase 17 — Web Live Development and Internet Preview

Recorded 2026-09-16. Tasks 135-140.

## Task 135/136 — dev:live watches Rust and never goes stale silently

`scripts/build-wasm-bridge.mjs`'s own doc comment had explicitly left
this open: "This is a one-shot build, not a watch loop -- Article 13's
full ... live-development behavior is Phase 17's job." New
`scripts/dev-live.mjs` is that job: builds the wasm bridge once, then
watches `crates/` (`fs.watch(..., { recursive: true })`) for any
`.rs` change, debounces (300ms) and reruns the same real
`build-wasm-bridge.mjs`, and runs Vite as a child process alongside
it -- one command (`npm run dev:live`), matching the task's own
wording exactly.

Task 136 ("must not leave the frontend silently using old
semantics"): proven with a real, deliberately broken build, not
reasoned about --

1. Started `npm run dev:live` with a genuine Rust syntax error already
   present: the real `cargo build` failed, a loud red terminal banner
   printed (`✗ WASM REBUILD FAILED -- the browser is still serving the
   PREVIOUS build...`), and Vite still started on the last real, valid
   bindings (there was nothing stale to fall back to on a from-scratch
   broken checkout, but the *iterative* case -- edit-while-running --
   is the one that matters and is covered next).
2. With `dev:live` already running against good code, edited
   `craftloop-web-bridge/src/lib.rs` to reintroduce the same syntax
   error: the watcher caught it, reran the build, and printed the same
   clear failure banner while Vite kept serving the last good build.
3. Reverted the file while still running: the watcher caught the fix,
   rebuilt successfully, and printed a clear
   `✓ wasm rebuild recovered -- the browser is back on current Rust
   semantics.` line.

**Real bug found and fixed along the way:** `build-wasm-bridge.mjs`'s
`execFileSync` failure wasn't caught, so every build failure printed a
raw Node stack trace on top of cargo's own (already useful) error
output -- noise burying signal, working against Task 136's "surface
build failures clearly." Wrapped in a `try`/`catch` that exits `1`
without the extra trace; cargo's/wasm-bindgen's own inherited stdout
is the whole useful message.

`scripts/web-live.ps1`'s own doc comment was stale (still said "This
script gains the cargo-watch / Wasm-rebuild responsibility in Phase
17... once the bridge crate exists to watch" -- both preconditions had
already been true since Phase 03/04). Updated to state the real
current behavior; the script's own logic needed no change since it
already just called `npm run dev:live`.

## Task 137 — Production build: already real, reverified

No new work -- `npm run build` (`prebuild` → real release wasm build →
`tsc -b` → `vite build`) has produced a real, deployable `dist/` on
every phase since Phase 06, and a failed Rust build already aborts the
whole command via npm's own `pre<script>` chaining (confirmed again
this phase via the deliberate-syntax-error test above, run against
`npm run build` too). Reverified rather than re-built.

## Task 138 — Preview providers documented, code stays provider-neutral

`apps/web-live/README.md` gained a "Deploying a preview" section with
concrete Vercel and Cloudflare Pages build-command/output-directory
instructions. No provider-specific file was added to the repository
(no `vercel.json`, no Cloudflare Worker/Functions code, no
provider SDK dependency) -- `dist/` is a plain static site or either
provider serves as one, keeping the actual codebase portable to any
static host, exactly as the task asks.

## Task 139/140 — Deferred pending real hosting authorization

Provisioning an actual Vercel or Cloudflare Pages project needs a real
account and credentials this environment does not have -- the same
category as every other "deferred native validation" entry across
this execution's evidence docs, applied here to deployment instead of
a device. Documented plainly in the README rather than fabricated.

Task 140's smoke test is nonetheless real, working infrastructure
today: `tests/e2e/preview-smoke.spec.ts` is gated on a `PREVIEW_URL`
environment variable (`test.skip(!previewUrl, ...)`), so it never
guesses or hard-codes a URL and never affects a normal local/CI run
(confirmed: it shows as `1 skipped` in the full suite run below, not
a failure). It is ready to run for real the moment a real preview URL
exists (`PREVIEW_URL=<url> npx playwright test
tests/e2e/preview-smoke.spec.ts`), exercising the same real
Sketch2D-line-creation flow every other e2e spec in this project
already proves against a local build.

## Test-first evidence

**Frontend unit** (`npm run test`): 61 passed, unchanged -- this
phase's changes are dev-tooling scripts and documentation, not
component behavior.

**Real browser** (`npm run test:e2e`): 42 passed, 1 skipped (the new,
correctly-gated `preview-smoke.spec.ts`), 0 failed.

**Dev-loop verification** (manual, transcript-backed, not
automatable in Vitest/Playwright since it drives a long-running
terminal process): the three-step break/watch-catch/fix-recover cycle
above, run for real against `npm run dev:live`.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 61/61
npm run test:e2e                                                    # 42/42 (1 correctly skipped)
npm run lint                                                        # oxlint clean
npm run dev:live                                                    # manually verified: builds, watches, rebuilds, recovers
```

## Phase Gate — CLOSED

All six tasks have evidence above -- 135-138 fully real and verified,
139-140 honestly deferred with real, ready infrastructure rather than
a fabricated URL. No Rust crate changed. UI/UX invariant untouched (no
app behavior changed this phase). Universal invariant held: all new
code is Node tooling scripts (`scripts/*.mjs`) and documentation, none
of it in a platform-neutral engineering crate. Continuing automatically
to Phase 18 (CI and Cross-Platform Build Hardening).
