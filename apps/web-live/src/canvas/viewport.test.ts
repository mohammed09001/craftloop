import { describe, expect, it } from 'vitest'
import {
  IDENTITY_VIEWPORT,
  clampZoom,
  fitToBounds,
  panBy,
  screenToWorld,
  worldToScreen,
  zoomAt,
} from './viewport'

describe('viewport', () => {
  it('round-trips world <-> screen through the identity viewport', () => {
    const world = { x: 12.5, y: -4 }
    const screen = worldToScreen(IDENTITY_VIEWPORT, world)
    expect(screen).toEqual({ x: 12.5, y: -4 })
    expect(screenToWorld(IDENTITY_VIEWPORT, screen)).toEqual(world)
  })

  it('round-trips world <-> screen through a panned and zoomed viewport', () => {
    const viewport = { panX: 100, panY: -50, zoom: 2 }
    const world = { x: 30, y: 40 }
    const screen = worldToScreen(viewport, world)
    const back = screenToWorld(viewport, screen)
    expect(back.x).toBeCloseTo(world.x)
    expect(back.y).toBeCloseTo(world.y)
  })

  it('panBy only changes pan, never zoom', () => {
    const viewport = { panX: 0, panY: 0, zoom: 3 }
    const panned = panBy(viewport, 10, -5)
    expect(panned).toEqual({ panX: 10, panY: -5, zoom: 3 })
  })

  it('zoomAt keeps the world point under the anchor fixed on screen', () => {
    const viewport = { panX: 0, panY: 0, zoom: 1 }
    const anchor = { x: 200, y: 150 }
    const worldUnderAnchorBefore = screenToWorld(viewport, anchor)

    const zoomed = zoomAt(viewport, anchor, 2)
    const worldUnderAnchorAfter = screenToWorld(zoomed, anchor)

    expect(worldUnderAnchorAfter.x).toBeCloseTo(worldUnderAnchorBefore.x)
    expect(worldUnderAnchorAfter.y).toBeCloseTo(worldUnderAnchorBefore.y)
    expect(zoomed.zoom).toBe(2)
  })

  it('clampZoom keeps zoom within [MIN_ZOOM, MAX_ZOOM]', () => {
    expect(clampZoom(0)).toBeGreaterThan(0)
    expect(clampZoom(1000)).toBeLessThan(1000)
    expect(clampZoom(1)).toBe(1)
  })

  it('fitToBounds centers the bounds and keeps them within the viewport', () => {
    const bounds = { minX: 0, minY: 0, maxX: 100, maxY: 50 }
    const size = { width: 800, height: 600 }
    const viewport = fitToBounds(bounds, size, 0)

    const topLeft = worldToScreen(viewport, { x: bounds.minX, y: bounds.minY })
    const bottomRight = worldToScreen(viewport, { x: bounds.maxX, y: bounds.maxY })
    expect(topLeft.x).toBeGreaterThanOrEqual(-1e-6)
    expect(topLeft.y).toBeGreaterThanOrEqual(-1e-6)
    expect(bottomRight.x).toBeLessThanOrEqual(size.width + 1e-6)
    expect(bottomRight.y).toBeLessThanOrEqual(size.height + 1e-6)

    // The wider axis (bounds is 2:1, viewport is ~1.33:1) is the
    // binding constraint, so it should fill available width closely.
    expect(bottomRight.x - topLeft.x).toBeCloseTo(size.width, 0)
  })
})
