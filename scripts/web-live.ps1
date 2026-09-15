<#
.SYNOPSIS
    Execution 03, Phase 02, Task 014 -- one-command Web Live View launcher.

.DESCRIPTION
    Article 13 (Live Development) defines the target `npm run dev:live`
    behavior: build the Wasm bridge if needed, watch the relevant Rust
    crates, start Vite, and reload Wasm on a successful Rust rebuild
    without ever continuing silently on a failed one.

    As of Phase 02 there is no `craftloop-web-bridge` crate yet (that is
    Phase 03), so this script honestly does only what exists today:
    installs npm dependencies if `node_modules` is missing and starts the
    Vite dev server for `apps/web-live`. It does not yet watch or rebuild
    any Rust crate -- do not read a clean run of this script as proof
    that Wasm hot-reload works (Article 65, No-Hallucination Contract).
    This script gains the cargo-watch / Wasm-rebuild responsibility in
    Phase 17 (Article 62) once the bridge crate exists to watch.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File scripts/web-live.ps1
#>

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$webLiveDir = Join-Path $repoRoot 'apps/web-live'

if (-not (Test-Path (Join-Path $webLiveDir 'node_modules'))) {
    Write-Host 'apps/web-live/node_modules missing -- running npm install first.'
    Push-Location $webLiveDir
    try {
        npm install
    } finally {
        Pop-Location
    }
}

Push-Location $webLiveDir
try {
    npm run dev:live
} finally {
    Pop-Location
}
