/**
 * Execution 03, Phase 07/09: the main tool registry. Phase 07 built
 * the Creative/Notebook tool set (Pen, Sketch, Select, Eraser, View,
 * Undo, Redo, Save, More); Phase 09 adds the Sketch2D tool set (Line,
 * Arc, Circle, Rectangle, Dimension, Constraint, Construction, Snap)
 * Task 052/064 name. Data only, no rendering here -- `Toolbar.tsx`
 * filters this one array by `modes` to morph between the two
 * (Task 064: "Replace Main tools in the same top-center anchor").
 *
 * `kind: 'toggle'` tools are mutually exclusive within their mode
 * (only one active at once, like a radio group); `kind: 'action'`
 * tools fire once and don't stay "on."
 *
 * Which tools are real yet: `Pen`/`Select`/`Eraser` (Creative) and
 * `Line`/`Arc`/`Circle`/`Rectangle` (Sketch) change real
 * `CanvasStack` interaction behavior. `Undo`/`Redo` call the real
 * `CraftLoopSession` methods Phase 04 already exposed, in both modes
 * (Task 077). `Sketch` is real as of Phase 08. `Dimension`/
 * `Constraint` are real as of Phase 09: `Dimension` (Task 072) calls
 * the real `createDimension` once 1-2 primitives are selected;
 * `Constraint` (Task 073) opens a selection-adaptive list of only the
 * constraint kinds the current selection can actually take, then
 * calls the real `applyConstraint`.
 *
 * `Construction` and `Snap` are real as of Phase 11: `Construction`
 * (Task 090/091) toggles real, persisted `is_construction` state on
 * the current selection via `session.setConstruction`; `Snap`
 * (Task 092/093) toggles grid visibility plus geometry/grid snapping
 * for shape-tool drags (`src/canvas/inference.ts`). `View` is real as
 * of Phase 13: an independent toggle (same "outside the `activeTool`
 * radio group" pattern as Snap/Show All) opening the real Orthographic
 * View Block panel (`src/orthographic/OrthographicPanel.tsx`), which
 * assigns/reads real `ViewBlock`/`OrthographicSet`/`MultiviewGraph`
 * state through `CraftLoopSession`. `Save` and `More` stay
 * deliberately disabled -- their real behavior depends on
 * infrastructure later phases build (browser persistence: Phase 14; an
 * overflow menu with real contents: not needed yet since nothing is
 * currently being hidden from either toolbar). Registering them now
 * (rather than omitting them) keeps the registry complete and honest
 * about what exists versus what is coming.
 *
 * No `Ellipse` tool, despite `craftloop-geometry::Ellipse2` existing
 * as a real, tested kernel (Task 070's own premise). Repository
 * evidence changed the plan (Article 3): `BeautifiedPrimitive` (the
 * type every primitive actually flows through -- scene snapshot,
 * dimension association, consistency validation, PDF/SVG export,
 * constraint point references) has exactly four variants, and adding
 * a fifth is a real, invasive change across seven-plus crates
 * including native `craftloop-mobile-ffi`'s own exhaustive matches --
 * disproportionate to "add a toolbar button" and indistinguishable in
 * spirit from Task 071's own Spline precedent ("do not show it unless
 * a tested kernel is added" -- read as "unless the *pipeline*
 * supports it," not just the bare geometry struct). Deferred to a
 * dedicated future task with its own cross-crate test coverage.
 */

export type ToolId =
  | 'pen'
  | 'select'
  | 'eraser'
  | 'sketch'
  | 'view'
  | 'undo'
  | 'redo'
  | 'save'
  | 'more'
  | 'line'
  | 'arc'
  | 'circle'
  | 'rectangle'
  | 'dimension'
  | 'constraint'
  | 'construction'
  | 'snap'
  | 'annotations'

export type ToolGroup = 'primary' | 'history' | 'document' | 'overflow'

export type WorkspaceModeFilter = 'creative' | 'sketch'

export interface ToolDefinition {
  id: ToolId
  /** Accessible name / tooltip text (Task 053: never rendered as visible label text). */
  label: string
  group: ToolGroup
  kind: 'toggle' | 'action'
  /** Which toolbar(s) show this tool (Task 064's morph). */
  modes: WorkspaceModeFilter[]
  /** Set only for tools this phase does not yet make functional. */
  deferredUntil?: string
}

export const TOOL_REGISTRY: readonly ToolDefinition[] = [
  // -- Creative (Notebook) tools ----------------------------------------
  { id: 'pen', label: 'Pen', group: 'primary', kind: 'toggle', modes: ['creative', 'sketch'] },
  // Select is also available in Sketch2D (not just Notebook): Task
  // 072/073's Dimension/Constraint are selection-adaptive, so without
  // a way to select an existing primitive while a shape tool owns the
  // canvas drag gesture, neither could ever have anything to act on.
  { id: 'select', label: 'Select', group: 'primary', kind: 'toggle', modes: ['creative', 'sketch'] },
  { id: 'eraser', label: 'Eraser', group: 'primary', kind: 'toggle', modes: ['creative'] },
  { id: 'sketch', label: 'Sketch', group: 'primary', kind: 'toggle', modes: ['creative'] },
  // View (Task 102-112): an independent toggle -- opens/closes the
  // real Orthographic View Block panel, which replaces the canvas
  // area's content while open (Task 106's "2D engineering regions,"
  // not a 3D viewport) but never becomes part of the `activeTool`
  // radio group, since it is not a drawing tool.
  { id: 'view', label: 'View', group: 'primary', kind: 'toggle', modes: ['creative'] },
  // -- Sketch2D tools ----------------------------------------------------
  { id: 'line', label: 'Line', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  { id: 'arc', label: 'Arc', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  { id: 'circle', label: 'Circle', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  { id: 'rectangle', label: 'Rectangle', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  { id: 'dimension', label: 'Dimension', group: 'primary', kind: 'action', modes: ['sketch'] },
  { id: 'constraint', label: 'Constraint', group: 'primary', kind: 'action', modes: ['sketch'] },
  // Construction (Task 090/091): an action applied to the current
  // selection (like Dimension/Constraint), not a persistent drawing
  // mode -- it toggles real `is_construction` state on already-drawn
  // primitives via `session.setConstruction`.
  { id: 'construction', label: 'Construction', group: 'primary', kind: 'action', modes: ['sketch'] },
  // Snap/Guide (Task 092/093): a real but *independent* boolean toggle
  // -- Toolbar/CanvasStack special-case it outside the mutually-
  // exclusive `activeTool` radio group, since snapping stays on (or
  // off) regardless of which drawing tool is active. "Compact
  // precision options" (Task 075's own wording) is why grid visibility
  // and geometry/grid snap share this one control rather than three.
  { id: 'snap', label: 'Snap/Guide', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  // Show All Dimensions/Constraints (Task 101): another independent
  // toggle, same reasoning as Snap -- default off ("on demand," the
  // task's own wording), so the canvas only shows annotations that
  // touch the current selection until explicitly asked to reveal
  // every dimension/constraint in the document at once.
  { id: 'annotations', label: 'Show All', group: 'primary', kind: 'toggle', modes: ['sketch'] },
  // -- Shared groups -------------------------------------------------------
  { id: 'undo', label: 'Undo', group: 'history', kind: 'action', modes: ['creative', 'sketch'] },
  { id: 'redo', label: 'Redo', group: 'history', kind: 'action', modes: ['creative', 'sketch'] },
  {
    id: 'save',
    label: 'Save',
    group: 'document',
    kind: 'action',
    modes: ['creative'],
    deferredUntil: 'Phase 14 (Persistence and Reload)',
  },
  {
    id: 'more',
    label: 'More',
    group: 'overflow',
    kind: 'action',
    modes: ['creative', 'sketch'],
    deferredUntil: 'no overflow content exists yet',
  },
] as const
