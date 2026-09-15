import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { Workspace } from './Workspace'

describe('Workspace', () => {
  it('renders a full-canvas shell and a reserved top-center toolbar anchor', () => {
    render(<Workspace />)

    expect(screen.getByTestId('canvas-area')).toBeInTheDocument()
    expect(screen.getByTestId('toolbar-host')).toBeInTheDocument()
  })

  it('renders no fake engineering objects (Task 015 invariant)', () => {
    render(<Workspace />)

    // The canvas area must be empty until Phase 06 wires a real
    // CraftLoopSession scene snapshot into it -- no placeholder
    // geometry, no demo content.
    expect(screen.getByTestId('canvas-area')).toBeEmptyDOMElement()
    expect(screen.getByTestId('toolbar-host')).toBeEmptyDOMElement()
  })
})
