import type { Viewport } from './viewport'

const WORLD_GRID_SPACING = 50

/**
 * Article 16's "background layer: paper/grid." Purely decorative --
 * never a source of snap/precision data (that is Article 33/Task 092's
 * job, later). Shares the same viewport transform as every other
 * layer so the grid stays visually locked to world space while
 * panning/zooming.
 */
export function BackgroundGrid({ viewport }: { viewport: Viewport }) {
  const tile = WORLD_GRID_SPACING * viewport.zoom
  const offsetX = viewport.panX % tile
  const offsetY = viewport.panY % tile

  return (
    <svg
      className="background-grid"
      data-testid="background-grid"
      style={{ position: 'absolute', inset: 0, width: '100%', height: '100%' }}
    >
      <defs>
        <pattern
          id="craftloop-grid"
          width={tile}
          height={tile}
          patternUnits="userSpaceOnUse"
          x={offsetX}
          y={offsetY}
        >
          <path d={`M ${tile} 0 L 0 0 0 ${tile}`} fill="none" stroke="#e2e2df" strokeWidth={1} />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#craftloop-grid)" />
    </svg>
  )
}
