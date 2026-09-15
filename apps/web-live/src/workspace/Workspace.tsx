import styles from './Workspace.module.css'

/**
 * Full-canvas shell (Execution 03, Phase 02, Task 015) plus the
 * reserved top-center toolbar anchor (Task 016). The canvas area is
 * intentionally empty here -- no fake geometry, no placeholder
 * shapes. Phase 06 fills it with the real Canvas/SVG rendering stack
 * driven by a live CraftLoopSession scene snapshot; Phase 07 fills
 * the toolbar host with the real top-center toolbar.
 */
export function Workspace() {
  return (
    <div className={styles.workspace}>
      <div className={styles.canvasArea} data-testid="canvas-area" />
      <div className={styles.toolbarHost} data-testid="toolbar-host" />
    </div>
  )
}
