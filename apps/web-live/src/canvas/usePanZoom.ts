import { useCallback, useState } from 'react'
import { IDENTITY_VIEWPORT, panBy, zoomAt, type ScreenPoint, type Viewport } from './viewport'

/**
 * Execution 03, Phase 06, Task 047: pan/zoom changes the viewport only
 * -- it never calls a `CraftLoopSession` method, never touches document
 * state. Wheel zooms toward the cursor; a middle-button drag pans.
 * Left-button drag is reserved for drawing (`InkCanvas`), so panning
 * never fights with the primary "mouse simulates pen" interaction
 * (Article 15).
 */
export function usePanZoom() {
  const [viewport, setViewport] = useState<Viewport>(IDENTITY_VIEWPORT)
  const [isPanning, setIsPanning] = useState(false)

  const onWheel = useCallback((event: React.WheelEvent, anchor: ScreenPoint) => {
    event.preventDefault()
    const factor = Math.exp(-event.deltaY * 0.001)
    setViewport((current) => zoomAt(current, anchor, factor))
  }, [])

  const beginPan = useCallback(() => setIsPanning(true), [])
  const endPan = useCallback(() => setIsPanning(false), [])

  const panByScreenDelta = useCallback((dx: number, dy: number) => {
    setViewport((current) => panBy(current, dx, dy))
  }, [])

  return { viewport, setViewport, isPanning, onWheel, beginPan, endPan, panByScreenDelta }
}
