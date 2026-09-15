import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 08 real-browser proof (Tasks 056-063): S/B
 * keyboard shortcuts and the Sketch/Pen toolbar buttons drive the same
 * real WorkspaceMode transitions, text-input focus suppresses the
 * shortcuts, and mode switching never touches document history.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
})

test('pressing S enters Sketch2D and pressing B returns to Creative (Task 059/060)', async ({
  page,
}) => {
  const toolbar = page.getByTestId('main-toolbar')
  await expect(toolbar).toHaveAttribute('data-workspace-mode', 'Creative')

  await page.keyboard.press('s')
  await expect(toolbar).toHaveAttribute('data-workspace-mode', 'Sketch2D')
  // Task 064's real morph: the Sketch2D tool set (Line, etc.) replaces
  // the Sketch button entirely, rather than the button staying and
  // showing pressed.
  await expect(page.getByTestId('tool-line')).toBeVisible()

  await page.keyboard.press('b')
  await expect(toolbar).toHaveAttribute('data-workspace-mode', 'Creative')
  await expect(page.getByTestId('tool-sketch')).toHaveAttribute('aria-pressed', 'false')
})

test('the Sketch/Pen toolbar buttons dispatch the same action as the keyboard (Task 061)', async ({
  page,
}) => {
  const toolbar = page.getByTestId('main-toolbar')

  await page.getByTestId('tool-sketch').click()
  await expect(toolbar).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  await page.getByTestId('tool-pen').click()
  await expect(toolbar).toHaveAttribute('data-workspace-mode', 'Creative')
})

test('S/B are ignored while a text input is focused (Task 060)', async ({ page }) => {
  await page.evaluate(() => {
    const input = document.createElement('input')
    input.id = 'test-text-input'
    document.body.appendChild(input)
    input.focus()
  })

  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')
})

test('Eraser is replaced by the Sketch2D tool set; Pen and Select stay available (Article 26, Task 064)', async ({
  page,
}) => {
  await page.keyboard.press('s')
  // Task 064: a real morph, not a disable -- Eraser (Notebook-only) is
  // gone from the DOM entirely, superseding Phase 08's earlier
  // disabled-button stopgap. Select stays: Dimension/Constraint need
  // it to have anything to act on.
  await expect(page.getByTestId('tool-eraser')).toHaveCount(0)
  await expect(page.getByTestId('tool-select')).toBeEnabled()
  await expect(page.getByTestId('tool-pen')).toBeEnabled()
  await expect(page.getByTestId('tool-line')).toBeEnabled()
})

test('mode switching never touches document history (Task 063)', async ({ page }) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')
  await page.mouse.move(box.x + 60, box.y + 60)
  await page.mouse.down()
  await page.mouse.move(box.x + 160, box.y + 100, { steps: 5 })
  await page.mouse.up()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
  await expect(page.getByTestId('tool-undo')).toBeEnabled()

  await page.keyboard.press('s')
  await page.keyboard.press('b')
  await page.keyboard.press('s')

  // The one real stroke is still there, and Undo still undoes exactly
  // that stroke -- not some mode-switch entry.
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
  await page.getByTestId('tool-undo').click()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(0)
  await expect(page.getByTestId('tool-undo')).toBeDisabled()
})
