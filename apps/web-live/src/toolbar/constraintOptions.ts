import type { PrimitiveSummary } from '../session/sceneTypes'

/**
 * Execution 03, Phase 09, Task 073: "Selection-adaptive control" --
 * only the constraint kinds the current selection can actually take,
 * never a permanent wall of every kind that exists (Article 30). Each
 * `payload` matches `WebConstraintKind`'s exact JSON shape
 * (`crates/craftloop-web-bridge/src/types.rs`), ready for
 * `JSON.stringify` + `session.applyConstraint`.
 *
 * Deliberately conservative: only the combinations this crate's own
 * `WebConstraintKind::into_domain` actually supports (a line-typed
 * `Coincident` mirrors native's own mapping exactly, so it is not
 * offered for a Rectangle/Circle/Arc pair even though the underlying
 * `SketchConstraintKind` enum has more variants than are exposed here
 * -- see Phase 04's `types.rs` doc comment on why only Article 17's
 * stable eight are exposed at all).
 */
export interface ConstraintOption {
  label: string
  payload: Record<string, { line: string } | { a: string; b: string }>
}

export function eligibleConstraintOptions(selected: PrimitiveSummary[]): ConstraintOption[] {
  if (selected.length === 1 && selected[0]?.kind === 'Line') {
    const line = selected[0].id
    return [
      { label: 'Horizontal', payload: { Horizontal: { line } } },
      { label: 'Vertical', payload: { Vertical: { line } } },
    ]
  }

  if (selected.length === 2 && selected.every((p) => p.kind === 'Line')) {
    const a = selected[0]!.id
    const b = selected[1]!.id
    return [
      { label: 'Parallel', payload: { Parallel: { a, b } } },
      { label: 'Perpendicular', payload: { Perpendicular: { a, b } } },
      { label: 'Equal Length', payload: { EqualLength: { a, b } } },
      { label: 'Coincident', payload: { Coincident: { a, b } } },
    ]
  }

  if (selected.length === 2 && selected.every((p) => p.kind === 'Circle')) {
    const a = selected[0]!.id
    const b = selected[1]!.id
    return [
      { label: 'Equal Radius', payload: { EqualRadius: { a, b } } },
      { label: 'Concentric', payload: { Concentric: { a, b } } },
    ]
  }

  return []
}
