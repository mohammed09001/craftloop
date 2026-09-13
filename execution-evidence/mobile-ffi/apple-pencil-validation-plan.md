# Apple Pencil Real-Device Validation Plan

Execution 01, Phase 29, Task 212. Authority: MCP Article 127 "Device
Capability Matrix"; `craftloop_input::SIMULATOR_DISCLAIMER`/`SIMULATOR_LIMITATIONS`
(Phase 04), mirroring `samsung-device-validation-plan.md`'s (Phase 28,
Task 205) structure for the iPad/Apple Pencil target.

## Why this list exists

Identical rationale to the Samsung plan: `PointerSource::SimulatedMouse`
vs. `PointerSource::Stylus` (Phase 03) exists precisely so no evidence
file can claim real-hardware validation by accident. Every scenario below
is a real Apple Pencil / real iPad behavior that neither Windows mouse
simulation nor the iOS Simulator (which does not simulate pencil
pressure/tilt/hover hardware signals at all) can exercise.

## Scenarios requiring a real Apple Pencil and a real iPad

1. **Full-range pressure (`force`) curve fidelity**, across Apple Pencil
   1st gen, 2nd gen, and Pro -- three distinct hardware generations with
   documented differences in force-sensing range and response curve;
   `pencilkit-mapping.md`'s `force` → `pressure` mapping needs real-device
   confirmation per generation, not just one.
2. **Tilt/azimuth at extreme angles**, confirming
   `pencilkit-mapping.md`'s altitude-from-screen-plane conversion
   (`90° - altitude`) is correct at near-horizontal pen angles, the same
   class of validation the Android plan names for Jetpack Ink's
   tilt/orientation.
3. **Hover detection** (Apple Pencil 2/Pro on supported iPads only) --
   `InputCapabilities.hover` can only be confirmed `true` by observing a
   real `UIHoverGestureRecognizer`/`UIPointerInteraction` event before
   contact; the iOS Simulator does not produce genuine hover-distance
   data even when "device" is set to a hover-capable iPad model.
4. **Palm rejection during natural two-handed sketching**, especially if
   Task 210's boundary decision leads to a raw `UITouch` capture path
   instead of `PKCanvasView`'s built-in filtering -- in that case palm
   rejection is entirely the adapter's own responsibility and has no
   simulator equivalent to validate against at all.
5. **Apple Pencil Pro's squeeze gesture and barrel roll.** Pro-only
   hardware features (haptic squeeze for tool switching, barrel roll for
   brush rotation) with zero Windows or Simulator equivalent; maps to
   this workspace's `PointerButtons`/command-grammar boundary
   (`crates/craftloop-mobile-ffi::resolve_command`, Phase 28/29) only
   once a real adapter decides how a squeeze/roll becomes a `Command`.
6. **Double-tap gesture** (2nd gen and Pro) for tool-switch, and its
   interaction with this product's own command grammar
   (`craftloop_command::grammar`, Phase 19, Article 237) -- does a
   double-tap map to a `CommandAction`, and if so which, is a real
   design decision only testable against real hardware timing.
7. **Pairing, battery, and charging-state interruptions mid-stroke.**
   Apple Pencil's magnetic-attach charging and Bluetooth pairing model is
   its own failure mode distinct from Android's BLE S Pen (Task 205's
   scenario 6) -- how `StrokeLifecycleValidator` (Phase 03) should behave
   if a Pencil disconnects mid-gesture needs real-device observation.
8. **Low-latency rendering under real display/digitizer pipeline
   latency**, same rationale as the Samsung plan's scenario 7 -- iPad's
   ProMotion/120Hz display and Apple Pencil's predicted-touch pipeline
   have their own real latency characteristics distinct from both
   Windows-mouse and Android-S-Pen timing.
9. **Ergonomics during sustained sketching sessions** -- hand/wrist
   fatigue and grip-angle effects on tilt/azimuth data distribution are
   a real usability signal no synthetic test can produce, named
   explicitly in Task 212's own objective ("ergonomics" alongside
   pressure/tilt/hover/palm-rejection/latency).
10. **Screen-protector/tempered-glass friction effects on stroke sampling
    density**, mirroring the Samsung plan's final scenario -- common iPad
    accessory glass changes effective sampling behavior versus the bare
    Apple Pencil-optimized display surface.

## What this plan does not cover

Scenarios already validated via Windows mouse simulation or provable via
Rust-only tests (stroke lifecycle logic, recognition/beautification math,
solver/constraint behavior, document persistence, export, and this
phase's own UniFFI Swift binding-generation smoke test) are out of scope
here, matching the Samsung plan's own boundary.

## Status

**Not yet executed.** No physical iPad or Apple Pencil is available in
this sandboxed development environment, and this sandbox has no macOS/
Xcode/Simulator access at all (confirmed: `which swift`/`swiftc`/
`xcodebuild` all fail). Task 212's objective is to produce this list, not
to run it. Each scenario above should become its own tracked validation
task once real device and macOS build access exist.
