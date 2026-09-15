import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { Workspace } from './Workspace'

describe('Workspace', () => {
  it('renders a full-canvas shell and a reserved top-center toolbar anchor', () => {
    render(<Workspace />)

    expect(screen.getByTestId('canvas-area')).toBeInTheDocument()
    expect(screen.getByTestId('toolbar-host')).toBeInTheDocument()
  })

  it('renders the real Phase 06 canvas stack inside the canvas area', () => {
    render(<Workspace />)

    expect(screen.getByTestId('canvas-stack')).toBeInTheDocument()
    expect(screen.getByTestId('background-grid')).toBeInTheDocument()
    expect(screen.getByTestId('geometry-layer')).toBeInTheDocument()
    expect(screen.getByTestId('ink-canvas')).toBeInTheDocument()
  })

  it('renders the real Phase 07 toolbar inside the reserved top-center anchor', () => {
    render(<Workspace />)

    const host = screen.getByTestId('toolbar-host')
    const toolbar = screen.getByTestId('main-toolbar')
    expect(host).toContainElement(toolbar)
    expect(screen.getByTestId('tool-pen')).toBeInTheDocument()
  })

  it('renders no fake engineering geometry before a real session is ready', () => {
    render(<Workspace />)

    // useCraftLoopSession starts from EMPTY_SNAPSHOT until the real
    // Wasm module resolves -- the geometry/selection layers must not
    // show anything invented in the meantime.
    expect(screen.getByTestId('geometry-layer').querySelectorAll('line, circle, polygon, path'))
      .toHaveLength(0)
    expect(screen.queryByTestId('selection-overlay')).not.toBeInTheDocument()
  })
})
