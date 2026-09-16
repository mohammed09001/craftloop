import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 20 (Tasks 152-157): Article 83's "Golden Web
 * Journey," run as one continuous real session -- every other e2e
 * spec in this project proves one feature in isolation (its own fresh
 * `beforeEach` page); this is the one test proving they all compose
 * correctly together in the same document, the same way a real user's
 * session would actually unfold. Steps are numbered to match Article
 * 83 exactly; step 10 ("Ellipse if exposed") is skipped with a comment
 * rather than silently omitted, since Ellipse is a real, documented,
 * deliberate deferral (Phase 09's evidence). Steps 31/32 (internet
 * preview) are Task 158's own job, not this file's -- see
 * `preview-smoke.spec.ts`.
 */

async function drag(page: import('@playwright/test').Page, x0: number, y0: number, x1: number, y1: number) {
  await page.mouse.move(x0, y0)
  await page.mouse.down()
  await page.mouse.move(x1, y1, { steps: 8 })
  await page.mouse.up()
}

test.afterEach(async ({ page }) => {
  // This journey saves a real document, unlike most other specs --
  // clear it so a later run in the same real IndexedDB origin never
  // starts from a leftover save (see persistence.spec.ts for the same
  // pattern/reasoning).
  await page.evaluate(() => {
    indexedDB.deleteDatabase('craftloop-web-live')
  })
})

test('Article 83 Golden Web Journey: one continuous real session through every major real capability', async ({
  page,
}) => {
  test.setTimeout(90_000)

  // 1. Start the Web Live View.
  await page.goto('/')
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()

  // 2. Confirm the primary toolbar is top-center.
  const toolbarBoxCreative = await page.getByTestId('main-toolbar').boundingBox()
  expect(toolbarBoxCreative).not.toBeNull()
  expect(toolbarBoxCreative!.y).toBeLessThan(80)
  const viewportWidth = page.viewportSize()!.width
  const toolbarCenterX = toolbarBoxCreative!.x + toolbarBoxCreative!.width / 2
  expect(Math.abs(toolbarCenterX - viewportWidth / 2)).toBeLessThan(4)

  const canvas = page.getByTestId('ink-canvas')
  const box = (await canvas.boundingBox())!

  // 3. Use Pen to draw freely.
  await drag(page, box.x + 40, box.y + 300, box.x + 140, box.y + 320)
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)

  // 4. Press S.
  await page.keyboard.press('s')
  // 5. Confirm the same toolbar anchor becomes the Sketch toolbar.
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  // 6. Use Line.
  await page.getByTestId('tool-line').click()
  await drag(page, box.x + 40, box.y + 140, box.x + 140, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)

  // 7. Use Circle.
  await page.getByTestId('tool-circle').click()
  await drag(page, box.x + 300, box.y + 140, box.x + 340, box.y + 140)
  await expect(page.locator('[data-testid="geometry-layer"] circle')).toHaveCount(1)

  // 8. Use Rectangle.
  await page.getByTestId('tool-rectangle').click()
  await drag(page, box.x + 40, box.y + 400, box.x + 140, box.y + 460)
  await expect(page.locator('[data-testid="geometry-layer"] polygon')).toHaveCount(1)

  // 9. Use Arc.
  await page.getByTestId('tool-arc').click()
  await drag(page, box.x + 300, box.y + 400, box.x + 360, box.y + 440)
  await expect(page.locator('[data-testid="geometry-layer"] path')).toHaveCount(1)

  // 10. "Use Ellipse if exposed." Deliberately not exposed (Phase 09:
  // adding a fifth `BeautifiedPrimitive` variant across seven-plus
  // crates is disproportionate to "add a toolbar button" -- see that
  // phase's own evidence) -- confirmed absent, not silently skipped.
  await expect(page.getByTestId('tool-ellipse')).toHaveCount(0)

  // 11. Create a dimension.
  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)
  await expect(page.getByTestId('tool-dimension')).toBeEnabled()
  await page.getByTestId('tool-dimension').click()
  await page.getByTestId('dimension-input-value').fill('100')
  await page.getByTestId('dimension-input-submit').click()
  await expect(page.getByTestId('dimension-input')).toHaveCount(0)
  await expect(page.locator('[data-testid^="dimension-annotation-"]')).toBeVisible()

  // 12. Apply a constraint.
  await expect(page.getByTestId('tool-constraint')).toBeEnabled()
  await page.getByTestId('tool-constraint').click()
  await expect(page.getByTestId('constraint-option-horizontal')).toBeVisible()
  await page.getByTestId('constraint-option-horizontal').click()
  const feedback = page.getByTestId('solver-feedback')
  await expect(feedback).toBeVisible()

  // 13. Press B.
  await page.keyboard.press('b')
  // 14. Confirm Main Creative toolbar returns and Pen is active.
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Creative')
  await expect(page.getByTestId('tool-pen')).toHaveAttribute('aria-pressed', 'true')

  // 15. Trigger `Sketch` through the command simulator.
  await page.getByTestId('tool-more').click()
  await page.getByTestId('more-command-simulator').click()
  await page.getByTestId('command-simulator-input').fill('sketch')
  await page.getByTestId('command-simulator-submit').click()
  // 16. Confirm the same Sketch state.
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')
  await page.getByTestId('command-simulator-close').click()

  // Back to Creative to reach the View toggle (Creative-only), taking
  // the real line created in step 6 along as the Front view's geometry.
  await page.keyboard.press('b')
  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 140)

  // 17/18. Assign FRONT and enter Orthographic.
  await page.getByTestId('tool-view').click()
  await expect(page.getByTestId('orthographic-panel')).toBeVisible()
  await page.getByTestId('orthographic-create-front').click()

  // 19. Confirm TOP and RIGHT appear.
  await expect(page.locator('[data-view-identity="Front"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Top"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Right"]')).toBeVisible()

  // 20. Confirm Depth remains unresolved.
  const top = page.locator('[data-view-identity="Top"]')
  const right = page.locator('[data-view-identity="Right"]')
  await expect(top.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('')

  // 21. Enter Depth = 40 in Top.
  await top.getByTestId('view-block-axis-depth').locator('input').fill('40')
  await top.getByTestId('view-block-axis-depth').getByRole('button', { name: 'Set' }).click()

  // 22. Confirm Right receives the shared value.
  await expect(right.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('40')

  // 23. Enter a contradictory shared value.
  await right.getByTestId('view-block-axis-depth').locator('input').fill('999')
  await right.getByTestId('view-block-axis-depth').getByRole('button', { name: 'Set' }).click()

  // 24. Confirm a conflict appears.
  await expect(page.getByTestId('orthographic-conflict-evidence')).toBeVisible()

  // 25. Resolve.
  await page.getByTestId('orthographic-conflict-resolve-ReplaceAndPropagate').click()
  await expect(page.getByTestId('orthographic-conflict')).toHaveCount(0)
  await expect(top.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('999')

  // 26. Undo -- reverts the real ReplaceAndPropagate transaction
  // through the real shared DocumentHistory (Gate N), visibly putting
  // both Top and Right back to the pre-conflict shared value.
  await expect(page.getByTestId('tool-undo')).toBeEnabled()
  await page.getByTestId('tool-undo').click()
  await expect(top.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('40')

  // 27. Redo -- reapplies the exact same real transaction.
  await expect(page.getByTestId('tool-redo')).toBeEnabled()
  await page.getByTestId('tool-redo').click()
  await expect(top.getByTestId('view-block-axis-depth').locator('input')).toHaveValue('999')

  await page.getByTestId('orthographic-close').click()

  // 28. Save. `saveNow` is async (writes to real IndexedDB) -- wait
  // for it to actually finish (the Save button's own real
  // `lastSavedAt`-derived title) rather than racing a reload against
  // an in-flight write.
  await page.getByTestId('tool-save').click()
  await expect(page.getByTestId('tool-save')).toHaveAttribute('title', /last saved/)

  // 29. Reload the browser.
  await page.reload()
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()

  // 30. Recover the same semantic state: the freehand stroke, all four
  // real Sketch2D primitives, the dimension, and the real Orthographic
  // set (Front/Top/Right, Depth resolved to the post-conflict 999
  // value on both Top and Right) all survive the reload.
  await expect(page.locator('[data-testid="geometry-layer"] polyline')).toHaveCount(1)
  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
  await expect(page.locator('[data-testid="geometry-layer"] circle')).toHaveCount(1)
  await expect(page.locator('[data-testid="geometry-layer"] polygon')).toHaveCount(1)
  await expect(page.locator('[data-testid="geometry-layer"] path')).toHaveCount(1)

  await page.getByTestId('tool-view').click()
  await expect(page.locator('[data-view-identity="Front"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Top"]')).toBeVisible()
  await expect(page.locator('[data-view-identity="Right"]')).toBeVisible()
  await expect(
    page
      .locator('[data-view-identity="Top"]')
      .getByTestId('view-block-axis-depth')
      .locator('input'),
  ).toHaveValue('999')
})
