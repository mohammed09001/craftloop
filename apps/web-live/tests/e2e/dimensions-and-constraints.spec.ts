import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 12 real-browser proof (Tasks 094-101): the
 * geometry-adjacent dimension popover actually creates/edits a real
 * dimension through `CraftLoopSession`, an invalid edit produces a
 * real, persisted conflict the UI must surface (not a client-side
 * pretend-rollback), the constraint popover's options come from the
 * real backend `eligibleConstraints` call, applying a constraint shows
 * real solver feedback, and the annotation visibility toggle actually
 * hides/reveals real dimension/constraint data. All against the real
 * WASM engine, not a mock.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')
})

async function drag(page: import('@playwright/test').Page, x0: number, y0: number, x1: number, y1: number) {
  await page.mouse.move(x0, y0)
  await page.mouse.down()
  await page.mouse.move(x1, y1, { steps: 8 })
  await page.mouse.up()
}

test('creating a dimension renders a real annotation near the selected geometry, and editing it changes the real value (Task 094-096)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)
  await expect(page.getByTestId('tool-dimension')).toBeEnabled()

  await page.getByTestId('tool-dimension').click()
  await page.getByTestId('dimension-input-value').fill('25')
  await page.getByTestId('dimension-input-submit').click()
  await expect(page.getByTestId('dimension-input')).toHaveCount(0)

  // Task 095: the annotation renders the real value from the scene
  // snapshot -- it is visible because the target line is selected
  // (Task 101's "near selection" default).
  const annotation = page.locator('[data-testid^="dimension-annotation-"]')
  await expect(annotation).toBeVisible()
  await expect(annotation).toHaveText('25')

  // Task 096: clicking the real annotation opens the same popover,
  // pre-filled, for editing.
  await annotation.click()
  await expect(page.getByTestId('dimension-input')).toBeVisible()
  await expect(page.getByTestId('dimension-input-value')).toHaveValue('25')
  await page.getByTestId('dimension-input-value').fill('60')
  await page.getByTestId('dimension-input-submit').click()
  await expect(page.getByTestId('dimension-input')).toHaveCount(0)
  await expect(annotation).toHaveText('60')
})

test('an edit that produces a real conflict keeps the last valid value and shows the real evidence (Task 097)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)
  await page.getByTestId('tool-dimension').click()
  await page.getByTestId('dimension-input-value').fill('100')
  await page.getByTestId('dimension-input-submit').click()

  const annotation = page.locator('[data-testid^="dimension-annotation-"]')
  await expect(annotation).toHaveText('100')

  // A negative linear dimension is not a valid driving value -- the
  // real `edit_dimension` rejects it and commits a real Conflict
  // entity instead of the edit.
  await annotation.click()
  await page.getByTestId('dimension-input-value').fill('-5')
  await page.getByTestId('dimension-input-submit').click()

  await expect(page.getByTestId('dimension-conflict-evidence')).toBeVisible()
  await expect(page.getByTestId('dimension-conflict-resolve-KeepExisting')).toBeVisible()

  await page.getByTestId('dimension-conflict-resolve-KeepExisting').click()
  await expect(page.getByTestId('dimension-input')).toHaveCount(0)
  // The geometry's real value never changed -- "keep last valid
  // geometry" (Task 097's own wording).
  await expect(annotation).toHaveText('100')
})

test('the constraint popover lists only the real backend-derived eligible kinds and shows real solver feedback (Task 098-100)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 190)

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 165)
  await expect(page.getByTestId('tool-constraint')).toBeEnabled()

  await page.getByTestId('tool-constraint').click()
  // Task 098: one Line selected -- exactly Horizontal/Vertical, the
  // real backend eligibility for that primitive kind/count, nothing
  // hand-listed on the frontend.
  await expect(page.getByTestId('constraint-option-horizontal')).toBeVisible()
  await expect(page.getByTestId('constraint-option-vertical')).toBeVisible()
  await expect(page.getByTestId('constraint-menu').getByRole('menuitem')).toHaveCount(2)

  await page.getByTestId('constraint-option-horizontal').click()

  // Task 100: a real, visible solver-status signal -- not just "the
  // shape moved (or didn't)."
  const feedback = page.getByTestId('solver-feedback')
  await expect(feedback).toBeVisible()
  await expect(feedback).toHaveAttribute('data-tone', 'success')

  const line = page.locator('[data-testid="geometry-layer"] line').first()
  await expect
    .poll(async () => {
      const y1 = Number(await line.getAttribute('y1'))
      const y2 = Number(await line.getAttribute('y2'))
      return Math.abs(y1 - y2)
    })
    .toBeLessThan(0.5)
})

test('Show All reveals dimension/constraint annotations beyond the current selection (Task 101)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)
  await page.getByTestId('tool-dimension').click()
  await page.getByTestId('dimension-input-value').fill('30')
  await page.getByTestId('dimension-input-submit').click()

  const annotation = page.locator('[data-testid^="dimension-annotation-"]')
  await expect(annotation).toBeVisible()

  // Deselecting hides it again by default (only annotations touching
  // the current selection show).
  await page.mouse.click(box.x + 300, box.y + 300)
  await expect(annotation).toHaveCount(0)

  expect(await page.getByTestId('tool-annotations').getAttribute('aria-pressed')).toBe('false')
  await page.getByTestId('tool-annotations').click()
  await expect(page.getByTestId('tool-annotations')).toHaveAttribute('aria-pressed', 'true')
  await expect(annotation).toBeVisible()
})
