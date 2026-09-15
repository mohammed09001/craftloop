import init, { CraftLoopSession } from '../wasm-bridge/craftloop_web_bridge.js'

/**
 * Lazily initializes the Wasm module and creates one `CraftLoopSession`
 * for the whole tab. A module-level singleton (not per-component state)
 * because the shared Rust document is the one source of engineering
 * truth for this page (Article 8) -- there is exactly one Web Live View
 * session per tab in this execution, not one per component instance.
 */
let sessionPromise: Promise<CraftLoopSession> | null = null

export function getCraftLoopSession(): Promise<CraftLoopSession> {
  sessionPromise ??= init().then(() => new CraftLoopSession())
  return sessionPromise
}

export type { CraftLoopSession }
export {
  WebCommandNamespace,
  WebDimensionKind,
  WebPrincipalViewIdentity,
  WebResolutionChoice,
  WebSharedAxis,
  resolveCommand,
} from '../wasm-bridge/craftloop_web_bridge.js'
