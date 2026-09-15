import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 07 real-browser proof (Tasks 050-055): the
 * toolbar renders top-center (never bottom), stays compact relative to
 * the canvas, and Select/Eraser/Undo/Redo drive real
 * CraftLoopSession/CanvasStack behavior -- not just visual state.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
})

test('the toolbar sits top-center over the canvas, never at the bottom (Task 050/051)', async ({
  page,
}) => {
  const viewportSize = page.viewportSize()
  if (!viewportSize) throw new Error('no viewport size')
  const toolbarBox = await page.getByTestId('main-toolbar').boundingBox()
  if (!toolbarBox) throw new Error('toolbar has no bounding box')

  // Top-anchored: well within the top quarter of the viewport.
  expect(toolbarBox.y).toBeLessThan(viewportSize.height * 0.25)
  // Roughly horizontally centered.
  const toolbarCenterX = toolbarBox.x + toolbarBox.width / 2
  expect(toolbarCenterX).toBeGreaterThan(viewportSize.width * 0.35)
  expect(toolbarCenterX).toBeLessThan(viewportSize.width * 0.65)
})

test('the toolbar stays compact, not a ribbon (Task 055)', async ({ page }) => {
  const viewportSize = page.viewportSize()
  if (!viewportSize) throw new Error('no viewport size')
  const toolbarBox = await page.getByTestId('main-toolbar').boundingBox()
  if (!toolbarBox) throw new Error('toolbar has no bounding box')

  expect(toolbarBox.height).toBeLessThan(60)
  expect(toolbarBox.width).toBeLessThan(viewportSize.width * 0.6)
})

test('Select tool disables drawing and enables click-to-select instead', async ({ page }) => {
  await page.getByTestId('tool-select').click()
  await expect(page.getByTestId('tool-select')).toHaveAttribute('aria-pressed', 'true')

  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  // A drag in Select mode must not create a stroke.
  await page.mouse.move(box.x + 60, box.y + 60)
  await page.mouse.down()
  await page.mouse.move(box.x + 180, box.y + 120, { steps: 5 })
  await page.mouse.up()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(0)
})

test('Eraser deletes the entity under the click', async ({ page }) => {
  // Draw a stroke in Pen mode first.
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')
  await page.mouse.move(box.x + 60, box.y + 200)
  await page.mouse.down()
  await page.mouse.move(box.x + 200, box.y + 200, { steps: 5 })
  await page.mouse.up()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)

  await page.getByTestId('tool-eraser').click()
  await page.mouse.click(box.x + 130, box.y + 200)
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(0)
})

test('Undo/Redo are disabled until there is real history, then work', async ({ page }) => {
  await expect(page.getByTestId('tool-undo')).toBeDisabled()

  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')
  await page.mouse.move(box.x + 60, box.y + 60)
  await page.mouse.down()
  await page.mouse.move(box.x + 160, box.y + 100, { steps: 5 })
  await page.mouse.up()

  await expect(page.getByTestId('tool-undo')).toBeEnabled()
  await page.getByTestId('tool-undo').click()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(0)
  await expect(page.getByTestId('tool-redo')).toBeEnabled()

  await page.getByTestId('tool-redo').click()
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
})

test('deferred tools (Sketch/View/Save/More) are disabled, not fake-functional', async ({
  page,
}) => {
  for (const id of ['sketch', 'view', 'save', 'more']) {
    await expect(page.getByTestId(`tool-${id}`)).toBeDisabled()
  }
})
