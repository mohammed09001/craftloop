import { expect, test } from '@playwright/test'

test('Web Live View opens and shows the canvas shell and toolbar anchor', async ({
  page,
}) => {
  await page.goto('/')

  await expect(page).toHaveTitle(/Craft Loop/)
  await expect(page.getByTestId('canvas-area')).toBeVisible()
  await expect(page.getByTestId('toolbar-host')).toBeVisible()
})
