<#
.SYNOPSIS
    Execution 03, Phase 02, Task 014 -- one-command Web Live View launcher.

.DESCRIPTION
    Article 13 (Live Development): build the Wasm bridge if needed,
    watch the relevant Rust crates, start Vite, and rebuild Wasm on a
    successful Rust change without ever continuing silently on a
    failed one -- real as of Phase 17 (`scripts/dev-live.mjs`, run via
    `npm run dev:live` below). This script itself installs npm
    dependencies if `node_modules` is missing, then hands off to that.

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
