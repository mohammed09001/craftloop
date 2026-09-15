import { describe, expect, it } from 'vitest'
import { hitTestPrimitive, hitTestScene } from './hitTest'
import type { PrimitiveSummary, StrokeSummary } from '../session/sceneTypes'

function line(id: string, a: { x: number; y: number }, b: { x: number; y: number }): PrimitiveSummary {
  return {
    id,
    kind: 'Line',
    min_x: Math.min(a.x, b.x),
    min_y: Math.min(a.y, b.y),
    max_x: Math.max(a.x, b.x),
    max_y: Math.max(a.y, b.y),
    geometry: { Line: { a, b } },
    is_construction: false,
  }
}

function circle(id: string, center: { x: number; y: number }, radius: number): PrimitiveSummary {
  return {
    id,
    kind: 'Circle',
    min_x: center.x - radius,
    min_y: center.y - radius,
    max_x: center.x + radius,
    max_y: center.y + radius,
    geometry: { Circle: { center, radius } },
    is_construction: false,
  }
}

function rectangle(
  id: string,
  corners: [
    { x: number; y: number },
    { x: number; y: number },
    { x: number; y: number },
    { x: number; y: number },
  ],
): PrimitiveSummary {
  const xs = corners.map((c) => c.x)
  const ys = corners.map((c) => c.y)
  return {
    id,
    kind: 'Rectangle',
    min_x: Math.min(...xs),
    min_y: Math.min(...ys),
    max_x: Math.max(...xs),
    max_y: Math.max(...ys),
    geometry: { Rectangle: { corners } },
    is_construction: false,
  }
}

describe('hitTestPrimitive', () => {
  it('hits a point near a line segment, not far off it', () => {
    const l = line('line-1', { x: 0, y: 0 }, { x: 10, y: 0 })
    expect(hitTestPrimitive({ x: 5, y: 0.5 }, l, 1)).toBe(true)
    expect(hitTestPrimitive({ x: 5, y: 5 }, l, 1)).toBe(false)
    // Beyond the segment's end, even if collinear.
    expect(hitTestPrimitive({ x: 15, y: 0 }, l, 1)).toBe(false)
  });

  it('hits a point near a circle boundary, not its interior or exterior', () => {
    const c = circle('circle-1', { x: 0, y: 0 }, 5)
    expect(hitTestPrimitive({ x: 5, y: 0 }, c, 0.5)).toBe(true)
    expect(hitTestPrimitive({ x: 0, y: 0 }, c, 0.5)).toBe(false)
    expect(hitTestPrimitive({ x: 100, y: 0 }, c, 0.5)).toBe(false)
  })

  it('hits a point near any rectangle edge', () => {
    const r = rectangle('rect-1', [
      { x: 0, y: 0 },
      { x: 10, y: 0 },
      { x: 10, y: 10 },
      { x: 0, y: 10 },
    ])
    expect(hitTestPrimitive({ x: 5, y: 0.2 }, r, 0.5)).toBe(true)
    expect(hitTestPrimitive({ x: 0.2, y: 5 }, r, 0.5)).toBe(true)
    expect(hitTestPrimitive({ x: 5, y: 5 }, r, 0.5)).toBe(false)
  })
})

describe('hitTestScene', () => {
  it('returns the real stable id of the hit entity', () => {
    const scene = {
      primitives: [line('line-1', { x: 0, y: 0 }, { x: 10, y: 0 })],
      strokes: [] as StrokeSummary[],
    }
    expect(hitTestScene({ x: 5, y: 0 }, scene, 1)).toEqual({ kind: 'primitive', id: 'line-1' })
    expect(hitTestScene({ x: 5, y: 50 }, scene, 1)).toBeNull()
  })

  it('prefers a primitive over a stroke at the same point, and the topmost primitive among overlaps', () => {
    const scene = {
      primitives: [
        line('line-bottom', { x: 0, y: 0 }, { x: 10, y: 0 }),
        line('line-top', { x: 0, y: 0 }, { x: 10, y: 0 }),
      ],
      strokes: [
        { id: 'stroke-1', sample_count: 2, points: [{ x: 0, y: 0 }, { x: 10, y: 0 }] },
      ] as StrokeSummary[],
    }
    expect(hitTestScene({ x: 5, y: 0 }, scene, 1)).toEqual({ kind: 'primitive', id: 'line-top' })
  })
})
