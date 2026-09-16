import { useCallback, useEffect, useRef, useState } from 'react'
import {
  createSession,
  getCraftLoopSession,
  resolveCommand as resolveCommandWasm,
  sessionFromJson,
  type CraftLoopSession,
  type WebCommandNamespace,
  type WebDimensionKind,
  type WebPrincipalViewIdentity,
  type WebResolutionChoice,
  type WebSharedAxis,
} from './craftLoopSession'
import type { GrammarMatch } from './commandTypes'
import { EMPTY_SNAPSHOT, type ConstraintOption, type SceneSnapshot } from './sceneTypes'
import type { PointerSampleInput } from './pointerTypes'
import { loadDocumentJson, saveDocumentJson } from '../persistence/documentStore'

/** Task 116: debounce after a committed transaction settles, never on every keystroke/pointer move within it. */
const AUTOSAVE_DEBOUNCE_MS = 800

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
      .then(async (freshSession) => {
        if (cancelled) return
        // Task 113/117: `getCraftLoopSession()` always hands back a
        // brand-new empty session -- if a real document was persisted
        // from an earlier visit, replace it with the real path-
        // independent round trip (`CraftLoopSession.fromJson`, the
        // same one `craftloop-web-bridge`'s own tests already cover)
        // rather than silently starting over.
        let session = freshSession
        try {
          const saved = await loadDocumentJson()
          if (saved) session = sessionFromJson(saved)
        } catch (err) {
          // A corrupt/unreadable saved document must not block the
          // workspace from opening -- fall back to the fresh session
          // and surface the problem, rather than getting stuck.
          setLastError(err instanceof Error ? err.message : String(err))
        }
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

  // -- Persistence (Phase 14, Tasks 114-118) ------------------------------

  const [lastSavedAt, setLastSavedAt] = useState<number | null>(null)
  const autosaveTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  const saveNow = useCallback(async () => {
    const session = sessionRef.current
    if (!session) return
    await saveDocumentJson(session.toJson())
    setLastSavedAt(Date.now())
  }, [])

  // Task 116: fires only when `snapshot.revision` -- the real
  // `Document::revision()`, which advances only on a committed
  // `DocumentChange`, never on ephemeral selection/workspace-mode/UI
  // state (Task 118) or a raw pointer move -- actually changes.
  useEffect(() => {
    if (!ready) return
    if (autosaveTimerRef.current !== null) clearTimeout(autosaveTimerRef.current)
    autosaveTimerRef.current = setTimeout(() => {
      void saveNow()
    }, AUTOSAVE_DEBOUNCE_MS)
    return () => {
      if (autosaveTimerRef.current !== null) clearTimeout(autosaveTimerRef.current)
    }
  }, [ready, snapshot.revision, saveNow])

  /**
   * Task 115's "New": a fresh, empty in-memory session -- like most
   * document apps' New, this does not delete the last real saved
   * document (`Open Last Saved` can still recover it); the fresh
   * empty document simply becomes what autosave persists from here on
   * once the user commits something.
   */
  const newDocument = useCallback(async () => {
    const session = await createSession()
    sessionRef.current = session
    setLastError(null)
    setSnapshot(JSON.parse(session.sceneSnapshot()) as SceneSnapshot)
  }, [])

  /** Task 115's "Open": discards any unsaved in-memory changes and reloads the last real saved document. */
  const openLastSaved = useCallback(async () => {
    const saved = await loadDocumentJson()
    if (!saved) return
    const session = sessionFromJson(saved)
    sessionRef.current = session
    setLastError(null)
    setSnapshot(JSON.parse(session.sceneSnapshot()) as SceneSnapshot)
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
      guard(
        (session) =>
          JSON.parse(session.submitStroke(JSON.stringify(samples))) as {
            stroke_id: string
            eligible_for_recognition: boolean
          },
      ),
    [guard],
  )

  /** Task 084: re-runs the real recognizer/beautifier against a stored stroke and, if it beautifies, replaces it with the resulting primitive -- returns the new primitive's id, or `undefined` if it stayed ink. */
  const acceptRecognition = useCallback(
    (strokeId: string) => guard((session) => session.acceptRecognition(strokeId)),
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

  const createPrimitiveArc = useCallback(
    (centerX: number, centerY: number, radius: number, startAngle: number, sweepAngle: number) =>
      guard((session) =>
        session.createPrimitiveArc(centerX, centerY, radius, startAngle, sweepAngle),
      ),
    [guard],
  )

  const applyConstraint = useCallback(
    (kindJson: string) => guard((session) => JSON.parse(session.applyConstraint(kindJson))),
    [guard],
  )

  const solveConstraints = useCallback(
    () => guard((session) => JSON.parse(session.solveConstraints())),
    [guard],
  )

  const setConstruction = useCallback(
    (primitiveId: string, flag: boolean) =>
      guard((session) => session.setConstruction(primitiveId, flag)),
    [guard],
  )

  const createDimension = useCallback(
    (kind: WebDimensionKind, targetIds: string[], value: number) =>
      guard((session) => session.createDimension(kind, targetIds, value)),
    [guard],
  )

  const editDimension = useCallback(
    (id: string, newValue: number) => guard((session) => session.editDimension(id, newValue)),
    [guard],
  )

  const resolveConflict = useCallback(
    (id: string, choice: WebResolutionChoice) =>
      guard((session) => session.resolveConflict(id, choice)),
    [guard],
  )

  const assignViewIdentity = useCallback(
    (viewId: string | undefined, identity: WebPrincipalViewIdentity) =>
      guard((session) => session.assignViewIdentity(viewId, identity)),
    [guard],
  )

  const addGeometryToView = useCallback(
    (viewId: string, primitiveIds: string[]) =>
      guard((session) => session.addGeometryToView(viewId, primitiveIds)),
    [guard],
  )

  const enterOrthographic = useCallback(
    (viewId: string) => guard((session) => session.enterOrthographic(viewId)),
    [guard],
  )

  const propagateSharedValue = useCallback(
    (viewId: string, axis: WebSharedAxis, value: number) =>
      guard(
        (session) =>
          JSON.parse(session.propagateSharedValue(viewId, axis, value)) as
            | { Propagated: { affected_views: string[] } }
            | { Conflict: { conflict_id: string } },
      ),
    [guard],
  )

  /**
   * Task 098: a real, backend-derived read -- no mutation, so (like
   * `resolveCommand`) it bypasses `guard`/`refresh`.
   */
  const eligibleConstraints = useCallback((selectedIds: string[]): ConstraintOption[] => {
    if (!sessionRef.current) return []
    return JSON.parse(sessionRef.current.eligibleConstraints(selectedIds)) as ConstraintOption[]
  }, [])

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

  const enterSketchMode = useCallback(
    () => guard((session) => session.enterSketchMode()),
    [guard],
  )
  const enterCreativePenMode = useCallback(
    () => guard((session) => session.enterCreativePenMode()),
    [guard],
  )

  /**
   * Task 062: resolve recognized text against the real Command Bus
   * grammar without performing any action -- a read-only lookup, so it
   * does not go through `guard`/`refresh` (nothing about the document
   * or session state changes).
   */
  const resolveCommand = useCallback(
    (input: string, namespace: WebCommandNamespace): GrammarMatch | undefined => {
      if (!sessionRef.current) return undefined
      return JSON.parse(resolveCommandWasm(input, namespace)) as GrammarMatch
    },
    [],
  )

  return {
    ready,
    snapshot,
    lastError,
    lastSavedAt,
    saveNow,
    newDocument,
    openLastSaved,
    submitStroke,
    acceptRecognition,
    createPrimitiveLine,
    createPrimitiveCircle,
    createPrimitiveRectangle,
    createPrimitiveArc,
    createDimension,
    editDimension,
    resolveConflict,
    applyConstraint,
    eligibleConstraints,
    solveConstraints,
    setConstruction,
    assignViewIdentity,
    addGeometryToView,
    enterOrthographic,
    propagateSharedValue,
    select,
    clearSelection,
    deleteSelected,
    undo,
    redo,
    enterSketchMode,
    enterCreativePenMode,
    resolveCommand,
  }
}

export type UseCraftLoopSession = ReturnType<typeof useCraftLoopSession>
