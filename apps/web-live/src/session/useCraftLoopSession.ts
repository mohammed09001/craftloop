import { useCallback, useEffect, useRef, useState } from 'react'
import {
  getCraftLoopSession,
  type CraftLoopSession,
  type WebDimensionKind,
} from './craftLoopSession'
import { EMPTY_SNAPSHOT, type SceneSnapshot } from './sceneTypes'
import type { PointerSampleInput } from './pointerTypes'

/**
 * Owns the one `CraftLoopSession` for this tab (Article 8) and exposes
 * its real scene snapshot as React state, refreshed after every
 * mutating call. Every action here is a thin pass-through to a real
 * `CraftLoopSession` method -- this hook never computes or invents
 * engineering state itself (Article 42's read-model discipline).
 */
export function useCraftLoopSession() {
  const sessionRef = useRef<CraftLoopSession | null>(null)
  const [snapshot, setSnapshot] = useState<SceneSnapshot>(EMPTY_SNAPSHOT)
  const [ready, setReady] = useState(false)
  const [lastError, setLastError] = useState<string | null>(null)

  const refresh = useCallback(() => {
    const session = sessionRef.current
    if (!session) return
    setSnapshot(JSON.parse(session.sceneSnapshot()) as SceneSnapshot)
  }, [])

  useEffect(() => {
    let cancelled = false
    getCraftLoopSession()
      .then((session) => {
        if (cancelled) return
        sessionRef.current = session
        setReady(true)
        setSnapshot(JSON.parse(session.sceneSnapshot()) as SceneSnapshot)
      })
      .catch((err: unknown) => {
        if (cancelled) return
        // The Wasm module failed to load/initialize (e.g. no real
        // browser fetch of the .wasm asset available). Surface it as a
        // visible error rather than leaving the workspace silently
        // stuck at `ready === false` forever.
        setLastError(err instanceof Error ? err.message : String(err))
      })
    return () => {
      cancelled = true
    }
  }, [])

  const guard = useCallback(
    <T,>(fn: (session: CraftLoopSession) => T): T | undefined => {
      const session = sessionRef.current
      if (!session) return undefined
      try {
        const result = fn(session)
        setLastError(null)
        refresh()
        return result
      } catch (err) {
        setLastError(err instanceof Error ? err.message : String(err))
        refresh()
        return undefined
      }
    },
    [refresh],
  )

  const submitStroke = useCallback(
    (samples: PointerSampleInput[]) =>
      guard((session) => JSON.parse(session.submitStroke(JSON.stringify(samples)))),
    [guard],
  )

  const createPrimitiveLine = useCallback(
    (x0: number, y0: number, x1: number, y1: number) =>
      guard((session) => session.createPrimitiveLine(x0, y0, x1, y1)),
    [guard],
  )

  const createPrimitiveCircle = useCallback(
    (centerX: number, centerY: number, radius: number) =>
      guard((session) => session.createPrimitiveCircle(centerX, centerY, radius)),
    [guard],
  )

  const createPrimitiveRectangle = useCallback(
    (x0: number, y0: number, x1: number, y1: number) =>
      guard((session) => session.createPrimitiveRectangle(x0, y0, x1, y1)),
    [guard],
  )

  const createDimension = useCallback(
    (kind: WebDimensionKind, targetIds: string[], value: number) =>
      guard((session) => session.createDimension(kind, targetIds, value)),
    [guard],
  )

  const select = useCallback(
    (ids: string[]) => guard((session) => session.select(ids)),
    [guard],
  )

  const clearSelection = useCallback(() => guard((session) => session.clearSelection()), [guard])

  const deleteSelected = useCallback(
    (ids: string[]) => guard((session) => session.deleteSelected(ids)),
    [guard],
  )

  const undo = useCallback(() => guard((session) => session.undo()), [guard])
  const redo = useCallback(() => guard((session) => session.redo()), [guard])

  return {
    ready,
    snapshot,
    lastError,
    submitStroke,
    createPrimitiveLine,
    createPrimitiveCircle,
    createPrimitiveRectangle,
    createDimension,
    select,
    clearSelection,
    deleteSelected,
    undo,
    redo,
  }
}

export type UseCraftLoopSession = ReturnType<typeof useCraftLoopSession>
