import { useState } from 'react'
import { GeometryLayer } from '../canvas/GeometryLayer'
import { fitToBounds } from '../canvas/viewport'
import type { PrimitiveSummary, DimensionSummary, SharedAxisName, ViewBlockSummary } from '../session/sceneTypes'
import { axesForIdentity, axisValue, isAxisUnresolved, readinessTone } from './orthographicSelectors'

const PREVIEW_WIDTH = 200
const PREVIEW_HEIGHT = 140

const TONE_COLOR: Record<ReturnType<typeof readinessTone>, string> = {
  success: '#065f46',
  neutral: '#374151',
  warning: '#92400e',
}
const TONE_BACKGROUND: Record<ReturnType<typeof readinessTone>, string> = {
  success: '#ecfdf5',
  neutral: '#f3f4f6',
  warning: '#fffbeb',
}

/**
 * Execution 03, Phase 13, Task 102-107: one real View Block -- its
 * identity, its own real geometry members (rendered with the same
 * `GeometryLayer` the main canvas uses, fit to a small preview box via
 * the existing `fitToBounds`, never a second shape-drawing
 * implementation), its real readiness/blockers, and its two real
 * shared-axis values (Task 108). "2D engineering regions" (Task 106):
 * a flat labeled panel, no 3D viewport chrome.
 */
export function ViewBlockCard({
  view,
  primitives,
  dimensions,
  onCommitAxis,
}: {
  view: ViewBlockSummary
  primitives: readonly PrimitiveSummary[]
  dimensions: readonly DimensionSummary[]
  onCommitAxis: (axis: SharedAxisName, value: number) => void
}) {
  const members = primitives.filter((p) => view.geometry_member_ids.includes(p.id))
  const tone = readinessTone(view.readiness)
  const axes = view.identity ? axesForIdentity(view.identity) : []

  const bounds =
    members.length > 0
      ? {
          minX: Math.min(...members.map((p) => p.min_x)),
          minY: Math.min(...members.map((p) => p.min_y)),
          maxX: Math.max(...members.map((p) => p.max_x)),
          maxY: Math.max(...members.map((p) => p.max_y)),
        }
      : null
  const previewViewport = bounds
    ? fitToBounds(bounds, { width: PREVIEW_WIDTH, height: PREVIEW_HEIGHT }, 16)
    : { panX: PREVIEW_WIDTH / 2, panY: PREVIEW_HEIGHT / 2, zoom: 1 }

  return (
    <div
      data-testid={`view-block-${view.id}`}
      data-view-identity={view.identity ?? undefined}
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: 8,
        padding: 12,
        background: '#ffffff',
        border: '1px solid #e5e4e1',
        borderRadius: 10,
        width: PREVIEW_WIDTH + 24,
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <span style={{ fontWeight: 600, fontSize: 13 }}>{view.identity ?? 'Unidentified'}</span>
        <span
          data-testid="view-block-readiness"
          style={{
            fontSize: 11,
            padding: '2px 6px',
            borderRadius: 6,
            color: TONE_COLOR[tone],
            background: TONE_BACKGROUND[tone],
          }}
        >
          {view.readiness}
        </span>
      </div>

      <div
        style={{
          width: PREVIEW_WIDTH,
          height: PREVIEW_HEIGHT,
          background: '#fafaf9',
          border: '1px solid #efeeeb',
          borderRadius: 6,
          overflow: 'hidden',
          position: 'relative',
        }}
      >
        {members.length > 0 ? (
          <GeometryLayer
            viewport={previewViewport}
            primitives={members}
            strokes={[]}
            selectedIds={new Set()}
          />
        ) : (
          <div
            data-testid="view-block-empty"
            style={{
              position: 'absolute',
              inset: 0,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: 11,
              color: '#9c9c98',
            }}
          >
            No geometry assigned yet
          </div>
        )}
      </div>

      {view.blockers.length > 0 && (
        <ul
          data-testid="view-block-blockers"
          style={{ margin: 0, paddingLeft: 16, fontSize: 11, color: '#b91c1c' }}
        >
          {view.blockers.map((blocker, i) => (
            <li key={i}>{blocker}</li>
          ))}
        </ul>
      )}

      {axes.map((axis) => {
        const value = axisValue(view, axis, dimensions)
        return (
          // Keyed on the real bound value, not just the axis: Task
          // 109's whole point is that propagating from one view
          // updates another view's own field without that field
          // having been touched itself, so when the real value
          // changes from *outside* this field, React should remount
          // it fresh (the standard "reset state when a prop changes"
          // pattern) rather than needing an effect to resync it.
          <AxisField
            key={`${axis}:${value ?? 'unresolved'}`}
            axis={axis}
            value={value}
            unresolved={isAxisUnresolved(view, axis)}
            onCommit={(v) => onCommitAxis(axis, v)}
          />
        )
      })}
    </div>
  )
}

function AxisField({
  axis,
  value,
  unresolved,
  onCommit,
}: {
  axis: SharedAxisName
  value: number | null
  unresolved: boolean
  onCommit: (value: number) => void
}) {
  const [raw, setRaw] = useState(value !== null ? String(value) : '')

  return (
    <form
      data-testid={`view-block-axis-${axis.toLowerCase()}`}
      onSubmit={(event) => {
        event.preventDefault()
        const parsed = Number.parseFloat(raw)
        if (Number.isFinite(parsed)) onCommit(parsed)
      }}
      style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 12 }}
    >
      <label style={{ width: 44, color: '#57534e' }}>{axis}</label>
      <input
        type="number"
        value={raw}
        placeholder={unresolved ? 'unresolved' : undefined}
        onChange={(event) => setRaw(event.target.value)}
        style={{ width: 70 }}
      />
      <button type="submit" style={{ fontSize: 11 }}>
        Set
      </button>
    </form>
  )
}
