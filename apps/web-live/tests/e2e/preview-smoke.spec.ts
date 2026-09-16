import { expect, test } from '@playwright/test'

/**
 * Execution 03, Phase 17, Task 140: "load the deployed URL and
 * exercise a basic real-core flow." This suite's other specs already
 * run against a real, locally-built `npm run preview` server
 * (`playwright.config.ts`'s `webServer`) -- this file is specifically
 * for a *real deployed* preview URL (Vercel/Cloudflare Pages/etc,
 * Task 138/139), which this repository does not have configured
 * (README.md's own "Deploying a preview" section: real deployment
 * needs account credentials this environment does not have).
 *
 * Skips entirely unless `PREVIEW_URL` is set, so it never fabricates
 * or guesses a URL and never fails a normal local/CI run. Once a real
 * preview exists:
 *
 *   PREVIEW_URL=https://your-real-preview.example npx playwright test tests/e2e/preview-smoke.spec.ts
 *
 * (No `webServer` is started for this file -- it talks to the real
 * remote URL directly.)
 */

const previewUrl = process.env['PREVIEW_URL']

test.skip(!previewUrl, 'PREVIEW_URL is not set -- no real deployed preview to test against.')

test('a real deployed preview opens and can draw a real primitive through the real WASM engine', async ({
  page,
}) => {
  await page.goto(previewUrl!)
  await expect(page.getByTestId('main-toolbar')).toBeVisible()
  await expect(page.locator('[data-session-ready="true"]')).toBeAttached()

  await page.keyboard.press('s')
  await expect(page.getByTestId('main-toolbar')).toHaveAttribute('data-workspace-mode', 'Sketch2D')

  const canvas = page.getByTestId('ink-canvas')
  const box = await canvas.boundingBox()
  if (!box) throw new Error('ink canvas has no bounding box')

  await page.getByTestId('tool-line').click()
  await page.mouse.move(box.x + 40, box.y + 140)
  await page.mouse.down()
  await page.mouse.move(box.x + 140, box.y + 140, { steps: 8 })
  await page.mouse.up()

  await expect(page.locator('[data-testid="geometry-layer"] line')).toHaveCount(1)
})
