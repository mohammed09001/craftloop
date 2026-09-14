// Execution 02, Phase 06 (Tasks 039-043). Owns the one CraftLoopSession
// for this Activity's lifetime; survives configuration changes because
// a ViewModel does, by construction (Task 043) -- no extra handling
// needed here beyond simply holding it in the ViewModel rather than in
// the Activity/Composable.

package com.craftloop.shell

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

/** Task 041: ephemeral UI state -- never crosses the FFI boundary,
 * never persisted, rebuilt fresh on every process restart. Kept in one
 * data class (rather than several loose StateFlows) so a Composable
 * can collect one flow for all of it. */
data class EphemeralUiState(
    val activeTool: Tool = Tool.PEN,
    val lastPointerSource: String = "none",
    val debugOverlayVisible: Boolean = true,
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
