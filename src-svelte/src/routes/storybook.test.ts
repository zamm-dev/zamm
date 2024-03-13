import {
  type Browser,
  webkit,
  type Page,
  type BrowserContext,
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
  variants: string[] | VariantConfig[];
  screenshotEntireBody?: boolean;
}

interface VariantConfig {
  name: string;
  assertDynamic?: boolean;
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
      "full-message-width",
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

  const takeScreenshot = async (page: Page, screenshotEntireBody?: boolean) => {
    const frame = page.frame({ name: "storybook-preview-iframe" });
    if (!frame) {
      throw new Error("Could not find Storybook iframe");
    }
    let locator = screenshotEntireBody
      ? "body"
      : "#storybook-root > :first-child";
    const elementClass = await frame.locator(locator).getAttribute("class");
    if (elementClass?.includes("storybook-wrapper")) {
      locator = ".storybook-wrapper > :first-child > :first-child";
    }
    return await frame.locator(locator).screenshot();
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
          const variantPrefix = `--${variantConfig.name}`;

          await page.goto(
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );
          // wait for fonts to load
          await page.locator("button[title='Hide addons [A]']").click();
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

          const screenshot = await takeScreenshot(
            page,
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
              page,
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
