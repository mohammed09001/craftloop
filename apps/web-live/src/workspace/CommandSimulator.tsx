import { useState } from 'react'
import type { CommandActionName } from '../session/commandTypes'
import {
  WebCommandNamespace,
  WebConfirmationOutcome,
  WebRiskLevel,
  commandMayExecute,
  commandRequiresConfirmation,
} from '../session/craftLoopSession'
import type { UseCraftLoopSession } from '../session/useCraftLoopSession'

type SimResult =
  | { kind: 'executed'; action: CommandActionName; via: 'reserved-alias' | 'grammar' }
  | { kind: 'ambiguous'; candidates: string[] }
  | { kind: 'no-match' }
  | { kind: 'not-confirmed'; risk: string }
  | { kind: 'unsupported-action'; action: CommandActionName }

/**
 * Execution 03, Phase 15, Task 119: a developer-only panel -- not a
 * real end-user input method -- for exercising the real Command Bus
 * grammar and confirmation policy with typed text instead of a
 * pointer gesture. Three real inputs, per the task's own wording:
 * text, source (implicitly `InkCommand`, Article 15's "simulated ink
 * source" this whole panel stands in for), and confirmation.
 *
 * Task 127: this is explicitly **not** handwriting recognition. It
 * starts from already-typed text, the same way `resolveCommand`
 * itself starts from already-recognized text (see that function's own
 * doc comment) -- no claim is made here, or anywhere in this
 * component, about recognizing real ink on a real device.
 *
 * Task 120-123: a literal "s"/"b" is treated as the same reserved,
 * deterministic alias the real `S`/`B` keyboard shortcuts use
 * (`useKeyboardShortcuts.ts`) -- never run through the ambiguous-text
 * grammar, since a reserved alias is not "recognized text" either.
 * Anything else goes through the real `session.resolveCommand`
 * (Task 125: an `Ambiguous` result is shown, never guessed).
 *
 * Task 124: `commandRequiresConfirmation`/`commandMayExecute` are the
 * real, unmodified `craftloop_command::risk` policy functions -- the
 * Risk selector is a deliberate, clearly-labeled developer override
 * (every action this harness actually dispatches, `Sketch`/
 * `ExitSketch`, is real Low risk and never needs one) so the policy's
 * Medium/High gating is concretely demonstrable through the browser.
 *
 * Task 126: an action that clears the gate calls `onEnterSketchMode`/
 * `onEnterCreativePenMode` -- the exact same `Workspace`-level
 * callbacks its own Sketch/Pen toolbar buttons call (not a separate
 * copy that only calls the underlying session method) -- so toolbar
 * and command-bus entry points always produce the same real semantic
 * state, `activeTool` included.
 */
export function CommandSimulator({
  session,
  onEnterSketchMode,
  onEnterCreativePenMode,
  onClose,
}: {
  session: UseCraftLoopSession
  onEnterSketchMode: () => void
  onEnterCreativePenMode: () => void
  onClose: () => void
}) {
  const [text, setText] = useState('')
  const [namespace, setNamespace] = useState<WebCommandNamespace>(WebCommandNamespace.Notebook)
  const [risk, setRisk] = useState<WebRiskLevel>(WebRiskLevel.Low)
  const [confirmation, setConfirmation] = useState<WebConfirmationOutcome>(
    WebConfirmationOutcome.Execute,
  )
  const [result, setResult] = useState<SimResult | null>(null)

  const handleSimulate = () => {
    const trimmed = text.trim()
    const lower = trimmed.toLowerCase()
    let action: CommandActionName
    let via: 'reserved-alias' | 'grammar'

    if (lower === 's') {
      action = 'Sketch'
      via = 'reserved-alias'
    } else if (lower === 'b') {
      action = 'ExitSketch'
      via = 'reserved-alias'
    } else {
      const match = session.resolveCommand(trimmed, namespace)
      if (!match || match === 'NoMatch') {
        setResult({ kind: 'no-match' })
        return
      }
      if ('Ambiguous' in match) {
        setResult({ kind: 'ambiguous', candidates: match.Ambiguous.candidates })
        return
      }
      via = 'grammar'
      action = 'Exact' in match ? match.Exact.action : match.UniquePrefix.action
    }

    if (commandRequiresConfirmation(risk) && !commandMayExecute(risk, confirmation)) {
      setResult({ kind: 'not-confirmed', risk: WebRiskLevel[risk] })
      return
    }

    if (action === 'Sketch') {
      onEnterSketchMode()
      setResult({ kind: 'executed', action, via })
    } else if (action === 'Pen' || action === 'ExitSketch') {
      onEnterCreativePenMode()
      setResult({ kind: 'executed', action, via })
    } else {
      setResult({ kind: 'unsupported-action', action })
    }
  }

  return (
    <div
      data-testid="command-simulator"
      style={{
        position: 'absolute',
        top: 72,
        left: '50%',
        transform: 'translateX(-50%)',
        display: 'flex',
        flexDirection: 'column',
        gap: 8,
        padding: 12,
        width: 320,
        background: '#ffffff',
        border: '1px solid #e5e4e1',
        borderRadius: 10,
        boxShadow: '0 2px 10px rgba(8, 6, 13, 0.12)',
        pointerEvents: 'auto',
        fontSize: 12,
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <span style={{ fontWeight: 600 }}>Command Simulator (Dev)</span>
        <button type="button" data-testid="command-simulator-close" onClick={onClose}>
          Close
        </button>
      </div>
      <p style={{ margin: 0, color: '#78716c' }}>
        Simulates already-recognized text reaching the real Command Bus -- not real handwriting
        recognition.
      </p>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        Text
        <input
          data-testid="command-simulator-input"
          value={text}
          onChange={(event) => setText(event.target.value)}
        />
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        Namespace
        <select
          data-testid="command-simulator-namespace"
          value={namespace}
          onChange={(event) => setNamespace(Number(event.target.value) as WebCommandNamespace)}
        >
          <option value={WebCommandNamespace.Notebook}>Notebook</option>
          <option value={WebCommandNamespace.Sketch}>Sketch</option>
          <option value={WebCommandNamespace.Orthographic}>Orthographic</option>
        </select>
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        Risk (developer override -- real dispatched actions are always Low)
        <select
          data-testid="command-simulator-risk"
          value={risk}
          onChange={(event) => setRisk(Number(event.target.value) as WebRiskLevel)}
        >
          <option value={WebRiskLevel.Low}>Low</option>
          <option value={WebRiskLevel.Medium}>Medium</option>
          <option value={WebRiskLevel.High}>High</option>
        </select>
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        Confirmation
        <select
          data-testid="command-simulator-confirmation"
          value={confirmation}
          disabled={!commandRequiresConfirmation(risk)}
          onChange={(event) =>
            setConfirmation(Number(event.target.value) as WebConfirmationOutcome)
          }
        >
          <option value={WebConfirmationOutcome.RemainsInk}>Remains Ink</option>
          <option value={WebConfirmationOutcome.Preview}>Preview</option>
          <option value={WebConfirmationOutcome.Execute}>Execute</option>
        </select>
      </label>

      <button type="button" data-testid="command-simulator-submit" onClick={handleSimulate}>
        Simulate
      </button>

      {result && (
        <div data-testid="command-simulator-result" style={{ paddingTop: 4 }}>
          {result.kind === 'executed' &&
            `Executed ${result.action} (${result.via === 'reserved-alias' ? 'reserved alias' : 'grammar'}) -- same real session call the toolbar uses.`}
          {result.kind === 'ambiguous' &&
            `Ambiguous: ${result.candidates.join(', ')} -- refused to guess.`}
          {result.kind === 'no-match' && 'No match in the real grammar for this namespace.'}
          {result.kind === 'not-confirmed' &&
            `${result.risk}-risk command was not confirmed strongly enough -- rejected.`}
          {result.kind === 'unsupported-action' &&
            `Resolved to ${result.action}, but the simulator only dispatches Sketch/Pen mode changes.`}
        </div>
      )}
    </div>
  )
}
