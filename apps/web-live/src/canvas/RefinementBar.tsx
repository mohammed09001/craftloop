/**
 * Execution 03, Phase 10, Task 085: "no silent conversion." Shown only
 * while a Draw-and-Hold candidate is pending (Task 083/084) -- the
 * primitive this replaces raw ink with is already real, already
 * rendered by `GeometryLayer` (with a ghost/dashed style while
 * pending), and *only* becomes permanent when the user explicitly
 * confirms. Cancel reverts through a real `session.undo()`, not a
 * client-side pretend-undo.
 */
export function RefinementBar({
  onConfirm,
  onCancel,
}: {
  onConfirm: () => void
  onCancel: () => void
}) {
  return (
    <div
      data-testid="refinement-bar"
      style={{
        position: 'absolute',
        left: '50%',
        bottom: 24,
        transform: 'translateX(-50%)',
        display: 'flex',
        gap: 8,
        padding: 8,
        background: '#ffffff',
        border: '1px solid #e5e4e1',
        borderRadius: 10,
        boxShadow: '0 2px 10px rgba(8, 6, 13, 0.12)',
        pointerEvents: 'auto',
      }}
    >
      <button type="button" data-testid="refinement-confirm" onClick={onConfirm}>
        Keep shape
      </button>
      <button type="button" data-testid="refinement-cancel" onClick={onCancel}>
        Keep ink
      </button>
    </div>
  )
}
