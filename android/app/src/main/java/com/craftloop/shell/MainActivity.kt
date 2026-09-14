// Execution 02, Phases 06-07. Replaces the Execution 01 placeholder
// (one Text() line calling resolveCommand) with a real single-Activity
// Compose host: a live Jetpack Ink canvas wired to CraftLoopSession,
// plus Article 8's Alpha-only Debug Region.

package com.craftloop.shell

import android.os.Bundle
import android.view.MotionEvent
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.ink.strokes.Stroke
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import uniffi.craftloop_mobile_ffi.FfiPointerSample

class MainActivity : ComponentActivity() {
    private val viewModel: CraftLoopViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface {
                    CraftLoopAlphaScreen(viewModel)
                }
            }
        }
    }

    /**
     * Article 11's stylus-only gate, implemented for real this time.
     *
     * Two earlier attempts tried to gate non-stylus pointers *inside*
     * Compose (consuming a `PointerInputChange` in a sibling
     * `pointerInput` block, then a `nextBrush = { ... null }` hook on
     * `InProgressStrokes`). Both were proven wrong by a real on-device
     * touch-swipe test that still produced a committed ink stroke.
     * Reading `InProgressShapesImpl`'s real source
     * (`androidx/ink/authoring/compose/InProgressShapes.kt`, the
     * `androidx.ink:ink-authoring-compose-android:1.0.0` sources jar)
     * explains why: it chains its own
     * `.pointerInput(...).pointerInteropFilter { ... }` internally, and
     * its own doc comment on that exact line says plainly: "the event
     * processing of pointerInteropFilter relative to pointerInput is
     * inconsistent in its order ... causing ordered event consumption
     * logic to be confusing and brittle." Any fix built out of
     * Compose-level consumption or state-update ordering is exactly the
     * kind of "ordered consumption logic" its own authors warn is
     * unreliable against that internal bridge -- which is why both
     * earlier attempts silently failed despite reasoning correctly
     * about the *documented* contract in isolation.
     *
     * `dispatchTouchEvent` runs before the event reaches the View tree
     * at all -- before the root `ComposeView`, before Compose's pointer
     * input system, before `InProgressShapesImpl`'s internal
     * `pointerInput`/`pointerInteropFilter`/`AndroidView` combination
     * gets a chance to see it. This is the standard, un-racy Android
     * mechanism for "this event must never reach certain descendants,"
     * and it does not depend on any ordering assumption between two
     * independent input systems. Real S Pen input reports
     * `MotionEvent.TOOL_TYPE_STYLUS` (Android's own documented API);
     * `adb shell input touchscreen` synthesizes `TOOL_TYPE_FINGER` --
     * confirmed by this session's own real device tests, not assumed.
     *
     * Records the observed tool type for the Debug Region even when
     * swallowing the event, so a rejected touch is still visible as
     * diagnostic evidence (Article 8) rather than silently invisible.
     *
     * Scoped to this phase's actual on-screen content (only the canvas
     * and a non-interactive debug overlay exist yet) -- blocking ALL
     * non-stylus input at the Activity level is behaviorally identical
     * to blocking it only within the canvas region for now. Phase 08
     * (pan/zoom) and Phase 09 (icon toolbar) will need finger input to
     * reach *other* on-screen controls, at which point this gate must
     * narrow to the canvas region specifically -- tracked here, not
     * silently deferred.
     */
    override fun dispatchTouchEvent(ev: MotionEvent): Boolean {
        val toolType = ev.getToolType(0)
        val sourceLabel =
            when (toolType) {
                MotionEvent.TOOL_TYPE_STYLUS -> "stylus"
                MotionEvent.TOOL_TYPE_ERASER -> "eraser"
                MotionEvent.TOOL_TYPE_FINGER -> "touch"
                MotionEvent.TOOL_TYPE_MOUSE -> "mouse"
                else -> "unknown"
            }
        viewModel.setLastPointerSource(sourceLabel)
        if (toolType != MotionEvent.TOOL_TYPE_STYLUS) {
            return false
        }
        return super.dispatchTouchEvent(ev)
    }
}

/** Article 8's screen structure: Canvas Region (most of the screen)
 * plus a collapsible Debug Region. Tool Region (Phase 09) and Context
 * Region (Phase 10/11) do not exist yet -- this phase's whole scope is
 * proving live ink reaches CraftLoopSession and back. */
@Composable
fun CraftLoopAlphaScreen(viewModel: CraftLoopViewModel) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val debugState by viewModel.debugState.collectAsStateWithLifecycle()

    // Task 049/050: committed raw ink is not yet re-materialized as a
    // renderable androidx.ink.strokes.Stroke from CraftLoopSession's
    // FfiSceneSnapshot (the snapshot carries only summaries -- id and
    // sample count, Article 13's coarse contract -- not full geometry
    // Jetpack Ink itself can render). Re-rendering committed ink from
    // the Rust-held FfiPointerSample data is real work with no task
    // number before Phase 09's toolbar exists to select/inspect it;
    // tracked as a named gap in the evidence file rather than silently
    // dropped or faked with a placeholder shape.
    val committedStrokes = remember { mutableStateListOf<Stroke>() }

    Column {
        InkCanvas(
            committedStrokes = committedStrokes,
            onStrokeFinished = { samples: List<FfiPointerSample> ->
                viewModel.submitStroke(samples)
            },
            modifier = Modifier.weight(1f),
        )
        if (uiState.debugOverlayVisible) {
            DebugRegion(
                lastPointerSource = uiState.lastPointerSource,
                revision = debugState.revision,
                canUndo = debugState.canUndo,
                canRedo = debugState.canRedo,
                transactionCount = debugState.transactionCount,
                unresolvedConflictCount = debugState.unresolvedConflictCount,
            )
        }
    }
}

/** Article 8's Debug Region -- Alpha-only diagnostic overlay, not
 * product UI. Deliberately plain (Article 7: no final visual design). */
@Composable
fun DebugRegion(
    lastPointerSource: String,
    revision: ULong,
    canUndo: Boolean,
    canRedo: Boolean,
    transactionCount: UInt,
    unresolvedConflictCount: UInt,
) {
    Card(modifier = Modifier.padding(8.dp)) {
        Column(modifier = Modifier.padding(8.dp)) {
            Text("Debug: pointer=$lastPointerSource revision=$revision")
            Text("txCount=$transactionCount canUndo=$canUndo canRedo=$canRedo")
            Text("unresolvedConflicts=$unresolvedConflictCount")
        }
    }
}
