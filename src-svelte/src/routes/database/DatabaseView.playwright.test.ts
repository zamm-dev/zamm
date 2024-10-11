import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Frame,
  type Locator,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import { PLAYWRIGHT_TIMEOUT, PLAYWRIGHT_TEST_TIMEOUT } from "$lib/test-helpers";

const DEBUG_LOGGING = false;

describe("Api Calls endless scroll test", () => {
  let page: Page;
  let frame: Frame;
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
    if (DEBUG_LOGGING) {
      page.on("console", (msg) => {
        console.log(msg);
      });
    }
  });

  const getScrollElement = async () => {
    const url = `http://localhost:6006/?path=/story/screens-database-list--full`;
    await page.goto(url);
    await page
      .locator("button[title='Hide addons [A]']")
      .dispatchEvent("click");

    const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
    if (!maybeFrame) {
      throw new Error("Could not find Storybook iframe");
    }
    frame = maybeFrame;

    const apiCallsScrollElement = frame.locator(".scroll-contents");
    return { apiCallsScrollElement };
  };

  const expectLastMessage = async (
    apiCallsScrollElement: Locator,
    expectedValue: string,
  ) => {
    // the actual last child is the bottom indicator that triggers the shadow and the
    // auto-load
    const lastMessageContainer = apiCallsScrollElement.locator(
      "a:nth-last-child(2) .message.instance .text-container",
    );
    await expect(lastMessageContainer).toHaveText(expectedValue, {
      timeout: PLAYWRIGHT_TIMEOUT,
    });
  };

  test(
    "loads more messages when scrolled to end",
    async () => {
      const { apiCallsScrollElement } = await getScrollElement();
      await expectLastMessage(apiCallsScrollElement, "Mocking number 10.");

      await apiCallsScrollElement.evaluate((el) => {
        el.scrollTop = el.scrollHeight;
      });
      await expectLastMessage(apiCallsScrollElement, "Mocking number 0.");
    },
    { retry: 2, timeout: PLAYWRIGHT_TEST_TIMEOUT },
  );
});
