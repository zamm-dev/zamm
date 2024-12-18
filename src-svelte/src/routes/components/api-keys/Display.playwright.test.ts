import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Frame,
  type Page,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import {
  PLAYWRIGHT_TIMEOUT,
  PLAYWRIGHT_TEST_TIMEOUT,
  getStorybookFrame,
} from "$lib/test-helpers";

describe("API keys display (animated)", () => {
  let page: Page;
  let browser: Browser;
  let context: BrowserContext;
  const openAiRowLocator = "div.row:has-text('OpenAI')";

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

  async function toggleOpenAIForm(frame: Frame) {
    await frame.click(openAiRowLocator);
  }

  test(
    "can open and close form",
    { timeout: PLAYWRIGHT_TEST_TIMEOUT },
    async () => {
      const frame = await getStorybookFrame(
        page,
        // eslint-disable-next-line max-len
        `http://localhost:6006/?path=/story/screens-dashboard-api-keys-display--fast`,
      );
      const form = frame.locator("form");
      await expect(form).not.toBeVisible();

      await toggleOpenAIForm(frame);
      await expect(form).toBeVisible();

      await toggleOpenAIForm(frame);
      await expect(form).not.toBeVisible();
    },
  );

  test(
    "preserves unsubmitted changes after opening and closing form",
    { timeout: PLAYWRIGHT_TEST_TIMEOUT },
    async () => {
      const frame = await getStorybookFrame(
        page,
        // eslint-disable-next-line max-len
        `http://localhost:6006/?path=/story/screens-dashboard-api-keys-display--fast`,
      );
      const testApiKeyInput = "0p3n41-4p1-k3y";
      const testFileInput = "/home/different/.bashrc";
      const form = frame.locator("form");
      const apiKeyInput = form.locator("input[name='apiKey']");
      const fileInput = form.locator("input[name='saveKeyLocation']");

      // open form and check that fields have default values
      await toggleOpenAIForm(frame);
      await expect(apiKeyInput).toHaveValue("");
      await expect(fileInput).toHaveValue(".bashrc");

      // fill in fields and check that values have changed
      await apiKeyInput.fill(testApiKeyInput);
      await expect(apiKeyInput).toHaveValue(testApiKeyInput);
      await fileInput.fill(testFileInput);
      await expect(fileInput).toHaveValue(testFileInput);

      // close form and check that form is hidden
      await toggleOpenAIForm(frame);
      await expect(apiKeyInput).not.toBeVisible();
      await expect(fileInput).not.toBeVisible();

      // reopen form and check that values are preserved
      await toggleOpenAIForm(frame);
      await expect(apiKeyInput).toHaveValue(testApiKeyInput);
      await expect(fileInput).toHaveValue(testFileInput);
    },
  );
});
