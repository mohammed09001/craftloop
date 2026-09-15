import { describe, expect, it } from 'vitest'
import { snapPoint } from './inference'
import type { PrimitiveSummary } from '../session/sceneTypes'

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

describe('snapPoint', () => {
  it('snaps to a nearby line endpoint', () => {
    const primitives = [line('a', { x: 0, y: 0 }, { x: 100, y: 0 })]
    const result = snapPoint({ x: 101, y: 2 }, primitives, 10)
    expect(result.point).toEqual({ x: 100, y: 0 })
    expect(result.guide).toEqual({ kind: 'endpoint', point: { x: 100, y: 0 } })
  })

  it('snaps to a line midpoint', () => {
    const primitives = [line('a', { x: 0, y: 0 }, { x: 100, y: 0 })]
    const result = snapPoint({ x: 48, y: 3 }, primitives, 10)
    expect(result.point).toEqual({ x: 50, y: 0 })
    expect(result.guide?.kind).toBe('midpoint')
  })

  it('snaps to a circle center', () => {
    const primitives = [circle('a', { x: 200, y: 200 }, 30)]
    const result = snapPoint({ x: 205, y: 195 }, primitives, 10)
    expect(result.point).toEqual({ x: 200, y: 200 })
    expect(result.guide?.kind).toBe('center')
  })

  it('falls back to the grid when nothing real is close', () => {
    const result = snapPoint({ x: 52, y: 3 }, [], 10)
    expect(result.point).toEqual({ x: 50, y: 0 })
    expect(result.guide?.kind).toBe('grid')
  })

  it('returns the point unchanged when nothing is within tolerance', () => {
    const result = snapPoint({ x: 17, y: 23 }, [], 5)
    expect(result.point).toEqual({ x: 17, y: 23 })
    expect(result.guide).toBeNull()
  })

  it('prefers real geometry over the grid when both are in range', () => {
    // A line endpoint deliberately placed off the 50-unit grid.
    const primitives = [line('a', { x: 12, y: 12 }, { x: 200, y: 200 })]
    const result = snapPoint({ x: 14, y: 14 }, primitives, 20)
    expect(result.point).toEqual({ x: 12, y: 12 })
    expect(result.guide?.kind).toBe('endpoint')
  })

  it('does not add or suggest any constraint -- only a coordinate (Task 089)', () => {
    const primitives = [line('a', { x: 0, y: 0 }, { x: 100, y: 0 })]
    const result = snapPoint({ x: 1, y: 1 }, primitives, 10)
    expect(Object.keys(result)).toEqual(['point', 'guide'])
    expect(result.guide).not.toHaveProperty('constraint')
  })
})
