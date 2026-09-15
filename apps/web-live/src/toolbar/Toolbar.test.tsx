import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { Toolbar } from './Toolbar'
import { TOOL_REGISTRY } from './toolRegistry'

function renderToolbar(overrides: Partial<React.ComponentProps<typeof Toolbar>> = {}) {
  const props = {
    activeTool: 'pen' as const,
    onSelectTool: vi.fn(),
    canUndo: false,
    canRedo: false,
    onUndo: vi.fn(),
    onRedo: vi.fn(),
    ...overrides,
  }
  render(<Toolbar {...props} />)
  return props
}

describe('Toolbar', () => {
  it('renders every tool in the registry (Task 052)', () => {
    renderToolbar()
    for (const tool of TOOL_REGISTRY) {
      expect(screen.getByTestId(`tool-${tool.id}`)).toBeInTheDocument()
    }
  })

  it('renders icon-first: no visible text label on any button (Task 053)', () => {
    renderToolbar()
    for (const tool of TOOL_REGISTRY) {
      const button = screen.getByTestId(`tool-${tool.id}`)
      expect(button.textContent).toBe('')
      expect(button).toHaveAccessibleName(tool.label)
    }
  })

  it('marks the active tool as pressed via aria-pressed, not color alone (Task 054)', () => {
    renderToolbar({ activeTool: 'select' })
    expect(screen.getByTestId('tool-select')).toHaveAttribute('aria-pressed', 'true')
    expect(screen.getByTestId('tool-pen')).toHaveAttribute('aria-pressed', 'false')
  })

  it('calls onSelectTool with the clicked tool id', async () => {
    const user = userEvent.setup()
    const props = renderToolbar()
    await user.click(screen.getByTestId('tool-eraser'))
    expect(props.onSelectTool).toHaveBeenCalledWith('eraser')
  })

  it('disables Undo/Redo based on real can_undo/can_redo, and enables them when true', async () => {
    const props = renderToolbar({ canUndo: true, canRedo: false })
    expect(screen.getByTestId('tool-undo')).toBeEnabled()
    expect(screen.getByTestId('tool-redo')).toBeDisabled()

    const user = userEvent.setup()
    await user.click(screen.getByTestId('tool-undo'))
    expect(props.onUndo).toHaveBeenCalledTimes(1)
  })

  it('keeps deferred tools disabled with a title explaining when they activate', () => {
    renderToolbar()
    for (const tool of TOOL_REGISTRY.filter((t) => t.deferredUntil)) {
      const button = screen.getByTestId(`tool-${tool.id}`)
      expect(button).toBeDisabled()
      expect(button).toHaveAttribute('title', expect.stringContaining(tool.deferredUntil!))
    }
  })
})
