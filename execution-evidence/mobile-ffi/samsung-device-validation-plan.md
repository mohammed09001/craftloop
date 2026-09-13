# Samsung Real-Device Validation Plan

Execution 01, Phase 28, Task 205. Authority: MCP Article 127 "Device
Capability Matrix"; `craftloop_input::SIMULATOR_DISCLAIMER`/`SIMULATOR_LIMITATIONS`
(Phase 04) already name the general principle this document makes
concrete for the specific target device class -- a Samsung Galaxy Tab
S-series tablet with S Pen, matching `android/app/build.gradle.kts`'s
tablet-class `minSdk`/`compileSdk` choice (Task 203).

## Why this list exists

`apps/windows-harness`'s `MouseSimulator` (Phase 04) is explicit that it
maps a mouse to a *simulated* pen -- `PointerSource::SimulatedMouse`,
never `PointerSource::Stylus` (Phase 03, Task 022's own type distinction
exists specifically so no test or evidence file can claim real-hardware
validation by accident). Every scenario below is a real-hardware
behavior that Windows mouse simulation is structurally incapable of
exercising, not merely inconvenient to simulate -- the No-Hallucination
Contract's "Implemented but not hardware-validated" category applies to
all of Phase 28's input-handling code until every scenario here has
actually been run on a physical S Pen and device.

## Scenarios requiring a real S Pen and a real Galaxy Tab S device

1. **Full-range pressure curve fidelity.** A simulated pen reports a
   fixed or trivially-varying pressure value
   (`craftloop_input::mouse_simulator`'s own documented default
   override); only a real S Pen exercises the full sensor range,
   including the low-pressure "just touching" threshold and high-pressure
   saturation behavior that determines whether
   `DimensionAssociation`/recognition confidence thresholds (Phase 18)
   tuned against synthetic ink actually feel right against real
   handwriting pressure variance.
2. **Tilt and orientation across the full physical range.** Confirms the
   polar-to-Cartesian conversion this phase's `jetpack-ink-mapping.md`
   documents is correct at extreme tilt angles (near-horizontal pen),
   where a sign or axis-swap error would be most visible.
3. **Hover/air-command detection.** `InputCapabilities.hover` (Phase 03)
   can only be genuinely confirmed `true` by observing real
   `ACTION_HOVER_MOVE` events before the pen tip contacts the screen; no
   mouse-based simulation produces a comparable "approaching but not yet
   touching" signal.
4. **Palm rejection during natural two-handed sketching.** The single
   scenario `jetpack-ink-mapping.md`'s capability table flags as
   non-queryable via a simple flag -- must be observed by an actual user
   resting their hand on the tablet while drawing with the S Pen in the
   other, confirming the OS/input-stack filtering behaves correctly with
   this product's actual UI layout (not a generic OS demo app's).
5. **S Pen button/air-action gestures.** The physical barrel button
   (`PointerButtons.barrel`, Phase 03) and Samsung's air-action gesture
   set (device- and S-Pen-generation-specific) have no Windows-mouse
   equivalent at all -- `barrel` exists in the type today but has never
   been exercised by anything except a synthetic test value.
6. **BLE S Pen disconnect/reconnect handling.** Detachable/BLE-connected
   S Pen models (Tab S7+ and later) can disconnect mid-stroke; the
   adapter's behavior when a stroke is interrupted this way (does it
   finalize a partial stroke? discard it? per
   `craftloop_input::lifecycle::StrokeLifecycleValidator`'s existing
   state machine, Phase 03) can only be observed with real hardware.
7. **Low-latency ink rendering under real display/touch-controller
   pipeline latency.** Jetpack Ink's whole `InProgressStrokesView`
   prediction system exists to compensate for real touch-to-photon
   latency; a Windows mouse pointer has no comparable rendering-pipeline
   latency to validate against.
8. **DeX / multi-window / split-screen behavior.** Samsung DeX mode and
   Android multi-window can change window focus, input routing, and
   available screen real estate for the drawing surface in ways specific
   to Samsung's Android skin, not stock Android or any Windows analogue.
9. **Thermal throttling under sustained sketching sessions.** Real
   solver/recognition CPU load (Phase 11-13, 16-18) sustained over a long
   drawing session may trigger real-device thermal throttling that
   changes measured performance from Phase 27's benchmark baselines
   (`execution-evidence/quality-tooling/phase-27-benchmark-results.txt`,
   captured on this development machine, not a tablet SoC).
10. **Screen-protector/glass surface friction and its effect on stroke
    sampling density.** Real accessory glass (common on Tab S devices)
    changes effective sampling rate/jitter versus a bare digitizer,
    which affects `craftloop-recognition`'s beautification residuals
    (Phase 06) in ways synthetic test strokes (uniform-spaced points)
    cannot represent.

## What this plan does not cover

Scenarios that Windows mouse simulation *can* validate today (basic
stroke lifecycle, recognition/beautification correctness against
synthetic geometry, solver/constraint behavior, document persistence,
export) are explicitly out of scope for this list -- re-listing them here
would dilute the point of naming exactly what mouse simulation cannot
reach.

## Status

**Not yet executed.** No Samsung device or S Pen is available in this
sandboxed development environment (Task 205's objective is to produce
this list, not to run it -- see Phase 28's own task text: "List the
scenarios that must be run," not "run them"). Each scenario above should
become its own tracked validation task once real device access exists.
