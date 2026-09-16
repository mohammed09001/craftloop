import type {
  AxisBinding,
  DimensionSummary,
  PrincipalViewIdentityName,
  SharedAxisName,
  ViewBlockSummary,
} from '../session/sceneTypes'

/**
 * Execution 03, Phase 13, Task 108/112: which two of the three shared
 * axes (Width/Height/Depth) a given principal view actually consumes.
 * Mirrors the real, static
 * `craftloop_document::multiview::axes_for_identity` table -- fixed by
 * third-angle projection convention, not user/document data, so this
 * is a safe presentation-routing mirror (same category as
 * `PrincipalViewIdentityName` itself), unlike real engineering
 * *values* (dimensions, constraints, conflicts), which always come
 * straight from the scene snapshot and are never computed here.
 */
export function axesForIdentity(identity: PrincipalViewIdentityName): [SharedAxisName, SharedAxisName] {
  switch (identity) {
    case 'Front':
    case 'Back':
      return ['Width', 'Height']
    case 'Top':
      return ['Width', 'Depth']
    case 'Right':
      return ['Depth', 'Height']
  }
}

export function axisBinding(view: ViewBlockSummary, axis: SharedAxisName): AxisBinding | undefined {
  return view.axis_bindings.find((b) => b.axis === axis)
}

/** Task 107: the axis's real bound value, or `null` when nothing is bound yet -- never a guessed default. */
export function axisValue(
  view: ViewBlockSummary,
  axis: SharedAxisName,
  dimensions: readonly DimensionSummary[],
): number | null {
  const binding = axisBinding(view, axis)
  if (!binding) return null
  return dimensions.find((d) => d.id === binding.dimension_id)?.value ?? null
}

export function isAxisUnresolved(view: ViewBlockSummary, axis: SharedAxisName): boolean {
  return view.unresolved_axes.includes(axis)
}

export type ReadinessTone = 'success' | 'neutral' | 'warning'

/** Task 107: a coarse visual tone for the real `OrthographicReadiness` value -- the label itself is always the real name, this only picks a color bucket. */
export function readinessTone(readiness: ViewBlockSummary['readiness']): ReadinessTone {
  if (readiness === 'Resolved' || readiness === 'Constrained') return 'success'
  if (readiness === 'LinkReady') return 'neutral'
  return 'warning'
}

/** Front first, then Top/Right in the order the graph reports them, Back last and only when it is real (Task 105: never fabricate a Back block). */
export function orderViewBlocks(views: readonly ViewBlockSummary[]): ViewBlockSummary[] {
  const order: Record<string, number> = { Front: 0, Top: 1, Right: 2, Back: 3 }
  return [...views].sort((a, b) => {
    const ai = a.identity ? (order[a.identity] ?? 99) : 99
    const bi = b.identity ? (order[b.identity] ?? 99) : 99
    return ai - bi
  })
}
