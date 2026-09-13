# Jetpack Ink → Normalized Input Contract Mapping

Execution 01, Phase 28, Task 204. Authority: MCP Article 127 "Device
Capability Matrix"; Engine Contract 27 (adapters translate platform input
into the shared normalized model, never the reverse).

**Confidence note, per the No-Hallucination Contract**: the `androidx.ink`
API surface described below reflects this assistant's training knowledge
of Jetpack Ink (`androidx.ink:ink-authoring`, `ink-strokes`, `ink-brush`,
`ink-geometry`), not a live fetch of current API documentation (this
sandbox has no Android SDK/docs access -- see `../../android/README.md`).
**Re-verify every class/method/field name against the actual
`androidx.ink` version pinned at integration time before writing real
adapter code against it.** This document's value is the *mapping logic*
(which concept maps to which, and why some conversions are non-trivial),
which does not depend on exact API surface staying identical.

## In-progress vs. final strokes

Jetpack Ink models a stroke's lifecycle in two distinct types:

- **In-progress**: `InProgressStrokesView`/`InProgressStrokeId` plus a
  live `MutableStrokeInputBatch` the app appends `StrokeInput` samples to
  as the user draws, rendered with low-latency prediction.
- **Final**: once the gesture ends, the batch is finalized into an
  immutable `androidx.ink.strokes.Stroke` (an immutable
  `StrokeInputBatch` plus the `Brush` it was drawn with).

This maps directly onto this workspace's own existing two-stage model,
already built in Phases 03/05 without any Android awareness:
`craftloop_input::PointerSample` (one live sample; the Windows harness's
`MouseSimulator`/`StrokeLifecycleValidator` already model the
down/move/up lifecycle Jetpack Ink's in-progress view has) and
`craftloop_ink::Stroke` (the finished, immutable, ID-bearing stroke Phase
05 built). An Android adapter appends one `FfiPointerSample`
(`crates/craftloop-mobile-ffi`, Task 201) per `StrokeInput` as it arrives
from `InProgressStrokesView`, and calls `validate_stroke`/constructs a
real `craftloop_ink::Stroke` once Jetpack Ink finalizes the batch --
exactly the same shape the Windows mouse-simulator adapter already uses,
just from a different platform input source.

## Pressure

`StrokeInput.pressure: Float` (Jetpack Ink; normalized 0.0-1.0, or a
sentinel for "not reported") maps directly to
`FfiPointerSample.pressure: Option<f64>` / `PointerSample.pressure`
(Phase 03, Task 022). The `Option`/nullable-vs-sentinel distinction
matters: this workspace's `InputCapabilities.pressure` flag (Article 4:
never fabricate a capability) must be set from whatever real query
Jetpack Ink/the Android input stack exposes for "does this stylus report
genuine hardware pressure" -- not assumed `true` merely because the field
exists on `StrokeInput`. A mouse or a non-pressure-sensitive touch input
reported through the same `StrokeInput` shape must still map to
`pressure: None` with `capability_pressure: false`.

## Tilt and orientation -- the one genuinely non-trivial conversion

This is the mapping this document exists to get right, because it is
**not** a direct field copy:

- **Jetpack Ink's model is polar**: `StrokeInput.tiltRadians` (angle from
  the perpendicular-to-screen axis, 0 = pen straight up) and
  `StrokeInput.orientationRadians` (compass-style azimuth of the tilt
  direction around the z-axis) -- one magnitude, one direction.
- **This workspace's model is Cartesian**: `tilt_x_deg`/`tilt_y_deg`
  (Phase 03, Task 022's `PointerSample`) -- independent tilt components
  along the screen's x and y axes, matching how Windows `PointerInfo`/
  iPad `UITouch.azimuthAngle`+`altitudeAngle` (Phase 29) more naturally
  decompose, and how most stylus SDKs outside Android report it.

The correct conversion (standard polar-to-Cartesian, both in degrees) an
Android adapter must perform -- **not** a direct field rename:

```text
tilt_x_deg = tiltRadians.toDegrees() * cos(orientationRadians)
tilt_y_deg = tiltRadians.toDegrees() * sin(orientationRadians)
```

Getting this backwards (treating `tiltRadians` as `tilt_x_deg` and
`orientationRadians` as `tilt_y_deg` directly) would silently produce
plausible-looking but wrong numbers -- exactly the kind of adapter bug
that would pass a superficial "does it compile" check and fail only on a
real device at specific pen angles, which is why it is named explicitly
here rather than left for whoever writes the real Android adapter to
discover.

## Storage / finalized-stroke persistence

Jetpack Ink's finalized `Stroke` (`StrokeInputBatch` + `Brush`) is an
Android-process-local object, not itself a serializable document format.
The adapter's job at finalization is exactly what `craftloop_ink::Stroke::new`
(this crate's `validate_stroke`, Task 201) already does: convert every
`StrokeInput` in the batch to an `FfiPointerSample`, validate the whole
batch forms one coherent gesture (non-empty, single `PointerSource`), and
hand the result to the shared `craftloop-document`/`craftloop-transactions`
persistence path (Phase 07/08) -- which already has no Android-specific
code and needs none, per Engine Contract 27's own boundary.

## Capability flags

| `InputCapabilities` field | Jetpack Ink / Android signal |
|---|---|
| `pressure` | `MotionEvent.getToolType()` is `TOOL_TYPE_STYLUS` and the device's input device info reports a pressure axis range wider than the default binary 0/1 (`InputDevice.getMotionRange(MotionEvent.AXIS_PRESSURE)`). |
| `tilt` | Stylus tool type and `InputDevice.getMotionRange(MotionEvent.AXIS_TILT)` present -- not all S Pen generations report this. |
| `hover` | `MotionEvent.ACTION_HOVER_MOVE` support, queryable via `InputDevice` source flags; not universal across Android stylus hardware. |
| `palm_rejection` | Not a single queryable flag -- inferred from whether the OS/input stack is already filtering `MotionEvent.ACTION_CANCEL`/classification for touch vs. stylus concurrently, which is exactly why Task 205 names real-device palm-rejection testing as unable to be simulated. |
| `eraser` | `MotionEvent.getToolType()` reports `TOOL_TYPE_ERASER`, or the S Pen's physical eraser button state via `InputDevice` button flags (Samsung-specific extensions vary by device generation). |

Every row above is a *real query the adapter must perform*, never a
constant `true` -- Article 4's rule, reiterated here because it is the
rule most likely to be silently violated by an adapter author reaching
for "just set pressure=true, S Pens all have pressure" as a shortcut.
