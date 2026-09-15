import { useCallback, useMemo, useState } from 'react'
import { CanvasStack } from '../canvas/CanvasStack'
import { useCraftLoopSession } from '../session/useCraftLoopSession'
import { WebDimensionKind } from '../session/craftLoopSession'
import { Toolbar } from '../toolbar/Toolbar'
import type { ToolId } from '../toolbar/toolRegistry'
import { eligibleConstraintOptions, type ConstraintOption } from '../toolbar/constraintOptions'
import { useKeyboardShortcuts } from './useKeyboardShortcuts'
import styles from './Workspace.module.css'

/**
 * Full-canvas shell (Execution 03, Phase 02, Task 015) filled with the
 * Phase 06 Canvas/SVG rendering stack, the Phase 07/09 top-center
 * toolbar (Task 016's anchor, now morphing between Creative and
 * Sketch2D tool sets), and Phase 08's real `WorkspaceMode` switching --
 * one semantic action (`session.enterSketchMode`/`enterCreativePenMode`)
 * shared by every entry source: the toolbar's Sketch/Pen buttons and
 * the `S`/`B` keyboard shortcuts all call the exact same session
 * methods. `activeTool` is owned here since both the toolbar (which
 * button is pressed) and the canvas (what a click/drag actually does)
 * need to agree on it. Dimension/Constraint eligibility (Task 072/073)
 * is derived directly from the real selected primitives, never a
 * separate guess about what is selected. Construction (Task 090/091)
 * and Snap (Task 092/093) follow the same pattern: `constructionEnabled`
 * is derived from the real selection, and `snapEnabled` is the one
 * piece of state both `CanvasStack` (drag snapping + grid visibility)
 * and `Toolbar` (the pressed state of the Snap button) need to agree
 * on, so it lives here rather than duplicated in either.
 */
export function Workspace() {
  const session = useCraftLoopSession()
  const [activeTool, setActiveTool] = useState<ToolId>('pen')
  const [snapEnabled, setSnapEnabled] = useState(true)

  const enterSketchMode = useCallback(() => {
    session.enterSketchMode()
    setActiveTool('pen')
  }, [session])

  const enterCreativePenMode = useCallback(() => {
    session.enterCreativePenMode()
    setActiveTool('pen')
  }, [session])

  useKeyboardShortcuts({
    onEnterSketchMode: enterSketchMode,
    onEnterCreativePenMode: enterCreativePenMode,
  })

  const selectedPrimitives = useMemo(() => {
    const selected = new Set(session.snapshot.selected_entity_ids)
    return session.snapshot.primitives.filter((p) => selected.has(p.id))
  }, [session.snapshot.selected_entity_ids, session.snapshot.primitives])

  const dimensionEnabled = selectedPrimitives.length === 1 || selectedPrimitives.length === 2

  const handleDimension = useCallback(() => {
    const raw = window.prompt('Dimension value')
    if (raw === null) return
    const value = Number.parseFloat(raw)
    if (!Number.isFinite(value)) return
    session.createDimension(
      WebDimensionKind.Linear,
      selectedPrimitives.map((p) => p.id),
      value,
    )
  }, [session, selectedPrimitives])

  const constraintOptions = useMemo(
    () => eligibleConstraintOptions(selectedPrimitives),
    [selectedPrimitives],
  )

  const handleApplyConstraint = useCallback(
    (payload: ConstraintOption['payload']) => {
      // Applying a constraint that isn't visibly reflected in the
      // geometry would look broken, not "real" (Task 073) -- solve
      // immediately afterward, the same real EzpzSolver call the
      // (currently absent) explicit Solve action would make.
      session.applyConstraint(JSON.stringify(payload))
      session.solveConstraints()
    },
    [session],
  )

  const constructionEnabled = selectedPrimitives.length > 0

  const handleToggleConstruction = useCallback(() => {
    if (selectedPrimitives.length === 0) return
    // Task 090/091: one action toggling every selected primitive to
    // the opposite of the *first* selected one's current state -- a
    // mixed selection (some construction, some not) becomes uniformly
    // construction on one click, matching what most drawing tools do
    // with a mixed-state toggle rather than leaving the selection in a
    // now-ambiguous mixed state.
    const next = !selectedPrimitives[0]!.is_construction
    for (const primitive of selectedPrimitives) {
      session.setConstruction(primitive.id, next)
    }
  }, [selectedPrimitives, session])

  const handleToggleSnap = useCallback(() => {
    setSnapEnabled((enabled) => !enabled)
  }, [])

  return (
    <div className={styles.workspace} data-session-ready={session.ready}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack session={session} activeTool={activeTool} snapEnabled={snapEnabled} />
      </div>
      <div className={styles.toolbarHost} data-testid="toolbar-host">
        <Toolbar
          activeTool={activeTool}
          onSelectTool={setActiveTool}
          workspaceMode={session.snapshot.workspace_mode}
          onEnterSketchMode={enterSketchMode}
          onEnterCreativePenMode={enterCreativePenMode}
          canUndo={session.snapshot.can_undo}
          canRedo={session.snapshot.can_redo}
          onUndo={session.undo}
          onRedo={session.redo}
          dimensionEnabled={dimensionEnabled}
          onDimension={handleDimension}
          constraintOptions={constraintOptions}
          onApplyConstraint={handleApplyConstraint}
          constructionEnabled={constructionEnabled}
          onToggleConstruction={handleToggleConstruction}
          snapEnabled={snapEnabled}
          onToggleSnap={handleToggleSnap}
        />
      </div>
    </div>
  )
}
