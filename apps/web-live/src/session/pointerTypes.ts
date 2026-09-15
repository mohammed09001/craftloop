/** Mirrors `craftloop_web_bridge::types::WebPointerSample` (Article 15). */

export type PointerSourceName = 'SimulatedMouse' | 'Stylus' | 'Touch'

export interface PointerSampleInput {
  x: number
  y: number
  timestamp_seconds: number
  pressure: number | null
  tilt_x_deg: number | null
  tilt_y_deg: number | null
  source: PointerSourceName
  button_primary: boolean
  button_secondary: boolean
  button_barrel: boolean
  capability_pressure: boolean
  capability_tilt: boolean
  capability_hover: boolean
  capability_palm_rejection: boolean
  capability_eraser: boolean
}
