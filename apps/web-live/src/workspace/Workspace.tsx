import { CanvasStack } from '../canvas/CanvasStack'
import { useCraftLoopSession } from '../session/useCraftLoopSession'
import styles from './Workspace.module.css'

/**
 * Full-canvas shell (Execution 03, Phase 02, Task 015), now filled with
 * the real Phase 06 Canvas/SVG rendering stack driven by a live
 * `CraftLoopSession` scene snapshot -- no fake geometry, everything
 * rendered comes from `useCraftLoopSession`'s real read model. The
 * top-center toolbar anchor (Task 016) stays empty until Phase 07.
 */
export function Workspace() {
  const session = useCraftLoopSession()

  return (
    <div className={styles.workspace}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack session={session} />
      </div>
      <div className={styles.toolbarHost} data-testid="toolbar-host" />
    </div>
  )
}
