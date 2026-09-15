/** Mirrors `craftloop_web_bridge::types::WebCommandAction`/`WebGrammarMatch` (Execution 03, Phase 08, Task 062). */

export type CommandActionName =
  | 'Pen'
  | 'Eraser'
  | 'Select'
  | 'Sketch'
  | 'Orthographic'
  | 'Line'
  | 'Circle'
  | 'Arc'
  | 'Rectangle'
  | 'Dimension'
  | 'ExitSketch'
  | 'AddView'
  | 'LabelView'
  | 'Link'
  | 'Resolve'

export type GrammarMatch =
  | { Exact: { action: CommandActionName } }
  | { UniquePrefix: { action: CommandActionName } }
  | { Ambiguous: { candidates: string[] } }
  | 'NoMatch'
