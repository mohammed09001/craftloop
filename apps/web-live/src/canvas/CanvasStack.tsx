import { useCallback, useMemo, useRef, useState } from 'react'
import type { UseCraftLoopSession } from '../session/useCraftLoopSession'
import type { PointerSampleInput } from '../session/pointerTypes'
import type { ToolId } from '../toolbar/toolRegistry'
import { BackgroundGrid } from './BackgroundGrid'
import { GeometryLayer } from './GeometryLayer'
import { InkCanvas } from './InkCanvas'
import { RefinementBar } from './RefinementBar'
import { SelectionOverlay } from './SelectionOverlay'
import { usePanZoom } from './usePanZoom'
import { hitTestScene } from './hitTest'
import { computeShape, type PreviewKind } from './shapePreview'
import { screenToWorld } from './viewport'

/** Below this many screen pixels of travel, a pointer gesture is a tap (select), not a drag (draw). */
const TAP_THRESHOLD_PX = 4
const HIT_TEST_TOLERANCE_SCREEN_PX = 6

/** Execution 03, Phase 09: Sketch2D tools that create a primitive from one drag, not free ink. */
const SHAPE_TOOLS = new Set<ToolId>(['line', 'arc', 'circle', 'rectangle'])

/** Execution 03, Phase 10: maps the active tool to what `InkCanvas` should live-preview while dragging (Task 078-081). */
function previewKindFor(tool: ToolId): PreviewKind {
  if (tool === 'line' || tool === 'arc' || tool === 'circle' || tool === 'rectangle') return tool
  return 'freehand'
}

/**
 * Execution 03, Phase 06/07/09: the composition root for the whole
 * canvas stack (Article 16's five layers, minus the toolbar layer).
 * Owns the one shared viewport transform (Task 043) and wires every
 * layer to it, wires the Pointer Events adapter to real
 * `CraftLoopSession` calls, and performs semantic hit testing (Task
 * 048).
 *
 * `activeTool` changes real interaction behavior: Pen keeps Phase
 * 06's draw-or-tap-to-select behavior (now with Phase 10's
 * Draw-and-Hold refinement, see below); Select disables drawing so
 * every click is a selection attempt; Eraser hit-tests a click and
 * deletes what it finds; the Sketch2D shape tools (Line/Arc/Circle/
 * Rectangle, Task 066-069) turn one drag into one real primitive,
 * previewed live while dragging (Task 078-081) via the same
 * `computeShape` the final creation call uses. This is deliberately
 * local component state, not the formal `WorkspaceMode` Phase 08
 * introduces on the session -- see `toolRegistry.ts`'s own doc
 * comment for the reasoning.
 *
 * Draw-and-Hold (Task 083-086): a Pen stroke that pauses for
 * `HOLD_DURATION_MS` before release is submitted as a real stroke,
 * then immediately run through the real recognizer/beautifier
 * (`acceptRecognition`, Task 084) -- the result renders as a dashed
 * "candidate" primitive (`GeometryLayer`'s `candidateId`) with a
 * confirm/cancel bar (Task 085). Confirm leaves it; Cancel calls the
 * real `session.undo()`, reverting exactly that one transaction and
 * restoring the raw ink (Task 086). A stroke released *without* a
 * hold, or one recognition finds nothing beautifiable, stays plain ink
 * with no interruption -- no silent conversion either way.
 */
export function CanvasStack({
  session,
  activeTool,
}: {
  session: UseCraftLoopSession
  activeTool: ToolId
}) {
  const containerRef = useRef<HTMLDivElement>(null)
  const { viewport, isPanning, onWheel, beginPan, endPan, panByScreenDelta } = usePanZoom()
  const lastPanPointRef = useRef<{ x: number; y: number } | null>(null)
  const [pendingCandidateId, setPendingCandidateId] = useState<string | null>(null)

  const selectedIds = useMemo(
    () => new Set(session.snapshot.selected_entity_ids),
    [session.snapshot.selected_entity_ids],
  )
  const selectedPrimitives = useMemo(
    () => session.snapshot.primitives.filter((p) => selectedIds.has(p.id)),
    [session.snapshot.primitives, selectedIds],
  )

  const handleWheel = useCallback(
    (event: React.WheelEvent) => {
      const rect = containerRef.current?.getBoundingClientRect()
      if (!rect) return
      onWheel(event, { x: event.clientX - rect.left, y: event.clientY - rect.top })
    },
    [onWheel],
  )

  const handlePanDown = useCallback(
    (event: React.PointerEvent) => {
      if (event.button !== 1) return
      event.preventDefault()
      event.currentTarget.setPointerCapture(event.pointerId)
      beginPan()
      lastPanPointRef.current = { x: event.clientX, y: event.clientY }
    },
    [beginPan],
  )

  const handlePanMove = useCallback(
    (event: React.PointerEvent) => {
      if (!isPanning || !lastPanPointRef.current) return
      const dx = event.clientX - lastPanPointRef.current.x
      const dy = event.clientY - lastPanPointRef.current.y
      lastPanPointRef.current = { x: event.clientX, y: event.clientY }
      panByScreenDelta(dx, dy)
    },
    [isPanning, panByScreenDelta],
  )

  const handlePanUp = useCallback(() => {
    endPan()
    lastPanPointRef.current = null
  }, [endPan])

  const createShapeFromDrag = useCallback(
    (tool: ToolId, first: { x: number; y: number }, last: { x: number; y: number }) => {
      // `computeShape` is coordinate-space-agnostic (pure geometry), so
      // the same function that drives `InkCanvas`'s live screen-space
      // preview (Task 078-081) also drives this world-space creation
      // call -- one source of truth, never two independently-computed
      // versions of "what shape does this drag define."
      const shape = computeShape(tool as PreviewKind, first, last)
      if (!shape) return
      switch (shape.kind) {
        case 'line':
          session.createPrimitiveLine(shape.a.x, shape.a.y, shape.b.x, shape.b.y)
          return
        case 'rectangle':
          session.createPrimitiveRectangle(shape.x, shape.y, shape.x + shape.width, shape.y + shape.height)
          return
        case 'circle':
          session.createPrimitiveCircle(shape.center.x, shape.center.y, shape.radius)
          return
        case 'arc':
          session.createPrimitiveArc(
            shape.center.x,
            shape.center.y,
            shape.radius,
            shape.startAngle,
            shape.endAngle - shape.startAngle,
          )
          return
      }
    },
    [session],
  )

  const handleStrokeComplete = useCallback(
    (
      samples: PointerSampleInput[],
      meta: { maxScreenDeviationPx: number; holdDetected: boolean },
    ) => {
      const first = samples[0]
      if (!first) return

      if (SHAPE_TOOLS.has(activeTool)) {
        const last = samples[samples.length - 1] ?? first
        createShapeFromDrag(activeTool, first, last)
        return
      }

      if (meta.maxScreenDeviationPx < TAP_THRESHOLD_PX) {
        // `InkCanvas` already converted every sample to world space
        // before calling back, so `first.x`/`first.y` is the tap's
        // real world-space point -- no re-derivation needed.
        const toleranceWorld = HIT_TEST_TOLERANCE_SCREEN_PX / viewport.zoom
        const hit = hitTestScene({ x: first.x, y: first.y }, session.snapshot, toleranceWorld)
        if (hit) {
          session.select([hit.id])
        } else {
          session.clearSelection()
        }
        return
      }

      const outcome = session.submitStroke(samples)
      // Task 083/084: only a held stroke (paused before release) gets a
      // refinement attempt -- a quick stroke stays plain ink with no
      // interruption, matching Draw-and-Hold's own name. `outcome` is
      // `undefined` if the stroke itself failed validation, in which
      // case there is nothing to refine either.
      if (meta.holdDetected && outcome?.eligible_for_recognition) {
        const primitiveId = session.acceptRecognition(outcome.stroke_id)
        if (primitiveId) setPendingCandidateId(primitiveId)
      }
    },
    [activeTool, createShapeFromDrag, session, viewport.zoom],
  )

  const handleConfirmCandidate = useCallback(() => {
    setPendingCandidateId(null)
  }, [])

  const handleCancelCandidate = useCallback(() => {
    // Reverts exactly the one `acceptRecognition` transaction (Task
    // 086) -- the same real undo Undo/Redo already use, not a
    // client-side pretend-rollback.
    session.undo()
    setPendingCandidateId(null)
  }, [session])

  /** Select/Eraser modes: InkCanvas is inactive, so a plain click reaches here directly. */
  const handleContainerClick = useCallback(
    (event: React.MouseEvent) => {
      if (activeTool !== 'select' && activeTool !== 'eraser') return
      const rect = containerRef.current?.getBoundingClientRect()
      if (!rect) return
      const world = screenToWorld(viewport, { x: event.clientX - rect.left, y: event.clientY - rect.top })
      const toleranceWorld = HIT_TEST_TOLERANCE_SCREEN_PX / viewport.zoom
      const hit = hitTestScene(world, session.snapshot, toleranceWorld)

      if (activeTool === 'eraser') {
        if (hit) session.deleteSelected([hit.id])
        return
      }
      if (hit) {
        session.select([hit.id])
      } else {
        session.clearSelection()
      }
    },
    [activeTool, session, viewport],
  )

  const inkCanvasActive = !isPanning && (activeTool === 'pen' || SHAPE_TOOLS.has(activeTool))

  return (
    <div
      ref={containerRef}
      className="canvas-stack"
      data-testid="canvas-stack"
      data-active-tool={activeTool}
      style={{ position: 'absolute', inset: 0, overflow: 'hidden' }}
      onWheel={handleWheel}
      onPointerDown={handlePanDown}
      onPointerMove={handlePanMove}
      onPointerUp={handlePanUp}
      onPointerCancel={handlePanUp}
      onClick={handleContainerClick}
    >
      <BackgroundGrid viewport={viewport} />
      <GeometryLayer
        viewport={viewport}
        primitives={session.snapshot.primitives}
        strokes={session.snapshot.strokes}
        selectedIds={selectedIds}
        candidateId={pendingCandidateId ?? undefined}
      />
      <SelectionOverlay viewport={viewport} selectedPrimitives={selectedPrimitives} />
      <InkCanvas
        viewport={viewport}
        active={inkCanvasActive}
        previewKind={previewKindFor(activeTool)}
        onStrokeComplete={handleStrokeComplete}
      />
      {pendingCandidateId && (
        <RefinementBar onConfirm={handleConfirmCandidate} onCancel={handleCancelCandidate} />
      )}
    </div>
  )
}
