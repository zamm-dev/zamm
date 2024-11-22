import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Locator,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import {
  PLAYWRIGHT_TIMEOUT,
  PLAYWRIGHT_TEST_TIMEOUT,
  getStorybookFrame,
} from "$lib/test-helpers";

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

  const getScrollElement = async (url: string) => {
    const frame = await getStorybookFrame(page, url);
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
    { retry: 2, timeout: PLAYWRIGHT_TEST_TIMEOUT },
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
  );

  test(
    "updates title and links when changing dropdown",
    { retry: 2, timeout: PLAYWRIGHT_TEST_TIMEOUT },
    async () => {
      const frame = await getStorybookFrame(
        page,
        `http://localhost:6006/?path=/story/screens-database-list--full-page`,
      );
      await expect(frame.locator("h2")).toHaveText("LLM API Calls", {
        timeout: PLAYWRIGHT_TIMEOUT,
      });
      const linkToApiCall = frame.locator("a:has-text('Mocking number 59.')");
      await expect(linkToApiCall).toHaveAttribute(
        "href",
        "/database/api-calls/d5ad1e49-f57f-4481-84fb-4d70ba8a7a59/",
      );

      await frame.locator("select").selectOption("Terminal Sessions");
      await expect(frame.locator("h2")).toHaveText("Terminal Sessions", {
        timeout: PLAYWRIGHT_TIMEOUT,
      });
      const linkToTerminalSession = frame.locator("a:has-text('python api')");
      await expect(linkToTerminalSession).toHaveAttribute(
        "href",
        "/database/terminal-sessions/3717ed48-ab52-4654-9f33-de5797af5118/",
      );
    },
  );
});
