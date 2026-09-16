import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 16 real-browser proof (Tasks 128-134): the
 * audit tasks are only "done" if the thing they audit is actually
 * checked against the real rendered page, not reasoned about in the
 * abstract -- each test here reproduces a concrete finding (or proves
 * the absence of one) rather than restating the task list.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/')
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()
})

test('the toolbar stays fully on-screen at a phone viewport instead of overflowing both edges (Task 134)', async ({
  page,
}) => {
  await page.setViewportSize({ width: 375, height: 667 })
  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  const toolbarBox = await page.getByTestId('main-toolbar').boundingBox()
  expect(toolbarBox).not.toBeNull()
  expect(toolbarBox!.x).toBeGreaterThanOrEqual(0)
  expect(toolbarBox!.x + toolbarBox!.width).toBeLessThanOrEqual(375)

  // Every real tool button (there is no clutter to remove -- Task
  // 128 -- everything present is a real, functional control) is still
  // individually reachable, just wrapped onto more than one row.
  await expect(page.getByTestId('tool-construction')).toBeVisible()
  await expect(page.getByTestId('tool-more')).toBeVisible()
})

test('the toolbar also stays top-center and on-screen at tablet and desktop widths (Task 134)', async ({
  page,
}) => {
  for (const width of [768, 1280]) {
    await page.setViewportSize({ width, height: 800 })
    const toolbarBox = await page.getByTestId('main-toolbar').boundingBox()
    expect(toolbarBox).not.toBeNull()
    expect(toolbarBox!.x).toBeGreaterThanOrEqual(0)
    expect(toolbarBox!.x + toolbarBox!.width).toBeLessThanOrEqual(width)
    expect(toolbarBox!.y).toBeLessThan(80)
  }
})

test('a pressed toggle has a real non-color indicator, not just a background/border tint (Task 133)', async ({
  page,
}) => {
  await page.keyboard.press('s')
  const snapButton = page.getByTestId('tool-snap')
  await expect(snapButton).toHaveAttribute('aria-pressed', 'true')

  const contentWhilePressed = await snapButton.evaluate(
    (el) => getComputedStyle(el, '::after').content,
  )
  expect(contentWhilePressed).not.toBe('none')

  // The dot only renders while pressed -- toggle off and confirm the
  // marker disappears too, proving it tracks the real state rather
  // than being permanently present.
  await snapButton.click()
  await expect(snapButton).toHaveAttribute('aria-pressed', 'false')
  const hasDotAfter = await snapButton.evaluate((el) => getComputedStyle(el, '::after').content)
  expect(hasDotAfter).toBe('none')
})

test('real transient outcomes are announced to assistive tech, not only shown visually (Task 133)', async ({
  page,
}) => {
  const canvas = page.getByTestId('ink-canvas')
  await page.keyboard.press('s')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await page.mouse.move(box.x + 40, box.y + 140)
  await page.mouse.down()
  await page.mouse.move(box.x + 140, box.y + 190, { steps: 8 })
  await page.mouse.up()

  await page.getByTestId('tool-select').click()
  await page.mouse.click(box.x + 90, box.y + 165)
  await page.getByTestId('tool-constraint').click()
  await page.getByTestId('constraint-option-horizontal').click()

  const feedback = page.getByTestId('solver-feedback')
  await expect(feedback).toHaveAttribute('role', 'status')
  await expect(feedback).toHaveAttribute('aria-live', 'polite')
})

test('every toolbar button has a real accessible name, focus is keyboard-reachable and visible (Task 133)', async ({
  page,
}) => {
  for (const id of ['pen', 'select', 'eraser', 'sketch', 'view', 'undo', 'redo', 'save', 'more']) {
    const button = page.getByTestId(`tool-${id}`)
    await expect(button).toHaveAccessibleName(/.+/)
  }

  await page.getByTestId('tool-pen').focus()
  await expect(page.getByTestId('tool-pen')).toBeFocused()
  const outline = await page
    .getByTestId('tool-pen')
    .evaluate((el) => getComputedStyle(el).outlineStyle)
  expect(outline).not.toBe('none')
})
