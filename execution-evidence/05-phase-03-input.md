# Phase 03 — Input Abstraction and Windows Simulation Contract — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-input`, depending on `craftloop-errors`, `craftloop-geometry`,
`craftloop-serialization`, `serde`, `serde_json`. No platform/UI dependency.

## Tasks 021–026

| Task | Module | Summary |
|---|---|---|
| 021 Normalized pointer samples | `sample.rs` | `PointerSample` (position, timestamp_seconds, optional pressure/tilt, source, buttons, capabilities); `PointerButtons`. Constructor validates ranges (pressure ∈ [0,1], tilt ∈ [-90,90], finite timestamp) as a structured `DomainError::Input`. |
| 022 Input capability descriptors | `capabilities.rs` | `InputCapabilities` (pressure/tilt/hover/palm_rejection/eraser), all `false` by default (`NONE`) — the honest baseline; an adapter sets a field `true` only when it has confirmed that capability. |
| 023 Mouse-to-pen simulation adapter | `mouse_simulator.rs` | `MouseSimulator::sample()` — constant placeholder pressure (0.5), tilt always `None`, capabilities always `InputCapabilities::NONE`. `PointerSample::is_pressure_authoritative()` lets callers distinguish "a value is present" from "the value is real sensor data." |
| 024 Stroke lifecycle events | `lifecycle.rs` | `PointerEvent` (Down/Move/Up/Cancel/Hover/CaptureLost), `StrokeLifecycleValidator` — a small state machine (Idle/Active) rejecting `Move`/`Up`/`Cancel` without a prior `Down` and doubled `Down`, as a structured error, not a panic. |
| 025 Deterministic recorded-input playback | `trace.rs` | `PointerTrace` (named, ordered `Vec<PointerEvent>`); validates via Task 024's validator before replay; canonical-JSON round trip (byte-identical across repeated serializations, via `craftloop-serialization`); `replay()` is a deterministic ordered callback. |
| 026 Document simulator limitations | `disclaimer.rs` | `SIMULATOR_DISCLAIMER` / `SIMULATOR_LIMITATIONS` constants, tested to explicitly name "Apple Pencil"/"stylus"/"simulated". `PointerSource::label()` for `SimulatedMouse` is tested to always contain "simulated". The actual UI banner is Phase 04's job (no UI exists yet); this phase establishes the one authoritative string Phase 04 must reuse rather than re-inventing wording. |

## Design decisions worth recording

- **Timestamps are stream-relative seconds (`f64`), not wall-clock time.**
  Required for Task 025's determinism: a trace replayed next week must
  produce identical relative timing to a trace replayed the moment it was
  recorded.
- **`craftloop-errors` gained a new `DomainError::Input` variant** (with
  `InputErrorKind::{OutOfRange, InvalidLifecycleSequence}`), extending the
  Phase 01 structured-error model to a new subsystem rather than reusing an
  ill-fitting existing variant or falling back to strings.
- **Mouse pressure is a deliberate, narrowly-scoped exception** to "never
  fabricate an unsupported value": Task 023 explicitly requires a constant
  placeholder pressure (for stroke-width rendering) while
  `capabilities.pressure` stays `false`. Tilt has no such exception — it is
  always `None` from the simulator, exactly as Task 023 specifies ("explicit
  constant pressure and absent tilt").

## Commands and results

```
cargo build -p craftloop-input          # clean
cargo test -p craftloop-input           # 32/32 passing (first run, no bugs found)
cargo fmt --all -- --check              # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
  # 1 finding: clippy::clone_on_copy in trace.rs tests (PointerEvent is Copy);
  # fixed by dereferencing instead of cloning; re-run clean.
cargo test --workspace                  # 139/139 tests passing workspace-wide
```

Full output: `test-reports/phase-03-cargo-test.txt`,
`test-reports/phase-03-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Actual Windows OS mouse-event wiring (winit/egui callbacks calling
  `MouseSimulator::sample`) — Phase 04, `apps/windows-harness`.
- The visible UI disclaimer banner itself — Phase 04; this phase only
  supplies the tested string content.
- Android (Jetpack Ink) and iPad (PencilKit) adapters producing real
  `PointerSample`s with genuine `InputCapabilities` — Phases 28–29; blocked
  today by absence of those platform toolchains on this workstation (True
  Blocker Policy: "unavailable platform toolchain required for exact
  platform-only verification" — noted, not worked around by fabrication).

## Phase Gate

- All six tasks (021–026) represented in repository code with passing
  tests.
- `cargo test --workspace`: 139/139 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no rendering/UI code, no stroke-recognition
  logic, no platform-specific adapter beyond the documented mouse
  simulation contract.
- Proceeding to Phase 04 automatically.
