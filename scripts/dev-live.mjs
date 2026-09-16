#!/usr/bin/env node
// Execution 03, Phase 17, Task 135/136: `npm run dev:live` -- one
// command that starts Vite *and* keeps `craftloop-web-bridge`'s real
// wasm bindings rebuilt as its Rust source changes, closing the gap
// `build-wasm-bridge.mjs`'s own doc comment left open ("this is a
// one-shot build, not a watch loop... Phase 17's job").
//
// Task 136 ("a failed Rust build must not leave the frontend silently
// using old semantics"): a failed rebuild leaves the previously-built
// `apps/web-live/src/wasm-bridge` output completely untouched (see
// `build-wasm-bridge.mjs`'s own `rmSync`/`wasm-bindgen` steps only run
// after a successful `cargo build`), so the browser keeps running the
// last real, valid Rust semantics -- but silently continuing on stale
// code is exactly the failure this task forbids, so every failed
// rebuild here prints a loud, impossible-to-miss banner in the same
// terminal Vite's own output appears in, and every successful rebuild
// after a failure prints an equally clear "recovered" line.

import { spawn } from 'node:child_process'
import { existsSync, watch } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)))
const webLiveDir = join(repoRoot, 'apps/web-live')
const cratesDir = join(repoRoot, 'crates')

const red = (s) => `\x1b[41m\x1b[97m${s}\x1b[0m`
const green = (s) => `\x1b[32m${s}\x1b[0m`

let building = false
let pendingRebuild = false
let lastBuildOk = true

function buildOnce() {
  return new Promise((resolve) => {
    const child = spawn('node', [join(repoRoot, 'scripts/build-wasm-bridge.mjs')], {
      cwd: repoRoot,
      stdio: 'inherit',
    })
    child.on('exit', (code) => resolve(code === 0))
    child.on('error', () => resolve(false))
  })
}

async function rebuild() {
  if (building) {
    pendingRebuild = true
    return
  }
  building = true
  const ok = await buildOnce()
  building = false

  if (ok) {
    if (!lastBuildOk) {
      console.log(
        `\n${green('✓ wasm rebuild recovered')} -- the browser is back on current Rust semantics.\n`,
      )
    }
    lastBuildOk = true
  } else {
    lastBuildOk = false
    console.error(
      `\n${red('✗ WASM REBUILD FAILED')} -- the browser is still serving the PREVIOUS build, not this change. Fix the Rust error above and save again.\n`,
    )
  }

  if (pendingRebuild) {
    pendingRebuild = false
    await rebuild()
  }
}

console.log('Building craftloop-web-bridge...')
await rebuild()

let debounceTimer = null
function scheduleRebuild() {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(rebuild, 300)
}

if (existsSync(cratesDir)) {
  try {
    // `recursive: true` is supported on Windows and macOS; Linux
    // support varies by Node version. Degrade to "no watch" rather
    // than crash the whole dev command on an unsupported platform --
    // `npm run dev` (the plain, non-watching script) remains the
    // documented fallback there.
    watch(cratesDir, { recursive: true }, (_event, filename) => {
      if (filename && filename.endsWith('.rs')) scheduleRebuild()
    })
    console.log(`Watching ${cratesDir} for Rust changes.`)
  } catch (err) {
    console.warn(
      `Could not watch ${cratesDir} recursively on this platform (${err.message}); wasm will not auto-rebuild -- rerun "npm run build:wasm" manually after Rust changes.`,
    )
  }
}

console.log('Starting Vite...')
const vite = spawn('npx', ['vite'], { cwd: webLiveDir, stdio: 'inherit', shell: true })
vite.on('exit', (code) => process.exit(code ?? 0))
