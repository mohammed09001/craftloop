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
import uniffi.craftloop_mobile_ffi.FfiConstraintKind
import uniffi.craftloop_mobile_ffi.FfiDebugState
import uniffi.craftloop_mobile_ffi.FfiPointerSample
import uniffi.craftloop_mobile_ffi.FfiPrincipalViewIdentity
import uniffi.craftloop_mobile_ffi.FfiSceneSnapshot
import uniffi.craftloop_mobile_ffi.FfiWorkspaceMode
import java.io.File
import kotlin.math.max
import kotlin.math.min

/** Task 039/040/058: the tools a user can select, now covering every
 * primary toolbar icon Article 7 requires. `DIMENSION` (Phase 10) opens
 * a real numeric-entry dialog (Task 065-068); `ORTHOGRAPHIC` calls the
 * already-built `enterOrthographic` narrowly -- see its own doc
 * comment for exactly what this phase does and does not do with the
 * result (full linked-view rendering is Phase 11's scope). `CONSTRAINT`/
 * `VIEW_IDENTITY` are popover triggers (Task 062/063), not drawing
 * modes a stylus stroke continues in, but still get an active-tool
 * highlight while their popover is open. */
enum class Tool {
    PEN,
    ERASER,
    SELECT,
    LINE,
    CIRCLE,
    RECTANGLE,
    DIMENSION,
    CONSTRAINT,
    VIEW_IDENTITY,
    ORTHOGRAPHIC,
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
    // Task 062/063/064: popover/dialog visibility -- ephemeral,
    // Android-only, same rationale as every other field here.
    val constraintPopoverVisible: Boolean = false,
    val viewIdentityPopoverVisible: Boolean = false,
    val inputCapabilityDialogVisible: Boolean = false,
    val alphaSettingsDialogVisible: Boolean = false,
    val overflowMenuVisible: Boolean = false,
    // Task 063: the single Alpha-only view this toolbar assigns
    // identity to -- Phase 11 is where "which view is current" becomes
    // a real, user-driven multi-view concept; until then, the first
    // (and, this phase, only) view ever created is reused for every
    // subsequent identity assignment rather than silently creating a
    // new one each time, which would make View Identity's `Front` then
    // `Top` sequence Article 35 describes impossible to reproduce.
    val currentViewId: String? = null,
    // Task 060: a one-shot, non-persisted status line for actions this
    // phase deliberately keeps minimal (Orthographic, Dimension) --
    // Article 7 forbids spending this phase's budget on a real toast/
    // snackbar design system, so this is read directly by a plain
    // `Text` in the Debug Region rather than a styled transient banner.
    val lastActionMessage: String = "",
    // Task 065/067: the numeric-entry dialog's visibility and mode --
    // `dimensionEditTargetId == null` means "create a new dimension
    // from the current selection" (Task 065/066); non-null means "edit
    // this existing dimension's value" (Task 067), reusing the same
    // dialog rather than building a second one, since both end in one
    // real number crossing the FFI boundary either way.
    val dimensionDialogVisible: Boolean = false,
    val dimensionEditTargetId: String? = null,
    val dimensionEditCurrentValue: Double = 0.0,
    // Task 067/068: a plain list of current dimensions/conflicts --
    // Article 7 forbids design polish, and no canvas annotation
    // rendering exists yet for dimensions to be tapped in place (Phase
    // 06-07's own documented gap), so this is the Alpha's one real way
    // to reach an existing dimension to edit, and to see a conflict
    // Task 068 requires be visible, not just logged.
    val dimensionListVisible: Boolean = false,
)

class CraftLoopViewModel : ViewModel() {
    // Task 039: the real session, held for this ViewModel's lifetime
    // (which spans configuration changes). `var`, not `val`, as of
    // Task 064: New/Open (overflow menu) replace the whole session
    // object outright -- CraftLoopSession itself exposes no "reset a
    // document in place" operation, and none should exist (a fresh
    // document *is* a fresh session, per Article 12's own object
    // lifecycle), so this ViewModel's job is closing the old one and
    // holding a new one, not asking the session to forget itself.
    private var session = CraftLoopSession()

    // Task 064: fixed Alpha default save/open path -- no document
    // picker exists yet (real file-picker UI belongs to whichever
    // future phase actually needs multiple named documents; nothing
    // before Phase 12 does), so every Save/Open/New this phase's
    // overflow menu offers acts on this one path, stated plainly here
    // rather than implied by silently working.
    private fun defaultDocumentPath(context: android.content.Context): String =
        File(context.filesDir, "craftloop-alpha-document.json").absolutePath

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

    /** Execution 03, Phase 19, Task 148: one semantic action shared by
     * every entry source (the toolbar's own Sketch button today, a
     * future keyboard/ink-command source tomorrow -- the same real
     * shape `apps/web-live`'s `Workspace.tsx` already established),
     * calling the real `CraftLoopSession.enterSketchMode`
     * (`FfiWorkspaceMode` mirrors `WebWorkspaceMode`) rather than
     * inventing a second, Android-local mode concept. `FfiSceneSnapshot
     * .workspaceMode` -- not a duplicated field on [EphemeralUiState]
     * -- is the one source of truth [Toolbar] reads to decide which
     * button set to show, matching engineering-truth-in-one-place even
     * for this ephemeral, non-persisted concept. */
    fun enterSketchMode() {
        viewModelScope.launch {
            session.enterSketchMode()
            refreshSnapshots()
            _uiState.value = _uiState.value.copy(activeTool = Tool.PEN)
        }
    }

    /** The return-side counterpart, mirroring
     * `CraftLoopSession.enterCreativePenMode`. */
    fun enterCreativePenMode() {
        viewModelScope.launch {
            session.enterCreativePenMode()
            refreshSnapshots()
            _uiState.value = _uiState.value.copy(activeTool = Tool.PEN)
        }
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

    /** Execution 03, Phase 19, Task 147: real bug found and fixed while
     * moving the toolbar to float top-center over a now full-screen
     * `InkCanvas` (previously the two were siblings in a `Column`, so
     * `canvasBoundsPx` already excluded the toolbar's row purely by
     * layout). With `InkCanvas` filling the whole screen,
     * `canvasBoundsPx` alone can no longer distinguish "a finger tap on
     * the floating toolbar" from "a finger tap on the canvas beneath
     * it" -- without this, `dispatchTouchEvent`'s finger branch
     * (`scaleGestureDetector`/`gestureDetector`, which always consumes
     * the event and never forwards to `super`) would swallow every
     * toolbar button tap before Compose's own click handling ever saw
     * it. Updated by [PrimaryToolbar]'s own wrapping
     * `Modifier.onGloballyPositioned`; `null` (its initial value, same
     * as [canvasBoundsPx]'s) means "no toolbar bounds known yet,"
     * correctly excluding nothing rather than everything. */
    var toolbarBoundsPx: Rect? = null

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

    /** Screen-space point to content-space point, via the current
     * viewport's inverse affine transform -- the exact inverse of
     * `InkCanvas`'s `graphicsLayer(scaleX/Y = zoom, translationX/Y =
     * pan, transformOrigin = (0,0))`, so a hit test here always agrees
     * with what the user actually sees rendered. */
    private fun screenToContent(
        screenX: Float,
        screenY: Float,
    ): Pair<Float, Float> {
        val viewport = _uiState.value.viewport
        return Pair((screenX - viewport.panX) / viewport.zoom, (screenY - viewport.panY) / viewport.zoom)
    }

    /** Task 055/Eraser: hit-test a screen-space point against every
     * current structured primitive's bounding box (expanded by a small
     * tolerance so a thin line is actually tappable), in content space.
     * Shared by [selectAt] and [eraseAt] -- both are "which primitive is
     * under this point," differing only in what happens to the hit. */
    private fun hitTestPrimitive(
        screenX: Float,
        screenY: Float,
    ): uniffi.craftloop_mobile_ffi.FfiPrimitiveSummary? {
        val (contentX, contentY) = screenToContent(screenX, screenY)
        val tolerance = 24f / _uiState.value.viewport.zoom
        return _sceneSnapshot.value.primitives.firstOrNull { p ->
            contentX >= p.minX - tolerance && contentX <= p.maxX + tolerance &&
                contentY >= p.minY - tolerance && contentY <= p.maxY + tolerance
        }
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
        val hit = hitTestPrimitive(screenX, screenY)
        viewModelScope.launch {
            if (hit != null) {
                session.select(listOf(hit.id))
            } else {
                session.clearSelection()
            }
            refreshSnapshots()
        }
    }

    /** Article 7's Eraser tool: a stylus tap directly removes whatever
     * primitive is under it, through the same real
     * `CraftLoopSession.deleteSelected` transaction Task 056's Delete
     * control already uses (select-then-delete collapsed into one
     * user-visible gesture, not a second removal mechanism) -- undoable
     * exactly like every other mutation here. A tap that hits nothing
     * is a no-op, not an error. */
    fun eraseAt(
        screenX: Float,
        screenY: Float,
    ) {
        val hit = hitTestPrimitive(screenX, screenY) ?: return
        viewModelScope.launch {
            session.deleteSelected(listOf(hit.id))
            refreshSnapshots()
        }
    }

    /** Task 060: Line/Circle/Rectangle tools' real, minimal drawing
     * mode -- a stylus drag's down/up points, converted to content
     * space, become the primitive's defining geometry directly (no
     * live-preview-while-dragging yet; Article 7's "intentionally
     * simple" and no task before this one asks for a rubber-band
     * preview). Zero-length drags (a tap, not a drag) are rejected by
     * each `create_primitive_*` call's own domain validation
     * (`craftloop-geometry`'s degenerate-geometry checks) and simply
     * produce no primitive -- not a crash, not a silently-inserted
     * degenerate shape. */
    fun createShapeFromDrag(
        tool: Tool,
        screenX0: Float,
        screenY0: Float,
        screenX1: Float,
        screenY1: Float,
    ) {
        val (x0, y0) = screenToContent(screenX0, screenY0)
        val (x1, y1) = screenToContent(screenX1, screenY1)
        viewModelScope.launch {
            runCatching {
                when (tool) {
                    Tool.LINE -> session.createPrimitiveLine(x0.toDouble(), y0.toDouble(), x1.toDouble(), y1.toDouble())
                    Tool.CIRCLE -> {
                        val radius = kotlin.math.hypot((x1 - x0).toDouble(), (y1 - y0).toDouble())
                        session.createPrimitiveCircle(x0.toDouble(), y0.toDouble(), radius)
                    }
                    Tool.RECTANGLE ->
                        session.createPrimitiveRectangle(x0.toDouble(), y0.toDouble(), x1.toDouble(), y1.toDouble())
                    else -> null
                }
            }
            refreshSnapshots()
        }
    }

    /** Task 062: apply one Article 17 constraint to the current
     * selection. Each `FfiConstraintKind` variant's own arity (one id
     * for Horizontal/Vertical, two for the rest) is enforced by
     * [Toolbar]'s popover only enabling options the current selection
     * count actually supports -- this method trusts its caller rather
     * than re-deriving that policy a second time. */
    fun applyConstraint(kind: FfiConstraintKind) {
        viewModelScope.launch {
            session.applyConstraint(kind)
            // Task 070: nothing before this session ever actually
            // invoked the solver, so a real solver conflict could never
            // have been observed -- solving immediately after every
            // applied constraint is what makes one possible to see at
            // all through this toolbar (see `solveConstraints`' own doc
            // comment for exactly how that outcome is reported).
            val outcome = runCatching { session.solveConstraints() }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(
                    constraintPopoverVisible = false,
                    lastActionMessage =
                        outcome.fold(
                            onSuccess = { o -> "Constraint added; solve: ${o.status}" },
                            onFailure = { err -> "Constraint added; solve failed: ${err.message}" },
                        ),
                )
        }
    }

    /** Task 063: assign `identity` to the Alpha's one current view,
     * creating it on first use (see [EphemeralUiState.currentViewId]'s
     * own doc comment for why later assignments reuse the same view
     * rather than creating a new one each time). Also feeds the current
     * selection into the view's `geometry_members` via
     * `add_geometry_to_view` -- without that, the view stays
     * perpetually `LinkReady`-ineligible (Article 30) with no way for
     * this phase's UI to have ever populated it, since Phase 11 (the
     * phase that would otherwise own that step) does not exist yet. */
    fun assignViewIdentity(identity: FfiPrincipalViewIdentity) {
        viewModelScope.launch {
            val newViewId = session.assignViewIdentity(_uiState.value.currentViewId, identity)
            val selected = _sceneSnapshot.value.selectedEntityIds
            if (selected.isNotEmpty()) {
                runCatching { session.addGeometryToView(newViewId, selected) }
            }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(currentViewId = newViewId, viewIdentityPopoverVisible = false)
        }
    }

    /** Task 060's real (not decorative) Orthographic action: enters
     * Orthographic mode from the Alpha's current view if one exists and
     * has reached `LinkReady`. This phase does not render the resulting
     * Top/Right view blocks anywhere (Phase 11's real scope -- linked
     * multi-view presentation) -- it only proves the existing Phase 04
     * `enter_orthographic` call is reachable end to end from a real
     * toolbar action, reporting how many derived views were created via
     * [EphemeralUiState.lastActionMessage] rather than silently
     * discarding the result. */
    fun enterOrthographic() {
        val viewId = _uiState.value.currentViewId
        if (viewId == null) {
            _uiState.value = _uiState.value.copy(lastActionMessage = "Assign a View Identity first")
            return
        }
        viewModelScope.launch {
            val result = runCatching { session.enterOrthographic(viewId) }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(
                    lastActionMessage =
                        result.fold(
                            onSuccess = { views -> "Orthographic: ${views.size} derived view(s)" },
                            onFailure = { err -> "Orthographic failed: ${err.message}" },
                        ),
                )
        }
    }

    /** Task 065: opens the numeric-entry dialog to create a new
     * dimension from the current selection -- Article 14's flow
     * ("Select eligible geometry. Show compact numeric input.") needs a
     * real target before there is anything to dimension, so 0 or >2
     * selected primitives shows a message instead of an empty dialog
     * (`create_dimension`'s own real arity, `DimensionTarget::Single`/
     * `Pair`, checked in `session.rs` -- not guessed). */
    fun openDimensionDialogForCreate() {
        val selected = _sceneSnapshot.value.selectedEntityIds
        if (selected.isEmpty() || selected.size > 2) {
            _uiState.value =
                _uiState.value.copy(
                    lastActionMessage = "Select 1 or 2 primitives to dimension first",
                )
            return
        }
        _uiState.value =
            _uiState.value.copy(
                dimensionDialogVisible = true,
                dimensionEditTargetId = null,
                dimensionEditCurrentValue = 0.0,
            )
    }

    /** Task 067: opens the same dialog pre-filled with an existing
     * dimension's current value, from [Toolbar]'s dimension list. */
    fun openDimensionDialogForEdit(
        dimensionId: String,
        currentValue: Double,
    ) {
        _uiState.value =
            _uiState.value.copy(
                dimensionDialogVisible = true,
                dimensionEditTargetId = dimensionId,
                dimensionEditCurrentValue = currentValue,
            )
    }

    fun dismissDimensionDialog() {
        _uiState.value = _uiState.value.copy(dimensionDialogVisible = false)
    }

    fun setDimensionListVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(dimensionListVisible = visible)
    }

    /** Task 065/066/067/068: create or edit a dimension depending on
     * [EphemeralUiState.dimensionEditTargetId]. Article 14 Task 068 /
     * Gate G: an invalid edit's real failure path
     * (`CraftLoopSession.edit_dimension`, `session.rs`) leaves the
     * dimension's own value unchanged but still *commits* a real
     * `Conflict` entity (`ConflictKind::DimensionConstraintMismatch`)
     * before returning `Err` -- so this catches the exception, still
     * refreshes snapshots (the conflict is real committed state even
     * though the edit itself was rejected), and reports the real
     * failure message rather than silently swallowing it. */
    fun submitDimension(
        kind: uniffi.craftloop_mobile_ffi.FfiDimensionKind,
        value: Double,
    ) {
        val editId = _uiState.value.dimensionEditTargetId
        val selected = _sceneSnapshot.value.selectedEntityIds
        viewModelScope.launch {
            val result =
                runCatching {
                    if (editId != null) {
                        session.editDimension(editId, value)
                    } else {
                        session.createDimension(kind, selected, value)
                    }
                }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(
                    dimensionDialogVisible = false,
                    lastActionMessage =
                        result.fold(
                            onSuccess = { if (editId != null) "Dimension updated" else "Dimension created" },
                            onFailure = { err -> "Dimension rejected: ${err.message}" },
                        ),
                )
        }
    }

    /** Task 069/070: solve the current sketch's constraints for real
     * (Task 069 is otherwise already done by [applyConstraint] alone --
     * nothing before this method ever actually invoked the solver, so
     * no real solver conflict could ever have been observed). Unlike
     * an invalid dimension edit, `solve_constraints` does **not** commit
     * a `Conflict` entity for an unsatisfied/failed solve (confirmed by
     * reading its real body in `session.rs`: it returns early with
     * `updated_primitive_count: 0` and no document change at all when
     * `SolveStatus::Failed`, and otherwise only ever commits the
     * primitives that actually moved) -- so an unsatisfied/failed solve
     * is reported the same way Orthographic's own outcome already is,
     * via `lastActionMessage`, not through the conflict list. */
    fun solveConstraints() {
        viewModelScope.launch {
            val result = runCatching { session.solveConstraints() }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(
                    lastActionMessage =
                        result.fold(
                            onSuccess = { outcome ->
                                "Solve: ${outcome.status} (${outcome.updatedPrimitiveCount} moved" +
                                    if (outcome.unsatisfiedConstraintIds.isNotEmpty()) {
                                        ", ${outcome.unsatisfiedConstraintIds.size} unsatisfied)"
                                    } else {
                                        ")"
                                    }
                            },
                            onFailure = { err -> "Solve failed: ${err.message}" },
                        ),
                )
        }
    }

    /** Phase 08/10 on-device verification helper only -- selects the
     * first current primitive, the same real `CraftLoopSession.select`
     * path a real stylus tap on it would take (`selectAt`), so
     * Dimension/Constraint's selection-gated dialogs/popovers are
     * exercisable on a real device without a physical stylus. Not a
     * product feature -- see [debugInsertTestLine]'s own doc comment
     * for why this class of helper is legitimate here. */
    fun debugSelectFirstPrimitive() {
        val first = _sceneSnapshot.value.primitives.firstOrNull() ?: return
        viewModelScope.launch {
            session.select(listOf(first.id))
            refreshSnapshots()
        }
    }

    /** Task 064: Save to the fixed Alpha document path. */
    fun save(context: android.content.Context) {
        viewModelScope.launch {
            val result = runCatching { session.save(defaultDocumentPath(context)) }
            _uiState.value =
                _uiState.value.copy(
                    lastActionMessage =
                        result.fold({ "Saved" }, { err -> "Save failed: ${err.message}" }),
                )
        }
    }

    /** Task 064: Open replaces the whole session with one loaded from
     * the fixed Alpha document path -- see the `session` field's own
     * doc comment for why this is a whole-object replacement, not an
     * in-place reset. */
    fun open(context: android.content.Context) {
        viewModelScope.launch {
            val result = runCatching { CraftLoopSession.open(defaultDocumentPath(context)) }
            result.onSuccess { opened ->
                session.close()
                session = opened
            }
            refreshSnapshots()
            _uiState.value =
                _uiState.value.copy(
                    currentViewId = null,
                    lastActionMessage =
                        result.fold({ "Opened" }, { err -> "Open failed: ${err.message}" }),
                )
        }
    }

    /** Task 064: New discards the current in-memory document for a
     * fresh, empty one -- does not touch whatever was last saved to
     * disk at [defaultDocumentPath] (only Save does that). */
    fun newDocument() {
        session.close()
        session = CraftLoopSession()
        refreshSnapshots()
        _uiState.value =
            _uiState.value.copy(currentViewId = null, lastActionMessage = "New document")
    }

    fun toggleDebugOverlay() {
        _uiState.value = _uiState.value.copy(debugOverlayVisible = !_uiState.value.debugOverlayVisible)
    }

    fun setConstraintPopoverVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(constraintPopoverVisible = visible)
    }

    fun setViewIdentityPopoverVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(viewIdentityPopoverVisible = visible)
    }

    fun setInputCapabilityDialogVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(inputCapabilityDialogVisible = visible)
    }

    fun setAlphaSettingsDialogVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(alphaSettingsDialogVisible = visible)
    }

    fun setOverflowMenuVisible(visible: Boolean) {
        _uiState.value = _uiState.value.copy(overflowMenuVisible = visible)
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
