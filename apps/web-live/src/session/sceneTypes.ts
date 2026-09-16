/**
 * TypeScript mirrors of craftloop-web-bridge's JSON DTOs
 * (crates/craftloop-web-bridge/src/types.rs). Kept as plain data types,
 * not classes -- every value here is JSON parsed from a real
 * `CraftLoopSession` call, never constructed by hand (Article 65).
 */

export interface Point2 {
  x: number
  y: number
}

export interface Segment2 {
  a: Point2
  b: Point2
}

export interface Circle2 {
  center: Point2
  radius: number
}

export interface Arc2 {
  center: Point2
  radius: number
  start_angle: number
  sweep_angle: number
}

export interface RelationalRectangle {
  corners: [Point2, Point2, Point2, Point2]
}

export type BeautifiedPrimitive =
  | { Line: Segment2 }
  | { Circle: Circle2 }
  | { Arc: Arc2 }
  | { Rectangle: RelationalRectangle }

export type PrimitiveKind = 'Line' | 'Circle' | 'Arc' | 'Rectangle'

export interface StrokeSummary {
  id: string
  sample_count: number
  points: Point2[]
}

export interface PrimitiveSummary {
  id: string
  kind: PrimitiveKind
  min_x: number
  min_y: number
  max_x: number
  max_y: number
  geometry: BeautifiedPrimitive
  /** Execution 03, Phase 11, Task 090: real, persisted backend state. */
  is_construction: boolean
}

export type DimensionKind = 'Linear' | 'Angular' | 'Radius' | 'Diameter'
export type DimensionRole = 'Driving' | 'Reference' | 'Derived' | 'Shared' | 'Bounded'

export interface DimensionSummary {
  id: string
  kind: DimensionKind
  role: DimensionRole
  value: number
  target_primitive_ids: string[]
}

export interface ConstraintSummary {
  id: string
  label: string
  primitive_ids: string[]
}

/**
 * Execution 03, Phase 12, Task 098: mirrors
 * `craftloop_web_bridge::types::WebConstraintOption` -- one real,
 * backend-derived entry in `session.eligibleConstraints(...)`'s
 * result. `payload` matches `WebConstraintKind`'s exact JSON shape,
 * ready for `JSON.stringify` + `session.applyConstraint`.
 */
export interface ConstraintOption {
  label: string
  payload: Record<string, { line: string } | { a: string; b: string }>
}

export type ConflictKind =
  | 'DegenerateGeometry'
  | 'DimensionConstraintMismatch'
  | 'UnitMisapplication'
  | 'CrossViewMismatch'
export type Severity = 'Info' | 'Warning' | 'Error' | 'Blocker'
export type ResolutionChoiceName =
  | 'KeepExisting'
  | 'ReplaceAndPropagate'
  | 'Unlink'
  | 'RemoveConstraint'
  | 'Cancel'

export interface ConflictSummary {
  id: string
  kind: ConflictKind
  severity: Severity
  unresolved: boolean
  affected_entities: string[]
  existing_truth: string
  proposed_truth: string
  evidence: string
  allowed_resolutions: ResolutionChoiceName[]
}

export type PrincipalViewIdentityName = 'Front' | 'Top' | 'Right' | 'Back'
export type SharedAxisName = 'Width' | 'Height' | 'Depth'
export type OrthographicReadinessName =
  | 'DraftReady'
  | 'IdentityReady'
  | 'LinkReady'
  | 'Resolved'
  | 'Constrained'

export interface AxisBinding {
  axis: SharedAxisName
  dimension_id: string
}

export interface ViewBlockSummary {
  id: string
  identity: PrincipalViewIdentityName | null
  geometry_member_ids: string[]
  readiness: OrthographicReadinessName
  blockers: string[]
  axis_bindings: AxisBinding[]
  unresolved_axes: SharedAxisName[]
}

export interface OrthographicSetSummary {
  id: string
  view_ids: string[]
}

/** Mirrors `craftloop_web_bridge::types::WebWorkspaceMode` (Execution 03, Phase 08, Task 056). */
export type WorkspaceModeName = 'Creative' | 'Sketch2D'

export interface SceneSnapshot {
  strokes: StrokeSummary[]
  primitives: PrimitiveSummary[]
  dimensions: DimensionSummary[]
  constraints: ConstraintSummary[]
  conflicts: ConflictSummary[]
  view_blocks: ViewBlockSummary[]
  orthographic_sets: OrthographicSetSummary[]
  selected_entity_ids: string[]
  revision: number
  can_undo: boolean
  can_redo: boolean
  workspace_mode: WorkspaceModeName
}

export const EMPTY_SNAPSHOT: SceneSnapshot = {
  strokes: [],
  primitives: [],
  dimensions: [],
  constraints: [],
  conflicts: [],
  view_blocks: [],
  orthographic_sets: [],
  selected_entity_ids: [],
  revision: 0,
  can_undo: false,
  can_redo: false,
  workspace_mode: 'Creative',
}
