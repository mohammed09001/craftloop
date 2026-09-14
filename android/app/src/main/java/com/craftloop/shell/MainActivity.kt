// Execution 02, Phases 06-07. Replaces the Execution 01 placeholder
// (one Text() line calling resolveCommand) with a real single-Activity
// Compose host: a live Jetpack Ink canvas wired to CraftLoopSession,
// plus Article 8's Alpha-only Debug Region.

package com.craftloop.shell

import android.os.Bundle
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
            onPointerSourceObserved = { source -> viewModel.setLastPointerSource(source) },
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
