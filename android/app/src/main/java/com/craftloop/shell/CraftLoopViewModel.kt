// Execution 02, Phase 06 (Tasks 039-043). Owns the one CraftLoopSession
// for this Activity's lifetime; survives configuration changes because
// a ViewModel does, by construction (Task 043) -- no extra handling
// needed here beyond simply holding it in the ViewModel rather than in
// the Activity/Composable.

package com.craftloop.shell

import androidx.compose.ui.geometry.Rect
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uniffi.craftloop_mobile_ffi.CraftLoopSession
import uniffi.craftloop_mobile_ffi.FfiDebugState
import uniffi.craftloop_mobile_ffi.FfiPointerSample
import uniffi.craftloop_mobile_ffi.FfiSceneSnapshot
import kotlin.math.max
import kotlin.math.min

/** Task 039/040: the tools a user can select. Purely an Android UI
 * concept -- CraftLoopSession has no notion of "active tool" at all,
 * only of what operations are actually invoked (Task 041's
 * ephemeral-vs-semantic split). Only Pen exists as a real drawing tool
 * this phase; the rest are named now so Phase 09's toolbar has a fixed
 * target enum to bind against, not built out yet. */
enum class Tool {
    PEN,
    ERASER,
    SELECT,
    LINE,
    CIRCLE,
    RECTANGLE,
}

/** Task 053/054: pan/zoom viewport. Purely an Android presentation
 * concept -- CraftLoopSession/Document have no notion of "current
 * viewport" (confirmed by reading session.rs before adding this; no
 * existing FFI state was reused because none exists to reuse). Never
 * crosses the FFI boundary, never persisted. */
data class ViewportState(
    val panX: Float = 0f,
    val panY: Float = 0f,
    val zoom: Float = 1f,
)

/** Task 041: ephemeral UI state -- never crosses the FFI boundary,
 * never persisted, rebuilt fresh on every process restart. Kept in one
 * data class (rather than several loose StateFlows) so a Composable
 * can collect one flow for all of it. */
data class EphemeralUiState(
    val activeTool: Tool = Tool.PEN,
    val lastPointerSource: String = "none",
    val debugOverlayVisible: Boolean = true,
    val viewport: ViewportState = ViewportState(),
)

class CraftLoopViewModel : ViewModel() {
    // Task 039: the real session, created once, held for this
    // ViewModel's lifetime (which spans configuration changes).
    private val session = CraftLoopSession()

    private val _sceneSnapshot = MutableStateFlow(session.sceneSnapshot())
    val sceneSnapshot: StateFlow<FfiSceneSnapshot> = _sceneSnapshot.asStateFlow()

    private val _debugState = MutableStateFlow(session.debugState())
    val debugState: StateFlow<FfiDebugState> = _debugState.asStateFlow()

    private val _uiState = MutableStateFlow(EphemeralUiState())
    val uiState: StateFlow<EphemeralUiState> = _uiState.asStateFlow()

    /** Task 042: refresh strategy is eager -- re-read both snapshots
     * after every single mutating session call. Article 26 forbids
     * optimizing without measurement, and no measurement exists yet
     * that this is too slow for a single-user Alpha document; batching
     * would be premature optimization against a problem not yet shown
     * to exist. */
    private fun refreshSnapshots() {
        _sceneSnapshot.value = session.sceneSnapshot()
        _debugState.value = session.debugState()
    }

    fun setActiveTool(tool: Tool) {
        _uiState.value = _uiState.value.copy(activeTool = tool)
    }

    fun setLastPointerSource(source: String) {
        _uiState.value = _uiState.value.copy(lastPointerSource = source)
    }

    /** Task 052/053: on-screen bounds of the canvas region, updated by
     * [InkCanvas]'s own `Modifier.onGloballyPositioned`. A plain field,
     * not a `StateFlow` -- `MainActivity.dispatchTouchEvent` (not a
     * Composable) reads only the current value at each real touch
     * event, and nothing needs to recompose when this changes. */
    var canvasBoundsPx: Rect? = null

    /** Task 053/054: apply one incremental pan/zoom gesture step.
     * `focus` is the screen-space point the zoom should appear to pivot
     * around (a pinch gesture's focal point); pan-only calls pass
     * `zoomFactor = 1f`. Content is rendered with `TransformOrigin(0,0)`
     * (see `InkCanvas`), so the screen/content mapping is the plain
     * affine `screen = content * zoom + pan` -- this is the exact
     * algebra that keeps the point under the user's fingers fixed
     * across a zoom step, not an approximation. */
    fun applyViewportGesture(
        panDeltaX: Float,
        panDeltaY: Float,
        zoomFactor: Float,
        focusX: Float,
        focusY: Float,
    ) {
        val current = _uiState.value.viewport
        val newZoom = (current.zoom * zoomFactor).coerceIn(0.1f, 10f)
        // Keep (focusX, focusY) fixed on screen while zoom changes:
        // content point under the focus before = (focus - pan) / zoom;
        // solve for the new pan that keeps that same content point under
        // the same screen focus at the new zoom.
        val contentUnderFocusX = (focusX - current.panX) / current.zoom
        val contentUnderFocusY = (focusY - current.panY) / current.zoom
        val newPanX = focusX - contentUnderFocusX * newZoom + panDeltaX
        val newPanY = focusY - contentUnderFocusY * newZoom + panDeltaY
        _uiState.value =
            _uiState.value.copy(
                viewport = ViewportState(panX = newPanX, panY = newPanY, zoom = newZoom),
            )
    }

    /** Task 057: frame every current structured primitive's bounding
     * box, with a margin, at zoom 1 or less (never zooms *in* past 1x
     * for a single small primitive -- Article 7's "intentionally
     * simple," not a polished camera behavior). Does nothing if there
     * is no structured geometry yet, rather than resetting the user's
     * current view to an arbitrary default. */
    fun fitToContent(
        viewportWidthPx: Float,
        viewportHeightPx: Float,
    ) {
        val primitives = _sceneSnapshot.value.primitives
        if (primitives.isEmpty()) return
        var minX = Float.MAX_VALUE
        var minY = Float.MAX_VALUE
        var maxX = -Float.MAX_VALUE
        var maxY = -Float.MAX_VALUE
        for (p in primitives) {
            minX = min(minX, p.minX.toFloat())
            minY = min(minY, p.minY.toFloat())
            maxX = max(maxX, p.maxX.toFloat())
            maxY = max(maxY, p.maxY.toFloat())
        }
        val contentWidth = (maxX - minX).coerceAtLeast(1f)
        val contentHeight = (maxY - minY).coerceAtLeast(1f)
        val margin = 0.9f
        val zoom =
            min(
                (viewportWidthPx / contentWidth) * margin,
                (viewportHeightPx / contentHeight) * margin,
            ).coerceIn(0.1f, 1f)
        val contentCenterX = (minX + maxX) / 2f
        val contentCenterY = (minY + maxY) / 2f
        val panX = viewportWidthPx / 2f - contentCenterX * zoom
        val panY = viewportHeightPx / 2f - contentCenterY * zoom
        _uiState.value = _uiState.value.copy(viewport = ViewportState(panX = panX, panY = panY, zoom = zoom))
    }

    /** Task 055: hit-test a stylus tap (already known to be a real
     * `MotionEvent.TOOL_TYPE_STYLUS` event by the time this is called --
     * see `MainActivity.dispatchTouchEvent`) against every current
     * structured primitive's bounding box, converting the screen-space
     * tap into content space via the current viewport's inverse
     * transform. Selects the first bounding-box hit (expanded by a
     * small tolerance so a thin line is actually tappable) or clears
     * selection if nothing was hit -- a real, if coarse, hit test using
     * the real bounding boxes `scene_snapshot()` now carries (Phase 08's
     * own addition to `FfiPrimitiveSummary`), not a placeholder. */
    fun selectAt(
        screenX: Float,
        screenY: Float,
    ) {
        val viewport = _uiState.value.viewport
        val contentX = (screenX - viewport.panX) / viewport.zoom
        val contentY = (screenY - viewport.panY) / viewport.zoom
        val tolerance = 24f / viewport.zoom
        val hit =
            _sceneSnapshot.value.primitives.firstOrNull { p ->
                contentX >= p.minX - tolerance && contentX <= p.maxX + tolerance &&
                    contentY >= p.minY - tolerance && contentY <= p.maxY + tolerance
            }
        viewModelScope.launch {
            if (hit != null) {
                session.select(listOf(hit.id))
            } else {
                session.clearSelection()
            }
            refreshSnapshots()
        }
    }

    /** Task 056: delete every currently selected entity through the
     * real session transaction (same undo/redo mechanism as every other
     * mutation -- `CraftLoopSession.deleteSelected` commits through
     * `DocumentHistory` exactly like `submitStroke` does). */
    fun deleteSelected() {
        val ids = _sceneSnapshot.value.selectedEntityIds
        if (ids.isEmpty()) return
        viewModelScope.launch {
            session.deleteSelected(ids)
            refreshSnapshots()
        }
    }

    /** Phase 08 on-device verification helper only, not a product
     * feature -- Phase 09/10 give the user a real way to create
     * structured geometry (typed Line/Circle/Rectangle tools, or a real
     * stylus stroke through recognition). Nothing in this Alpha can
     * currently create a `Primitive` without either a real S Pen stroke
     * (unavailable to an automated on-device test harness -- there is
     * no way to synthesize a genuine stylus `MotionEvent` from `adb`)
     * or this direct call, so this exists purely to make Select/Delete/
     * Fit exercisable and verifiable on a real device before Phase 09's
     * real creation tools exist. */
    fun debugInsertTestLine() {
        viewModelScope.launch {
            session.createPrimitiveLine(100.0, 100.0, 400.0, 300.0)
            refreshSnapshots()
        }
    }

    fun undo() {
        viewModelScope.launch {
            session.undo()
            refreshSnapshots()
        }
    }

    fun redo() {
        viewModelScope.launch {
            session.redo()
            refreshSnapshots()
        }
    }

    /** Task 046/048: one call per *completed* stroke, never per sample
     * (Article 9's whole point -- live ink is Android-local, only the
     * finished gesture crosses the FFI boundary). Runs off the main
     * thread since it touches a Mutex-guarded Rust session and this
     * phase does not need the result synchronously for anything the
     * user is still interacting with. */
    fun submitStroke(samples: List<FfiPointerSample>) {
        if (samples.isEmpty()) return
        viewModelScope.launch {
            session.submitStroke(samples)
            refreshSnapshots()
        }
    }

    override fun onCleared() {
        session.close()
        super.onCleared()
    }
}
