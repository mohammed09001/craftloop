import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 11 real-browser proof (Tasks 087-093): precision
 * inference (snap-to-geometry/grid) actually changes what coordinate a
 * real primitive gets created at -- not just a preview effect -- and
 * Construction toggles real, persisted `is_construction` state that
 * renders visibly differently. Both exercised against the real
 * `CraftLoopSession`/WASM engine, not a mock.
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

test('Snap is on by default and pulls a new endpoint exactly onto an existing one (Task 087/093)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  expect(await page.getByTestId('tool-snap').getAttribute('aria-pressed')).toBe('true')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  // The second line's end point (145, 143 relative to the box) lands
  // within snap tolerance of the first line's real endpoint -- with
  // Snap on, the primitive that actually gets created must land
  // exactly on it, not at the raw drag coordinate.
  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 300, box.y + 300, box.x + 145, box.y + 143)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(2)

  const secondLine = page.locator('[data-testid="geometry-layer"] line').nth(1)
  await expect(secondLine).toHaveAttribute('x2', '140')
  await expect(secondLine).toHaveAttribute('y2', '140')
})

test('turning Snap off leaves a new endpoint at the raw drag coordinate, even near existing geometry (Task 092/093)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  await page.getByTestId('tool-snap').click()
  await expect(page.getByTestId('tool-snap')).toHaveAttribute('aria-pressed', 'false')
  // Task 092: the background grid is part of the same precision
  // feature -- it disappears along with snapping, not independently.
  await expect(page.getByTestId('background-grid')).toHaveCount(0)

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 300, box.y + 300, box.x + 145, box.y + 143)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(2)

  const secondLine = page.locator('[data-testid="geometry-layer"] line').nth(1)
  await expect(secondLine).toHaveAttribute('x2', '145')
  await expect(secondLine).toHaveAttribute('y2', '143')
})

test('Construction toggles real, persisted is_construction state on the selected primitive and renders it distinctly (Task 090/091)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)

  await page.getByTestId('tool-select').click()
  await expect(page.getByTestId('tool-construction')).toBeDisabled()
  await page.mouse.click(box.x + 90, box.y + 140)
  await expect(page.getByTestId('tool-construction')).toBeEnabled()

  const line = page.locator('[data-testid="geometry-layer"] line').first()
  await expect(line).not.toHaveAttribute('data-construction')

  await page.getByTestId('tool-construction').click()
  await expect(line).toHaveAttribute('data-construction', 'true')

  // Toggling again on the same (still-selected) primitive flips it back.
  await page.getByTestId('tool-construction').click()
  await expect(line).not.toHaveAttribute('data-construction')
})
