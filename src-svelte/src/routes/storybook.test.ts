import {
  type Browser,
  chromium,
  webkit,
  type Page,
  type BrowserContext,
  type Frame,
} from "@playwright/test";
import { afterAll, beforeAll, beforeEach, describe, test } from "vitest";
import {
  toMatchImageSnapshot,
  type MatchImageSnapshotOptions,
} from "jest-image-snapshot";
import * as fs from "fs/promises";
import { existsSync } from "fs";
import sizeOf from "image-size";
import {
  PLAYWRIGHT_TIMEOUT,
  PLAYWRIGHT_TEST_TIMEOUT,
  getStorybookFrame,
} from "$lib/test-helpers";

const NARROW_WINDOW_SIZE = 675;

const SCREENSHOTS_BASE_DIR =
  process.env.SCREENSHOTS_BASE_DIR === undefined
    ? "screenshots"
    : process.env.SCREENSHOTS_BASE_DIR;

interface ComponentTestConfig {
  path: string[]; // Represents the Storybook hierarchy path
  variants: (string | VariantConfig)[];
  screenshotEntireBody?: boolean;
}

interface VariantConfig {
  name: string;
  prefix?: string;
  browser?: "webkit" | "chromium";
  selector?: string;
  assertDynamic?: boolean;
  narrowWindow?: boolean;
  tallWindow?: boolean;
  extraTimeout?: number;
  additionalAction?: (frame: Frame, page: Page) => Promise<void>;
}

const waitForSidebarHeight = async (frame: Frame) => {
  await frame.waitForFunction(() => {
    // note: we can't seem to make use of local variables outside of the
    // function, possibly because it's executed in the browser
    const header = document.querySelector("header");
    const headerDimensions = header?.getBoundingClientRect();
    return headerDimensions?.height && headerDimensions.height > 300;
  });
};

const components: ComponentTestConfig[] = [
  {
    path: ["reusable", "switch"],
    variants: ["on", "off"],
    screenshotEntireBody: true,
  },
  {
    path: ["reusable", "slider"],
    variants: [
      "tiny-phone-screen",
      "tiny-phone-screen-with-long-label",
      "tablet",
    ],
    screenshotEntireBody: true,
  },
  {
    path: ["reusable", "button"],
    variants: ["regular", "disabled"],
  },
  {
    path: ["reusable", "button", "group"],
    variants: ["wide", "narrow"],
  },
  {
    path: ["reusable", "infobox"],
    variants: [
      "regular",
      {
        name: "transparent",
        browser: "chromium",
        additionalAction: async (_) => {
          // wait for background to load; probably due to fonts
          await new Promise((r) => setTimeout(r, 2_000));
        },
      },
    ],
    screenshotEntireBody: true,
  },
  {
    path: ["reusable", "external-link"],
    variants: ["external-link"],
  },
  {
    path: ["reusable", "scrollable", "fixed"],
    variants: ["top", "bottom"],
  },
  {
    path: ["reusable", "scrollable", "growable"],
    variants: ["small", "large"],
  },
  {
    path: ["reusable", "warning"],
    variants: ["short", "long"],
  },
  {
    path: ["layout", "background"],
    variants: [
      {
        name: "static",
        assertDynamic: false,
      },
      {
        name: "dynamic",
        assertDynamic: true,
      },
    ],
  },
  {
    path: ["layout", "app"],
    variants: ["static"],
  },
  {
    path: ["layout", "sidebar"],
    variants: [
      {
        name: "dashboard-selected",
        additionalAction: waitForSidebarHeight,
      },
      {
        name: "settings-selected",
        additionalAction: waitForSidebarHeight,
      },
      {
        name: "credits-selected",
        additionalAction: waitForSidebarHeight,
      },
    ],
  },
  {
    path: ["layout", "snackbar", "message"],
    variants: ["error", "info"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "dashboard", "api-keys-display"],
    variants: ["loading", "unknown", "known", "editing", "editing-pre-filled"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "dashboard", "metadata"],
    variants: ["loading", "loaded"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "settings"],
    variants: ["tiny-phone-screen", "large-phone-screen", "tablet"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "chat", "message"],
    variants: [
      "human",
      "ai",
      "ai-multiline",
      "human-code",
      "highlighted-human-code",
      "ai-code",
      "highlighted-ai-code",
      "mixed-khmer-and-english",
    ],
  },
  {
    path: ["screens", "chat", "conversation"],
    variants: [
      "empty",
      "not-empty",
      "multiline-chat",
      "extra-long-input",
      "bottom-scroll-indicator",
      "typing-indicator-static",
      {
        name: "full-message-width",
        tallWindow: true,
        additionalAction: async (frame: Frame, page: Page) => {
          await new Promise((r) => setTimeout(r, 1000));
          // need to do a manual scroll because Storybook resize messes things up on CI
          const scrollContents = frame.locator(".scroll-contents");
          await scrollContents.focus();
          await page.keyboard.press("End");
        },
      },
      {
        name: "new-message-sent",
        prefix: "extra-long-input",
        additionalAction: async (frame: Frame) => {
          await frame.click('button[aria-label="Send"]');
          await frame.click('button[title="Dismiss"]');
        },
      },
    ],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "credits", "creditor"],
    variants: [
      "regular",
      "github-contributor",
      "typodermic-font",
      "dependency-with-icon",
    ],
  },
  {
    path: ["screens", "database", "llm-call", "new"],
    variants: ["blank", "edit-continued-conversation", "busy", "with-emoji"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "database", "llm-call", "import"],
    variants: ["static"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "database", "llm-call"],
    variants: [
      {
        name: "narrow",
        prefix: "regular",
        narrowWindow: true,
        tallWindow: true,
      },
      {
        name: "wide",
        prefix: "regular",
        tallWindow: true,
      },
      "khmer",
      {
        name: "lots-of-code",
        tallWindow: true,
      },
      "variant",
      "unknown-provider-prompt",
    ],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "database", "llm-call", "actions"],
    variants: ["wide"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "database", "list"],
    variants: ["empty", "small", "full"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "database", "terminal-session"],
    variants: ["new", "in-progress", "finished"],
    screenshotEntireBody: true,
  },
];

async function findVariantFiles(
  directoryPath: string,
  filePrefix: string,
): Promise<string[]> {
  try {
    await fs.access(directoryPath);
  } catch (_) {
    return [];
  }

  const files = await fs.readdir(directoryPath);
  return files
    .filter((file) => {
      return (
        file.startsWith(filePrefix) && file.match(/-variant-\d+\.png$/) !== null
      );
    })
    .map((file) => `${directoryPath}/${file}`);
}

async function checkVariants(variantFiles: string[], screenshot: Buffer) {
  for (const file of variantFiles) {
    const fileBuffer = await fs.readFile(file);
    if (Buffer.compare(fileBuffer, screenshot) === 0) {
      return true;
    }
  }
  return false;
}

describe.concurrent("Storybook visual tests", () => {
  let webkitBrowser: Browser;
  let webkitBrowserContext: BrowserContext;
  let chromiumBrowser: Browser;
  let chromiumBrowserContext: BrowserContext;

  beforeAll(async () => {
    webkitBrowser = await webkit.launch({ headless: true });
    webkitBrowserContext = await webkitBrowser.newContext();
    webkitBrowserContext.setDefaultTimeout(PLAYWRIGHT_TIMEOUT);

    chromiumBrowser = await chromium.launch({ headless: true });
    chromiumBrowserContext = await chromiumBrowser.newContext();
    chromiumBrowserContext.setDefaultTimeout(PLAYWRIGHT_TIMEOUT);

    console.log(
      `Running tests with Webkit v${webkitBrowser.version()} and ` +
        `Chromium v${chromiumBrowser.version()}`,
    );

    try {
      await fs.rm(`${SCREENSHOTS_BASE_DIR}/testing`, {
        recursive: true,
        force: true,
      });
    } catch (e) {
      // ignore, it's okay if the folder already doesn't exist
    }
  });

  afterAll(async () => {
    await webkitBrowserContext.close();
    await webkitBrowser.close();
    await chromiumBrowserContext.close();
    await chromiumBrowser.close();
  });

  beforeEach(async (context) => {
    context.expect.extend({ toMatchImageSnapshot });
  });

  const getScreenshotElement = async (frame: Frame, selector?: string) => {
    if (selector) {
      // override has been specified
      return frame.locator(selector);
    }

    const firstChild = frame.locator("#storybook-root > :first-child");
    if ((await firstChild.getAttribute("id")) === "mock-app-layout") {
      // we're wrapping the story with the MockAppLayoutDecorator
      return frame.locator(
        "#mock-app-layout > #animation-control > :first-child",
      );
    }

    return firstChild;
  };

  const takeScreenshot = async (
    frame: Frame,
    page: Page,
    tallWindow: boolean,
    selector?: string,
    extraTimeout?: number,
    addMargins?: boolean,
  ) => {
    const screenshotMarginPx = addMargins ? 18 : 0;
    const elementLocator = await getScreenshotElement(frame, selector);
    await elementLocator.waitFor({ state: "visible" });

    if (tallWindow) {
      const currentViewport = page.viewportSize();
      if (currentViewport === null) {
        throw new Error("Viewport is null");
      }

      const elementHeight = await elementLocator.evaluate(
        (el) => el.clientHeight,
      );
      // height taken up by Storybook elements, plus a little extra to make sure the
      // screenshot margin is included in the screenshot
      const storybookHeight = 60 + screenshotMarginPx;
      const effectiveViewportHeight = currentViewport.height - storybookHeight;
      const extraHeightNeeded = elementHeight - effectiveViewportHeight;

      if (extraHeightNeeded > 0) {
        await page.setViewportSize({
          width: currentViewport.width,
          height: currentViewport.height + extraHeightNeeded,
        });
      }
    }

    if (extraTimeout) {
      await page.waitForTimeout(extraTimeout);
    }

    // https://github.com/microsoft/playwright/issues/28394#issuecomment-2329352801
    const boundingBox = await elementLocator.boundingBox();
    if (!boundingBox) {
      throw new Error(`No bounding box found for element ${selector}`);
    }

    const documentWidth = await page.evaluate(() => document.body.clientWidth);
    const documentHeight = await page.evaluate(
      () => document.body.clientHeight,
    );

    const scrollX = await page.evaluate(() => window.scrollX);
    const scrollY = await page.evaluate(() => window.scrollY);

    const x = Math.max(scrollX + boundingBox.x - screenshotMarginPx, 0);
    const y = Math.max(scrollY + boundingBox.y - screenshotMarginPx, 0);
    const width = Math.min(
      boundingBox.width + 2 * screenshotMarginPx,
      documentWidth - x,
    );
    const height = Math.min(
      boundingBox.height + 2 * screenshotMarginPx,
      documentHeight - y,
    );

    return await page.screenshot({
      clip: { x, y, width, height },
      fullPage: true,
    });
  };

  const baseMatchOptions: MatchImageSnapshotOptions = {
    comparisonMethod: "ssim",
    failureThreshold: 0.005,
    failureThresholdType: "percent",
    allowSizeMismatch: true,
    storeReceivedOnFailure: true,
    customSnapshotsDir: `${SCREENSHOTS_BASE_DIR}/baseline`,
    customDiffDir: `${SCREENSHOTS_BASE_DIR}/testing/diff`,
    customReceivedDir: `${SCREENSHOTS_BASE_DIR}/testing/actual`,
    customReceivedPostfix: "",
  };

  const makeWindowNarrow = async (page: Page) => {
    const currentViewport = page.viewportSize();
    if (currentViewport === null) {
      throw new Error("Viewport is null");
    }

    await page.setViewportSize({
      width: NARROW_WINDOW_SIZE,
      height: currentViewport.height,
    });
  };

  for (const config of components) {
    const storybookUrl = config.path.join("-");
    const storybookPath = config.path.join("/");
    for (const variant of config.variants) {
      const variantConfig =
        typeof variant === "string"
          ? {
              name: variant,
            }
          : variant;
      const testName = `${storybookPath}/${variantConfig.name}.png`;
      test(
        `${testName} should render the same`,
        {
          retry: 1,
          timeout: PLAYWRIGHT_TEST_TIMEOUT,
        },
        async ({ expect }) => {
          if (process.env.CI === "true" && !variantConfig.assertDynamic) {
            const goldFilePath =
              `${SCREENSHOTS_BASE_DIR}/baseline/${storybookPath}` +
              `/${variantConfig.name}.png`;
            expect(
              existsSync(goldFilePath),
              `No baseline found for ${goldFilePath}`,
            ).toBeTruthy();
          }

          const page =
            variantConfig.browser === "chromium"
              ? await chromiumBrowserContext.newPage()
              : await webkitBrowserContext.newPage();
          const variantPrefixStr = variantConfig.prefix ?? variantConfig.name;
          const variantPrefix = `--${variantPrefixStr}`;

          if (variantConfig.narrowWindow) {
            await makeWindowNarrow(page);
          }

          const frame = await getStorybookFrame(
            page,
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );
          // wait for fonts to load
          await frame.evaluate(() => document.fonts.ready);
          // wait for images to load
          const imagesLocator = frame.locator("//img");
          const images = await imagesLocator.evaluateAll((images) => {
            return images.map((i) => {
              i.scrollIntoView();
              return i as HTMLImageElement;
            });
          });
          const imagePromises = images.map(
            (i) => i.complete || new Promise((f) => (i.onload = f)),
          );
          await Promise.all(imagePromises);

          if (variantConfig.additionalAction) {
            await variantConfig.additionalAction(frame, page);
          }

          const screenshot = await takeScreenshot(
            frame,
            page,
            variantConfig.tallWindow ?? false,
            variantConfig.selector,
            variantConfig.extraTimeout,
            config.screenshotEntireBody,
          );

          const uint8ArrayWorkaround = new Uint8Array(
            screenshot.buffer,
            screenshot.byteOffset,
            screenshot.byteLength,
          );
          const screenshotSize = sizeOf(uint8ArrayWorkaround);
          const diffDirection =
            screenshotSize.width &&
            screenshotSize.height &&
            screenshotSize.width > screenshotSize.height
              ? "vertical"
              : "horizontal";
          const matchOptions = {
            ...baseMatchOptions,
            diffDirection,
            customSnapshotIdentifier: `${storybookPath}/${variantConfig.name}`,
          };

          // don't compare dynamic screenshots against baseline
          if (!variantConfig.assertDynamic) {
            // look for all files in filesystem with suffix -variant-x.png
            const variantFiles = await findVariantFiles(
              `${SCREENSHOTS_BASE_DIR}/baseline/${storybookPath}`,
              variantConfig.name,
            );
            const variantsMatch = await checkVariants(variantFiles, screenshot);
            if (!variantsMatch) {
              // @ts-ignore
              expect(screenshot).toMatchImageSnapshot(matchOptions);
            }
          }

          if (variantConfig.assertDynamic !== undefined) {
            await new Promise((r) => setTimeout(r, 1000));
            const newScreenshot = await takeScreenshot(
              frame,
              page,
              variantConfig.tallWindow ?? false,
              variantConfig.selector,
              variantConfig.extraTimeout,
              config.screenshotEntireBody,
            );

            if (variantConfig.assertDynamic) {
              expect(
                Buffer.compare(screenshot, newScreenshot) !== 0,
              ).toBeTruthy();
            } else {
              // do the same assertion from before so that we can see what changed the
              // second time around if a static screenshot turns out to be dynamic
              //
              // @ts-ignore
              expect(newScreenshot).toMatchImageSnapshot(matchOptions);
            }
          }

          await page.close();
        },
      );
    }
  }
});
