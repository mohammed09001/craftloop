import { useCallback, useMemo, useState } from 'react'
import type { UseCraftLoopSession } from '../session/useCraftLoopSession'
import { WebPrincipalViewIdentity, WebResolutionChoice, WebSharedAxis } from '../session/craftLoopSession'
import type { SharedAxisName } from '../session/sceneTypes'
import { orderViewBlocks } from './orthographicSelectors'
import { ViewBlockCard } from './ViewBlockCard'

/**
 * Execution 03, Phase 13, Task 102-112: the real Orthographic View
 * Block panel -- opened by the toolbar's View toggle, replacing the
 * canvas area's content while open (Task 106: flat 2D engineering
 * regions, not a 3D viewport). Every view block, its readiness, and
 * its shared-axis values come straight from the real
 * `CraftLoopSession` scene snapshot; the only thing this component
 * invents is layout.
 *
 * Task 112 ("no permanent master view"): every view block's axis
 * fields call the same `handleCommitAxis`, passing *that* block's own
 * real id -- editing Depth from the Top block and editing it from the
 * Right block are the exact same code path, so nothing here treats
 * Front as more authoritative than any other view.
 */
export function OrthographicPanel({
  session,
  selectedPrimitiveIds,
  onClose,
}: {
  session: UseCraftLoopSession
  selectedPrimitiveIds: string[]
  onClose: () => void
}) {
  const [activeConflictId, setActiveConflictId] = useState<string | null>(null)

  const orthographicSet = session.snapshot.orthographic_sets[0]
  const viewBlocks = useMemo(() => {
    if (!orthographicSet) return []
    const ids = new Set(orthographicSet.view_ids)
    return orderViewBlocks(session.snapshot.view_blocks.filter((v) => ids.has(v.id)))
  }, [orthographicSet, session.snapshot.view_blocks])

  const handleCreateFrontView = useCallback(() => {
    if (selectedPrimitiveIds.length === 0) return
    const frontId = session.assignViewIdentity(undefined, WebPrincipalViewIdentity.Front)
    if (!frontId) return
    session.addGeometryToView(frontId, selectedPrimitiveIds)
    session.enterOrthographic(frontId)
  }, [session, selectedPrimitiveIds])

  const handleCommitAxis = useCallback(
    (viewId: string, axis: SharedAxisName, value: number) => {
      const outcome = session.propagateSharedValue(viewId, WebSharedAxis[axis], value)
      if (outcome && 'Conflict' in outcome) {
        setActiveConflictId(outcome.Conflict.conflict_id)
      } else {
        setActiveConflictId(null)
      }
    },
    [session],
  )

  const conflict = activeConflictId
    ? session.snapshot.conflicts.find((c) => c.id === activeConflictId)
    : undefined

  const handleResolveConflict = useCallback(
    (choice: WebResolutionChoice) => {
      if (!activeConflictId) return
      session.resolveConflict(activeConflictId, choice)
      setActiveConflictId(null)
    },
    [activeConflictId, session],
  )

  return (
    <div
      data-testid="orthographic-panel"
      style={{
        position: 'absolute',
        inset: 0,
        background: '#f5f5f4',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'auto',
      }}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          padding: '12px 16px',
          borderBottom: '1px solid #e5e4e1',
        }}
      >
        <span style={{ fontWeight: 600 }}>Orthographic Views</span>
        <button type="button" data-testid="orthographic-close" onClick={onClose}>
          Close
        </button>
      </div>

      <div style={{ padding: 16, display: 'flex', flexDirection: 'column', gap: 16 }}>
        {viewBlocks.length === 0 ? (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
            <p style={{ margin: 0, fontSize: 13, color: '#57534e' }}>
              No linked views yet. Select geometry in Sketch2D, then assign it as the Front view to
              begin.
            </p>
            <button
              type="button"
              data-testid="orthographic-create-front"
              disabled={selectedPrimitiveIds.length === 0}
              onClick={handleCreateFrontView}
            >
              Assign selection as Front View
            </button>
          </div>
        ) : (
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: 16 }}>
            {viewBlocks.map((view) => (
              <ViewBlockCard
                key={view.id}
                view={view}
                primitives={session.snapshot.primitives}
                dimensions={session.snapshot.dimensions}
                onCommitAxis={(axis, value) => handleCommitAxis(view.id, axis, value)}
              />
            ))}
          </div>
        )}

        {conflict && (
          <div
            data-testid="orthographic-conflict"
            style={{
              padding: 12,
              background: '#fef2f2',
              border: '1px solid #fecaca',
              borderRadius: 8,
              display: 'flex',
              flexDirection: 'column',
              gap: 8,
              maxWidth: 420,
            }}
          >
            <div data-testid="orthographic-conflict-evidence" style={{ fontSize: 12, color: '#7c2d12' }}>
              {conflict.evidence}
            </div>
            <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
              {conflict.allowed_resolutions.map((choice) => (
                <button
                  key={choice}
                  type="button"
                  data-testid={`orthographic-conflict-resolve-${choice}`}
                  onClick={() => handleResolveConflict(WebResolutionChoice[choice])}
                >
                  {choice}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
