import { useEffect } from 'react'

/**
 * Execution 03, Phase 08, Task 059/060: `S`/`B` as literal keyboard
 * shortcuts -- deterministic because a discrete keypress is not
 * ambiguous text (unlike typed/recognized-text grammar resolution,
 * Task 062's `resolveCommand`, where "s" genuinely matches both
 * "select" and "sketch" in the real Notebook vocabulary). Ignored
 * whenever focus is in a text input, so typing "s"/"b" into a future
 * dimension-value field or the command simulator never fires a mode
 * switch underneath the user.
 */
export function isTextInputFocused(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false
  // `target.isContentEditable` is the spec-correct check, but jsdom
  // (this project's Vitest environment) does not implement it -- it
  // always reports `false` regardless of the attribute, which would
  // make this branch untestable. Reading the attribute directly is
  // portable across both a real browser and jsdom and means the same
  // thing here (a truthy `contenteditable` other than the literal
  // string `"false"`).
  const contentEditableAttr = target.getAttribute('contenteditable')
  if (contentEditableAttr !== null && contentEditableAttr !== 'false') return true
  const tag = target.tagName
  return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT'
}

export function useKeyboardShortcuts({
  onEnterSketchMode,
  onEnterCreativePenMode,
}: {
  onEnterSketchMode: () => void
  onEnterCreativePenMode: () => void
}) {
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if (event.metaKey || event.ctrlKey || event.altKey) return
      if (isTextInputFocused(event.target)) return

      const key = event.key.toLowerCase()
      if (key === 's') {
        event.preventDefault()
        onEnterSketchMode()
      } else if (key === 'b') {
        event.preventDefault()
        onEnterCreativePenMode()
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [onEnterSketchMode, onEnterCreativePenMode])
}
