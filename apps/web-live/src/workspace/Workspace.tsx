import { useCallback, useState } from 'react'
import { CanvasStack } from '../canvas/CanvasStack'
import { useCraftLoopSession } from '../session/useCraftLoopSession'
import { Toolbar } from '../toolbar/Toolbar'
import type { ToolId } from '../toolbar/toolRegistry'
import { useKeyboardShortcuts } from './useKeyboardShortcuts'
import styles from './Workspace.module.css'

/**
 * Full-canvas shell (Execution 03, Phase 02, Task 015) filled with the
 * Phase 06 Canvas/SVG rendering stack, the Phase 07 top-center toolbar
 * (Task 016's anchor), and Phase 08's real Creative/Sketch2D
 * `WorkspaceMode` switching -- one semantic action
 * (`session.enterSketchMode`/`enterCreativePenMode`) shared by every
 * entry source (Task 057/058/061): the toolbar's Sketch/Pen buttons
 * and the `S`/`B` keyboard shortcuts all call the exact same session
 * methods, never separate code paths. `activeTool` is owned here since
 * both the toolbar (which button is pressed) and the canvas (what a
 * click/drag actually does) need to agree on it.
 */
export function Workspace() {
  const session = useCraftLoopSession()
  const [activeTool, setActiveTool] = useState<ToolId>('pen')

  const enterSketchMode = useCallback(() => {
    session.enterSketchMode()
    setActiveTool('pen')
  }, [session])

  const enterCreativePenMode = useCallback(() => {
    session.enterCreativePenMode()
    setActiveTool('pen')
  }, [session])

  useKeyboardShortcuts({
    onEnterSketchMode: enterSketchMode,
    onEnterCreativePenMode: enterCreativePenMode,
  })

  return (
    <div className={styles.workspace} data-session-ready={session.ready}>
      <div className={styles.canvasArea} data-testid="canvas-area">
        <CanvasStack session={session} activeTool={activeTool} />
      </div>
      <div className={styles.toolbarHost} data-testid="toolbar-host">
        <Toolbar
          activeTool={activeTool}
          onSelectTool={setActiveTool}
          workspaceMode={session.snapshot.workspace_mode}
          onEnterSketchMode={enterSketchMode}
          onEnterCreativePenMode={enterCreativePenMode}
          canUndo={session.snapshot.can_undo}
          canRedo={session.snapshot.can_redo}
          onUndo={session.undo}
          onRedo={session.redo}
        />
      </div>
    </div>
  )
}
