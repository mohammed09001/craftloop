import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 10 real-browser proof (Tasks 083-086): a held
 * Pen stroke gets a real refinement candidate (via the real recognizer
 * /beautifier), shown as a dashed ghost with confirm/cancel, and a
 * quick (non-held) stroke stays plain ink with no interruption.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
})

test('a quick stroke (no hold) stays plain ink with no refinement prompt', async ({ page }) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.mouse.move(box.x + 100, box.y + 200)
  await page.mouse.down()
  await page.mouse.move(box.x + 250, box.y + 200, { steps: 15 })
  await page.mouse.up()

  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
  await expect(page.getByTestId('refinement-bar')).toHaveCount(0)
})

test('a held stroke offers a real refinement candidate, and Confirm keeps it (Task 083-085)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.mouse.move(box.x + 100, box.y + 200)
  await page.mouse.down()
  await page.mouse.move(box.x + 250, box.y + 200, { steps: 20 })
  // Hold: pointer stays down, no movement, long enough to clear the
  // real 500ms hold-detection timer.
  await page.waitForTimeout(700)
  await page.mouse.up()

  await expect(page.getByTestId('refinement-bar')).toBeVisible()
  const candidate = page.locator('[data-testid="geometry-layer"] [data-candidate="true"]')
  await expect(candidate).toHaveCount(1)
  // A near-straight horizontal drag should really beautify to a Line.
  await expect(page.locator('[data-testid="geometry-layer"] line[data-candidate="true"]')).toHaveCount(1)

  await page.getByTestId('refinement-confirm').click()
  await expect(page.getByTestId('refinement-bar')).toHaveCount(0)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
  await expect(page.locator('[data-testid="geometry-layer"] line[data-candidate="true"]')).toHaveCount(0)
  // The raw stroke was really replaced, not merely hidden underneath.
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(0)
})

test('Cancel really reverts through undo, restoring the raw ink (Task 086)', async ({ page }) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.mouse.move(box.x + 100, box.y + 260)
  await page.mouse.down()
  await page.mouse.move(box.x + 250, box.y + 260, { steps: 20 })
  await page.waitForTimeout(700)
  await page.mouse.up()

  await expect(page.getByTestId('refinement-bar')).toBeVisible()
  await page.getByTestId('refinement-cancel').click()

  await expect(page.getByTestId('refinement-bar')).toHaveCount(0)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(0)
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
})
