import { useCallback, useMemo, useRef } from 'react'
import type { UseCraftLoopSession } from '../session/useCraftLoopSession'
import type { PointerSampleInput } from '../session/pointerTypes'
import { BackgroundGrid } from './BackgroundGrid'
import { GeometryLayer } from './GeometryLayer'
import { InkCanvas } from './InkCanvas'
import { SelectionOverlay } from './SelectionOverlay'
import { usePanZoom } from './usePanZoom'
import { hitTestScene } from './hitTest'

/** Below this many screen pixels of travel, a pointer gesture is a tap (select), not a drag (draw). */
const TAP_THRESHOLD_PX = 4

/**
 * Execution 03, Phase 06: the composition root for the whole canvas
 * stack (Article 16's five layers, minus the toolbar layer, which is
 * Phase 07). Owns the one shared viewport transform (Task 043) and
 * wires every layer to it, wires the Pointer Events adapter to real
 * `CraftLoopSession` calls (`submitStroke`/`select`), and performs
 * semantic hit testing (Task 048) for click-to-select.
 */
export function CanvasStack({ session }: { session: UseCraftLoopSession }) {
  const containerRef = useRef<HTMLDivElement>(null)
  const { viewport, isPanning, onWheel, beginPan, endPan, panByScreenDelta } = usePanZoom()
  const lastPanPointRef = useRef<{ x: number; y: number } | null>(null)

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

  const handleStrokeComplete = useCallback(
    (samples: PointerSampleInput[], meta: { maxScreenDeviationPx: number }) => {
      const first = samples[0]
      if (!first) return

      if (meta.maxScreenDeviationPx < TAP_THRESHOLD_PX) {
        // `InkCanvas` already converted every sample to world space
        // before calling back, so `first.x`/`first.y` is the tap's
        // real world-space point -- no re-derivation needed.
        const toleranceWorld = 6 / viewport.zoom
        const hit = hitTestScene({ x: first.x, y: first.y }, session.snapshot, toleranceWorld)
        if (hit) {
          session.select([hit.id])
        } else {
          session.clearSelection()
        }
        return
      }
      session.submitStroke(samples)
    },
    [session, viewport.zoom],
  )

  return (
    <div
      ref={containerRef}
      className="canvas-stack"
      data-testid="canvas-stack"
      style={{ position: 'absolute', inset: 0, overflow: 'hidden' }}
      onWheel={handleWheel}
      onPointerDown={handlePanDown}
      onPointerMove={handlePanMove}
      onPointerUp={handlePanUp}
      onPointerCancel={handlePanUp}
    >
      <BackgroundGrid viewport={viewport} />
      <GeometryLayer
        viewport={viewport}
        primitives={session.snapshot.primitives}
        strokes={session.snapshot.strokes}
        selectedIds={selectedIds}
      />
      <SelectionOverlay viewport={viewport} selectedPrimitives={selectedPrimitives} />
      <InkCanvas viewport={viewport} active={!isPanning} onStrokeComplete={handleStrokeComplete} />
    </div>
  )
}
