import { TOOL_REGISTRY, type ToolId } from './toolRegistry'
import { ToolIcon } from './icons'
import type { WorkspaceModeName } from '../session/sceneTypes'
import styles from './Toolbar.module.css'

export interface ToolbarProps {
  activeTool: ToolId
  onSelectTool: (id: ToolId) => void
  workspaceMode: WorkspaceModeName
  onEnterSketchMode: () => void
  onEnterCreativePenMode: () => void
  canUndo: boolean
  canRedo: boolean
  onUndo: () => void
  onRedo: () => void
}

/**
 * Execution 03, Phase 07, Task 050: the floating top-center toolbar
 * surface, rendered inside `Workspace`'s reserved anchor (Task 016).
 * Task 051: this is the *only* toolbar `apps/web-live` renders --
 * there is no bottom or side counterpart anywhere in this app, unlike
 * the native Android layout Phase 00's audit found
 * (`MainActivity`'s `Column { InkCanvas; PrimaryToolbar }`).
 *
 * Phase 08, Task 061: Sketch/Pen dispatch the exact same
 * `session.enterSketchMode`/`enterCreativePenMode` calls the `S`/`B`
 * keyboard shortcuts use (`useKeyboardShortcuts.ts`) -- one semantic
 * action per transition, not a button-only code path.
 */
export function Toolbar({
  activeTool,
  onSelectTool,
  workspaceMode,
  onEnterSketchMode,
  onEnterCreativePenMode,
  canUndo,
  canRedo,
  onUndo,
  onRedo,
}: ToolbarProps) {
  const inSketch = workspaceMode === 'Sketch2D'

  const handlePrimaryClick = (id: ToolId) => {
    if (id === 'sketch') {
      onEnterSketchMode()
      return
    }
    if (id === 'pen') {
      if (inSketch) onEnterCreativePenMode()
      onSelectTool('pen')
      return
    }
    onSelectTool(id)
  }

  const groups: Array<{ key: string; render: () => React.ReactNode }> = [
    {
      key: 'primary',
      render: () =>
        TOOL_REGISTRY.filter((t) => t.group === 'primary').map((tool) => {
          const pressed =
            tool.id === 'sketch'
              ? inSketch
              : tool.kind === 'toggle'
                ? activeTool === tool.id
                : undefined
          // Article 26: Pen stays available in Sketch Mode. Select/
          // Eraser are Notebook-only concepts (CommandAction validity)
          // and pause while Sketch2D is active, until Phase 09 gives
          // Sketch its own dedicated tool set.
          const pausedInSketch = inSketch && (tool.id === 'select' || tool.id === 'eraser')
          const disabled = tool.deferredUntil !== undefined || pausedInSketch
          return (
            <ToolButton
              key={tool.id}
              id={tool.id}
              label={tool.label}
              pressed={pressed}
              disabled={disabled}
              deferredUntil={tool.deferredUntil}
              titleOverride={pausedInSketch ? `${tool.label} (back in Creative Mode -- press B)` : undefined}
              onClick={() => handlePrimaryClick(tool.id)}
            />
          )
        }),
    },
    {
      key: 'history',
      render: () => (
        <>
          <ToolButton id="undo" label="Undo" disabled={!canUndo} onClick={onUndo} />
          <ToolButton id="redo" label="Redo" disabled={!canRedo} onClick={onRedo} />
        </>
      ),
    },
    {
      key: 'document',
      render: () =>
        TOOL_REGISTRY.filter((t) => t.group === 'document').map((tool) => (
          <ToolButton
            key={tool.id}
            id={tool.id}
            label={tool.label}
            disabled
            deferredUntil={tool.deferredUntil}
            onClick={() => {}}
          />
        )),
    },
    {
      key: 'overflow',
      render: () =>
        TOOL_REGISTRY.filter((t) => t.group === 'overflow').map((tool) => (
          <ToolButton
            key={tool.id}
            id={tool.id}
            label={tool.label}
            disabled
            deferredUntil={tool.deferredUntil}
            onClick={() => {}}
          />
        )),
    },
  ]

  return (
    <div
      className={styles.toolbar}
      data-testid="main-toolbar"
      data-workspace-mode={workspaceMode}
      role="toolbar"
      aria-label="Craft Loop tools"
    >
      {groups.map((group, i) => (
        <div key={group.key} className={styles.group} data-testid={`toolbar-group-${group.key}`}>
          {i > 0 && <div className={styles.divider} aria-hidden="true" />}
          {group.render()}
        </div>
      ))}
    </div>
  )
}

function ToolButton({
  id,
  label,
  pressed,
  disabled,
  deferredUntil,
  titleOverride,
  onClick,
}: {
  id: ToolId
  label: string
  pressed?: boolean | undefined
  disabled?: boolean | undefined
  deferredUntil?: string | undefined
  titleOverride?: string | undefined
  onClick: () => void
}) {
  const title = titleOverride ?? (deferredUntil ? `${label} (coming in ${deferredUntil})` : label)
  return (
    <button
      type="button"
      className={styles.button}
      data-testid={`tool-${id}`}
      aria-label={label}
      title={title}
      aria-pressed={pressed}
      aria-disabled={disabled || undefined}
      disabled={disabled}
      onClick={onClick}
    >
      <ToolIcon id={id} />
    </button>
  )
}
