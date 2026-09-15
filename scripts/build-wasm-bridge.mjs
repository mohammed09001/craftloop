#!/usr/bin/env node
// Execution 03, Phase 06: one-command build for craftloop-web-bridge's
// wasm-bindgen output, consumed by apps/web-live.
//
// Invoked automatically via apps/web-live's `predev`/`predev:live`/
// `prebuild` npm hooks (npm runs `pre<script>` before `<script>`
// automatically), so `npm run dev` works on a clean checkout without a
// separate manual step. Requires `wasm32-unknown-unknown` (`rustup
// target add wasm32-unknown-unknown`) and a `wasm-bindgen-cli` whose
// version matches the `wasm-bindgen` crate dependency exactly (schema
// versions must match or wasm-bindgen refuses to load the module) --
// see execution-evidence/execution-03/phase-03-wasm-compatibility-audit.md
// for the exact `cargo install wasm-bindgen-cli --version <x> --locked`
// command this repository was built against.
//
// This is a one-shot build, not a watch loop -- Article 13's full
// "rebuild Wasm on Rust changes, reload the module, surface build
// failures clearly" live-development behavior is Phase 17's job. Until
// then, re-run this script (or `npm run dev`/`dev:live`, which reruns
// it automatically) after editing craftloop-web-bridge or any shared
// domain crate it depends on.
//
// `--release`: used by `prebuild` (Article 14's static internet
// preview). The unoptimized dev-profile .wasm is ~12MB; a release
// build is the difference between a usable preview deploy and one
// that isn't.

import { execFileSync } from 'node:child_process'
import { existsSync, mkdirSync, rmSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const release = process.argv.includes('--release')

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)))
const outDir = join(repoRoot, 'apps/web-live/src/wasm-bridge')
const profileDir = release ? 'release' : 'debug'
const wasmPath = join(
  repoRoot,
  `target/wasm32-unknown-unknown/${profileDir}/craftloop_web_bridge.wasm`,
)

function run(command, args) {
  console.log(`> ${command} ${args.join(' ')}`)
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit' })
}

const cargoArgs = ['build', '--target', 'wasm32-unknown-unknown', '-p', 'craftloop-web-bridge']
if (release) cargoArgs.push('--release')
run('cargo', cargoArgs)

if (!existsSync(wasmPath)) {
  console.error(`Expected build output missing: ${wasmPath}`)
  process.exit(1)
}

rmSync(outDir, { recursive: true, force: true })
mkdirSync(outDir, { recursive: true })

run('wasm-bindgen', [wasmPath, '--out-dir', outDir, '--target', 'web'])

console.log(`craftloop-web-bridge bindings (${profileDir}) written to ${outDir}`)
