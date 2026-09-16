import type { ConstraintSummary, DimensionSummary, PrimitiveSummary } from '../session/sceneTypes'
import { cssTransform, type Viewport, type WorldPoint } from './viewport'

/**
 * Execution 03, Phase 12, Task 095/101: real dimension/constraint
 * annotations, read directly from the scene snapshot's own
 * `dimensions`/`constraints` arrays (`GeometryLayer`'s own doc comment
 * deliberately deferred this rendering here). Placement (leader-line
 * anchor, label offset) is presentation-only geometry computed here in
 * JS from the real target primitives' bounding boxes -- never a
 * duplicate dimension/constraint *model* (Article's backend
 * invariant): the value, kind, role, and target ids are exactly what
 * `WebDimensionSummary`/`WebConstraintSummary` already carry, nothing
 * invented.
 *
 * Task 101: `showAll` false (the default) renders only annotations
 * that touch a currently selected primitive -- "near selection," the
 * same restraint Task 094's geometry-adjacent input applies -- so a
 * document with many dimensions is not permanently dense (Creative
 * Precision). `showAll` true reveals every dimension/constraint in the
 * document, on demand.
 */
function primitiveCenter(primitive: PrimitiveSummary): WorldPoint {
  return {
    x: (primitive.min_x + primitive.max_x) / 2,
    y: (primitive.min_y + primitive.max_y) / 2,
  }
}

function formatValue(value: number): string {
  return Number.isInteger(value) ? value.toString() : value.toFixed(2)
}

const DIMENSION_LABEL_OFFSET_WORLD = 18

export function AnnotationLayer({
  viewport,
  primitives,
  dimensions,
  constraints,
  selectedIds,
  showAll,
  onDimensionClick,
}: {
  viewport: Viewport
  primitives: PrimitiveSummary[]
  dimensions: DimensionSummary[]
  constraints: ConstraintSummary[]
  selectedIds: ReadonlySet<string>
  showAll: boolean
  onDimensionClick: (dimensionId: string, anchorWorld: WorldPoint) => void
}) {
  const byId = new Map(primitives.map((p) => [p.id, p]))
  const touchesSelection = (ids: readonly string[]) => ids.some((id) => selectedIds.has(id))

  const visibleDimensions = dimensions.filter(
    (d) => showAll || touchesSelection(d.target_primitive_ids),
  )
  const visibleConstraints = constraints.filter(
    (c) => showAll || touchesSelection(c.primitive_ids),
  )

  return (
    <svg
      className="annotation-layer"
      data-testid="annotation-layer"
      style={{ position: 'absolute', inset: 0, width: '100%', height: '100%' }}
    >
      <g style={{ transform: cssTransform(viewport), transformOrigin: '0 0' }}>
        {visibleDimensions.map((dimension) => {
          const targets = dimension.target_primitive_ids
            .map((id) => byId.get(id))
            .filter((p): p is PrimitiveSummary => p !== undefined)
          if (targets.length === 0) return null
          const centers = targets.map(primitiveCenter)
          const anchorWorld: WorldPoint =
            centers.length === 1
              ? { x: centers[0]!.x, y: centers[0]!.y - DIMENSION_LABEL_OFFSET_WORLD / viewport.zoom }
              : {
                  x: (centers[0]!.x + centers[1]!.x) / 2,
                  y: (centers[0]!.y + centers[1]!.y) / 2 - DIMENSION_LABEL_OFFSET_WORLD / viewport.zoom,
                }
          return (
            <g
              key={dimension.id}
              data-testid={`dimension-annotation-${dimension.id}`}
              data-dimension-id={dimension.id}
              style={{ cursor: 'pointer' }}
              onClick={(event) => {
                event.stopPropagation()
                onDimensionClick(dimension.id, anchorWorld)
              }}
            >
              {centers.map((center, i) => (
                <line
                  key={i}
                  x1={center.x}
                  y1={center.y}
                  x2={anchorWorld.x}
                  y2={anchorWorld.y}
                  stroke="#6b6b68"
                  strokeWidth={1 / viewport.zoom}
                  strokeDasharray={`${3 / viewport.zoom} ${3 / viewport.zoom}`}
                />
              ))}
              <text
                x={anchorWorld.x}
                y={anchorWorld.y}
                fontSize={12 / viewport.zoom}
                textAnchor="middle"
                fill="#1f2937"
                stroke="#f5f5f4"
                strokeWidth={3 / viewport.zoom}
                paintOrder="stroke"
              >
                {formatValue(dimension.value)}
              </text>
            </g>
          )
        })}
        {visibleConstraints.map((constraint) => {
          const targets = constraint.primitive_ids
            .map((id) => byId.get(id))
            .filter((p): p is PrimitiveSummary => p !== undefined)
          if (targets.length === 0) return null
          const centers = targets.map(primitiveCenter)
          const badgeWorld: WorldPoint =
            centers.length === 1
              ? centers[0]!
              : { x: (centers[0]!.x + centers[1]!.x) / 2, y: (centers[0]!.y + centers[1]!.y) / 2 }
          return (
            <g
              key={constraint.id}
              data-testid={`constraint-annotation-${constraint.id}`}
              data-constraint-id={constraint.id}
            >
              <circle
                cx={badgeWorld.x}
                cy={badgeWorld.y}
                r={7 / viewport.zoom}
                fill="#fef3c7"
                stroke="#b45309"
                strokeWidth={1 / viewport.zoom}
              />
              <text
                x={badgeWorld.x}
                y={badgeWorld.y}
                fontSize={9 / viewport.zoom}
                textAnchor="middle"
                dominantBaseline="central"
                fill="#78350f"
              >
                {constraint.label.charAt(0)}
              </text>
              <title>{constraint.label}</title>
            </g>
          )
        })}
      </g>
    </svg>
  )
}
