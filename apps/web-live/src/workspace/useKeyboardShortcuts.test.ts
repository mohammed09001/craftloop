import { describe, expect, it } from 'vitest'
import { isTextInputFocused } from './useKeyboardShortcuts'

describe('isTextInputFocused', () => {
  it('is false for null/non-element targets', () => {
    expect(isTextInputFocused(null)).toBe(false)
  })

  it('is false for a plain element like the canvas or body', () => {
    expect(isTextInputFocused(document.createElement('div'))).toBe(false)
    expect(isTextInputFocused(document.createElement('canvas'))).toBe(false)
  })

  it('is true for input/textarea/select elements', () => {
    expect(isTextInputFocused(document.createElement('input'))).toBe(true)
    expect(isTextInputFocused(document.createElement('textarea'))).toBe(true)
    expect(isTextInputFocused(document.createElement('select'))).toBe(true)
  })

  it('is true for a contentEditable element', () => {
    const div = document.createElement('div')
    div.setAttribute('contenteditable', 'true')
    document.body.appendChild(div)
    try {
      expect(isTextInputFocused(div)).toBe(true)
    } finally {
      div.remove()
    }
  })
})
