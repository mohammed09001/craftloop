import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { AnnotationLayer } from './AnnotationLayer'
import type { ConstraintSummary, DimensionSummary, PrimitiveSummary } from '../session/sceneTypes'
import { IDENTITY_VIEWPORT } from './viewport'

afterEach(() => cleanup())

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

function dimension(id: string, targets: string[], value: number): DimensionSummary {
  return { id, kind: 'Linear', role: 'Driving', value, target_primitive_ids: targets }
}

function constraint(id: string, label: string, primitiveIds: string[]): ConstraintSummary {
  return { id, label, primitive_ids: primitiveIds }
}

describe('AnnotationLayer', () => {
  const primitives = [line('a', { x: 0, y: 0 }, { x: 100, y: 0 })]

  it('renders a real dimension value from the scene snapshot, not an invented label (Task 095)', () => {
    render(
      <AnnotationLayer
        viewport={IDENTITY_VIEWPORT}
        primitives={primitives}
        dimensions={[dimension('d1', ['a'], 42.5)]}
        constraints={[]}
        selectedIds={new Set(['a'])}
        showAll={false}
        onDimensionClick={vi.fn()}
      />,
    )
    expect(screen.getByTestId('dimension-annotation-d1')).toHaveTextContent('42.5')
  })

  it('hides annotations that do not touch the selection when showAll is false (Task 101)', () => {
    render(
      <AnnotationLayer
        viewport={IDENTITY_VIEWPORT}
        primitives={primitives}
        dimensions={[dimension('d1', ['a'], 42.5)]}
        constraints={[constraint('c1', 'Horizontal', ['a'])]}
        selectedIds={new Set()}
        showAll={false}
        onDimensionClick={vi.fn()}
      />,
    )
    expect(screen.queryByTestId('dimension-annotation-d1')).not.toBeInTheDocument()
    expect(screen.queryByTestId('constraint-annotation-c1')).not.toBeInTheDocument()
  })

  it('shows every annotation regardless of selection when showAll is true (Task 101)', () => {
    render(
      <AnnotationLayer
        viewport={IDENTITY_VIEWPORT}
        primitives={primitives}
        dimensions={[dimension('d1', ['a'], 42.5)]}
        constraints={[constraint('c1', 'Horizontal', ['a'])]}
        selectedIds={new Set()}
        showAll={true}
        onDimensionClick={vi.fn()}
      />,
    )
    expect(screen.getByTestId('dimension-annotation-d1')).toBeInTheDocument()
    expect(screen.getByTestId('constraint-annotation-c1')).toBeInTheDocument()
  })

  it('calls onDimensionClick with the real dimension id and a world-space anchor when clicked (Task 096)', async () => {
    const onDimensionClick = vi.fn()
    render(
      <AnnotationLayer
        viewport={IDENTITY_VIEWPORT}
        primitives={primitives}
        dimensions={[dimension('d1', ['a'], 42.5)]}
        constraints={[]}
        selectedIds={new Set(['a'])}
        showAll={false}
        onDimensionClick={onDimensionClick}
      />,
    )
    screen.getByTestId('dimension-annotation-d1').dispatchEvent(
      new MouseEvent('click', { bubbles: true }),
    )
    expect(onDimensionClick).toHaveBeenCalledTimes(1)
    const [id, anchor] = onDimensionClick.mock.calls[0]!
    expect(id).toBe('d1')
    expect(anchor).toEqual({ x: 50, y: expect.any(Number) })
  })
})
