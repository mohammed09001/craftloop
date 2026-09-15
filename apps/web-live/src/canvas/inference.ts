import type { PrimitiveSummary } from '../session/sceneTypes'
import type { WorldPoint } from './viewport'

/**
 * Execution 03, Phase 11, Task 087-089: precision inference -- given a
 * raw point and the real primitives already on the page, suggest a
 * nearby "obviously meant" point (an endpoint, a midpoint, a center,
 * a grid intersection) to snap to. Pure geometry over data the scene
 * snapshot already provides -- this never invents or stores anything;
 * it only adjusts the coordinate a caller is about to pass to a real
 * `createPrimitive*`/drag call.
 *
 * Task 089's "conservative auto-constraint policy": this module
 * returns a *coordinate*, never a constraint. Landing a new line's
 * endpoint exactly on an existing line's endpoint is a numeric
 * coincidence the solver can't tell apart from a real `Coincident`
 * constraint -- but it is not one. Only an explicit Constraint action
 * (Task 073) ever adds a real `SketchConstraintKind`. No dense,
 * silently-created permanent relations.
 */

export type InferenceGuideKind = 'endpoint' | 'midpoint' | 'center' | 'corner' | 'grid'

export interface InferenceGuide {
  kind: InferenceGuideKind
  point: WorldPoint
}

const GRID_SPACING_WORLD = 50

/** How close (screen px) the pointer must be to a real candidate point to snap to it -- shared by `InkCanvas`'s live preview and `CanvasStack`'s final creation call so they always agree. */
export const SNAP_TOLERANCE_SCREEN_PX = 10

function candidatePoints(primitive: PrimitiveSummary): InferenceGuide[] {
  const g = primitive.geometry
  if ('Line' in g) {
    const { a, b } = g.Line
    return [
      { kind: 'endpoint', point: a },
      { kind: 'endpoint', point: b },
      { kind: 'midpoint', point: { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 } },
    ]
  }
  if ('Circle' in g) {
    return [{ kind: 'center', point: g.Circle.center }]
  }
  if ('Arc' in g) {
    return [{ kind: 'center', point: g.Arc.center }]
  }
  return g.Rectangle.corners.map((corner) => ({ kind: 'corner' as const, point: corner }))
}

function nearestGridPoint(point: WorldPoint, spacing: number): WorldPoint {
  return {
    x: Math.round(point.x / spacing) * spacing,
    y: Math.round(point.y / spacing) * spacing,
  }
}

function distance(a: WorldPoint, b: WorldPoint): number {
  return Math.hypot(a.x - b.x, a.y - b.y)
}

/**
 * Snaps `point` (world space) to the nearest real geometry candidate
 * within `toleranceWorld`, falling back to the nearest grid
 * intersection, or returns `point` unchanged if nothing is close
 * enough. Geometry candidates win over the grid when both are in
 * range -- aligning to real, already-drawn geometry is normally more
 * useful than aligning to an arbitrary grid line.
 */
export function snapPoint(
  point: WorldPoint,
  primitives: readonly PrimitiveSummary[],
  toleranceWorld: number,
): { point: WorldPoint; guide: InferenceGuide | null } {
  let best: InferenceGuide | null = null
  let bestDistance = toleranceWorld
  for (const primitive of primitives) {
    for (const candidate of candidatePoints(primitive)) {
      const d = distance(point, candidate.point)
      if (d < bestDistance) {
        bestDistance = d
        best = candidate
      }
    }
  }
  if (best) return { point: best.point, guide: best }

  const gridPoint = nearestGridPoint(point, GRID_SPACING_WORLD)
  if (distance(point, gridPoint) < toleranceWorld) {
    const guide: InferenceGuide = { kind: 'grid', point: gridPoint }
    return { point: gridPoint, guide }
  }

  return { point, guide: null }
}
