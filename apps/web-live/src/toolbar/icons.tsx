import type { ToolId } from './toolRegistry'

/**
 * Minimal 24x24 line icons, one per tool (Task 053: icon-first, no
 * persistent text labels). `aria-hidden` -- the accessible name lives
 * on the enclosing `<button>`'s `aria-label`, not on the glyph.
 */
const strokeProps = {
  fill: 'none',
  stroke: 'currentColor',
  strokeWidth: 1.75,
  strokeLinecap: 'round' as const,
  strokeLinejoin: 'round' as const,
}

function Svg({ children }: { children: React.ReactNode }) {
  return (
    <svg viewBox="0 0 24 24" width={20} height={20} aria-hidden="true" {...strokeProps}>
      {children}
    </svg>
  )
}

const ICONS: Record<ToolId, React.ReactNode> = {
  pen: (
    <Svg>
      <path d="M4 20l1-4.5L15.5 5l3.5 3.5L8.5 19 4 20z" />
      <path d="M13 7l3.5 3.5" />
    </Svg>
  ),
  select: (
    <Svg>
      <path d="M5 4l6.5 15 2-6.5L20 10.5 5 4z" strokeLinejoin="round" />
    </Svg>
  ),
  eraser: (
    <Svg>
      <rect x="5.5" y="10.5" width="11" height="7" rx="1" transform="rotate(-20 11 14)" />
      <path d="M8.5 14.5L15 5l4 3-6 10.5" />
    </Svg>
  ),
  sketch: (
    <Svg>
      <rect x="4" y="4" width="16" height="16" rx="1.5" />
      <path d="M4 15l5-5 4 4 7-7" />
    </Svg>
  ),
  view: (
    <Svg>
      <path d="M2.5 12S6 6 12 6s9.5 6 9.5 6-3.5 6-9.5 6S2.5 12 2.5 12z" />
      <circle cx="12" cy="12" r="2.5" />
    </Svg>
  ),
  undo: (
    <Svg>
      <path d="M8 7L4 11l4 4" />
      <path d="M4 11h9.5a5.5 5.5 0 110 11H12" />
    </Svg>
  ),
  redo: (
    <Svg>
      <path d="M16 7l4 4-4 4" />
      <path d="M20 11h-9.5a5.5 5.5 0 100 11H12" />
    </Svg>
  ),
  save: (
    <Svg>
      <path d="M5 4h11l3 3v13H5V4z" strokeLinejoin="round" />
      <path d="M8 4v5h7V4" />
      <path d="M8 14h8v6H8z" />
    </Svg>
  ),
  more: (
    <Svg>
      <circle cx="6" cy="12" r="1.4" fill="currentColor" stroke="none" />
      <circle cx="12" cy="12" r="1.4" fill="currentColor" stroke="none" />
      <circle cx="18" cy="12" r="1.4" fill="currentColor" stroke="none" />
    </Svg>
  ),
  line: (
    <Svg>
      <path d="M4 20L20 4" />
    </Svg>
  ),
  arc: (
    <Svg>
      <path d="M4 18a14 14 0 0116-14" />
    </Svg>
  ),
  circle: (
    <Svg>
      <circle cx="12" cy="12" r="8" />
    </Svg>
  ),
  rectangle: (
    <Svg>
      <rect x="4" y="6" width="16" height="12" rx="1" />
    </Svg>
  ),
  dimension: (
    <Svg>
      <path d="M4 8v8M20 8v8M4 12h16" />
      <path d="M7 9.5L4 12l3 2.5M17 9.5L20 12l-3 2.5" />
    </Svg>
  ),
  constraint: (
    <Svg>
      <path d="M4 6h16M4 12h16M4 18h10" />
    </Svg>
  ),
  construction: (
    <Svg>
      <path d="M4 20L20 4" strokeDasharray="3 3" />
    </Svg>
  ),
  snap: (
    <Svg>
      <path d="M4 4v4M4 4h4M20 4v4M20 4h-4M4 20v-4M4 20h4M20 20v-4M20 20h-4" />
      <circle cx="12" cy="12" r="1.4" fill="currentColor" stroke="none" />
    </Svg>
  ),
  annotations: (
    <Svg>
      <circle cx="5.5" cy="12" r="2" />
      <circle cx="18.5" cy="12" r="2" />
      <path d="M7.5 12h9M9 9l1.5-3M15 9l-1.5-3" />
    </Svg>
  ),
}

export function ToolIcon({ id }: { id: ToolId }) {
  return <>{ICONS[id]}</>
}
