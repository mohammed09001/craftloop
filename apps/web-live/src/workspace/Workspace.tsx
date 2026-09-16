import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { usePanZoom } from '../canvas/usePanZoom'
import type { WorldPoint } from '../canvas/viewport'
import { worldToScreen } from '../canvas/viewport'
import { CanvasStack } from '../canvas/CanvasStack'
import { useCraftLoopSession } from '../session/useCraftLoopSession'
import { WebDimensionKind, WebResolutionChoice } from '../session/craftLoopSession'
import type { ConflictSummary, ConstraintOption } from '../session/sceneTypes'
import { Toolbar } from '../toolbar/Toolbar'
import type { ToolId } from '../toolbar/toolRegistry'
import { DimensionInputPopover } from './DimensionInputPopover'
import { SolverFeedback, type SolverFeedbackTone } from './SolverFeedback'
import { useKeyboardShortcuts } from './useKeyboardShortcuts'
import styles from './Workspace.module.css'

type DimensionPopoverState =
  | { mode: 'create'; anchorWorld: WorldPoint }
  | { mode: 'edit'; dimensionId: string; anchorWorld: WorldPoint; initialValue: number }

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
 * need to agree on it. Construction (Task 090/091) and Snap (Task
 * 092/093) follow the same pattern: `constructionEnabled` is derived
 * from the real selection, and `snapEnabled` is the one piece of state
 * both `CanvasStack` and `Toolbar` need to agree on, so it lives here.
 *
 * Phase 12 moves `usePanZoom` up from `CanvasStack` to here: the
 * geometry-adjacent Dimension popover (Task 094/096) is positioned
 * with the exact same `worldToScreen` transform every canvas layer
 * uses, computed from the real selection's own bounding box -- never a
 * distant global panel (`window.prompt`, Phase 09's stopgap).
 * Dimension/Constraint eligibility (Task 072/073/098) comes straight
 * from the real `session.eligibleConstraints` backend call, not a
 * second, frontend-maintained copy of the arity/type rules.
 */
export function Workspace() {
  const session = useCraftLoopSession()
  const [activeTool, setActiveTool] = useState<ToolId>('pen')
  const [snapEnabled, setSnapEnabled] = useState(true)
  const [showAllAnnotations, setShowAllAnnotations] = useState(false)
  const { viewport, isPanning, onWheel, beginPan, endPan, panByScreenDelta } = usePanZoom()

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

  const selectionAnchorWorld = useCallback((): WorldPoint | null => {
    if (selectedPrimitives.length === 0) return null
    const centers = selectedPrimitives.map((p) => ({
      x: (p.min_x + p.max_x) / 2,
      y: (p.min_y + p.max_y) / 2,
    }))
    return centers.length === 1
      ? centers[0]!
      : { x: (centers[0]!.x + centers[1]!.x) / 2, y: (centers[0]!.y + centers[1]!.y) / 2 }
  }, [selectedPrimitives])

  const dimensionEnabled = selectedPrimitives.length === 1 || selectedPrimitives.length === 2

  // -- Dimension input (Task 094/096/097) --------------------------------

  const [dimensionPopover, setDimensionPopover] = useState<DimensionPopoverState | null>(null)
  const [dimensionError, setDimensionError] = useState<string | null>(null)
  const [dimensionConflict, setDimensionConflict] = useState<ConflictSummary | null>(null)
  const pendingEditRef = useRef<{ dimensionId: string; attemptedValue: number } | null>(null)

  const handleDimension = useCallback(() => {
    const anchorWorld = selectionAnchorWorld()
    if (!anchorWorld) return
    setDimensionError(null)
    setDimensionConflict(null)
    setDimensionPopover({ mode: 'create', anchorWorld })
  }, [selectionAnchorWorld])

  const handleEditDimensionRequest = useCallback(
    (dimensionId: string, anchorWorld: WorldPoint) => {
      const dimension = session.snapshot.dimensions.find((d) => d.id === dimensionId)
      if (!dimension) return
      setDimensionError(null)
      setDimensionConflict(null)
      setDimensionPopover({ mode: 'edit', dimensionId, anchorWorld, initialValue: dimension.value })
    },
    [session.snapshot.dimensions],
  )

  const closeDimensionPopover = useCallback(() => {
    setDimensionPopover(null)
    setDimensionError(null)
    setDimensionConflict(null)
    pendingEditRef.current = null
  }, [])

  const handleDimensionSubmit = useCallback(
    (value: number) => {
      if (!dimensionPopover) return
      if (dimensionPopover.mode === 'create') {
        const id = session.createDimension(
          WebDimensionKind.Linear,
          selectedPrimitives.map((p) => p.id),
          value,
        )
        if (id) {
          closeDimensionPopover()
        } else {
          setDimensionError(session.lastError ?? 'Could not create dimension')
        }
        return
      }
      pendingEditRef.current = { dimensionId: dimensionPopover.dimensionId, attemptedValue: value }
      session.editDimension(dimensionPopover.dimensionId, value)
    },
    [dimensionPopover, session, selectedPrimitives, closeDimensionPopover],
  )

  // Task 097: "keep last valid geometry and show explanation." A
  // failed `editDimension` still commits a real `Conflict` entity
  // (`session.rs::edit_dimension`'s own error branch) -- the dimension
  // value itself is untouched (never a client-side pretend-rollback).
  // This effect correlates that real, persisted conflict back to the
  // edit attempt once the refreshed snapshot lands.
  useEffect(() => {
    const pending = pendingEditRef.current
    if (!pending || dimensionPopover?.mode !== 'edit') return
    const current = session.snapshot.dimensions.find((d) => d.id === pending.dimensionId)
    if (current && Math.abs(current.value - pending.attemptedValue) < 1e-9) {
      pendingEditRef.current = null
      closeDimensionPopover()
      return
    }
    const conflict = session.snapshot.conflicts.find(
      (c) => c.unresolved && c.affected_entities.some((e) => e.includes(pending.dimensionId)),
    )
    if (conflict) {
      pendingEditRef.current = null
      setDimensionConflict(conflict)
    }
  }, [session.snapshot, dimensionPopover, closeDimensionPopover])

  const handleResolveDimensionConflict = useCallback(
    (choice: WebResolutionChoice) => {
      if (!dimensionConflict) return
      session.resolveConflict(dimensionConflict.id, choice)
      closeDimensionPopover()
    },
    [dimensionConflict, session, closeDimensionPopover],
  )

  // -- Constraints (Task 072/073/098/099/100) -----------------------------

  const constraintOptions: ConstraintOption[] = useMemo(
    () => session.eligibleConstraints(selectedPrimitives.map((p) => p.id)),
    [session, selectedPrimitives],
  )

  const [solverFeedback, setSolverFeedback] = useState<{
    tone: SolverFeedbackTone
    message: string
  } | null>(null)
  const solverFeedbackTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  const showSolverFeedback = useCallback((tone: SolverFeedbackTone, message: string) => {
    if (solverFeedbackTimerRef.current !== null) clearTimeout(solverFeedbackTimerRef.current)
    setSolverFeedback({ tone, message })
    solverFeedbackTimerRef.current = setTimeout(() => setSolverFeedback(null), 4000)
  }, [])

  const handleApplyConstraint = useCallback(
    (payload: ConstraintOption['payload']) => {
      // Applying a constraint that isn't visibly reflected in the
      // geometry would look broken, not "real" (Task 073) -- solve
      // immediately afterward, the same real EzpzSolver call the
      // (currently absent) explicit Solve action would make.
      const outcome = session.applyConstraint(JSON.stringify(payload))
      if (!outcome) {
        showSolverFeedback('error', session.lastError ?? 'Could not apply constraint')
        return
      }
      if ('Redundant' in outcome) {
        showSolverFeedback('warning', 'Already implied by existing constraints (redundant)')
        return
      }
      const solved = session.solveConstraints()
      if (!solved) {
        showSolverFeedback('error', session.lastError ?? 'Solve failed')
        return
      }
      if (solved.status === 'Solved') {
        showSolverFeedback('success', `Solved (${solved.updated_primitive_count} updated)`)
      } else if (solved.status === 'Unsatisfied') {
        showSolverFeedback(
          'warning',
          `Unsatisfied: ${solved.unsatisfied_constraint_ids.length} constraint(s) could not be satisfied`,
        )
      } else {
        showSolverFeedback('error', 'Solve failed')
      }
    },
    [session, showSolverFeedback],
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

  const handleToggleAnnotations = useCallback(() => {
    setShowAllAnnotations((shown) => !shown)
  }, [])

  const dimensionAnchorScreen = dimensionPopover
    ? worldToScreen(viewport, dimensionPopover.anchorWorld)
    : null

  return (
    <div className={styles.workspace} data-session-ready={session.ready}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack
          session={session}
          activeTool={activeTool}
          snapEnabled={snapEnabled}
          showAllAnnotations={showAllAnnotations}
          viewport={viewport}
          isPanning={isPanning}
          onWheel={onWheel}
          beginPan={beginPan}
          endPan={endPan}
          panByScreenDelta={panByScreenDelta}
          onEditDimensionRequest={handleEditDimensionRequest}
        />
        {dimensionPopover && dimensionAnchorScreen && (
          <DimensionInputPopover
            mode={dimensionPopover.mode}
            anchorScreen={dimensionAnchorScreen}
            initialValue={dimensionPopover.mode === 'edit' ? dimensionPopover.initialValue : undefined}
            errorMessage={dimensionError}
            conflict={dimensionConflict}
            onSubmit={handleDimensionSubmit}
            onCancel={closeDimensionPopover}
            onResolve={handleResolveDimensionConflict}
          />
        )}
        {solverFeedback && <SolverFeedback tone={solverFeedback.tone} message={solverFeedback.message} />}
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
          showAllAnnotations={showAllAnnotations}
          onToggleAnnotations={handleToggleAnnotations}
        />
      </div>
    </div>
  )
}
