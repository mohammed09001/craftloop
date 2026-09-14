// Execution 02, Phases 06-08. Replaces the Execution 01 placeholder
// (one Text() line calling resolveCommand) with a real single-Activity
// Compose host: a live Jetpack Ink canvas wired to CraftLoopSession,
// pan/zoom/selection/deletion/fit-to-content (Phase 08), plus Article
// 8's Alpha-only Debug Region.

package com.craftloop.shell

import android.os.Bundle
import android.view.GestureDetector
import android.view.MotionEvent
import android.view.ScaleGestureDetector
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
import androidx.compose.ui.layout.boundsInWindow
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.unit.dp
import androidx.ink.strokes.Stroke
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import uniffi.craftloop_mobile_ffi.FfiPointerSample

class MainActivity : ComponentActivity() {
    private val viewModel: CraftLoopViewModel by viewModels()

    // Task 053/054: real Android gesture detectors driving the
    // Kotlin-local viewport, fed only the events dispatchTouchEvent
    // below decides are "finger input inside the canvas" -- not built
    // on Compose's own pointerInput/detectTransformGestures, since this
    // phase's stylus-only gate already proved that competing with
    // InProgressStrokes' internal pointerInteropFilter bridge from
    // *inside* Compose is unreliable (see InkCanvas.kt's doc comment).
    // Handling gestures here, entirely outside that bridge, sidesteps
    // the same brittleness rather than risking it a third way.
    private val gestureDetector by lazy {
        GestureDetector(
            this,
            object : GestureDetector.SimpleOnGestureListener() {
                override fun onScroll(
                    e1: MotionEvent?,
                    e2: MotionEvent,
                    distanceX: Float,
                    distanceY: Float,
                ): Boolean {
                    // GestureDetector reports the distance already
                    // *travelled* (i.e. the delta to subtract to follow
                    // the finger), not a target delta to add.
                    viewModel.applyViewportGesture(-distanceX, -distanceY, 1f, e2.x, e2.y)
                    return true
                }

                override fun onSingleTapUp(e: MotionEvent): Boolean {
                    // Task 055/Eraser: select or erase on a stylus tap
                    // only. Real bug found and fixed here (Phase 08):
                    // this same `gestureDetector` is also fed finger
                    // events (for `onScroll` pan), so a plain finger tap
                    // -- zero movement, same as any pan gesture's
                    // degenerate case -- also satisfies Android's own
                    // `onSingleTapUp` contract and would select/erase
                    // too if this checked nothing. The tool-type check
                    // is against this exact event, not which
                    // dispatchTouchEvent branch happened to feed the
                    // detector, so it holds regardless of call site.
                    if (e.getToolType(0) == MotionEvent.TOOL_TYPE_STYLUS) {
                        if (viewModel.uiState.value.activeTool == Tool.ERASER) {
                            viewModel.eraseAt(e.x, e.y)
                        } else {
                            viewModel.selectAt(e.x, e.y)
                        }
                    }
                    return true
                }
            },
        )
    }

    // Task 060: Line/Circle/Rectangle's real drawing mode -- a stylus
    // down/up pair captured directly in dispatchTouchEvent (not routed
    // through InProgressStrokes/Jetpack Ink at all, since these tools
    // create one direct primitive from two points, not a freeform ink
    // stroke). `null` outside an active drag.
    private var shapeDragStart: Pair<Float, Float>? = null
    private val scaleGestureDetector by lazy {
        ScaleGestureDetector(
            this,
            object : ScaleGestureDetector.SimpleOnScaleGestureListener() {
                override fun onScale(detector: ScaleGestureDetector): Boolean {
                    viewModel.applyViewportGesture(
                        0f,
                        0f,
                        detector.scaleFactor,
                        detector.focusX,
                        detector.focusY,
                    )
                    return true
                }
            },
        )
    }

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
     * Article 11's stylus-only-draws / finger-navigates gate.
     *
     * **Task 052's real scope, corrected from Phase 07's version**:
     * Phase 07 blocked every non-stylus `MotionEvent` outright, which
     * was correct for proving the ink surface alone but is not Article
     * 11's actual policy -- "Finger -> pan, zoom, interface controls"
     * requires finger input to keep working, just never as ink. This
     * version routes by *where* and *what tool is active*, still
     * deciding before the event ever reaches the View tree (the one
     * part of the three-attempt ink-gate saga that was proven reliable
     * -- see `InkCanvas.kt`'s doc comment for why anything relying on
     * Compose-internal ordering against `InProgressStrokes` was not):
     *
     * - Outside the canvas region (e.g. over a Phase 08 control button
     *   below it): always forwarded normally, regardless of tool type
     *   -- buttons need real finger taps.
     * - Inside the canvas region, stylus/eraser input while the Pen
     *   tool is active: forwarded normally, reaching `InProgressStrokes`
     *   exactly as Phase 07 proved works.
     * - Inside the canvas region, stylus input while a non-drawing tool
     *   (Select) is active: never forwarded to the View tree (Select
     *   mode must not draw ink either); fed to `gestureDetector` instead
     *   so a real stylus tap can hit-test a primitive.
     * - Inside the canvas region, non-stylus input: never forwarded to
     *   the View tree (must never draw ink, Article 11's core rule);
     *   fed to the scale/gesture detectors for pan/zoom instead.
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

        // Real bug found and fixed here: `canvasBoundsPx` is null until
        // Compose's first `onGloballyPositioned` callback fires, which
        // has not necessarily happened yet on the very first touch
        // after a cold launch. Treating "bounds unknown" as "outside
        // the canvas" (the original version of this check) fails open
        // -- it forwards the event unconditionally via
        // `super.dispatchTouchEvent`, including a finger event, which
        // then reaches `InProgressStrokes` with nothing filtering it at
        // all. A real on-device test caught this: a single synthetic
        // touch swipe run immediately after a fresh launch produced
        // five committed transactions. Fail closed instead: unknown
        // bounds means "assume inside the canvas" (true almost always
        // anyway -- the canvas fills nearly the whole screen), which
        // routes through the same stylus/tool gating below rather than
        // skipping it.
        val bounds = viewModel.canvasBoundsPx
        val insideCanvas =
            bounds == null || bounds.contains(androidx.compose.ui.geometry.Offset(ev.x, ev.y))
        if (!insideCanvas) {
            return super.dispatchTouchEvent(ev)
        }

        val isStylus = toolType == MotionEvent.TOOL_TYPE_STYLUS || toolType == MotionEvent.TOOL_TYPE_ERASER
        val activeTool = viewModel.uiState.value.activeTool
        val shapeToolActive = activeTool == Tool.LINE || activeTool == Tool.CIRCLE || activeTool == Tool.RECTANGLE

        return if (isStylus && activeTool == Tool.PEN) {
            super.dispatchTouchEvent(ev)
        } else if (isStylus && shapeToolActive) {
            // Task 060: Line/Circle/Rectangle -- capture the drag's
            // down/up points directly, never reaching InProgressStrokes
            // (this is not freeform ink, so it must not create any).
            when (ev.actionMasked) {
                MotionEvent.ACTION_DOWN -> shapeDragStart = ev.x to ev.y
                MotionEvent.ACTION_UP -> {
                    val start = shapeDragStart
                    shapeDragStart = null
                    if (start != null) {
                        viewModel.createShapeFromDrag(activeTool, start.first, start.second, ev.x, ev.y)
                    }
                }
                MotionEvent.ACTION_CANCEL -> shapeDragStart = null
            }
            true
        } else if (isStylus) {
            // Select/Eraser tool: real stylus tap, hit-test instead of
            // ink (which one happens is decided in onSingleTapUp above,
            // by the same real active-tool state).
            gestureDetector.onTouchEvent(ev)
            true
        } else {
            // Finger/mouse inside the canvas: pan/zoom, never ink.
            scaleGestureDetector.onTouchEvent(ev)
            gestureDetector.onTouchEvent(ev)
            true
        }
    }
}

/** Article 8's screen structure: Canvas Region (most of the screen)
 * plus a collapsible Debug Region and (Phase 08) a temporary control
 * row -- Phase 09 replaces this row with the real icon toolbar; these
 * are plain text buttons only to make Select/Delete/Fit/Undo/Redo
 * exercisable and verifiable on-device before that exists. */
@Composable
fun CraftLoopAlphaScreen(viewModel: CraftLoopViewModel) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val debugState by viewModel.debugState.collectAsStateWithLifecycle()
    val sceneSnapshot by viewModel.sceneSnapshot.collectAsStateWithLifecycle()

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
            viewport = uiState.viewport,
            modifier =
                Modifier.weight(1f).onGloballyPositioned { coordinates ->
                    // `boundsInWindow` matches the coordinate space
                    // `MotionEvent.x`/`.y` arrive in at
                    // `Activity.dispatchTouchEvent` (window-relative,
                    // not root-Composable-relative) -- using
                    // `boundsInRoot`/`positionInRoot` here would silently
                    // misalign the hit-test whenever the window itself
                    // isn't at (0,0), e.g. multi-window/split-screen.
                    viewModel.canvasBoundsPx = coordinates.boundsInWindow()
                },
        )
        // Phase 09's real icon-first toolbar (Article 7) replaces
        // Phase 08's temporary text-button row.
        PrimaryToolbar(viewModel)
        if (uiState.debugOverlayVisible) {
            DebugRegion(
                lastPointerSource = uiState.lastPointerSource,
                revision = debugState.revision,
                canUndo = debugState.canUndo,
                canRedo = debugState.canRedo,
                transactionCount = debugState.transactionCount,
                unresolvedConflictCount = debugState.unresolvedConflictCount,
                selectedCount = sceneSnapshot.selectedEntityIds.size,
                primitiveCount = sceneSnapshot.primitives.size,
                zoom = uiState.viewport.zoom,
                panX = uiState.viewport.panX,
                panY = uiState.viewport.panY,
                activeTool = uiState.activeTool.name,
                lastActionMessage = uiState.lastActionMessage,
                onInsertTestLine = { viewModel.debugInsertTestLine() },
                onSelectFirstPrimitive = { viewModel.debugSelectFirstPrimitive() },
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
    selectedCount: Int,
    primitiveCount: Int,
    zoom: Float,
    panX: Float,
    panY: Float,
    activeTool: String,
    lastActionMessage: String,
    onInsertTestLine: () -> Unit,
    onSelectFirstPrimitive: () -> Unit,
) {
    Card(modifier = Modifier.padding(8.dp)) {
        Column(modifier = Modifier.padding(8.dp)) {
            Text("Debug: pointer=$lastPointerSource revision=$revision tool=$activeTool")
            Text("txCount=$transactionCount canUndo=$canUndo canRedo=$canRedo")
            Text("unresolvedConflicts=$unresolvedConflictCount")
            Text("selected=$selectedCount primitives=$primitiveCount")
            Text("zoom=%.2f pan=(%.0f, %.0f)".format(zoom, panX, panY))
            if (lastActionMessage.isNotEmpty()) {
                Text("last=$lastActionMessage")
            }
            // Debug-only verification helpers (see CraftLoopViewModel's
            // own doc comments on debugInsertTestLine/
            // debugSelectFirstPrimitive) -- no physical stylus exists
            // in this harness to draw/select a real primitive, so these
            // stand in for it. Alpha-only diagnostic, not product UI.
            androidx.compose.foundation.layout.Row {
                androidx.compose.material3.TextButton(onClick = onInsertTestLine) { Text("+Line") }
                androidx.compose.material3.TextButton(onClick = onSelectFirstPrimitive) { Text("Select 1st") }
            }
        }
    }
}
