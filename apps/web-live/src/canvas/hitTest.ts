import type { PrimitiveSummary, StrokeSummary } from '../session/sceneTypes'
import type { WorldPoint } from './viewport'

/**
 * Execution 03, Phase 06, Task 048: semantic hit testing. Returns the
 * real, stable domain id (`scene_snapshot().primitives[_].id`, etc.)
 * of whatever real entity sits under `point`, or `null` -- never a
 * synthesized id, never a guess about geometry the frontend does not
 * actually have (every shape tested here comes straight from a real
 * `WebPrimitiveSummary.geometry`/`WebStrokeSummary.points`).
 */
export function hitTestPrimitive(
  point: WorldPoint,
  primitive: PrimitiveSummary,
  toleranceWorld: number,
): boolean {
  const g = primitive.geometry
  if ('Line' in g) {
    return distanceToSegment(point, g.Line.a, g.Line.b) <= toleranceWorld
  }
  if ('Circle' in g) {
    const d = distance(point, g.Circle.center)
    return Math.abs(d - g.Circle.radius) <= toleranceWorld
  }
  if ('Arc' in g) {
    const { center, radius, start_angle, sweep_angle } = g.Arc
    const d = distance(point, center)
    if (Math.abs(d - radius) > toleranceWorld) return false
    const angle = normalizeAngle(Math.atan2(point.y - center.y, point.x - center.x))
    return angleWithinArc(angle, start_angle, sweep_angle)
  }
  if ('Rectangle' in g) {
    const [c0, c1, c2, c3] = g.Rectangle.corners
    const edges: [WorldPoint, WorldPoint][] = [
      [c0, c1],
      [c1, c2],
      [c2, c3],
      [c3, c0],
    ]
    return edges.some(([a, b]) => distanceToSegment(point, a, b) <= toleranceWorld)
  }
  return false
}

export function hitTestStroke(
  point: WorldPoint,
  stroke: StrokeSummary,
  toleranceWorld: number,
): boolean {
  for (let i = 0; i + 1 < stroke.points.length; i += 1) {
    if (distanceToSegment(point, stroke.points[i]!, stroke.points[i + 1]!) <= toleranceWorld) {
      return true
    }
  }
  return false
}

export interface HitTestableScene {
  primitives: PrimitiveSummary[]
  strokes: StrokeSummary[]
}

export type HitResult = { kind: 'primitive' | 'stroke'; id: string } | null

/**
 * Tests primitives before strokes (a beautified/structured shape wins
 * over a raw stroke it was recognized from, since only one of the two
 * is ever real document geometry the user is likely trying to select),
 * and within each group, the last-inserted entity wins ties -- iterated
 * in reverse so a shape drawn on top of another is preferred, matching
 * ordinary "topmost object" click semantics.
 */
export function hitTestScene(
  point: WorldPoint,
  scene: HitTestableScene,
  toleranceWorld: number,
): HitResult {
  for (let i = scene.primitives.length - 1; i >= 0; i -= 1) {
    const primitive = scene.primitives[i]!
    if (hitTestPrimitive(point, primitive, toleranceWorld)) {
      return { kind: 'primitive', id: primitive.id }
    }
  }
  for (let i = scene.strokes.length - 1; i >= 0; i -= 1) {
    const stroke = scene.strokes[i]!
    if (hitTestStroke(point, stroke, toleranceWorld)) {
      return { kind: 'stroke', id: stroke.id }
    }
  }
  return null
}

function distance(a: WorldPoint, b: WorldPoint): number {
  return Math.hypot(a.x - b.x, a.y - b.y)
}

function distanceToSegment(point: WorldPoint, a: WorldPoint, b: WorldPoint): number {
  const abx = b.x - a.x
  const aby = b.y - a.y
  const lengthSquared = abx * abx + aby * aby
  if (lengthSquared === 0) return distance(point, a)
  const t = Math.max(
    0,
    Math.min(1, ((point.x - a.x) * abx + (point.y - a.y) * aby) / lengthSquared),
  )
  const projection = { x: a.x + t * abx, y: a.y + t * aby }
  return distance(point, projection)
}

function normalizeAngle(angle: number): number {
  const twoPi = Math.PI * 2
  return ((angle % twoPi) + twoPi) % twoPi
}

function angleWithinArc(angle: number, startAngle: number, sweepAngle: number): boolean {
  const start = normalizeAngle(startAngle)
  const sweep = sweepAngle
  if (sweep >= 0) {
    const end = normalizeAngle(start + sweep)
    return end >= start ? angle >= start && angle <= end : angle >= start || angle <= end
  }
  const end = normalizeAngle(start + sweep)
  return end <= start ? angle <= start && angle >= end : angle <= start || angle >= end
}
