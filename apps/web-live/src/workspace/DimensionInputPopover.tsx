import { useState } from 'react'
import type { ConflictSummary } from '../session/sceneTypes'
import { WebResolutionChoice } from '../session/craftLoopSession'
import type { ScreenPoint } from '../canvas/viewport'

/**
 * Execution 03, Phase 12, Task 094: numeric dimension entry positioned
 * right next to the real selection it targets (`anchorScreen`, a
 * `worldToScreen`-projected point `Workspace` computes from the
 * selection's own real geometry) -- never a distant global panel
 * (Task 094's own wording; this replaces the earlier `window.prompt`).
 *
 * Task 096: the same component both creates a new dimension and edits
 * an existing one -- `mode` only changes the submit label and the
 * pre-filled value, not the interaction.
 *
 * Task 097: when editing produces a real, persisted conflict (kept
 * open by `Workspace` because the new value did not take effect --
 * "keep last valid geometry"), `conflict` carries its real evidence
 * and legal resolutions; the numeric input is replaced by that
 * explanation and its real resolution buttons until the conflict is
 * resolved.
 */
export function DimensionInputPopover({
  mode,
  anchorScreen,
  initialValue,
  errorMessage,
  conflict,
  onSubmit,
  onCancel,
  onResolve,
}: {
  mode: 'create' | 'edit'
  anchorScreen: ScreenPoint
  initialValue?: number | undefined
  errorMessage?: string | null | undefined
  conflict?: ConflictSummary | null | undefined
  onSubmit: (value: number) => void
  onCancel: () => void
  onResolve: (choice: WebResolutionChoice) => void
}) {
  const [raw, setRaw] = useState(initialValue !== undefined ? String(initialValue) : '')

  const popoverWidth = 240
  const viewportWidth = typeof window !== 'undefined' ? window.innerWidth : 1280
  // Anchored by its left edge, not centered on the anchor point: a
  // selection near either edge of the canvas (a common case -- Task
  // 094's whole point is staying close to the real geometry) must not
  // push the popover half off-screen, which a `translateX(-50%)`
  // centering does for anything within half its own width of an edge.
  const left = Math.max(8, Math.min(anchorScreen.x, viewportWidth - popoverWidth - 8))

  return (
    <div
      data-testid="dimension-input"
      style={{
        position: 'absolute',
        left,
        top: anchorScreen.y,
        // Grows downward from the anchor, not upward: the anchor
        // itself already sits just above the target geometry (see
        // `AnnotationLayer`'s own offset), so anchoring the popover's
        // *bottom* edge there instead risked pushing a taller panel
        // (the Task 097 conflict view, with its evidence text and
        // resolution buttons) off the top of the viewport.
        transform: 'translateY(12px)',
        display: 'flex',
        flexDirection: 'column',
        gap: 6,
        padding: 8,
        width: popoverWidth,
        background: '#ffffff',
        border: '1px solid #e5e4e1',
        borderRadius: 10,
        boxShadow: '0 2px 10px rgba(8, 6, 13, 0.12)',
        pointerEvents: 'auto',
      }}
    >
      {conflict ? (
        <>
          <div
            data-testid="dimension-conflict-evidence"
            style={{ fontSize: 12, color: '#7c2d12', wordBreak: 'break-word' }}
          >
            {conflict.evidence}
          </div>
          <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
            {conflict.allowed_resolutions.map((choice) => (
              <button
                key={choice}
                type="button"
                data-testid={`dimension-conflict-resolve-${choice}`}
                onClick={() => onResolve(WebResolutionChoice[choice])}
              >
                {choice}
              </button>
            ))}
          </div>
        </>
      ) : (
        <>
          <form
            onSubmit={(event) => {
              event.preventDefault()
              const value = Number.parseFloat(raw)
              if (Number.isFinite(value)) onSubmit(value)
            }}
            style={{ display: 'flex', gap: 6 }}
          >
            <input
              data-testid="dimension-input-value"
              type="number"
              autoFocus
              value={raw}
              onChange={(event) => setRaw(event.target.value)}
              style={{ width: 90 }}
            />
            <button type="submit" data-testid="dimension-input-submit">
              {mode === 'create' ? 'Add' : 'Update'}
            </button>
            <button type="button" data-testid="dimension-input-cancel" onClick={onCancel}>
              Cancel
            </button>
          </form>
          {errorMessage && (
            <div data-testid="dimension-input-error" style={{ fontSize: 12, color: '#b91c1c' }}>
              {errorMessage}
            </div>
          )}
        </>
      )}
    </div>
  )
}
