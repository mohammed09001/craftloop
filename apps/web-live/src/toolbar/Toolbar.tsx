import { TOOL_REGISTRY, type ToolId } from './toolRegistry'
import { ToolIcon } from './icons'
import styles from './Toolbar.module.css'

export interface ToolbarProps {
  activeTool: ToolId
  onSelectTool: (id: ToolId) => void
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
 */
export function Toolbar({ activeTool, onSelectTool, canUndo, canRedo, onUndo, onRedo }: ToolbarProps) {
  const groups: Array<{ key: string; render: () => React.ReactNode }> = [
    {
      key: 'primary',
      render: () =>
        TOOL_REGISTRY.filter((t) => t.group === 'primary').map((tool) => (
          <ToolButton
            key={tool.id}
            id={tool.id}
            label={tool.label}
            pressed={tool.kind === 'toggle' ? activeTool === tool.id : undefined}
            disabled={tool.deferredUntil !== undefined}
            deferredUntil={tool.deferredUntil}
            onClick={() => onSelectTool(tool.id)}
          />
        )),
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
    <div className={styles.toolbar} data-testid="main-toolbar" role="toolbar" aria-label="Craft Loop tools">
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
  onClick,
}: {
  id: ToolId
  label: string
  pressed?: boolean | undefined
  disabled?: boolean | undefined
  deferredUntil?: string | undefined
  onClick: () => void
}) {
  const title = deferredUntil ? `${label} (coming in ${deferredUntil})` : label
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
