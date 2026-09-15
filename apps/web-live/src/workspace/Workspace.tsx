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
 * separate guess about what is selected.
 */
export function Workspace() {
  const session = useCraftLoopSession()
  const [activeTool, setActiveTool] = useState<ToolId>('pen')

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

  return (
    <div className={styles.workspace} data-session-ready={session.ready}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack session={session} activeTool={activeTool} />
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
        />
      </div>
    </div>
  )
}
