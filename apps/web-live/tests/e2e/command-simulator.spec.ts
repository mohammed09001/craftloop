import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 15 real-browser proof (Tasks 119-126): the
 * developer-only Command Simulator actually routes through the real
 * Command Bus grammar (`resolveCommand`) and the real risk/
 * confirmation policy (`commandRequiresConfirmation`/
 * `commandMayExecute`), and a resolved Sketch/Pen action produces the
 * exact same real workspace-mode change the toolbar's own buttons
 * produce.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
  await page.getByTestId('tool-more').click()
  await page.getByTestId('more-command-simulator').click()
  await expect(page.getByTestId('command-simulator')).toBeVisible()
})

test('a reserved "s" alias enters Sketch2D, and "sketch" (the real grammar) does too (Task 120/121)', async ({
  page,
}) => {
  await page.getByTestId('command-simulator-input').fill('s')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('reserved alias')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  // Back to Creative so the second, full-word case starts from the
  // same real Notebook state "sketch" is registered under.
  await page.keyboard.press('b')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')

  await page.getByTestId('command-simulator-input').fill('sketch')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('grammar')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')
})

test('a reserved "b" alias and the real grammar word "exit sketch" both return to Creative Pen (Task 122/123)', async ({
  page,
}) => {
  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  await page.getByTestId('command-simulator-input').fill('b')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('reserved alias')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')

  await page.keyboard.press('s')
  await page.getByTestId('command-simulator-namespace').selectOption('1') // Sketch
  await page.getByTestId('command-simulator-input').fill('exit sketch')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('grammar')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')
})

test('an ambiguous prefix is named, never guessed, and never changes the real mode (Task 125)', async ({
  page,
}) => {
  // "o" matches both real Notebook words "orthographic" and its
  // curated synonym "ortho" -- genuinely ambiguous by word count even
  // though both name the same action (Article 237's own example).
  // "s"/"b" are this simulator's own reserved aliases (Task 120/122),
  // so this uses a real grammar-level ambiguity instead.
  await page.getByTestId('command-simulator-input').fill('o')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('Ambiguous')
  await expect(page.getByTestId('command-simulator-result')).toContainText('orthographic')
  await expect(page.getByTestId('command-simulator-result')).toContainText('ortho')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')
})

test('a Medium-risk command without strong enough confirmation is rejected; Execute clears it (Task 124)', async ({
  page,
}) => {
  await page.getByTestId('command-simulator-risk').selectOption('1') // Medium
  await page.getByTestId('command-simulator-input').fill('sketch')

  await page.getByTestId('command-simulator-confirmation').selectOption('0') // RemainsInk
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('not confirmed')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')

  await page.getByTestId('command-simulator-confirmation').selectOption('2') // Execute
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('command-simulator-result')).toContainText('grammar')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')
})

test('the simulated command produces the exact same real state the toolbar button does (Task 126)', async ({
  page,
}) => {
  await page.getByTestId('command-simulator-input').fill('sketch')
  await page.getByTestId('command-simulator-submit').click()
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  // The real toolbar Sketch/Pen buttons are gone from this mode's set
  // (Task 064's real morph) -- the exact same observable consequence
  // clicking "Sketch" in the toolbar itself would have produced.
  await expect(page.getByTestId('tool-sketch')).toHaveCount(0)
  await expect(page.getByTestId('tool-line')).toBeVisible()
})
