import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 14 real-browser proof (Tasks 113-118): the real
 * document JSON (`CraftLoopSession.toJson()`) actually reaches
 * IndexedDB -- both through the debounced autosave and an explicit
 * Save -- and reload/New/Open all round-trip through it correctly.
 * jsdom has no real IndexedDB, so this phase's persistence behavior is
 * proven here, against a real browser, rather than in Vitest.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
})

async function drag(page: import('@playwright/test').Page, x0: number, y0: number, x1: number, y1: number) {
  await page.mouse.move(x0, y0)
  await page.mouse.down()
  await page.mouse.move(x1, y1, { steps: 8 })
  await page.mouse.up()
}

test.afterEach(async ({ page }) => {
  // Playwright already isolates storage per test by default, but
  // clearing explicitly documents the real intent and guards against
  // a future config change quietly sharing an origin between tests.
  await page.evaluate(() => {
    indexedDB.deleteDatabase('craftloop-web-live')
  })
})

test('autosave (no explicit Save) persists a committed change, and reloading the page recovers it (Task 114/116/117)', async ({
  page,
}) => {
  await page.keyboard.press('s')
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  // Task 116: no Save click here at all -- wait past the real
  // debounce for the autosave to actually write to IndexedDB.
  await page.waitForTimeout(1200)

  await page.reload()
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
})

test('New starts a real fresh document, and Open Last Saved recovers the real prior one (Task 115)', async ({
  page,
}) => {
  await page.keyboard.press('s')
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  // Explicit Save (Task 115): immediate, no debounce wait needed.
  await page.getByTestId('tool-save').click()

  await page.getByTestId('tool-more').click()
  await page.getByTestId('more-new-document').click()
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(0)
  // A real fresh session has real empty history too.
  await expect(page.getByTestId('tool-undo')).toBeDisabled()

  await page.getByTestId('tool-more').click()
  await page.getByTestId('more-open-last-saved').click()
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
})

test('UI-only preferences are never part of the persisted document (Task 118)', async ({ page }) => {
  await page.keyboard.press('s')
  await expect(page.getByTestId('tool-snap')).toHaveAttribute('aria-pressed', 'true')
  await page.getByTestId('tool-snap').click()
  await expect(page.getByTestId('tool-snap')).toHaveAttribute('aria-pressed', 'false')

  await page.waitForTimeout(1200)
  await page.reload()
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
  await page.keyboard.press('s')

  // Snap is ephemeral React state, seeded fresh on every load -- it
  // is never written into the real document JSON, so it comes back to
  // its default rather than the value from before reload.
  await expect(page.getByTestId('tool-snap')).toHaveAttribute('aria-pressed', 'true')
})
