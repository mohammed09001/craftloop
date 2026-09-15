import { describe, expect, it } from 'vitest'
import { eligibleConstraintOptions } from './constraintOptions'
import type { PrimitiveSummary } from '../session/sceneTypes'

function line(id: string): PrimitiveSummary {
  return {
    id,
    kind: 'Line',
    min_x: 0,
    min_y: 0,
    max_x: 1,
    max_y: 1,
    geometry: { Line: { a: { x: 0, y: 0 }, b: { x: 1, y: 1 } } },
  }
}

function circle(id: string): PrimitiveSummary {
  return {
    id,
    kind: 'Circle',
    min_x: -1,
    min_y: -1,
    max_x: 1,
    max_y: 1,
    geometry: { Circle: { center: { x: 0, y: 0 }, radius: 1 } },
  }
}

function rectangle(id: string): PrimitiveSummary {
  return {
    id,
    kind: 'Rectangle',
    min_x: 0,
    min_y: 0,
    max_x: 1,
    max_y: 1,
    geometry: {
      Rectangle: {
        corners: [
          { x: 0, y: 0 },
          { x: 1, y: 0 },
          { x: 1, y: 1 },
          { x: 0, y: 1 },
        ],
      },
    },
  }
}

describe('eligibleConstraintOptions', () => {
  it('offers Horizontal/Vertical for exactly one selected line', () => {
    const options = eligibleConstraintOptions([line('a')])
    expect(options.map((o) => o.label)).toEqual(['Horizontal', 'Vertical'])
    expect(options[0]?.payload).toEqual({ Horizontal: { line: 'a' } })
  })

  it('offers line-pair constraints for two selected lines', () => {
    const options = eligibleConstraintOptions([line('a'), line('b')])
    expect(options.map((o) => o.label)).toEqual([
      'Parallel',
      'Perpendicular',
      'Equal Length',
      'Coincident',
    ])
    expect(options[0]?.payload).toEqual({ Parallel: { a: 'a', b: 'b' } })
  })

  it('offers circle-pair constraints for two selected circles', () => {
    const options = eligibleConstraintOptions([circle('a'), circle('b')])
    expect(options.map((o) => o.label)).toEqual(['Equal Radius', 'Concentric'])
  })

  it('offers nothing for an unsupported combination', () => {
    expect(eligibleConstraintOptions([])).toEqual([])
    expect(eligibleConstraintOptions([rectangle('a')])).toEqual([])
    expect(eligibleConstraintOptions([line('a'), circle('b')])).toEqual([])
    expect(eligibleConstraintOptions([line('a'), line('b'), line('c')])).toEqual([])
  })
})
