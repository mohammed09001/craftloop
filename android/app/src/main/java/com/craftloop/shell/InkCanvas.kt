// Execution 02, Phase 07 (Tasks 044-051). Article 9's Immediate Ink
// Architecture: androidx.ink renders the in-progress stroke live and
// entirely on-device; only the *completed* stroke, normalized into
// FfiPointerSample, crosses into CraftLoopSession.submitStroke.
//
// Exact API shapes below (InProgressStrokes' real parameter list,
// StrokeInput's NO_PRESSURE/NO_TILT sentinels, CanvasStrokeRenderer's
// android.graphics.Canvas/Matrix types, StockBrushes.pressurePen())
// were confirmed against the real androidx.ink 1.0.0 *sources* jars
// (downloaded from maven.google.com and read directly), not guessed --
// the public reference doc pages for these classes render as an empty
// JS shell to this tooling's fetcher, so this was the only reliable
// ground truth available. See phase-06-07-state-and-ink-surface.md.

package com.craftloop.shell

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.input.pointer.PointerEventPass
import androidx.compose.ui.input.pointer.PointerType
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.ink.authoring.compose.InProgressStrokes
import androidx.ink.brush.Brush
import androidx.ink.brush.StockBrushes
import androidx.ink.rendering.android.canvas.CanvasStrokeRenderer
import androidx.ink.strokes.Stroke
import uniffi.craftloop_mobile_ffi.FfiPointerSample
import uniffi.craftloop_mobile_ffi.FfiPointerSource

/** Task 044: one plain, functional brush -- no final visual design
 * (Article 7). Pressure-sensitive stock brush, since Gate P's whole
 * point is observing whatever pressure the real device reports. */
private fun defaultBrush(): Brush =
    Brush.createWithColorIntArgb(
        family = StockBrushes.pressurePen(),
        colorIntArgb = Color.Black.toArgb(),
        size = 5f,
        epsilon = 0.1f,
    )

/** Task 047: normalize one finished androidx.ink Stroke's real input
 * samples into FfiPointerSample. [StrokeInput.hasPressure]/[hasTilt]
 * are the library's own real-vs-absent check (backed by the
 * NO_PRESSURE/NO_TILT sentinels), not a guess. Capability flags come
 * from a real, queried MotionRange check (Article 24/27 -- never
 * hardcoded true). `source` is always Stylus: [InkCanvas]'s own
 * pointer-input gate below never lets a non-stylus pointer reach
 * [InProgressStrokes] in the first place, so every finished stroke
 * this callback ever sees already is one. */
private fun normalizeStroke(
    stroke: Stroke,
    capabilities: RealInputCapabilities,
): List<FfiPointerSample> {
    val inputs = stroke.inputs
    val samples = mutableListOf<FfiPointerSample>()
    for (i in 0 until inputs.size) {
        val input = inputs[i]
        samples.add(
            FfiPointerSample(
                x = input.x.toDouble(),
                y = input.y.toDouble(),
                timestampSeconds = input.elapsedTimeMillis / 1000.0,
                pressure = if (input.hasPressure) input.pressure.toDouble() else null,
                tiltXDeg = if (input.hasTilt) Math.toDegrees(input.tiltRadians.toDouble()) else null,
                tiltYDeg = null,
                source = FfiPointerSource.STYLUS,
                buttonPrimary = false,
                buttonSecondary = false,
                buttonBarrel = false,
                capabilityPressure = capabilities.pressure,
                capabilityTilt = capabilities.tilt,
                capabilityHover = capabilities.hover,
                capabilityPalmRejection = capabilities.palmRejection,
                capabilityEraser = capabilities.eraser,
            ),
        )
    }
    return samples
}

/** Task 087/088/089/090/091 groundwork (Phase 13 does the full
 * physical diagnostics pass): real, queried capability flags, not
 * assumptions. Palm rejection is left `false` -- this Alpha implements
 * no rejection logic beyond the stylus-only input gate below, and
 * Android exposes no direct "device supports palm rejection" query to
 * report instead. */
data class RealInputCapabilities(
    val pressure: Boolean,
    val tilt: Boolean,
    val hover: Boolean,
    val palmRejection: Boolean,
    val eraser: Boolean,
)

fun queryRealInputCapabilities(context: android.content.Context): RealInputCapabilities {
    val inputManager =
        context.getSystemService(android.content.Context.INPUT_SERVICE) as? android.hardware.input.InputManager
    var pressure = false
    var tilt = false
    var hover = false
    val deviceIds = inputManager?.inputDeviceIds ?: IntArray(0)
    for (id in deviceIds) {
        val device = inputManager?.getInputDevice(id) ?: continue
        val isStylusSource =
            (device.sources and android.view.InputDevice.SOURCE_STYLUS) == android.view.InputDevice.SOURCE_STYLUS
        if (!isStylusSource) continue
        if (device.getMotionRange(android.view.MotionEvent.AXIS_PRESSURE) != null) pressure = true
        if (device.getMotionRange(android.view.MotionEvent.AXIS_TILT) != null) tilt = true
        if (device.supportsSource(android.view.InputDevice.SOURCE_STYLUS)) hover = true
    }
    return RealInputCapabilities(
        pressure = pressure,
        tilt = tilt,
        hover = hover,
        palmRejection = false,
        eraser = false,
    )
}

/**
 * Article 8's Canvas Region. Task 052 (finger-navigation-vs-pen-drawing
 * separation) is Phase 08's full scope; this phase only implements the
 * narrower Article 11 gate this task needs regardless: a non-stylus
 * pointer must never become ink.
 *
 * [InProgressStrokes] takes no `Modifier` (its real source, read
 * directly from the 1.0.0 sources jar, confirms this) and, per its own
 * documented contract, skips any pointer whose
 * `PointerInputChange.isConsumed` is already `true` on its down event.
 * The outer `Box`'s `pointerInput` modifier below runs at
 * [PointerEventPass.Initial] -- before children, per Compose's own
 * pass ordering -- and consumes every non-stylus pointer there, which
 * is exactly the mechanism the library's own doc comment names for
 * excluding specific pointers from drawing. Verified for real on the
 * connected device in Part E, not assumed from the doc comment alone.
 */
@Composable
fun InkCanvas(
    committedStrokes: List<Stroke>,
    onStrokeFinished: (List<FfiPointerSample>) -> Unit,
    onPointerSourceObserved: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    val context = LocalContext.current
    val capabilities = remember { queryRealInputCapabilities(context) }
    val renderer = remember { CanvasStrokeRenderer.create() }

    Box(
        modifier =
            modifier
                .fillMaxSize()
                .pointerInput(Unit) {
                    awaitPointerEventScope {
                        while (true) {
                            val event = awaitPointerEvent(PointerEventPass.Initial)
                            val change = event.changes.firstOrNull() ?: continue
                            val sourceLabel =
                                when (change.type) {
                                    PointerType.Stylus -> "stylus"
                                    PointerType.Eraser -> "eraser"
                                    PointerType.Touch -> "touch"
                                    PointerType.Mouse -> "mouse"
                                    else -> "unknown"
                                }
                            onPointerSourceObserved(sourceLabel)
                            if (change.type != PointerType.Stylus) {
                                // Article 11: finger/mouse input must
                                // never create ink -- consumed here,
                                // before InProgressStrokes ever sees it.
                                event.changes.forEach { it.consume() }
                            }
                        }
                    }
                },
    ) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            committedStrokes.forEach { stroke ->
                renderer.draw(
                    canvas = drawContext.canvas.nativeCanvas,
                    stroke = stroke,
                    strokeToScreenTransform = android.graphics.Matrix(),
                )
            }
        }
        InProgressStrokes(
            defaultBrush = remember { defaultBrush() },
            onStrokesFinished = { finished ->
                finished.forEach { stroke ->
                    val samples = normalizeStroke(stroke, capabilities)
                    if (samples.isNotEmpty()) onStrokeFinished(samples)
                }
            },
        )
    }
}
