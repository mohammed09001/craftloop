import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 06 real-browser proof (Tasks 043-049): drawing,
 * semantic hit-test selection, and pan/zoom, all against the real
 * CraftLoopSession running in Wasm -- not a mock.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('ink-canvas')).toBeVisible()
  // Wait for the real Wasm CraftLoopSession to finish initializing, so
  // interactions below aren't racing the async module load.
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
})

test('a drawn stroke appears in the structured geometry layer with real points', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  const start = { x: box.x + 80, y: box.y + 80 }
  const end = { x: box.x + 220, y: box.y + 160 }

  await page.mouse.move(start.x, start.y)
  await page.mouse.down()
  await page.mouse.move((start.x + end.x) / 2, (start.y + end.y) / 2, { steps: 5 })
  await page.mouse.move(end.x, end.y, { steps: 5 })
  await page.mouse.up()

  const strokePolyline = page.locator('[data-testid="geometry-layer"] polyline[data-entity-kind="stroke"]')
  await expect(strokePolyline).toHaveCount(1)
  const points = await strokePolyline.getAttribute('points')
  expect(points).toBeTruthy()
  expect(points!.trim().split(' ').length).toBeGreaterThanOrEqual(2)
})

test('clicking a drawn stroke selects it, and clicking empty space clears the selection', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  const start = { x: box.x + 100, y: box.y + 300 }
  const end = { x: box.x + 300, y: box.y + 300 }
  await page.mouse.move(start.x, start.y)
  await page.mouse.down()
  await page.mouse.move((start.x + end.x) / 2, end.y, { steps: 5 })
  await page.mouse.up()

  const strokePolyline = page.locator('[data-testid="geometry-layer"] polyline[data-entity-kind="stroke"]')
  await expect(strokePolyline).toHaveCount(1)
  const unselectedColor = await strokePolyline.evaluate((el) => el.getAttribute('stroke'))

  // A short tap (no drag) on the stroke's midpoint selects it.
  await page.mouse.click((start.x + end.x) / 2, end.y)
  await expect(strokePolyline).toHaveAttribute('stroke', '#2563eb')

  // A tap on empty space clears the selection.
  await page.mouse.click(box.x + 20, box.y + 20)
  await expect(strokePolyline).toHaveAttribute('stroke', unselectedColor!)
})

test('the wheel zooms the viewport and a middle-button drag pans it', async ({ page }) => {
  const stack = page.getByTestId('canvas-stack')
  const box = await stack.boundingBox()
  if (!box) throw new Error('canvas stack has no bounding box')
  const center = { x: box.x + box.width / 2, y: box.y + box.height / 2 }

  const geometryGroup = page.locator('[data-testid="geometry-layer"] g')
  const initialTransform = await geometryGroup.evaluate((el) => el.getAttribute('style'))

  await page.mouse.move(center.x, center.y)
  await page.mouse.wheel(0, -400)
  await expect
    .poll(async () => geometryGroup.evaluate((el) => el.getAttribute('style')))
    .not.toBe(initialTransform)
  const afterZoom = await geometryGroup.evaluate((el) => el.getAttribute('style'))
  expect(afterZoom).toContain('scale(')
  expect(afterZoom).not.toContain('scale(1)')

  await page.mouse.move(center.x, center.y)
  await page.mouse.down({ button: 'middle' })
  await page.mouse.move(center.x + 60, center.y + 30, { steps: 5 })
  await page.mouse.up({ button: 'middle' })
  await expect
    .poll(async () => geometryGroup.evaluate((el) => el.getAttribute('style')))
    .not.toBe(afterZoom)
})
