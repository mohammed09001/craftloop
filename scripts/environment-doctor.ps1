<#
.SYNOPSIS
    Execution 02, Phase 01, Task 011 -- Windows Android toolchain doctor.

.DESCRIPTION
    Article 21 (Windows Host Toolchain Contract) requires a repository
    script that checks every tool the Android Tablet Engineering Alpha
    build/USB workflow depends on: Git, Rust/rustup and the Android Rust
    target, Java, the Android SDK, Platform Tools (adb), the Android NDK,
    the project's Gradle Wrapper, and a connected/authorized device.

    This script only detects and reports. It never installs anything and
    never fabricates a pass -- every check runs a real command and reports
    exactly what that command returned (Article 27, No-Hallucination
    Policy). Run it after any toolchain change and before trusting a
    "clean checkout" build claim.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File scripts/environment-doctor.ps1
#>

[CmdletBinding()]
param()

$ErrorActionPreference = 'Continue'

# Re-read PATH/ANDROID_HOME/ANDROID_SDK_ROOT straight from the registry
# (Machine + User scope) rather than trusting this process's inherited
# environment block. A long-lived shell (including this repo's own
# Claude Code tool session) can be started before an installer or a
# `[System.Environment]::SetEnvironmentVariable(..., "User")` call ran,
# and would otherwise report a false BLOCKER for a tool that a brand-new
# terminal would see just fine.
$machinePath = [System.Environment]::GetEnvironmentVariable('Path', 'Machine')
$userPath = [System.Environment]::GetEnvironmentVariable('Path', 'User')
$env:Path = "$machinePath;$userPath"
foreach ($var in @('ANDROID_HOME', 'ANDROID_SDK_ROOT', 'JAVA_HOME')) {
    $userValue = [System.Environment]::GetEnvironmentVariable($var, 'User')
    $machineValue = [System.Environment]::GetEnvironmentVariable($var, 'Machine')
    if ($userValue) { Set-Item "env:$var" $userValue }
    elseif ($machineValue) { Set-Item "env:$var" $machineValue }
}

$results = New-Object System.Collections.Generic.List[object]

function Add-Result {
    param(
        [string]$Name,
        [ValidateSet('PASS', 'FAIL', 'BLOCKER')]
        [string]$Status,
        [string]$Detail,
        [string]$Remediation = ''
    )
    $results.Add([pscustomobject]@{
        Name        = $Name
        Status      = $Status
        Detail      = $Detail
        Remediation = $Remediation
    })
}

function Test-CommandExists {
    param([string]$Command)
    try {
        $null = Get-Command $Command -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

# --- Git -------------------------------------------------------------------
if (Test-CommandExists 'git') {
    $version = (git --version) 2>&1
    Add-Result -Name 'Git' -Status 'PASS' -Detail "$version"
} else {
    Add-Result -Name 'Git' -Status 'BLOCKER' -Detail 'git not found on PATH' `
        -Remediation 'Install Git for Windows: https://git-scm.com/download/win'
}

# --- Rust / rustup / cargo ---------------------------------------------------
if (Test-CommandExists 'rustup') {
    $rustupVersion = (rustup --version) 2>&1 | Select-Object -First 1
    Add-Result -Name 'rustup' -Status 'PASS' -Detail "$rustupVersion"
} else {
    Add-Result -Name 'rustup' -Status 'BLOCKER' -Detail 'rustup not found on PATH' `
        -Remediation 'Install from https://rustup.rs'
}

if (Test-CommandExists 'cargo') {
    $cargoVersion = (cargo --version) 2>&1
    Add-Result -Name 'cargo' -Status 'PASS' -Detail "$cargoVersion"
} else {
    Add-Result -Name 'cargo' -Status 'BLOCKER' -Detail 'cargo not found on PATH' `
        -Remediation 'Install via rustup: https://rustup.rs'
}

# --- Rust Android target (Article 20: aarch64-linux-android) ----------------
if (Test-CommandExists 'rustup') {
    $targets = (rustup target list --installed) 2>&1
    if ($targets -match 'aarch64-linux-android') {
        Add-Result -Name 'Rust Android target (aarch64-linux-android)' -Status 'PASS' `
            -Detail 'installed'
    } else {
        Add-Result -Name 'Rust Android target (aarch64-linux-android)' -Status 'BLOCKER' `
            -Detail "not installed. Currently installed targets:`n$targets" `
            -Remediation 'rustup target add aarch64-linux-android'
    }
}

# --- Java (JDK 17, matching android/app/build.gradle.kts's compileOptions) --
if (Test-CommandExists 'java') {
    $javaVersion = (java -version) 2>&1 | Select-Object -First 1
    Add-Result -Name 'Java (JDK)' -Status 'PASS' -Detail "$javaVersion"
} else {
    Add-Result -Name 'Java (JDK)' -Status 'BLOCKER' -Detail 'java not found on PATH' `
        -Remediation 'Install a JDK 17 (e.g. Eclipse Temurin 17 LTS) and add it to PATH/JAVA_HOME. Android Gradle Plugin 8.6.1 (android/build.gradle.kts) requires JDK 17.'
}

# --- Android SDK --------------------------------------------------------------
$sdkRoot = $env:ANDROID_HOME
if (-not $sdkRoot) { $sdkRoot = $env:ANDROID_SDK_ROOT }
if (-not $sdkRoot) { $sdkRoot = Join-Path $env:LOCALAPPDATA 'Android\Sdk' }

if ($sdkRoot -and (Test-Path $sdkRoot)) {
    Add-Result -Name 'Android SDK' -Status 'PASS' -Detail "found at $sdkRoot"
} else {
    Add-Result -Name 'Android SDK' -Status 'BLOCKER' `
        -Detail "no ANDROID_HOME/ANDROID_SDK_ROOT set, and $sdkRoot does not exist" `
        -Remediation 'Install Android Studio (bundles the SDK manager) or the command-line tools, then set ANDROID_HOME. compileSdk 35 / minSdk 30 / targetSdk 35 are required (android/app/build.gradle.kts).'
}

# --- Platform Tools / adb -----------------------------------------------------
if (Test-CommandExists 'adb') {
    $adbVersion = (adb version) 2>&1 | Select-Object -First 1
    Add-Result -Name 'Platform Tools (adb)' -Status 'PASS' -Detail "$adbVersion"
} else {
    $bundledAdb = if ($sdkRoot) { Join-Path $sdkRoot 'platform-tools\adb.exe' } else { $null }
    if ($bundledAdb -and (Test-Path $bundledAdb)) {
        Add-Result -Name 'Platform Tools (adb)' -Status 'FAIL' `
            -Detail "adb exists at $bundledAdb but is not on PATH" `
            -Remediation "Add $sdkRoot\platform-tools to PATH"
    } else {
        Add-Result -Name 'Platform Tools (adb)' -Status 'BLOCKER' -Detail 'adb not found on PATH or in the SDK' `
            -Remediation 'Install Android SDK Platform Tools (via Android Studio SDK Manager or sdkmanager "platform-tools") and add them to PATH.'
    }
}

# --- Android NDK (Article 20: pinned version, not "latest") ------------------
$ndkFound = $false
if ($sdkRoot) {
    $ndkDir = Join-Path $sdkRoot 'ndk'
    if (Test-Path $ndkDir) {
        $ndkVersions = Get-ChildItem $ndkDir -Directory -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Name
        if ($ndkVersions) {
            $ndkFound = $true
            Add-Result -Name 'Android NDK' -Status 'PASS' -Detail "found version(s): $($ndkVersions -join ', ')"
        }
    }
}
if (-not $ndkFound) {
    Add-Result -Name 'Android NDK' -Status 'BLOCKER' -Detail 'no NDK found under the Android SDK ndk/ directory' `
        -Remediation 'Install a pinned NDK version via Android Studio SDK Manager (SDK Tools tab -> NDK (Side by side)), then record the exact version used in execution-evidence/execution-02/.'
}

# --- Project Gradle Wrapper ---------------------------------------------------
$repoRoot = Split-Path -Parent $PSScriptRoot
$gradlewBat = Join-Path $repoRoot 'android\gradlew.bat'
if (Test-Path $gradlewBat) {
    Add-Result -Name 'Gradle Wrapper (android/gradlew.bat)' -Status 'PASS' -Detail 'present'
} else {
    Add-Result -Name 'Gradle Wrapper (android/gradlew.bat)' -Status 'BLOCKER' `
        -Detail 'android/gradlew.bat does not exist' `
        -Remediation 'Run "gradle wrapper" from a machine with Gradle installed (or Android Studio''s Gradle sync) inside android/, then commit gradlew, gradlew.bat, and gradle/wrapper/. See Article 19/Phase 02.'
}

# --- Connected device ---------------------------------------------------------
if (Test-CommandExists 'adb') {
    # `adb devices` prints "* daemon not running; starting now..." lines
    # on its first invocation in a session (to stderr) before its real
    # "List of devices attached" output -- filter those out rather than
    # merging stderr with 2>&1, which wraps each line in an ErrorRecord
    # (not a plain string) under Windows PowerShell 5.1 and breaks
    # string methods like .Trim() on it.
    $devices = adb devices 2>$null
    $listIndex = ($devices | Select-String -Pattern '^List of devices attached' | Select-Object -First 1).LineNumber
    $deviceLines = if ($listIndex) {
        $devices | Select-Object -Skip $listIndex | Where-Object { $_.Trim() -ne '' }
    } else {
        @()
    }
    if ($deviceLines) {
        $authorized = $deviceLines | Where-Object { $_ -match '\tdevice$' }
        $unauthorized = $deviceLines | Where-Object { $_ -match '\tunauthorized$' }
        if ($authorized) {
            Add-Result -Name 'Connected Android device' -Status 'PASS' `
                -Detail "$($authorized.Count) authorized device(s) (serial redacted per Article 40's USB Evidence rule)"
        } elseif ($unauthorized) {
            Add-Result -Name 'Connected Android device' -Status 'FAIL' `
                -Detail 'device detected but unauthorized' `
                -Remediation 'Accept the RSA debugging authorization prompt on the tablet, then re-run: adb devices'
        } else {
            Add-Result -Name 'Connected Android device' -Status 'FAIL' -Detail "unrecognized adb devices output:`n$devices"
        }
    } else {
        Add-Result -Name 'Connected Android device' -Status 'FAIL' -Detail 'no device listed' `
            -Remediation 'Enable Developer Options + USB debugging on the Samsung tablet, connect a data-capable USB cable, then run: adb devices'
    }
} else {
    Add-Result -Name 'Connected Android device' -Status 'BLOCKER' -Detail 'cannot check -- adb is not available'
}

# --- Report -------------------------------------------------------------------
Write-Host ''
Write-Host 'Craft Loop Android Environment Doctor' -ForegroundColor Cyan
Write-Host '======================================' -ForegroundColor Cyan
Write-Host ''

foreach ($r in $results) {
    $color = switch ($r.Status) {
        'PASS'    { 'Green' }
        'FAIL'    { 'Yellow' }
        'BLOCKER' { 'Red' }
    }
    Write-Host "[$($r.Status)] $($r.Name)" -ForegroundColor $color
    Write-Host "    $($r.Detail)"
    if ($r.Remediation) {
        Write-Host "    -> $($r.Remediation)" -ForegroundColor DarkGray
    }
}

Write-Host ''
$blockers = @($results | Where-Object { $_.Status -eq 'BLOCKER' }).Count
$fails = @($results | Where-Object { $_.Status -eq 'FAIL' }).Count
$passes = @($results | Where-Object { $_.Status -eq 'PASS' }).Count
Write-Host "Summary: $passes passed, $fails failed, $blockers blocker(s)."

if ($blockers -gt 0) {
    Write-Host 'Result: NOT READY for Android build/USB workflow.' -ForegroundColor Red
    exit 1
} elseif ($fails -gt 0) {
    Write-Host 'Result: PARTIALLY READY -- resolve FAIL items above.' -ForegroundColor Yellow
    exit 1
} else {
    Write-Host 'Result: READY.' -ForegroundColor Green
    exit 0
}
