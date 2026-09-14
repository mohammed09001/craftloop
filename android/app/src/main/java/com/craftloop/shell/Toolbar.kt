// Execution 02, Phase 09 (Tasks 058-064). Article 7's icon-first
// toolbar: no persistent text labels on primary tools, every icon
// carries a real accessibility contentDescription (Task 059) and a
// long-press tooltip (Task 061) naming it, and the active tool is
// visually distinguished (Task 060). Material Icons (bundled with
// material3 plus the `material-icons-extended` artifact for the
// handful this toolbar needs beyond the default small set) stand in
// for a design system that does not exist yet -- Article 7 explicitly
// forbids spending this phase's budget on one.

package com.craftloop.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Backspace
import androidx.compose.material.icons.automirrored.filled.Redo
import androidx.compose.material.icons.automirrored.filled.Undo
import androidx.compose.material.icons.filled.Circle
import androidx.compose.material.icons.filled.CropSquare
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.filled.GridOn
import androidx.compose.material.icons.filled.Link
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.NearMe
import androidx.compose.material.icons.filled.Save
import androidx.compose.material.icons.filled.Straighten
import androidx.compose.material.icons.filled.Timeline
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.PlainTooltip
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TooltipBox
import androidx.compose.material3.TooltipDefaults
import androidx.compose.material3.rememberTooltipState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import uniffi.craftloop_mobile_ffi.FfiConstraintKind
import uniffi.craftloop_mobile_ffi.FfiDimensionKind
import uniffi.craftloop_mobile_ffi.FfiPrincipalViewIdentity

/** One primary toolbar entry: Article 7's fixed icon list.
 * `Constraint`/`ViewIdentity` open popovers; `Dimension` (Phase 10)
 * opens the real numeric-entry dialog; `Orthographic` calls the real,
 * already-built `enterOrthographic` narrowly (see its own doc comment
 * in `CraftLoopViewModel` for exactly what this phase does and does
 * not do with the result). */
private data class ToolbarEntry(
    val tool: Tool?,
    val icon: ImageVector,
    val label: String,
    val onClick: (CraftLoopViewModel) -> Unit,
)

private val primaryEntries =
    listOf(
        ToolbarEntry(Tool.PEN, Icons.Filled.Edit, "Pen") { it.setActiveTool(Tool.PEN) },
        ToolbarEntry(Tool.ERASER, Icons.AutoMirrored.Filled.Backspace, "Eraser") { it.setActiveTool(Tool.ERASER) },
        ToolbarEntry(Tool.SELECT, Icons.Filled.NearMe, "Select") { it.setActiveTool(Tool.SELECT) },
        ToolbarEntry(Tool.LINE, Icons.Filled.Timeline, "Line") { it.setActiveTool(Tool.LINE) },
        ToolbarEntry(Tool.CIRCLE, Icons.Filled.Circle, "Circle") { it.setActiveTool(Tool.CIRCLE) },
        ToolbarEntry(Tool.RECTANGLE, Icons.Filled.CropSquare, "Rectangle") { it.setActiveTool(Tool.RECTANGLE) },
        ToolbarEntry(Tool.DIMENSION, Icons.Filled.Straighten, "Dimension") {
            it.setActiveTool(Tool.DIMENSION)
            it.openDimensionDialogForCreate()
        },
        ToolbarEntry(Tool.CONSTRAINT, Icons.Filled.Link, "Constraint") {
            it.setActiveTool(Tool.CONSTRAINT)
            it.setConstraintPopoverVisible(true)
        },
        ToolbarEntry(Tool.VIEW_IDENTITY, Icons.Filled.Visibility, "View Identity") {
            it.setActiveTool(Tool.VIEW_IDENTITY)
            it.setViewIdentityPopoverVisible(true)
        },
        ToolbarEntry(Tool.ORTHOGRAPHIC, Icons.Filled.GridOn, "Orthographic") {
            it.setActiveTool(Tool.ORTHOGRAPHIC)
            it.enterOrthographic()
        },
    )

/** Task 058-061: the primary icon-only toolbar (Article 8's Tool
 * Region). Undo/Redo/Save are real one-shot actions, not tools that
 * stay "active" -- they never change [EphemeralUiState.activeTool]. */
@Composable
fun PrimaryToolbar(viewModel: CraftLoopViewModel) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val debugState by viewModel.debugState.collectAsStateWithLifecycle()
    val context = LocalContext.current

    Row(modifier = Modifier.padding(4.dp)) {
        for (entry in primaryEntries) {
            ToolbarIconButton(
                icon = entry.icon,
                label = entry.label,
                active = entry.tool != null && entry.tool == uiState.activeTool,
                onClick = { entry.onClick(viewModel) },
            )
        }
        ToolbarIconButton(
            icon = Icons.AutoMirrored.Filled.Undo,
            label = "Undo",
            active = false,
            enabled = debugState.canUndo,
            onClick = { viewModel.undo() },
        )
        ToolbarIconButton(
            icon = Icons.AutoMirrored.Filled.Redo,
            label = "Redo",
            active = false,
            enabled = debugState.canRedo,
            onClick = { viewModel.redo() },
        )
        ToolbarIconButton(
            icon = Icons.Filled.Save,
            label = "Save",
            active = false,
            onClick = { viewModel.save(context) },
        )
        AlphaOverflowButton(viewModel)
    }

    if (uiState.constraintPopoverVisible) {
        ConstraintPopover(viewModel)
    }
    if (uiState.viewIdentityPopoverVisible) {
        ViewIdentityPopover(viewModel)
    }
    if (uiState.inputCapabilityDialogVisible) {
        InputCapabilityDialog(viewModel)
    }
    if (uiState.alphaSettingsDialogVisible) {
        AlphaSettingsDialog(viewModel)
    }
    if (uiState.dimensionDialogVisible) {
        DimensionDialog(viewModel)
    }
    if (uiState.dimensionListVisible) {
        DimensionAndConflictList(viewModel)
    }
}

/** Task 059 (real contentDescription, checked against the actual
 * accessibility tree via `adb shell uiautomator dump`, not assumed)
 * and Task 061 (long-press tooltip, Material3's own `TooltipBox` --
 * confirmed current/stable for `compose-bom:2024.09.02` by building,
 * not guessed) combined so every primary icon gets both from one call
 * site rather than each entry repeating the wiring. Task 060's active-
 * tool highlight is a plain background tint -- Article 7 forbids
 * spending this phase's budget on a real design system. */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ToolbarIconButton(
    icon: ImageVector,
    label: String,
    active: Boolean,
    enabled: Boolean = true,
    onClick: () -> Unit,
) {
    TooltipBox(
        positionProvider = TooltipDefaults.rememberPlainTooltipPositionProvider(),
        tooltip = { PlainTooltip { Text(label) } },
        state = rememberTooltipState(),
    ) {
        val background = if (active) MaterialTheme.colorScheme.primaryContainer else MaterialTheme.colorScheme.surface
        Box(modifier = Modifier.padding(2.dp).background(background, CircleShape)) {
            IconButton(onClick = onClick, enabled = enabled) {
                Icon(icon, contentDescription = label)
            }
        }
    }
}

/** Task 064: the Alpha overflow menu (a plain `DropdownMenu` --
 * Article 7 forbids a styled bottom-sheet design pass this phase). */
@Composable
private fun AlphaOverflowButton(viewModel: CraftLoopViewModel) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val context = LocalContext.current
    Box {
        IconButton(onClick = { viewModel.setOverflowMenuVisible(true) }) {
            Icon(Icons.Filled.MoreVert, contentDescription = "More")
        }
        DropdownMenu(
            expanded = uiState.overflowMenuVisible,
            onDismissRequest = { viewModel.setOverflowMenuVisible(false) },
        ) {
            DropdownMenuItem(text = { Text("New") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.newDocument()
            })
            DropdownMenuItem(text = { Text("Open") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.open(context)
            })
            DropdownMenuItem(text = { Text("Delete selected") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.deleteSelected()
            })
            DropdownMenuItem(text = { Text("Fit View") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                val bounds = viewModel.canvasBoundsPx
                if (bounds != null) viewModel.fitToContent(bounds.width, bounds.height)
            })
            DropdownMenuItem(text = { Text("Debug Inspector") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.toggleDebugOverlay()
            })
            DropdownMenuItem(text = { Text("Dimensions & Conflicts") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.setDimensionListVisible(true)
            })
            DropdownMenuItem(text = { Text("Input Capability Inspector") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.setInputCapabilityDialogVisible(true)
            })
            DropdownMenuItem(text = { Text("Alpha Settings") }, onClick = {
                viewModel.setOverflowMenuVisible(false)
                viewModel.setAlphaSettingsDialogVisible(true)
            })
        }
    }
}

/** Task 062: Article 17's stable constraint list. Only offers a
 * constraint whose real arity (from `FfiConstraintKind`, checked
 * against the actual Rust enum in `session.rs` rather than assumed)
 * the current selection count actually satisfies -- Horizontal/
 * Vertical need exactly one selected primitive (`line`), the rest need
 * exactly two (`a`/`b`); anything else offers nothing rather than
 * present a button that would just fail. There is no plain "Equal"
 * entry: `SketchConstraintKind` only has `EqualLength`/`EqualRadius`
 * (Phase 04's own real vocabulary, not this phase's simplification),
 * so both are offered distinctly rather than inventing a merged one
 * that would have to guess which the user meant. */
@Composable
private fun ConstraintPopover(viewModel: CraftLoopViewModel) {
    val sceneSnapshot by viewModel.sceneSnapshot.collectAsStateWithLifecycle()
    val selected = sceneSnapshot.selectedEntityIds
    AlertDialog(
        onDismissRequest = { viewModel.setConstraintPopoverVisible(false) },
        title = { Text("Constraint") },
        text = {
            Column {
                if (selected.size == 1) {
                    val line = selected[0]
                    DropdownMenuItem(text = { Text("Horizontal") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Horizontal(line))
                    })
                    DropdownMenuItem(text = { Text("Vertical") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Vertical(line))
                    })
                } else if (selected.size == 2) {
                    val (a, b) = selected[0] to selected[1]
                    DropdownMenuItem(text = { Text("Coincident") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Coincident(a, b))
                    })
                    DropdownMenuItem(text = { Text("Parallel") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Parallel(a, b))
                    })
                    DropdownMenuItem(text = { Text("Perpendicular") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Perpendicular(a, b))
                    })
                    DropdownMenuItem(text = { Text("Equal Length") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.EqualLength(a, b))
                    })
                    DropdownMenuItem(text = { Text("Equal Radius") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.EqualRadius(a, b))
                    })
                    DropdownMenuItem(text = { Text("Concentric") }, onClick = {
                        viewModel.applyConstraint(FfiConstraintKind.Concentric(a, b))
                    })
                } else {
                    Text("Select 1 primitive (Horizontal/Vertical) or 2 (the rest) first.")
                }
            }
        },
        confirmButton = {
            DropdownMenuItem(text = { Text("Close") }, onClick = { viewModel.setConstraintPopoverVisible(false) })
        },
    )
}

/** Task 063: Front/Top/Right/Back -- `FfiPrincipalViewIdentity`'s
 * exact four variants, no more (Article 30 defers Left/Bottom). */
@Composable
private fun ViewIdentityPopover(viewModel: CraftLoopViewModel) {
    val options =
        listOf(
            "Front" to FfiPrincipalViewIdentity.FRONT,
            "Top" to FfiPrincipalViewIdentity.TOP,
            "Right" to FfiPrincipalViewIdentity.RIGHT,
            "Back" to FfiPrincipalViewIdentity.BACK,
        )
    AlertDialog(
        onDismissRequest = { viewModel.setViewIdentityPopoverVisible(false) },
        title = { Text("View Identity") },
        text = {
            Column {
                for ((label, identity) in options) {
                    DropdownMenuItem(text = { Text(label) }, onClick = { viewModel.assignViewIdentity(identity) })
                }
            }
        },
        confirmButton = {
            DropdownMenuItem(text = { Text("Close") }, onClick = { viewModel.setViewIdentityPopoverVisible(false) })
        },
    )
}

/** Task 064's Input Capability Inspector: reuses [queryRealInputCapabilities]
 * from `InkCanvas.kt` directly rather than re-deriving the same real
 * capability query a second time. */
@Composable
private fun InputCapabilityDialog(viewModel: CraftLoopViewModel) {
    val context = LocalContext.current
    val capabilities = queryRealInputCapabilities(context)
    AlertDialog(
        onDismissRequest = { viewModel.setInputCapabilityDialogVisible(false) },
        title = { Text("Input Capabilities (real, queried)") },
        text = {
            Column {
                Text("pressure=${capabilities.pressure}")
                Text("tilt=${capabilities.tilt}")
                Text("hover=${capabilities.hover}")
                Text("palmRejection=${capabilities.palmRejection}")
                Text("eraser=${capabilities.eraser}")
            }
        },
        confirmButton = {
            DropdownMenuItem(text = { Text("Close") }, onClick = { viewModel.setInputCapabilityDialogVisible(false) })
        },
    )
}

/** Task 064's Alpha-only settings placeholder -- no real settings
 * exist yet; this states that rather than inventing fake ones. */
@Composable
private fun AlphaSettingsDialog(viewModel: CraftLoopViewModel) {
    AlertDialog(
        onDismissRequest = { viewModel.setAlphaSettingsDialogVisible(false) },
        title = { Text("Alpha Settings") },
        text = { Text("No settings exist yet in this Engineering Alpha.") },
        confirmButton = {
            DropdownMenuItem(text = { Text("Close") }, onClick = { viewModel.setAlphaSettingsDialogVisible(false) })
        },
    )
}

/** Task 065/066/067: Article 14's "compact numeric input" -- one plain
 * `OutlinedTextField` plus a kind selector, no design polish. Reused
 * for both create (Task 065/066, `dimensionEditTargetId == null`) and
 * edit (Task 067, pre-filled with the existing value) since both are
 * "one real number crossing the FFI boundary," differing only in
 * which `CraftLoopSession` call [CraftLoopViewModel.submitDimension]
 * ends up making. `FfiDimensionKind`'s four real variants (checked in
 * `session.rs`, not guessed) have no arity restriction of their own in
 * `create_dimension` -- any kind may target 1 or 2 primitives -- so all
 * four are always offered rather than second-guessing which kinds
 * "should" pair with which selection count. */
@Composable
private fun DimensionDialog(viewModel: CraftLoopViewModel) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val isEdit = uiState.dimensionEditTargetId != null
    var valueText by remember(uiState.dimensionEditTargetId) {
        mutableStateOf(if (isEdit) uiState.dimensionEditCurrentValue.toString() else "")
    }
    var kind by remember { mutableStateOf(FfiDimensionKind.LINEAR) }
    AlertDialog(
        onDismissRequest = { viewModel.dismissDimensionDialog() },
        title = { Text(if (isEdit) "Edit Dimension" else "New Dimension") },
        text = {
            Column {
                if (!isEdit) {
                    for (option in FfiDimensionKind.entries) {
                        DropdownMenuItem(
                            text = { Text(if (option == kind) "✓ $option" else "$option") },
                            onClick = { kind = option },
                        )
                    }
                }
                OutlinedTextField(
                    value = valueText,
                    onValueChange = { valueText = it },
                    label = { Text("Value") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                )
            }
        },
        confirmButton = {
            DropdownMenuItem(
                text = { Text("Commit") },
                onClick = {
                    val value = valueText.toDoubleOrNull()
                    if (value != null) viewModel.submitDimension(kind, value)
                },
            )
        },
        dismissButton = {
            DropdownMenuItem(text = { Text("Cancel") }, onClick = { viewModel.dismissDimensionDialog() })
        },
    )
}

/** Task 067/068: a plain list of every current dimension (tap to edit)
 * and every current conflict (Task 068 -- "show invalid dimension
 * conflict," visibility only, no resolution UI before Phase 14/18). No
 * canvas annotation rendering exists yet (Phase 06-07's documented
 * gap) for either to be reached by tapping the canvas directly, so
 * this list is the Alpha's one real way to reach an existing
 * dimension, and the one real way to *see* a conflict Task 068
 * requires be visible. */
@Composable
private fun DimensionAndConflictList(viewModel: CraftLoopViewModel) {
    val sceneSnapshot by viewModel.sceneSnapshot.collectAsStateWithLifecycle()
    AlertDialog(
        onDismissRequest = { viewModel.setDimensionListVisible(false) },
        title = { Text("Dimensions & Conflicts") },
        text = {
            Column {
                Text("Dimensions", style = MaterialTheme.typography.titleSmall)
                if (sceneSnapshot.dimensions.isEmpty()) {
                    Text("(none)")
                }
                for (dim in sceneSnapshot.dimensions) {
                    DropdownMenuItem(
                        text = { Text("${dim.kind} = ${dim.value} (${dim.id.take(8)})") },
                        onClick = { viewModel.openDimensionDialogForEdit(dim.id, dim.value) },
                    )
                }
                Text("Conflicts", style = MaterialTheme.typography.titleSmall)
                if (sceneSnapshot.conflicts.isEmpty()) {
                    Text("(none)")
                }
                for (conflict in sceneSnapshot.conflicts) {
                    Text("${conflict.kind} unresolved=${conflict.unresolved} (${conflict.id.take(8)})")
                }
            }
        },
        confirmButton = {
            DropdownMenuItem(text = { Text("Close") }, onClick = { viewModel.setDimensionListVisible(false) })
        },
    )
}
