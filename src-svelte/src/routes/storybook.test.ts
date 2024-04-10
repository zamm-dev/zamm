import {
  type Browser,
  webkit,
  type Page,
  type BrowserContext,
  type Frame,
} from "@playwright/test";
import {
  afterAll,
  beforeAll,
  afterEach,
  beforeEach,
  describe,
  test,
} from "vitest";
import {
  toMatchImageSnapshot,
  type MatchImageSnapshotOptions,
} from "jest-image-snapshot";
import * as fs from "fs/promises";
import sizeOf from "image-size";

const DEFAULT_TIMEOUT =
  process.env.PLAYWRIGHT_TIMEOUT === undefined
    ? 9_000
    : parseInt(process.env.PLAYWRIGHT_TIMEOUT);

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
  assertDynamic?: boolean;
  resizeWindow?: boolean;
  additionalAction?: (frame: Frame, page: Page) => Promise<void>;
}

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
    variants: ["regular"],
  },
  {
    path: ["reusable", "infobox"],
    variants: ["regular"],
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
    variants: ["dashboard-selected", "settings-selected", "credits-selected"],
  },
  {
    path: ["layout", "snackbar"],
    variants: ["message"],
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
        resizeWindow: true,
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
          await frame.click('button:has-text("Send")');
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
    path: ["screens", "llm-call", "individual"],
    variants: [
      {
        name: "narrow",
        resizeWindow: true,
      },
      "wide",
      "khmer",
    ],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "llm-call", "list"],
    variants: ["empty", "small", "full"],
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

interface StorybookTestContext {
  page: Page;
}

describe.concurrent("Storybook visual tests", () => {
  let browser: Browser;
  let browserContext: BrowserContext;

  beforeAll(async () => {
    browser = await webkit.launch({ headless: true });
    console.log(`Running tests with Webkit version ${browser.version()}`);
    browserContext = await browser.newContext();
    browserContext.setDefaultTimeout(DEFAULT_TIMEOUT);

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
    await browserContext.close();
    await browser.close();
  });

  beforeEach<StorybookTestContext>(async (context) => {
    context.page = await browserContext.newPage();
    context.expect.extend({ toMatchImageSnapshot });
  });

  afterEach<StorybookTestContext>(async (context) => {
    if (context.task.result?.state === "pass") {
      await context.page.close();
    }
  });

  const takeScreenshot = async (
    frame: Frame,
    page: Page,
    resizeWindow: boolean,
    screenshotEntireBody?: boolean,
  ) => {
    let locatorStr = screenshotEntireBody
      ? "body"
      : "#storybook-root > :first-child";
    const elementClass = await frame.locator(locatorStr).getAttribute("class");
    if (elementClass?.includes("storybook-wrapper")) {
      locatorStr = ".storybook-wrapper > :first-child > :first-child";
    }
    const elementLocator = frame.locator(locatorStr);
    await elementLocator.waitFor({ state: "visible" });

    if (resizeWindow) {
      const currentViewport = page.viewportSize();
      if (currentViewport === null) {
        throw new Error("Viewport is null");
      }

      const elementHeight = await elementLocator.evaluate(
        (el) => el.clientHeight,
      );
      const storybookHeight = 60; // height taken up by Storybook elements
      const effectiveViewportHeight = currentViewport.height - storybookHeight;
      const extraHeightNeeded = elementHeight - effectiveViewportHeight;

      if (extraHeightNeeded > 0) {
        await page.setViewportSize({
          width: currentViewport.width,
          height: currentViewport.height + extraHeightNeeded,
        });
      }
    }

    return await elementLocator.screenshot();
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
      test<StorybookTestContext>(
        `${testName} should render the same`,
        async ({ expect, page }) => {
          const variantPrefixStr = variantConfig.prefix ?? variantConfig.name;
          const variantPrefix = `--${variantPrefixStr}`;

          await page.goto(
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );
          // hide bottom drawer
          await page.locator("button[title='Hide addons [A]']").click();
          // wait for fonts to load
          await page.evaluate(() => document.fonts.ready);
          // wait for images to load
          const imagesLocator = page.locator("//img");
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

          const frame = page.frame({ name: "storybook-preview-iframe" });
          if (!frame) {
            throw new Error("Could not find Storybook iframe");
          }

          if (variantConfig.additionalAction) {
            await variantConfig.additionalAction(frame, page);
          }

          const screenshot = await takeScreenshot(
            frame,
            page,
            variantConfig.resizeWindow ?? false,
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
              variantConfig.resizeWindow ?? false,
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
        },
        {
          retry: 1,
          timeout: DEFAULT_TIMEOUT * 2.2,
        },
      );
    }
  }
});
