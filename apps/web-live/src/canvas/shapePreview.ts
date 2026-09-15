import type { ScreenPoint } from './viewport'

/**
 * Execution 03, Phase 10, Task 078-081: the single source of truth for
 * "what shape does this drag define" -- used both for the live
 * rubber-band/ghost preview `InkCanvas` draws every `pointermove`
 * (Article 28's "Explicit Geometry Preview") and for the final
 * `CraftLoopSession.createPrimitive*` call `CanvasStack` makes on
 * release, so the preview never drifts from what actually gets
 * created. Pure geometry only -- this is never treated as real
 * document state; it exists purely for the transient Canvas layer, the
 * same as the raw-ink trace it replaces for these tools (Article 16).
 *
 * No `ellipse`/`spline` kind: Phase 09 deliberately did not add those
 * tools (see `toolRegistry.ts`'s doc comment), so there is nothing to
 * preview for them either.
 */
export type PreviewKind = 'freehand' | 'line' | 'circle' | 'rectangle' | 'arc'

export interface LineShape {
  kind: 'line'
  a: ScreenPoint
  b: ScreenPoint
}
export interface CircleShape {
  kind: 'circle'
  center: ScreenPoint
  radius: number
}
export interface RectangleShape {
  kind: 'rectangle'
  x: number
  y: number
  width: number
  height: number
}
export interface ArcShape {
  kind: 'arc'
  center: ScreenPoint
  radius: number
  startAngle: number
  /** A fixed quarter-circle sweep -- see `CanvasStack.tsx`'s own note on this being Phase 09's simplest honest 2-point mapping. */
  endAngle: number
}
export type Shape = LineShape | CircleShape | RectangleShape | ArcShape

/** `null` for `'freehand'`, or when `first`/`last` coincide (a zero-size drag has nothing meaningful to preview). */
export function computeShape(
  kind: PreviewKind,
  first: ScreenPoint,
  last: ScreenPoint,
): Shape | null {
  switch (kind) {
    case 'line':
      return { kind: 'line', a: first, b: last }
    case 'circle':
      return {
        kind: 'circle',
        center: first,
        radius: Math.hypot(last.x - first.x, last.y - first.y),
      }
    case 'rectangle':
      return {
        kind: 'rectangle',
        x: Math.min(first.x, last.x),
        y: Math.min(first.y, last.y),
        width: Math.abs(last.x - first.x),
        height: Math.abs(last.y - first.y),
      }
    case 'arc': {
      const radius = Math.hypot(last.x - first.x, last.y - first.y)
      const startAngle = Math.atan2(last.y - first.y, last.x - first.x)
      return { kind: 'arc', center: first, radius, startAngle, endAngle: startAngle + Math.PI / 2 }
    }
    default:
      return null
  }
}
