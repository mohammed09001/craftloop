import type { PrimitiveSummary } from '../session/sceneTypes'
import { cssTransform, type Viewport } from './viewport'

/**
 * Execution 03, Phase 06, Task 049: selection handles, kept lightweight
 * and platform-neutral -- a dashed bounding-box outline plus corner
 * dots read straight from each selected primitive's real
 * `min_x`/`min_y`/`max_x`/`max_y` (already in the scene snapshot, no
 * new geometry computed here). No resize/rotate drag affordance yet --
 * that is a later phase's job once an editing tool needs it; this
 * phase only needs a visible "this is selected" signal (Article 16's
 * "interaction overlay").
 */
export function SelectionOverlay({
  viewport,
  selectedPrimitives,
}: {
  viewport: Viewport
  selectedPrimitives: PrimitiveSummary[]
}) {
  if (selectedPrimitives.length === 0) return null

  return (
    <svg
      className="selection-overlay"
      data-testid="selection-overlay"
      style={{ position: 'absolute', inset: 0, width: '100%', height: '100%', pointerEvents: 'none' }}
    >
      <g style={{ transform: cssTransform(viewport), transformOrigin: '0 0' }}>
        {selectedPrimitives.map((primitive) => {
          const width = primitive.max_x - primitive.min_x
          const height = primitive.max_y - primitive.min_y
          const padding = 4 / viewport.zoom
          const x = primitive.min_x - padding
          const y = primitive.min_y - padding
          const w = width + padding * 2
          const h = height + padding * 2
          const corners = [
            { x, y },
            { x: x + w, y },
            { x: x + w, y: y + h },
            { x, y: y + h },
          ]
          const dotRadius = 3 / viewport.zoom
          return (
            <g key={primitive.id} data-testid="selection-handle" data-entity-id={primitive.id}>
              <rect
                x={x}
                y={y}
                width={w}
                height={h}
                fill="none"
                stroke="#2563eb"
                strokeWidth={1 / viewport.zoom}
                strokeDasharray={`${4 / viewport.zoom} ${4 / viewport.zoom}`}
              />
              {corners.map((corner, i) => (
                <circle
                  key={i}
                  cx={corner.x}
                  cy={corner.y}
                  r={dotRadius}
                  fill="#ffffff"
                  stroke="#2563eb"
                  strokeWidth={1 / viewport.zoom}
                />
              ))}
            </g>
          )
        })}
      </g>
    </svg>
  )
}
