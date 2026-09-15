import { useState } from 'react'
import { TOOL_REGISTRY, type ToolId, type WorkspaceModeFilter } from './toolRegistry'
import { ToolIcon } from './icons'
import type { ConstraintOption } from './constraintOptions'
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
  dimensionEnabled: boolean
  onDimension: () => void
  constraintOptions: ConstraintOption[]
  onApplyConstraint: (payload: ConstraintOption['payload']) => void
}

/**
 * Execution 03, Phase 07/09: the floating top-center toolbar surface,
 * rendered inside `Workspace`'s reserved anchor (Task 016). Task 051:
 * this is the *only* toolbar `apps/web-live` renders.
 *
 * Task 064 (Phase 09): a real morph, not a disable -- the primary
 * group's tool list is filtered by `workspaceMode` from the one shared
 * `TOOL_REGISTRY`, so Sketch2D genuinely shows a different button set
 * (Line/Arc/Circle/Rectangle/Dimension/Constraint/...) rather than the
 * Creative set with some buttons grayed out (Phase 08's stopgap,
 * superseded here). History (Task 077) and Pen (Task 065, Article 26)
 * stay present in both.
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
  dimensionEnabled,
  onDimension,
  constraintOptions,
  onApplyConstraint,
}: ToolbarProps) {
  const [constraintMenuOpen, setConstraintMenuOpen] = useState(false)
  const modeFilter: WorkspaceModeFilter = workspaceMode === 'Sketch2D' ? 'sketch' : 'creative'
  const forMode = (tools: readonly (typeof TOOL_REGISTRY)[number][]) =>
    tools.filter((t) => t.modes.includes(modeFilter))

  const handlePrimaryClick = (id: ToolId) => {
    if (id === 'sketch') {
      onEnterSketchMode()
      return
    }
    if (id === 'pen') {
      if (modeFilter === 'sketch') onEnterCreativePenMode()
      onSelectTool('pen')
      return
    }
    if (id === 'dimension') {
      onDimension()
      return
    }
    if (id === 'constraint') {
      setConstraintMenuOpen((open) => !open)
      return
    }
    onSelectTool(id)
  }

  const groups: Array<{ key: string; tools: (typeof TOOL_REGISTRY)[number][] }> = [
    { key: 'primary', tools: forMode(TOOL_REGISTRY.filter((t) => t.group === 'primary')) },
    { key: 'history', tools: forMode(TOOL_REGISTRY.filter((t) => t.group === 'history')) },
    { key: 'document', tools: forMode(TOOL_REGISTRY.filter((t) => t.group === 'document')) },
    { key: 'overflow', tools: forMode(TOOL_REGISTRY.filter((t) => t.group === 'overflow')) },
  ].filter((group) => group.tools.length > 0)

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
          {group.tools.map((tool) => {
            if (tool.id === 'undo') {
              return <ToolButton key="undo" id="undo" label="Undo" disabled={!canUndo} onClick={onUndo} />
            }
            if (tool.id === 'redo') {
              return <ToolButton key="redo" id="redo" label="Redo" disabled={!canRedo} onClick={onRedo} />
            }
            if (tool.id === 'dimension') {
              return (
                <ToolButton
                  key="dimension"
                  id="dimension"
                  label="Dimension"
                  disabled={!dimensionEnabled}
                  titleOverride={dimensionEnabled ? undefined : 'Dimension (select 1-2 shapes first)'}
                  onClick={() => handlePrimaryClick('dimension')}
                />
              )
            }
            if (tool.id === 'constraint') {
              return (
                <div key="constraint" className={styles.constraintWrapper}>
                  <ToolButton
                    id="constraint"
                    label="Constraint"
                    disabled={constraintOptions.length === 0}
                    pressed={constraintMenuOpen}
                    titleOverride={
                      constraintOptions.length === 0
                        ? 'Constraint (select a shape or pair first)'
                        : undefined
                    }
                    onClick={() => handlePrimaryClick('constraint')}
                  />
                  {constraintMenuOpen && constraintOptions.length > 0 && (
                    <div className={styles.constraintMenu} data-testid="constraint-menu" role="menu">
                      {constraintOptions.map((option) => (
                        <button
                          key={option.label}
                          type="button"
                          role="menuitem"
                          className={styles.constraintMenuItem}
                          data-testid={`constraint-option-${option.label.replace(/\s+/g, '-').toLowerCase()}`}
                          onClick={() => {
                            onApplyConstraint(option.payload)
                            setConstraintMenuOpen(false)
                          }}
                        >
                          {option.label}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              )
            }
            const pressed =
              tool.id === 'sketch'
                ? modeFilter === 'sketch'
                : tool.kind === 'toggle'
                  ? activeTool === tool.id
                  : undefined
            return (
              <ToolButton
                key={tool.id}
                id={tool.id}
                label={tool.label}
                pressed={pressed}
                disabled={tool.deferredUntil !== undefined}
                deferredUntil={tool.deferredUntil}
                onClick={() => handlePrimaryClick(tool.id)}
              />
            )
          })}
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
