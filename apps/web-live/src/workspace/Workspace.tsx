import { useState } from 'react'
import { CanvasStack } from '../canvas/CanvasStack'
import { useCraftLoopSession } from '../session/useCraftLoopSession'
import { Toolbar } from '../toolbar/Toolbar'
import type { ToolId } from '../toolbar/toolRegistry'
import styles from './Workspace.module.css'

/**
 * Full-canvas shell (Execution 03, Phase 02, Task 015) filled with the
 * Phase 06 Canvas/SVG rendering stack, and now the real Phase 07
 * top-center toolbar (Task 016's anchor). `activeTool` is owned here
 * since both the toolbar (which button is pressed) and the canvas
 * (what a click/drag actually does) need to agree on it.
 */
export function Workspace() {
  const session = useCraftLoopSession()
  const [activeTool, setActiveTool] = useState<ToolId>('pen')

  return (
    <div className={styles.workspace}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack session={session} activeTool={activeTool} />
      </div>
      <div className={styles.toolbarHost} data-testid="toolbar-host">
        <Toolbar
          activeTool={activeTool}
          onSelectTool={setActiveTool}
          canUndo={session.snapshot.can_undo}
          canRedo={session.snapshot.can_redo}
          onUndo={session.undo}
          onRedo={session.redo}
        />
      </div>
    </div>
  )
}
