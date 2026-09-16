import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 13 real-browser proof (Tasks 102-112): the same
 * "golden alpha journey" `craftloop-web-bridge`'s own
 * `golden_alpha_journey_end_to_end_through_json_round_trip` test
 * proves at the Rust level, driven here through the real browser UI --
 * assigning a Front view from real selected geometry, entering
 * Orthographic (which creates real Top/Right/Back view blocks and one
 * real `OrthographicSet`), propagating a shared axis value from a
 * non-Front view, hitting a real cross-view conflict, and resolving
 * it -- all against the real WASM engine, not a mock.
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

test('assigning Front, entering Orthographic, propagating a shared axis, and resolving a real cross-view conflict (Task 102-112)', async ({
  page,
}) => {
  // Draw and select a line in Sketch2D, then return to Creative mode --
  // View lives in the Creative toolbar, and selection is real ephemeral
  // session state that survives the mode switch (Phase 08).
  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')
  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)

  await page.keyboard.press('b')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')

  await expect(page.getByTestId('tool-view')).toBeEnabled()
  await page.getByTestId('tool-view').click()
  await expect(page.getByTestId('orthographic-panel')).toBeVisible()
  expect(await page.getByTestId('tool-view').getAttribute('aria-pressed')).toBe('true')

  // Task 102-104: assigning Front and entering Orthographic creates
  // real Top/Right/Back view blocks in one real OrthographicSet.
  await expect(page.getByTestId('orthographic-create-front')).toBeEnabled()
  await page.getByTestId('orthographic-create-front').click()

  await expect(page.locator('[data-view-identity="Front"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Top"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Right"]')).toBeVisible()
  // Task 105: Back is rendered because the real engine actually
  // produced one for this case -- never fabricated proactively.
  await expect(page.locator('[data-view-identity="Back"]')).toBeVisible()

  // Task 106/107: Front shows the real line geometry; Top/Right have
  // no geometry members yet, so readiness/blockers stay real and
  // explicit, not invented.
  const front = page.locator('[data-view-identity="Front"]')
  await expect(front.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
  const top = page.locator('[data-view-identity="Top"]')
  await expect(top.getByTestId('view-block-empty')).toBeVisible()

  // Task 108/112: entering Depth from Top (not Front) propagates
  // through the real multiview graph to Right -- proving no view is a
  // permanent master.
  await top.getByTestId('view-block-axis-depth').locator('input').fill('40')
  await top.getByTestId('view-block-axis-depth').getByRole('button', { name: 'Set' }).click()

  const right = page.locator('[data-view-identity="Right"]')
  await expect(right.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('40')

  // Task 110: a contradictory Depth entered from Right produces a
  // real, persisted conflict with real evidence and real legal
  // resolutions -- not a silent overwrite.
  await right.getByTestId('view-block-axis-depth').locator('input').fill('999')
  await right.getByTestId('view-block-axis-depth').getByRole('button', { name: 'Set' }).click()

  await expect(page.getByTestId('orthographic-conflict-evidence')).toBeVisible()
  await expect(page.getByTestId('orthographic-conflict-resolve-ReplaceAndPropagate')).toBeVisible()

  // Task 111: resolving with the real ReplaceAndPropagate choice
  // actually propagates the winning value back to Top too.
  await page.getByTestId('orthographic-conflict-resolve-ReplaceAndPropagate').click()
  await expect(page.getByTestId('orthographic-conflict')).toHaveCount(0)
  await expect(top.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('999')

  await page.getByTestId('orthographic-close').click()
  await expect(page.getByTestId('orthographic-panel')).toHaveCount(0)
  expect(await page.getByTestId('tool-view').getAttribute('aria-pressed')).toBe('false')
})
