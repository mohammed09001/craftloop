import { describe, expect, it } from 'vitest'
import { computeShape } from './shapePreview'

describe('computeShape', () => {
  it('returns null for freehand (nothing to preview as a shape)', () => {
    expect(computeShape('freehand', { x: 0, y: 0 }, { x: 10, y: 10 })).toBeNull()
  })

  it('line: endpoints are exactly the drag start/end', () => {
    const shape = computeShape('line', { x: 0, y: 0 }, { x: 10, y: 5 })
    expect(shape).toEqual({ kind: 'line', a: { x: 0, y: 0 }, b: { x: 10, y: 5 } })
  })

  it('circle: center at drag start, radius is the drag distance', () => {
    const shape = computeShape('circle', { x: 0, y: 0 }, { x: 3, y: 4 })
    expect(shape).toEqual({ kind: 'circle', center: { x: 0, y: 0 }, radius: 5 })
  })

  it('rectangle: normalizes to a positive x/y/width/height regardless of drag direction', () => {
    const shape = computeShape('rectangle', { x: 10, y: 10 }, { x: 0, y: 0 })
    expect(shape).toEqual({ kind: 'rectangle', x: 0, y: 0, width: 10, height: 10 })
  })

  it('arc: center at start, radius/angle from the end point, fixed quarter-circle sweep', () => {
    const shape = computeShape('arc', { x: 0, y: 0 }, { x: 10, y: 0 })
    expect(shape?.kind).toBe('arc')
    if (shape?.kind !== 'arc') throw new Error('expected arc')
    expect(shape.radius).toBeCloseTo(10)
    expect(shape.startAngle).toBeCloseTo(0)
    expect(shape.endAngle).toBeCloseTo(Math.PI / 2)
  })
})
