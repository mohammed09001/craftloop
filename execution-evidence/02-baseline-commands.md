# Baseline Commands — Phase 00, Task 004

Recorded: 2026-09-13, Windows 11, PowerShell 5.1 primary shell (Bash tool also available).

## Toolchain versions observed

```
cargo 1.98.1 (797e8a9bc 2026-08-05)
rustc 1.98.1 (48a229cea 2026-09-01)
Python 3.14.6
git version 2.54.0.windows.1
```

No Android SDK, no Xcode/macOS toolchain, and no UniFFI installation were
present at execution start. Android- and iOS-toolchain-dependent verification
is therefore deferred/blocked per the True Blocker Policy ("unavailable
platform toolchain required for exact platform-only verification") until that
phase is reached; unaffected core work is not blocked by this.

## Pre-existing (Python bootstrap) — worked at baseline

```
cd windows-simulator
py -m unittest -v
```

Result at baseline: 5 tests, all passed, 0.001s.

```
py -m unittest -v
```
Ran: `test_bent_stroke_is_not_line_like`, `test_circle_candidate`,
`test_closed_stroke`, `test_document_export_is_versioned`,
`test_straight_stroke_is_line_like` — all `ok`.

Run (manual, human-operable):
```
py .\app.py
```

## Rust workspace — did not exist at execution start

No `Cargo.toml`, no `crates/`, no `Cargo.lock` existed before Phase 01. The
canonical build/test/lint/format commands for the shared core are established
in Phase 01 and recorded again once the workspace exists (see
`03-phase-01-workspace.md`).

## No CI workflow existed at execution start

No `.github/workflows/*.yml` or equivalent. CI is addressed in Phase 27
(Quality Tooling and Continuous Integration), not fabricated here.
