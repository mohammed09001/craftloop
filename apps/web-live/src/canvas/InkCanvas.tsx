import { useCallback, useEffect, useRef } from 'react'
import type { PointerSampleInput, PointerSourceName } from '../session/pointerTypes'
import { computeShape, type PreviewKind } from './shapePreview'
import { screenToWorld, type ScreenPoint, type Viewport } from './viewport'

/** Execution 03, Phase 10, Task 083: how long the pointer must stay down without moving to count as a "hold." */
const HOLD_DURATION_MS = 500

/**
 * Execution 03, Phase 06/10, Task 044/046: the transient ink layer
 * (Article 16) plus the Pointer Events adapter (Article 15). Captures
 * every sample in screen space for cheap, immediate drawing while the
 * stroke is in progress (Task 044's "appears immediately" -- no
 * transform math on the hot path), then converts the whole stroke to
 * world space and hands it to `onStrokeComplete` on pointerup, which
 * is the only point this component ever touches real document state
 * (via the caller's `CraftLoopSession` calls).
 *
 * `previewKind` (Task 078-081): for a Sketch2D shape tool, every
 * `pointermove` redraws the *actual shape that will be created* (a
 * straight rubber-band line, a live circle, ...) via `computeShape`,
 * not a raw trace of the mouse path -- `'freehand'` (Pen) keeps the
 * original raw-trace behavior, since that trace *is* the real ink.
 *
 * Mouse is the default simulated pen (Article 15) -- `pointerType ===
 * 'mouse'` maps to `SimulatedMouse`, never claimed as real S Pen/Apple
 * Pencil data. `capability_palm_rejection`/`capability_eraser` are
 * always `false`: the Pointer Events API has no way to actually query
 * either from JS, so `false` is the honest answer, not a guess
 * (Article 65 -- the same "never fabricated" rule native's own
 * `FfiPointerSample` doc comment states for its platform capabilities).
 */
export function InkCanvas({
  viewport,
  active,
  previewKind,
  onStrokeComplete,
}: {
  viewport: Viewport
  /** `false` while panning or while a non-drawing tool owns the pointer. */
  active: boolean
  previewKind: PreviewKind
  onStrokeComplete: (
    samples: PointerSampleInput[],
    meta: { maxScreenDeviationPx: number; holdDetected: boolean },
  ) => void
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const drawingRef = useRef(false)
  const pointerIdRef = useRef<number | null>(null)
  const screenPointsRef = useRef<ScreenPoint[]>([])
  const samplesRef = useRef<PointerSampleInput[]>([])
  const viewportRef = useRef(viewport)
  useEffect(() => {
    viewportRef.current = viewport
  }, [viewport])
  const previewKindRef = useRef(previewKind)
  useEffect(() => {
    previewKindRef.current = previewKind
  }, [previewKind])
  const holdTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const holdDetectedRef = useRef(false)

  const resizeCanvas = useCallback(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const rect = canvas.getBoundingClientRect()
    const dpr = window.devicePixelRatio || 1
    canvas.width = Math.max(1, Math.round(rect.width * dpr))
    canvas.height = Math.max(1, Math.round(rect.height * dpr))
    const ctx = canvas.getContext('2d')
    ctx?.setTransform(dpr, 0, 0, dpr, 0, 0)
  }, [])

  useEffect(() => {
    resizeCanvas()
    window.addEventListener('resize', resizeCanvas)
    return () => window.removeEventListener('resize', resizeCanvas)
  }, [resizeCanvas])

  const redraw = useCallback(() => {
    const canvas = canvasRef.current
    const ctx = canvas?.getContext('2d')
    if (!canvas || !ctx) return
    const dpr = window.devicePixelRatio || 1
    ctx.clearRect(0, 0, canvas.width / dpr, canvas.height / dpr)
    const points = screenPointsRef.current
    if (points.length < 2) return

    ctx.strokeStyle = '#08060d'
    ctx.lineWidth = 2
    ctx.lineCap = 'round'
    ctx.lineJoin = 'round'

    const kind = previewKindRef.current
    if (kind === 'freehand') {
      ctx.beginPath()
      ctx.moveTo(points[0]!.x, points[0]!.y)
      for (let i = 1; i < points.length; i += 1) {
        ctx.lineTo(points[i]!.x, points[i]!.y)
      }
      ctx.stroke()
      return
    }

    const shape = computeShape(kind, points[0]!, points[points.length - 1]!)
    if (!shape) return
    ctx.beginPath()
    switch (shape.kind) {
      case 'line':
        ctx.moveTo(shape.a.x, shape.a.y)
        ctx.lineTo(shape.b.x, shape.b.y)
        break
      case 'circle':
        ctx.arc(shape.center.x, shape.center.y, shape.radius, 0, Math.PI * 2)
        break
      case 'rectangle':
        ctx.rect(shape.x, shape.y, shape.width, shape.height)
        break
      case 'arc':
        ctx.arc(shape.center.x, shape.center.y, shape.radius, shape.startAngle, shape.endAngle)
        break
    }
    ctx.stroke()
  }, [])

  const sourceFor = (pointerType: string): PointerSourceName => {
    if (pointerType === 'pen') return 'Stylus'
    if (pointerType === 'touch') return 'Touch'
    return 'SimulatedMouse'
  }

  const sampleFrom = useCallback((event: React.PointerEvent): PointerSampleInput => {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
    const screen = { x: event.clientX - rect.left, y: event.clientY - rect.top }
    const world = screenToWorld(viewportRef.current, screen)
    const isPen = event.pointerType === 'pen'
    return {
      x: world.x,
      y: world.y,
      timestamp_seconds: event.timeStamp / 1000,
      pressure: isPen ? event.pressure : null,
      tilt_x_deg: isPen ? event.tiltX : null,
      tilt_y_deg: isPen ? event.tiltY : null,
      source: sourceFor(event.pointerType),
      button_primary: (event.buttons & 1) !== 0,
      button_secondary: (event.buttons & 2) !== 0,
      button_barrel: (event.buttons & 32) !== 0,
      capability_pressure: isPen,
      capability_tilt: isPen,
      capability_hover: event.pointerType !== 'touch',
      // Neither is queryable through the Pointer Events API -- `false`
      // is the honest default, not a guess (see module doc comment).
      capability_palm_rejection: false,
      capability_eraser: false,
    }
  }, [])

  /** Task 083: (re)arms the hold-detection timer -- called on pointerdown and every pointermove. */
  const armHoldTimer = useCallback(() => {
    if (holdTimerRef.current !== null) clearTimeout(holdTimerRef.current)
    holdTimerRef.current = setTimeout(() => {
      holdDetectedRef.current = true
    }, HOLD_DURATION_MS)
  }, [])

  const clearHoldTimer = useCallback(() => {
    if (holdTimerRef.current !== null) clearTimeout(holdTimerRef.current)
    holdTimerRef.current = null
  }, [])

  const onPointerDown = useCallback(
    (event: React.PointerEvent) => {
      if (!active || event.button !== 0) return
      event.currentTarget.setPointerCapture(event.pointerId)
      drawingRef.current = true
      pointerIdRef.current = event.pointerId
      holdDetectedRef.current = false
      armHoldTimer()
      const rect = event.currentTarget.getBoundingClientRect()
      screenPointsRef.current = [{ x: event.clientX - rect.left, y: event.clientY - rect.top }]
      samplesRef.current = [sampleFrom(event)]
      redraw()
    },
    [active, armHoldTimer, redraw, sampleFrom],
  )

  const onPointerMove = useCallback(
    (event: React.PointerEvent) => {
      if (!drawingRef.current || event.pointerId !== pointerIdRef.current) return
      armHoldTimer()
      const rect = event.currentTarget.getBoundingClientRect()
      screenPointsRef.current.push({ x: event.clientX - rect.left, y: event.clientY - rect.top })
      samplesRef.current.push(sampleFrom(event))
      redraw()
    },
    [armHoldTimer, redraw, sampleFrom],
  )

  const finishStroke = useCallback(
    (event: React.PointerEvent) => {
      if (!drawingRef.current || event.pointerId !== pointerIdRef.current) return
      drawingRef.current = false
      pointerIdRef.current = null
      clearHoldTimer()
      const holdDetected = holdDetectedRef.current
      holdDetectedRef.current = false
      const samples = samplesRef.current
      const points = screenPointsRef.current
      screenPointsRef.current = []
      samplesRef.current = []
      redraw()
      // A true click (down+up with no move in between) never fires
      // `onPointerMove`, so `points`/`samples` can be as short as one
      // entry -- that single-sample case still needs to reach
      // `onStrokeComplete` so the caller's tap/select path runs; only
      // an event with no samples at all (should not happen once
      // `drawingRef.current` is true) is skipped.
      if (points.length === 0) return

      // How far the pointer actually traveled from where it went down --
      // distinguishes a tap (selection intent) from a real drag-draw
      // gesture, without needing a separate tool-mode switch this phase
      // does not yet have (that is Phase 08's WorkspaceMode).
      const first = points[0]!
      let maxDeviation = 0
      for (const point of points) {
        maxDeviation = Math.max(maxDeviation, Math.hypot(point.x - first.x, point.y - first.y))
      }
      onStrokeComplete(samples, { maxScreenDeviationPx: maxDeviation, holdDetected })
    },
    [clearHoldTimer, onStrokeComplete, redraw],
  )

  return (
    <canvas
      ref={canvasRef}
      className="ink-canvas"
      data-testid="ink-canvas"
      style={{
        position: 'absolute',
        inset: 0,
        width: '100%',
        height: '100%',
        touchAction: 'none',
        cursor: active ? 'crosshair' : 'default',
      }}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={finishStroke}
      onPointerCancel={finishStroke}
    />
  )
}
