/**
 * Execution 03, Phase 07, Task 052: the main tool registry -- Pen,
 * Sketch, Select, Eraser, View, Undo, Redo, Save, More, exactly the
 * task's own named set. Data only, no rendering here.
 *
 * `kind: 'toggle'` tools are mutually exclusive (only one active at
 * once, like a radio group); `kind: 'action'` tools fire once and
 * don't stay "on."
 *
 * Which tools are real yet: `Pen`/`Select`/`Eraser` change real
 * `CanvasStack` interaction behavior this phase (a small, natural
 * extension of Phase 06's drawing/selection code). `Undo`/`Redo` call
 * the real `CraftLoopSession` methods Phase 04 already exposed.
 * `Sketch`, `View`, `Save`, and `More` are deliberately disabled here
 * -- their real behavior depends on infrastructure later phases build
 * (Sketch Mode's toolbar morph: Phase 08-09; Orthographic View Block
 * rendering: Phase 13; browser persistence: Phase 14; an overflow
 * menu with real contents: Phase 09's "Keep history actions
 * reachable"/Article 76). Registering them now (rather than omitting
 * them) keeps Task 052's registry complete and honest about what
 * exists versus what is coming, instead of a toolbar that silently
 * grows extra buttons every later phase.
 */

export type ToolId = 'pen' | 'select' | 'eraser' | 'sketch' | 'view' | 'undo' | 'redo' | 'save' | 'more'

export type ToolGroup = 'primary' | 'history' | 'document' | 'overflow'

export interface ToolDefinition {
  id: ToolId
  /** Accessible name / tooltip text (Task 053: never rendered as visible label text). */
  label: string
  group: ToolGroup
  kind: 'toggle' | 'action'
  /** Set only for tools this phase does not yet make functional. */
  deferredUntil?: string
}

export const TOOL_REGISTRY: readonly ToolDefinition[] = [
  { id: 'pen', label: 'Pen', group: 'primary', kind: 'toggle' },
  { id: 'select', label: 'Select', group: 'primary', kind: 'toggle' },
  { id: 'eraser', label: 'Eraser', group: 'primary', kind: 'toggle' },
  {
    id: 'sketch',
    label: 'Sketch',
    group: 'primary',
    kind: 'toggle',
    deferredUntil: 'Phase 08-09 (Sketch Mode)',
  },
  {
    id: 'view',
    label: 'View',
    group: 'primary',
    kind: 'action',
    deferredUntil: 'Phase 13 (Orthographic Linked-View Completion)',
  },
  { id: 'undo', label: 'Undo', group: 'history', kind: 'action' },
  { id: 'redo', label: 'Redo', group: 'history', kind: 'action' },
  {
    id: 'save',
    label: 'Save',
    group: 'document',
    kind: 'action',
    deferredUntil: 'Phase 14 (Persistence and Reload)',
  },
  {
    id: 'more',
    label: 'More',
    group: 'overflow',
    kind: 'action',
    deferredUntil: 'Phase 09 (overflow menu contents)',
  },
] as const
