import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 09 real-browser proof (Tasks 064-077): the
 * Sketch2D toolbar morph, real Line/Arc/Circle/Rectangle creation from
 * a drag, and selection-driven Dimension/Constraint against the real
 * CraftLoopSession -- not mocks.
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

test('Line/Circle/Rectangle/Arc tools each create the real primitive from one drag (Task 066-069)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 40, box.x + 140, box.y + 40)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  await page.getByTestId('tool-circle').click()
  await drag(page, box.x + 200, box.y + 200, box.x + 250, box.y + 200)
  await expect(page.locator('[data-testid="geometry-layer"] circle')).toHaveCount(1)

  await page.getByTestId('tool-rectangle').click()
  await drag(page, box.x + 300, box.y + 300, box.x + 380, box.y + 360)
  await expect(page.locator('[data-testid="geometry-layer"] polygon')).toHaveCount(1)

  await page.getByTestId('tool-arc').click()
  // Kept well below the top-center toolbar (roughly y:16-62), which
  // otherwise intercepts the drag before it ever reaches the canvas.
  await drag(page, box.x + 400, box.y + 140, box.x + 450, box.y + 180)
  await expect(page.locator('[data-testid="geometry-layer"] path')).toHaveCount(1)
})

test('Select works inside Sketch2D so Dimension/Constraint have something to act on (Task 072/073)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 40, box.x + 140, box.y + 40)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  await page.getByTestId('tool-select').click()
  await expect(page.getByTestId('tool-dimension')).toBeDisabled()
  await page.mouse.click(box.x + 90, box.y + 40)
  await expect(page.getByTestId('tool-dimension')).toBeEnabled()

  // Execution 03, Phase 12, Task 094: a geometry-adjacent popover, not
  // a `window.prompt` dialog.
  await page.getByTestId('tool-dimension').click()
  await expect(page.getByTestId('dimension-input')).toBeVisible()
  await page.getByTestId('dimension-input-value').fill('42')
  await page.getByTestId('dimension-input-submit').click()
  await expect(page.getByTestId('dimension-input')).toHaveCount(0)
  await expect(page.locator('[data-testid="geometry-layer"]')).toBeVisible()
})

test('Constraint offers only eligible kinds for the current selection and applies through the real solver (Task 073)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  // A visibly tilted line, so a real Horizontal solve is observable.
  await drag(page, box.x + 40, box.y + 40, box.x + 140, box.y + 90)

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 65)
  await expect(page.getByTestId('tool-constraint')).toBeEnabled()

  await page.getByTestId('tool-constraint').click()
  await expect(page.getByTestId('constraint-menu')).toBeVisible()
  await expect(page.getByTestId('constraint-option-horizontal')).toBeVisible()
  await expect(page.getByTestId('constraint-option-vertical')).toBeVisible()

  await page.getByTestId('constraint-option-horizontal').click()
  await expect(page.getByTestId('constraint-menu')).toHaveCount(0)

  const line = page.locator('[data-testid="geometry-layer"] line').first()
  await expect
    .poll(async () => {
      const y1 = Number(await line.getAttribute('y1'))
      const y2 = Number(await line.getAttribute('y2'))
      return Math.abs(y1 - y2)
    })
    .toBeLessThan(0.5)
})
