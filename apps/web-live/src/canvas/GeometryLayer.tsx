import type { PrimitiveSummary, StrokeSummary } from '../session/sceneTypes'
import { cssTransform, type Viewport } from './viewport'

/**
 * Execution 03, Phase 06, Task 045: the structured SVG layer (Article
 * 16). Renders exactly the real geometry a `CraftLoopSession` scene
 * snapshot carries -- nothing here is computed or guessed; every
 * coordinate comes straight from `WebPrimitiveSummary.geometry`/
 * `WebStrokeSummary.points`.
 *
 * Dimension/constraint *annotation* rendering (leader lines, value
 * labels, constraint badges) is deliberately out of scope here --
 * Phase 12 (Dimensions and Adaptive Constraints UX) owns that
 * presentation layer; this phase renders confirmed geometry only.
 */
export function GeometryLayer({
  viewport,
  primitives,
  strokes,
  selectedIds,
  candidateId,
}: {
  viewport: Viewport
  primitives: PrimitiveSummary[]
  strokes: StrokeSummary[]
  selectedIds: ReadonlySet<string>
  /** Execution 03, Phase 10, Task 084: a Draw-and-Hold candidate awaiting confirm/cancel -- rendered as a dashed "ghost," not yet final. */
  candidateId?: string | undefined
}) {
  return (
    <svg
      className="geometry-layer"
      data-testid="geometry-layer"
      style={{ position: 'absolute', inset: 0, width: '100%', height: '100%' }}
    >
      <g style={{ transform: cssTransform(viewport), transformOrigin: '0 0' }}>
        {strokes.map((stroke) => (
          <polyline
            key={stroke.id}
            data-entity-id={stroke.id}
            data-entity-kind="stroke"
            points={stroke.points.map((p) => `${p.x},${p.y}`).join(' ')}
            fill="none"
            stroke={selectedIds.has(stroke.id) ? '#2563eb' : '#3a3a38'}
            strokeWidth={2 / viewport.zoom}
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        ))}
        {primitives.map((primitive) => (
          <PrimitiveShape
            key={primitive.id}
            primitive={primitive}
            selected={selectedIds.has(primitive.id)}
            isCandidate={primitive.id === candidateId}
            strokeWidthWorld={2 / viewport.zoom}
          />
        ))}
      </g>
    </svg>
  )
}

function PrimitiveShape({
  primitive,
  selected,
  isCandidate,
  strokeWidthWorld,
}: {
  primitive: PrimitiveSummary
  selected: boolean
  isCandidate: boolean
  strokeWidthWorld: number
}) {
  const color = isCandidate ? '#7c3aed' : selected ? '#2563eb' : '#08060d'
  const common = {
    'data-entity-id': primitive.id,
    'data-entity-kind': 'primitive',
    'data-candidate': isCandidate || undefined,
    fill: 'none',
    stroke: color,
    strokeWidth: strokeWidthWorld,
    strokeDasharray: isCandidate ? `${strokeWidthWorld * 3} ${strokeWidthWorld * 2}` : undefined,
  } as const

  const geometry = primitive.geometry
  if ('Line' in geometry) {
    const { a, b } = geometry.Line
    return <line {...common} x1={a.x} y1={a.y} x2={b.x} y2={b.y} />
  }
  if ('Circle' in geometry) {
    const { center, radius } = geometry.Circle
    return <circle {...common} cx={center.x} cy={center.y} r={radius} />
  }
  if ('Arc' in geometry) {
    const { center, radius, start_angle, sweep_angle } = geometry.Arc
    return <path {...common} d={arcPath(center, radius, start_angle, sweep_angle)} />
  }
  const points = geometry.Rectangle.corners.map((c) => `${c.x},${c.y}`).join(' ')
  return <polygon {...common} points={points} />
}

function arcPath(
  center: { x: number; y: number },
  radius: number,
  startAngle: number,
  sweepAngle: number,
): string {
  const start = {
    x: center.x + radius * Math.cos(startAngle),
    y: center.y + radius * Math.sin(startAngle),
  }
  const endAngle = startAngle + sweepAngle
  const end = {
    x: center.x + radius * Math.cos(endAngle),
    y: center.y + radius * Math.sin(endAngle),
  }
  const largeArc = Math.abs(sweepAngle) > Math.PI ? 1 : 0
  const sweepFlag = sweepAngle > 0 ? 1 : 0
  return `M ${start.x} ${start.y} A ${radius} ${radius} 0 ${largeArc} ${sweepFlag} ${end.x} ${end.y}`
}
