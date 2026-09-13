# Phase 04 — Windows Native Engineering Harness — Evidence

Recorded: 2026-09-13

## App created

`apps/windows-harness` (added to workspace `members`), a binary crate using
`eframe`/`egui` 0.32, depending on `craftloop-errors`, `craftloop-geometry`,
`craftloop-input`, `craftloop-serialization`. Per the Architecture Decision
in Execution 01, this is the durable Rust harness; the Python/Tk bootstrap
in `windows-simulator/` is retained untouched (not expanded) until this
reaches equivalent basic functionality, per Phase 00's evidence map
decision.

## Module boundary (Task 033)

`app.rs` is the **only** file depending on `egui`/`eframe` types. Every
other module (`viewport.rs`, `tool.rs`, `state.rs`, `scenario.rs`,
`diagnostic_export.rs`) is plain Rust with its own `#[cfg(test)]` unit
tests that run with no window system — verified by the fact that
`cargo test -p windows-harness` (26 tests) runs headlessly in this
sandboxed environment, which has no display.

## Tasks 027–033

| Task | Module | Summary |
|---|---|---|
| 027 Scaffold native harness | `main.rs`, `app.rs` | `eframe::run_native` entry point; `HarnessApp: eframe::App`. Window title itself states "disposable — not production UI." |
| 028 Canvas and viewport | `viewport.rs` | `Viewport` (pan/zoom/screen↔world mapping, `zoom_at` keeps the cursor-anchored world point fixed, `fit_bounds`). 8 unit tests incl. round-trip and anchor-fixed-point properties. Wired into `app.rs`: mouse-wheel zoom, secondary-button-drag pan, "Fit view" button computing real stroke bounds. |
| 029 Engine state inspectors | `state.rs`, `diagnostic_export.rs`, `app.rs` right panel | Shows active tool, completed/in-progress stroke counts, zoom, and the simulator disclaimer. Deliberately does **not** show "recognized primitives," "dimensions," or "constraints" panels yet — those engines do not exist until Phases 06/10/12, and stub panels would be exactly the fabricated completeness the No-Hallucination Contract forbids. Documented in `state.rs`'s module doc as a to-be-extended contract. |
| 030 Mouse pen controls | `tool.rs` | `HarnessTool` (Pen/Line/Circle/Rectangle/Eraser/Select) with distinct labels; `SimulatedControls` (pressure override, range-validated, never edits `InputCapabilities`). |
| 031 Scenario loader | `scenario.rs` | `Scenario` (name/description/`Vec<PointerTrace>`), `ScenarioLoadError` (Io/Parse/Invalid via `thiserror`), validates every trace's lifecycle before accepting. Wired into `app.rs` ("Load scenario" / "Save current strokes as scenario"). |
| 032 Diagnostic export | `diagnostic_export.rs` | `HarnessDiagnostic::capture()` snapshots tool/viewport/stroke/disclaimer state to canonical JSON; wired to an "Export diagnostic JSON" button writing to a temp file — explicit repository-evidence requirement ("without requiring screenshots as the only evidence") satisfied by a real, tested, machine-readable export path. |
| 033 Keep harness disposable | module layout | Enforced by keeping `egui`/`eframe` imports confined to `app.rs`; every other module is independently unit-testable and none of them could be mistaken for production mobile code (no Android/iOS types anywhere in this crate). |

## Example fixture

`examples/generate_scenario.rs` produces `scenarios/example-line.json`, a
real scenario built from `MouseSimulator`/`PointerTrace` (not hand-typed
JSON assumed to match the schema). A dedicated test,
`scenario::tests::checked_in_example_scenario_loads_and_validates`, loads
this exact checked-in file through the same `Scenario::load_from_file` path
the UI uses, so the fixture is proven loadable, not merely present.

## Manual/visual verification — explicitly limited

Per the No-Hallucination Contract, the following is **not** claimed:

- No screenshot or interactive click-through was performed. This sandboxed
  Windows environment has no display and this session has no tool
  equivalent to browser automation for a native Win32/eframe window.
- What **was** verified: `cargo build -p windows-harness` succeeds with
  zero warnings; `cargo run -p windows-harness` was launched under a hard
  timeout (`timeout 8 cargo run ...`) and ran the full 8 seconds without
  panicking or exiting on its own — it was killed only by the external
  timeout (see `test-reports/phase-04-harness-launch.log`, exit code 124
  from `timeout` / 143 from the killed child, no panic backtrace in the
  log). This is evidence the app *starts and stays alive*, not evidence
  that mouse interaction, drawing, or the inspector panels render or behave
  correctly on screen.
- **Deferred to a human with a keyboard/mouse**: Front End Skill's "Windows
  Harness Frontend Rules" checklist (canvas, mouse-as-pen, tool state, raw
  stroke visibility, inspectors, scenario loader, export) — a manual pass
  is still required before this phase's UI can be called visually verified,
  and is recorded here as an open item, not silently skipped.

## Commands and results

```
cargo build -p windows-harness                # clean, 0 warnings (after fixing an f32-literal
                                                #   fallback warning and 4 dead_code warnings by
                                                #   wiring pan/zoom/fit/save-scenario into app.rs)
cargo test -p windows-harness                  # 26/26 passing
cargo run -p windows-harness --example generate_scenario   # writes scenarios/example-line.json
timeout 8 cargo run -p windows-harness         # ran 8s, no panic, killed by timeout (see log)
cargo fmt --all -- --check                     # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings   # clean
cargo test --workspace                         # 165/165 tests passing workspace-wide
```

Full output: `test-reports/phase-04-cargo-test.txt`,
`test-reports/phase-04-cargo-clippy.txt`,
`test-reports/phase-04-harness-launch.log`.

## Deferred (explicitly, not silently)

- Manual interactive verification (see above) — needs a human.
- Recognized-primitive/dimension/constraint/conflict/orthographic inspector
  panels — added alongside their engines (Phases 06, 10, 12, 14, 20-24).
- Retiring `windows-simulator/` (the Python bootstrap) — not yet, per the
  Architecture Decision's "retire once the Rust harness reaches equivalent
  basic functionality"; this phase reaches rough parity (tools, drawing,
  export) but has not yet been manually confirmed equivalent, so the Python
  harness stays for now.

## Phase Gate

- All seven tasks (027–033) represented in repository code with passing
  tests where testable without a display, and an honestly-scoped manual
  gap recorded where it is not.
- `cargo test --workspace`: 165/165 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no recognition/dimension/constraint logic
  added to the harness ahead of its owning engine; `egui` stayed confined
  to `app.rs`.
- Proceeding to Phase 05 automatically.
