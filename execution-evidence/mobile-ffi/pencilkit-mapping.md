# PencilKit → Normalized Input Contract Mapping

Execution 01, Phase 29, Task 209. Authority: MCP Article 127 "Device
Capability Matrix"; Engine Contract 27.

**Confidence note, per the No-Hallucination Contract**: the `PencilKit`
API surface below (`PKDrawing`, `PKStroke`, `PKStrokePoint`) reflects
training knowledge, not a live fetch of current Apple documentation (no
macOS/Xcode docs access in this sandbox -- see
`uniffi-swift-decision-record.md`). **Re-verify every type/property name
against the current PencilKit SDK before writing real adapter code.**
This document's lasting value is the mapping *logic*, mirroring
`jetpack-ink-mapping.md`'s Android counterpart (Task 204) so both mobile
platforms feed the exact same `FfiPointerSample` shape
(`crates/craftloop-mobile-ffi`, Phase 28 Task 201, confirmed Swift-capable
with zero changes -- Task 207).

## In-progress vs. final strokes

PencilKit's live-drawing surface is `PKCanvasView`, which accumulates
strokes into a `PKDrawing` (`drawing.strokes: [PKStroke]`) as the user
draws; there is no separate "in-progress stroke" handle exposed the way
Jetpack Ink's `InProgressStrokeId` is -- PencilKit's live rendering is
managed internally by `PKCanvasView` and only becomes inspectable data
once a stroke is appended to `drawing.strokes`. This is a real,
documented platform difference from Android worth naming explicitly: an
iPad adapter built directly against `PKCanvasView` would need to poll or
observe `drawing.strokes` changes (e.g. via `PKCanvasViewDelegate`'s
`canvasViewDrawingDidChange`) to get per-stroke data as it completes,
rather than streaming individual samples the way a hand-rolled
`UIGestureRecognizer`/`UITouch`-based capture path (bypassing
`PKCanvasView` entirely) would. **Which of these two approaches Craft
Loop's real iPad adapter uses is exactly Task 210's PaperKit-adjacent
boundary decision, not resolved here** -- this document only maps the
data shape once samples exist, from whichever capture path is chosen.

## Stroke point data

`PKStrokePoint` (found on `PKStroke.path`, a `PKStrokePath` of these)
carries `location: CGPoint`, `timeOffset: TimeInterval`, `size: CGSize`,
`opacity: CGFloat`, `force: CGFloat` (pressure), `azimuth: CGFloat`
(radians), and `altitude: CGFloat` (radians) -- maps onto
`FfiPointerSample` as follows:

| PencilKit | `FfiPointerSample` |
|---|---|
| `location.x` / `location.y` | `x` / `y` |
| `timeOffset` (relative to stroke start) | `timestamp_seconds` (this workspace's own convention is stream-relative, not wall-clock -- matches directly, see `craftloop_input::PointerSample`'s own doc comment) |
| `force` | `pressure` (`Option<f64>`; PencilKit reports `0.0` for a device with no pressure sensing rather than a true absent value -- the adapter must map that to `None`, not `Some(0.0)`, to avoid fabricating a "zero pressure was measured" claim Article 4 forbids) |
| `azimuth` + `altitude` | `tilt_x_deg` / `tilt_y_deg` -- **the same non-trivial polar-to-Cartesian conversion `jetpack-ink-mapping.md` documents for Android**, with Apple's own convention: `altitude` is the angle from the *screen plane* (not from vertical, the reverse of Jetpack Ink's `tiltRadians`), so the conversion is `tilt_magnitude_deg = (90° - altitude.toDegrees())`, then `tilt_x_deg = tilt_magnitude_deg * cos(azimuth)`, `tilt_y_deg = tilt_magnitude_deg * sin(azimuth)`. Getting the altitude/vertical convention backwards here is the iPad-specific version of the same class of silent bug the Android document names. |

## Storage / finalized-stroke persistence

Same shape as the Android mapping: once a `PKStroke` is complete, convert
every `PKStrokePoint` to an `FfiPointerSample` and hand the batch to
`validate_stroke`/`craftloop_ink::Stroke::new` -- the same shared-core
call path Phase 28's Kotlin adapter would use, now proven Swift-callable
by this phase's binding smoke test. `PKDrawing` itself has its own
binary serialization (`PKDrawing.dataRepresentation()`), but that format
is PencilKit/Apple-specific presentation data (stroke ink rendering,
brush/tool metadata), not this product's engineering-semantic storage --
`craftloop-document`'s persistence (Phase 07/08) remains the single
source of truth for saved documents, matching Article 128's document
model, exactly as Task 210 requires for PaperKit.

## Capability flags

| `InputCapabilities` field | PencilKit / iPadOS signal |
|---|---|
| `pressure` | `UITouch.type == .pencil` and `PKStrokePoint.force` reporting non-degenerate variance (Apple Pencil 1st/2nd gen and Pro all report real force; a finger touch reports a fixed/absent value that must not be mapped to `true`). |
| `tilt` | `UITouch.type == .pencil` -- all Apple Pencil generations report `azimuthAngle`/`altitudeAngle`, unlike some Android S Pen generations (Task 204's mapping notes tilt as *not* universal on Android; it is effectively universal for any real Apple Pencil, but the adapter should still query rather than hardcode `true`, per Article 4). |
| `hover` | Apple Pencil 2 (on supported iPads) and Apple Pencil Pro report hover via `UIHoverGestureRecognizer`/`UIPointerInteraction`; 1st-generation Apple Pencil does not -- a real per-device query, not a blanket `true` for "any Apple Pencil." |
| `palm_rejection` | Handled transparently by `PKCanvasView`'s own touch-type filtering when using PencilKit's built-in capture path; if Task 210 leads to a raw `UITouch`-based capture path instead, palm rejection becomes the adapter's own responsibility to reimplement -- a real architectural cost of bypassing `PKCanvasView`, worth flagging for whoever makes that call. |
| `eraser` | Apple Pencil Pro's squeeze gesture and PencilKit's `PKEraserTool` are a UI-level tool selection, not a physical eraser-end sensor the way some S Pens have (Task 204's Android eraser row) -- this capability maps to "the app's current tool is eraser," a `craftloop-command` (Phase 19) state, not a hardware capability query at all. Recorded here as a deliberate cross-platform asymmetry, not an oversight. |
