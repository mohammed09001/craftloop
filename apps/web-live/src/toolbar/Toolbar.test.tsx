import { cleanup, render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { Toolbar } from './Toolbar'
import { TOOL_REGISTRY } from './toolRegistry'

/** Unmounts any previous render first, so re-rendering with new props in the same test doesn't leave two toolbars mounted at once. */
function renderToolbar(overrides: Partial<React.ComponentProps<typeof Toolbar>> = {}) {
  cleanup()
  const props = {
    activeTool: 'pen' as const,
    onSelectTool: vi.fn(),
    workspaceMode: 'Creative' as const,
    onEnterSketchMode: vi.fn(),
    onEnterCreativePenMode: vi.fn(),
    canUndo: false,
    canRedo: false,
    onUndo: vi.fn(),
    onRedo: vi.fn(),
    dimensionEnabled: false,
    onDimension: vi.fn(),
    constraintOptions: [],
    onApplyConstraint: vi.fn(),
    constructionEnabled: false,
    onToggleConstruction: vi.fn(),
    snapEnabled: false,
    onToggleSnap: vi.fn(),
    showAllAnnotations: false,
    onToggleAnnotations: vi.fn(),
    viewOpen: false,
    onToggleView: vi.fn(),
    lastSavedAt: null,
    onSave: vi.fn(),
    onNewDocument: vi.fn(),
    onOpenLastSaved: vi.fn(),
    onOpenCommandSimulator: vi.fn(),
    ...overrides,
  }
  render(<Toolbar {...props} />)
  return props
}

const CREATIVE_TOOLS = TOOL_REGISTRY.filter((t) => t.modes.includes('creative'))
const SKETCH_TOOLS = TOOL_REGISTRY.filter((t) => t.modes.includes('sketch'))

describe('Toolbar', () => {
  it('renders every Creative-mode tool from the registry (Task 052)', () => {
    renderToolbar()
    for (const tool of CREATIVE_TOOLS) {
      expect(screen.getByTestId(`tool-${tool.id}`)).toBeInTheDocument()
    }
  })

  it('renders every Sketch2D-mode tool from the registry, replacing the Creative-only set (Task 064)', () => {
    renderToolbar({ workspaceMode: 'Sketch2D' })
    for (const tool of SKETCH_TOOLS) {
      expect(screen.getByTestId(`tool-${tool.id}`)).toBeInTheDocument()
    }
    // Notebook-only tools are gone, not just disabled -- a real morph.
    // Select stays (Dimension/Constraint need a way to select a
    // primitive), Eraser does not.
    expect(screen.getByTestId('tool-select')).toBeInTheDocument()
    expect(screen.queryByTestId('tool-eraser')).not.toBeInTheDocument()
  })

  it('renders icon-first: no visible text label on any button (Task 053)', () => {
    renderToolbar()
    for (const tool of CREATIVE_TOOLS) {
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

  it('every registered tool is real as of Phase 14 -- none deferred', () => {
    // The registry still supports `deferredUntil` for a genuinely
    // not-yet-functional future tool, but nothing currently in it
    // uses that escape hatch -- a regression guard, not a no-op.
    for (const tool of TOOL_REGISTRY) {
      expect(tool.deferredUntil, `${tool.id} is deferred`).toBeUndefined()
    }
  })

  it('calls onEnterSketchMode when Sketch is clicked (Task 061)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar()
    expect(screen.getByTestId('tool-sketch')).toBeEnabled()
    await user.click(screen.getByTestId('tool-sketch'))
    expect(props.onEnterSketchMode).toHaveBeenCalledTimes(1)

    // The real morph (Task 064) replaces the Sketch button with the
    // Sketch2D tool set entirely -- there is no "Sketch" button to
    // show pressed once already inside Sketch2D.
    renderToolbar({ workspaceMode: 'Sketch2D' })
    expect(screen.queryAllByTestId('tool-sketch')).toHaveLength(0)
  })

  it('clicking Pen while in Sketch2D calls onEnterCreativePenMode', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ workspaceMode: 'Sketch2D' })
    await user.click(screen.getByTestId('tool-pen'))
    expect(props.onEnterCreativePenMode).toHaveBeenCalledTimes(1)
  })

  it('Pen stays enabled in Sketch2D and Line/Arc/Circle/Rectangle are real toggle tools (Task 065-069)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ workspaceMode: 'Sketch2D' })
    expect(screen.getByTestId('tool-pen')).toBeEnabled()
    for (const id of ['line', 'arc', 'circle', 'rectangle']) {
      expect(screen.getByTestId(`tool-${id}`)).toBeEnabled()
    }
    await user.click(screen.getByTestId('tool-circle'))
    expect(props.onSelectTool).toHaveBeenCalledWith('circle')
  })

  it('Dimension is disabled until dimensionEnabled is true, and calls onDimension when clicked (Task 072)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ workspaceMode: 'Sketch2D', dimensionEnabled: true })
    expect(screen.getByTestId('tool-dimension')).toBeEnabled()
    await user.click(screen.getByTestId('tool-dimension'))
    expect(props.onDimension).toHaveBeenCalledTimes(1)

    renderToolbar({ workspaceMode: 'Sketch2D', dimensionEnabled: false })
    expect(screen.getAllByTestId('tool-dimension').at(-1)).toBeDisabled()
  })

  it('Constraint is disabled with no options, and opens a menu of only the eligible kinds (Task 073)', async () => {
    const user = userEvent.setup()
    renderToolbar({ workspaceMode: 'Sketch2D', constraintOptions: [] })
    expect(screen.getByTestId('tool-constraint')).toBeDisabled()

    const props = renderToolbar({
      workspaceMode: 'Sketch2D',
      constraintOptions: [
        { label: 'Horizontal', payload: { Horizontal: { line: 'a' } } },
        { label: 'Vertical', payload: { Vertical: { line: 'a' } } },
      ],
    })
    const constraintButton = screen.getAllByTestId('tool-constraint').at(-1)!
    expect(constraintButton).toBeEnabled()
    await user.click(constraintButton)
    expect(screen.getByTestId('constraint-menu')).toBeInTheDocument()
    expect(screen.getByTestId('constraint-option-horizontal')).toBeInTheDocument()

    await user.click(screen.getByTestId('constraint-option-horizontal'))
    expect(props.onApplyConstraint).toHaveBeenCalledWith({ Horizontal: { line: 'a' } })
    expect(screen.queryByTestId('constraint-menu')).not.toBeInTheDocument()
  })

  it('Construction is disabled with nothing selected, and calls onToggleConstruction when clicked (Task 090/091)', async () => {
    const user = userEvent.setup()
    renderToolbar({ workspaceMode: 'Sketch2D', constructionEnabled: false })
    expect(screen.getByTestId('tool-construction')).toBeDisabled()

    const props = renderToolbar({ workspaceMode: 'Sketch2D', constructionEnabled: true })
    const button = screen.getAllByTestId('tool-construction').at(-1)!
    expect(button).toBeEnabled()
    await user.click(button)
    expect(props.onToggleConstruction).toHaveBeenCalledTimes(1)
  })

  it('Snap is a real independent toggle -- pressed state follows snapEnabled, not activeTool (Task 092/093)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ workspaceMode: 'Sketch2D', activeTool: 'line', snapEnabled: true })
    expect(screen.getByTestId('tool-snap')).toHaveAttribute('aria-pressed', 'true')
    // Snap is enabled regardless of which drawing tool owns the canvas.
    expect(screen.getByTestId('tool-snap')).toBeEnabled()

    await user.click(screen.getByTestId('tool-snap'))
    expect(props.onToggleSnap).toHaveBeenCalledTimes(1)
    // Toggling Snap must never also select it as the active drawing tool.
    expect(props.onSelectTool).not.toHaveBeenCalledWith('snap')
  })

  it('Show All is a real independent toggle for dimension/constraint annotations (Task 101)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ workspaceMode: 'Sketch2D', showAllAnnotations: true })
    expect(screen.getByTestId('tool-annotations')).toHaveAttribute('aria-pressed', 'true')

    await user.click(screen.getByTestId('tool-annotations'))
    expect(props.onToggleAnnotations).toHaveBeenCalledTimes(1)
    expect(props.onSelectTool).not.toHaveBeenCalledWith('annotations')
  })

  it('View is a real, enabled independent toggle for the Orthographic panel (Task 102-112)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar({ viewOpen: true })
    expect(screen.getByTestId('tool-view')).toBeEnabled()
    expect(screen.getByTestId('tool-view')).toHaveAttribute('aria-pressed', 'true')

    await user.click(screen.getByTestId('tool-view'))
    expect(props.onToggleView).toHaveBeenCalledTimes(1)
    expect(props.onSelectTool).not.toHaveBeenCalledWith('view')
  })

  it('Save is enabled and calls onSave (Task 116)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar()
    expect(screen.getByTestId('tool-save')).toBeEnabled()
    await user.click(screen.getByTestId('tool-save'))
    expect(props.onSave).toHaveBeenCalledTimes(1)
  })

  it('More opens a real menu with New Document and Open Last Saved (Task 115)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar()
    expect(screen.getByTestId('tool-more')).toBeEnabled()
    await user.click(screen.getByTestId('tool-more'))
    expect(screen.getByTestId('more-menu')).toBeVisible()

    await user.click(screen.getByTestId('more-new-document'))
    expect(props.onNewDocument).toHaveBeenCalledTimes(1)
    expect(screen.queryByTestId('more-menu')).not.toBeInTheDocument()

    await user.click(screen.getByTestId('tool-more'))
    await user.click(screen.getByTestId('more-open-last-saved'))
    expect(props.onOpenLastSaved).toHaveBeenCalledTimes(1)
  })

  it('More also offers the developer-only Command Simulator (Task 119)', async () => {
    const user = userEvent.setup()
    const props = renderToolbar()
    await user.click(screen.getByTestId('tool-more'))
    await user.click(screen.getByTestId('more-command-simulator'))
    expect(props.onOpenCommandSimulator).toHaveBeenCalledTimes(1)
  })
})
