import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Locator,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import { PLAYWRIGHT_TIMEOUT, PLAYWRIGHT_TEST_TIMEOUT } from "$lib/test-helpers";

const DEBUG_LOGGING = false;

describe("Database View", () => {
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
    if (DEBUG_LOGGING) {
      page.on("console", (msg) => {
        console.log(msg);
      });
    }
  });

  const getFrame = async (url: string) => {
    await page.goto(url);
    await page
      .locator("button[title='Hide addons [A]']")
      .dispatchEvent("click");

    const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
    if (!maybeFrame) {
      throw new Error("Could not find Storybook iframe");
    }
    return maybeFrame;
  };

  const getScrollElement = async (url: string) => {
    const frame = await getFrame(url);
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
      "a:nth-last-child(2) .blurb.instance .text-container",
    );
    await expect(lastMessageContainer).toHaveText(expectedValue, {
      timeout: PLAYWRIGHT_TIMEOUT,
    });
  };

  test(
    "loads more messages when scrolled to end",
    async () => {
      const { apiCallsScrollElement } = await getScrollElement(
        `http://localhost:6006/?path=/story/screens-database-list--full`,
      );
      await expectLastMessage(apiCallsScrollElement, "Mocking number 10.");

      await apiCallsScrollElement.evaluate((el) => {
        el.scrollTop = el.scrollHeight;
      });
      await expectLastMessage(apiCallsScrollElement, "Mocking number 0.");
    },
    { retry: 2, timeout: PLAYWRIGHT_TEST_TIMEOUT },
  );

  test(
    "updates title when changing dropdown",
    async () => {
      const frame = await getFrame(
        `http://localhost:6006/?path=/story/screens-database-list--full-page`,
      );
      await expect(frame.locator("h2")).toHaveText("LLM API Calls", {
        timeout: PLAYWRIGHT_TIMEOUT,
      });

      await frame.locator("select").selectOption("Terminal Sessions");
      await expect(frame.locator("h2")).toHaveText("Terminal Sessions", {
        timeout: PLAYWRIGHT_TIMEOUT,
      });
    },
    { retry: 2, timeout: PLAYWRIGHT_TEST_TIMEOUT },
  );
});
