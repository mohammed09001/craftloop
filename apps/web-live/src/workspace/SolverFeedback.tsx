/**
 * Execution 03, Phase 12, Task 100: "no visual-only success state."
 * `Workspace` calls the real `applyConstraint`/`solveConstraints`
 * pair and shows exactly the real `WebSolveOutcome`/
 * `WebConstraintOutcome` this returns -- a redundant constraint says
 * so explicitly (the geometry may not visibly move at all), and an
 * unsatisfied/failed solve is reported as such rather than left to be
 * inferred from "the shape didn't move."
 */
export type SolverFeedbackTone = 'success' | 'warning' | 'error'

export function SolverFeedback({ tone, message }: { tone: SolverFeedbackTone; message: string }) {
  const background = tone === 'success' ? '#ecfdf5' : tone === 'warning' ? '#fffbeb' : '#fef2f2'
  const border = tone === 'success' ? '#a7f3d0' : tone === 'warning' ? '#fde68a' : '#fecaca'
  const color = tone === 'success' ? '#065f46' : tone === 'warning' ? '#92400e' : '#991b1b'
  return (
    <div
      data-testid="solver-feedback"
      data-tone={tone}
      style={{
        position: 'absolute',
        top: 72,
        left: '50%',
        transform: 'translateX(-50%)',
        padding: '6px 12px',
        borderRadius: 8,
        background,
        border: `1px solid ${border}`,
        color,
        fontSize: 13,
        pointerEvents: 'none',
      }}
    >
      {message}
    </div>
  )
}
