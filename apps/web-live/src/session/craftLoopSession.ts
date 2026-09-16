import init, { CraftLoopSession } from '../wasm-bridge/craftloop_web_bridge.js'

/**
 * Lazily initializes the Wasm module (once per tab -- re-running
 * `init()` would re-fetch/re-instantiate the same module for no
 * reason) and hands back the one `CraftLoopSession` this tab starts
 * with. A module-level singleton (not per-component state) because
 * the shared Rust document is the one source of engineering truth for
 * this page (Article 8) -- there is exactly one *initial* Web Live
 * View session per tab, not one per component instance.
 *
 * Task 115's "New"/"Open" (Phase 14) still need to construct a further
 * `CraftLoopSession` later in the same tab's lifetime (a fresh empty
 * one, or one rebuilt from a saved JSON payload) -- `createSession`
 * exists for exactly that, sharing the same one-time Wasm init.
 */
let initPromise: Promise<void> | null = null

function ensureWasmInit(): Promise<void> {
  initPromise ??= init().then(() => undefined)
  return initPromise
}

let sessionPromise: Promise<CraftLoopSession> | null = null

export function getCraftLoopSession(): Promise<CraftLoopSession> {
  sessionPromise ??= ensureWasmInit().then(() => new CraftLoopSession())
  return sessionPromise
}

/** A brand-new, empty `CraftLoopSession` -- Task 115's "New Document." */
export async function createSession(): Promise<CraftLoopSession> {
  await ensureWasmInit()
  return new CraftLoopSession()
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

/** Execution 03, Phase 14, Task 113/117: reconstructs a session from a real, previously-serialized `CraftLoopSession.toJson()` payload -- the same path-independent round trip `craftloop-web-bridge` already tests, just called from the reload path instead of a fresh-session path. */
export function sessionFromJson(json: string): CraftLoopSession {
  return CraftLoopSession.fromJson(json)
}
