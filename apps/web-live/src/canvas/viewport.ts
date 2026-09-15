/**
 * Execution 03, Phase 06, Task 043: one viewport transform shared by
 * every geometry layer (Article 16: "All geometry layers share one
 * viewport transform"). Pure math only -- no DOM, no React -- so the
 * background grid, the transient ink Canvas, and the structured SVG
 * layer can each apply exactly the same `panX`/`panY`/`zoom` and stay
 * pixel-aligned with each other.
 */

export interface Viewport {
  panX: number
  panY: number
  zoom: number
}

export interface ScreenPoint {
  x: number
  y: number
}

export interface WorldPoint {
  x: number
  y: number
}

export const IDENTITY_VIEWPORT: Viewport = { panX: 0, panY: 0, zoom: 1 }

export const MIN_ZOOM = 0.1
export const MAX_ZOOM = 20

export function clampZoom(zoom: number): number {
  return Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom))
}

/** World-space engineering coordinates -> screen-space pixel coordinates. */
export function worldToScreen(viewport: Viewport, world: WorldPoint): ScreenPoint {
  return {
    x: world.x * viewport.zoom + viewport.panX,
    y: world.y * viewport.zoom + viewport.panY,
  }
}

/** Screen-space pixel coordinates -> world-space engineering coordinates. */
export function screenToWorld(viewport: Viewport, screen: ScreenPoint): WorldPoint {
  return {
    x: (screen.x - viewport.panX) / viewport.zoom,
    y: (screen.y - viewport.panY) / viewport.zoom,
  }
}

/** A CSS transform applying this viewport, for an SVG/HTML layer's root group. */
export function cssTransform(viewport: Viewport): string {
  return `translate(${viewport.panX}px, ${viewport.panY}px) scale(${viewport.zoom})`
}

/** Pan by a screen-space pixel delta. Viewport-only -- never touches document state. */
export function panBy(viewport: Viewport, dx: number, dy: number): Viewport {
  return { ...viewport, panX: viewport.panX + dx, panY: viewport.panY + dy }
}

/**
 * Zoom by `factor` while keeping the world point currently under
 * `screenAnchor` fixed on screen (the usual "zoom toward the cursor"
 * behavior), clamped to `[MIN_ZOOM, MAX_ZOOM]`.
 */
export function zoomAt(viewport: Viewport, screenAnchor: ScreenPoint, factor: number): Viewport {
  const newZoom = clampZoom(viewport.zoom * factor)
  const world = screenToWorld(viewport, screenAnchor)
  return {
    zoom: newZoom,
    panX: screenAnchor.x - world.x * newZoom,
    panY: screenAnchor.y - world.y * newZoom,
  }
}

/**
 * A viewport that fits `bounds` (world-space) into `viewportSizePx`
 * (screen-space), with `paddingPx` of margin on every side. Used for
 * fit-to-content.
 */
export function fitToBounds(
  bounds: { minX: number; minY: number; maxX: number; maxY: number },
  viewportSizePx: { width: number; height: number },
  paddingPx = 48,
): Viewport {
  const boundsWidth = Math.max(bounds.maxX - bounds.minX, 1e-6)
  const boundsHeight = Math.max(bounds.maxY - bounds.minY, 1e-6)
  const availableWidth = Math.max(viewportSizePx.width - paddingPx * 2, 1)
  const availableHeight = Math.max(viewportSizePx.height - paddingPx * 2, 1)
  const zoom = clampZoom(Math.min(availableWidth / boundsWidth, availableHeight / boundsHeight))

  const centerWorldX = (bounds.minX + bounds.maxX) / 2
  const centerWorldY = (bounds.minY + bounds.maxY) / 2
  const centerScreenX = viewportSizePx.width / 2
  const centerScreenY = viewportSizePx.height / 2

  return {
    zoom,
    panX: centerScreenX - centerWorldX * zoom,
    panY: centerScreenY - centerWorldY * zoom,
  }
}
