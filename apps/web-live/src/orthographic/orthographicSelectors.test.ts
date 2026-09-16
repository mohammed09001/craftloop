import { describe, expect, it } from 'vitest'
import {
  axesForIdentity,
  axisValue,
  isAxisUnresolved,
  orderViewBlocks,
  readinessTone,
} from './orthographicSelectors'
import type { DimensionSummary, ViewBlockSummary } from '../session/sceneTypes'

function view(overrides: Partial<ViewBlockSummary>): ViewBlockSummary {
  return {
    id: 'v1',
    identity: 'Front',
    geometry_member_ids: [],
    readiness: 'DraftReady',
    blockers: [],
    axis_bindings: [],
    unresolved_axes: [],
    ...overrides,
  }
}

describe('axesForIdentity', () => {
  it('mirrors the real Front/Back -> Width+Height, Top -> Width+Depth, Right -> Depth+Height table', () => {
    expect(axesForIdentity('Front')).toEqual(['Width', 'Height'])
    expect(axesForIdentity('Back')).toEqual(['Width', 'Height'])
    expect(axesForIdentity('Top')).toEqual(['Width', 'Depth'])
    expect(axesForIdentity('Right')).toEqual(['Depth', 'Height'])
  })
})

describe('axisValue', () => {
  const dimensions: DimensionSummary[] = [
    { id: 'd1', kind: 'Linear', role: 'Driving', value: 40, target_primitive_ids: [] },
  ]

  it('returns the real bound dimension value', () => {
    const v = view({ axis_bindings: [{ axis: 'Depth', dimension_id: 'd1' }] })
    expect(axisValue(v, 'Depth', dimensions)).toBe(40)
  })

  it('returns null when the axis has no binding -- never a guessed default', () => {
    const v = view({ axis_bindings: [] })
    expect(axisValue(v, 'Depth', dimensions)).toBeNull()
  })
})

describe('isAxisUnresolved', () => {
  it('reflects the real unresolved_axes list', () => {
    const v = view({ unresolved_axes: ['Depth'] })
    expect(isAxisUnresolved(v, 'Depth')).toBe(true)
    expect(isAxisUnresolved(v, 'Width')).toBe(false)
  })
})

describe('readinessTone', () => {
  it('buckets Resolved/Constrained as success, LinkReady as neutral, everything earlier as warning', () => {
    expect(readinessTone('Resolved')).toBe('success')
    expect(readinessTone('Constrained')).toBe('success')
    expect(readinessTone('LinkReady')).toBe('neutral')
    expect(readinessTone('IdentityReady')).toBe('warning')
    expect(readinessTone('DraftReady')).toBe('warning')
  })
})

describe('orderViewBlocks', () => {
  it('orders Front, Top, Right, Back and only includes a Back block when one is real', () => {
    const views = [
      view({ id: 'r', identity: 'Right' }),
      view({ id: 'f', identity: 'Front' }),
      view({ id: 't', identity: 'Top' }),
    ]
    expect(orderViewBlocks(views).map((v) => v.id)).toEqual(['f', 't', 'r'])
  })

  it('places a real Back block last', () => {
    const views = [
      view({ id: 'b', identity: 'Back' }),
      view({ id: 'f', identity: 'Front' }),
      view({ id: 't', identity: 'Top' }),
      view({ id: 'r', identity: 'Right' }),
    ]
    expect(orderViewBlocks(views).map((v) => v.id)).toEqual(['f', 't', 'r', 'b'])
  })
})
