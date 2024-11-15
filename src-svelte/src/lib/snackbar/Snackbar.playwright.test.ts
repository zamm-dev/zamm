import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import {
  PLAYWRIGHT_TIMEOUT,
  PLAYWRIGHT_TEST_TIMEOUT,
  getStorybookFrame,
} from "$lib/test-helpers";

describe("Snackbar", () => {
  let page: Page;
  let browser: Browser;
  let context: BrowserContext;

  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    context.setDefaultTimeout(PLAYWRIGHT_TIMEOUT);
  });

  afterAll(async () => {
    await browser.close();
  });

  beforeEach(async () => {
    page = await context.newPage();
    page.setDefaultTimeout(PLAYWRIGHT_TIMEOUT);
  });

  test(
    "should hide a message if the dismiss button is clicked",
    { timeout: PLAYWRIGHT_TEST_TIMEOUT },
    async () => {
      const frame = await getStorybookFrame(
        page,
        `http://localhost:6006/?path=/story/layout-snackbar--default`,
      );
      const newErrorButton = frame.locator("button:has-text('Show Error')");
      await newErrorButton.click();

      const errorAlert = frame.locator("div[role='alertdialog']");
      const dismissButton = errorAlert.locator("button[title='Dismiss']");
      await dismissButton.click();
      // timeout here is much shorter because otherwise the test may pass due to the
      // alert going away naturally
      await expect(errorAlert).toHaveCount(0, { timeout: 1000 });
    },
  );
});
