# Setting up Playwright component testing

## Using test components

Follow instructions [here](https://playwright.dev/docs/test-components):

```bash
$ yarn create playwright --ct
yarn create v1.22.19
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
success Installed "create-playwright@1.17.129" with binaries:
      - create-playwright
[#########] 9/9Getting started with writing end-to-end tests with Playwright:
Initializing project in '.'
✔ Which framework do you use? (experimental) · svelte
✔ Install Playwright browsers (can be done manually via 'yarn playwright install')? (Y/n) · true

✔ Install Playwright operating system dependencies (requires sudo / root - can be done manually via 'sudo yarn playwright install-deps')? (y/N) · false


Installing Playwright Component Testing (yarn add --dev @playwright/experimental-ct-svelte)…
yarn add v1.22.19
...
✔ Success! Created a Playwright Test project at /root/zamm/src-svelte

...

We suggest that you begin by typing:

  yarn run test-ct

Visit https://playwright.dev/docs/intro for more information. ✨

Happy hacking! 🎭
Done in 32.47s.
```

Then edit `src-svelte/playwright/index.ts`:

```ts
import '../src/routes/styles.css';
```

And `src-svelte/playwright-ct.config.ts`:

```ts
import { defineConfig, devices } from '@playwright/experimental-ct-svelte';
import * as path from "path";

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: './',
  /* The base directory, relative to the config file, for snapshot files created with toMatchSnapshot and toHaveScreenshot. */
  snapshotDir: './snapshots',
  ...
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    ...

    ctViteConfig: { 
      resolve: {
        alias: {
          $lib: path.resolve("src/lib"),
        },
      },
    },
  },
```

As it turns out, Playwright [cannot currently alias SvelteKit paths](https://github.com/microsoft/playwright/issues/19411). As such, we give up on this endeavor for now.

## Using Storybook

Follow the instructions at [`storybook.md`](./storybook.md). Then

```bash
$ yarn add -D @playwright/test playwright
$ yarn add -D jest-image-snapshot @types/jest-image-snapshot
```

and using the instructions [here](https://www.the-koi.com/projects/how-to-run-playwright-within-vitest/) and [here](https://greif-matthias.medium.com/how-to-setup-visual-testing-with-storybook-jest-and-puppeteer-c489b4f64c21), and the documentation for options [here](https://github.com/americanexpress/jest-image-snapshot), and the fact that Vitest is [already compatible](https://vitest.dev/guide/snapshot.html#image-snapshots) with `jest-image-snapshot`, we create a file `src-svelte/src/routes/storybook.test.ts`:

```ts
import { type Browser, chromium, expect, type Page } from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import { toMatchImageSnapshot } from "jest-image-snapshot";

expect.extend({ toMatchImageSnapshot });

describe("Storybook visual tests", () => {
  let page: Page;
  let browser: Browser;

  const components = {
    "settings-api-keys-display": ["loading", "unknown", "known"],
  };

  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext();
    page = await context.newPage();
  });

  afterAll(async () => {
    await browser.close();
  });

  for (const [component, variants] of Object.entries(components)) {
    describe(component, () => {
      for (const variant of variants) {
        const testName = variant ? variant : component;
        test(`${testName} should render the same`, async () => {
          const variantPrefix = variant ? `--${variant}` : "";

          await page.goto(
            `http://localhost:6006/?path=/story/${component}${variantPrefix}`,
          );

          const frame = page.frame({ name: "storybook-preview-iframe" });
          if (!frame) {
            throw new Error("Could not find Storybook iframe");
          }

          const rootElement = await frame.waitForSelector("#storybook-root");
          const screenshot = await rootElement.screenshot();

          // @ts-ignore
          expect(screenshot).toMatchImageSnapshot({
            diffDirection: "vertical",
            storeReceivedOnFailure: true,
            customSnapshotsDir: "screenshots/baseline",
            customSnapshotIdentifier: `${storybookPath}/${testName}`,
            customDiffDir: "screenshots/testing/diff",
            customReceivedDir: "screenshots/testing/actual",
            customReceivedPostfix: "",
          });
        });
      }
    });
  }
});
```

The `ts-ignore` is to avoid this error:

```
/root/zamm/src-svelte/src/routes/storybook.test.ts:103:30
Error: Property 'toMatchImageSnapshot' does not exist on type 'MakeMatchers<void, Buffer>'. Did you mean 'toMatchSnapshot'? 

          expect(screenshot).toMatchImageSnapshot({
            diffDirection: "vertical",
```

Unlike the example app [here](https://github.com/vitest-dev/vitest/blob/0c13c39/examples/image-snapshot/test/basic.test.ts), copying the module declaration here does not seem to get rid of the TypeScript error. The error is a spurious one because the test itself clearly executes successfully.

Make sure to add a `src-svelte/screenshots/.gitignore` to ignore the new output files. Because we've customized the output directories to be consistent with [WebdriverIO](/zamm/resources/tutorials/setup/tauri/e2e-testing.md), we do

```
testing/
```

If you have instead kept the defaults, then yours should look like:

```
__diff_output__/
__received_output__/
```

Note that we'll have to edit the timeout in `src-svelte/src/lib/__mocks__/invoke.ts` after all for this test -- it appears Storybook behavior is subtly different on different platforms:

```
    if (nextShouldWait) {
      setTimeout(() => {
        resolve(nextResolution);
      }, 1_000_000); // the re-render never happens, so any timeout is fine
    } else {
      resolve(nextResolution);
    }
```

### Automatically starting Storybook before tests

Note that this requires `yarn storybook` to be running first before the tests run. We can automate this by installing

```bash
$ yarn add -D node-fetch
```

We observe that a regular Storybook startup looks like this:

```bash
$ yarn storybook
yarn run v1.22.19
$ storybook dev -p 6006
@storybook/cli v7.4.0

WARN The "@storybook/addon-mdx-gfm" addon is meant as a migration assistant for Storybook 7.0; and will likely be removed in a future version.
WARN It's recommended you read this document:
WARN https://storybook.js.org/docs/react/writing-docs/mdx#lack-of-github-flavored-markdown-gfm
WARN 
WARN Once you've made the necessary changes, you can remove the addon from your package.json and storybook config.
info => Serving static files from ././static at /
info => Starting manager..
WARN No story files found for the specified pattern: src/**/*.mdx
The following Vite config options will be overridden by SvelteKit:
  - base


╭─────────────────────────────────────────────────╮
│                                                 │
│   Storybook 7.4.0 for sveltekit started         │
│   267 ms for manager and 1.06 s for preview     │
│                                                 │
│    Local:            http://localhost:6006/     │
│    On your network:  http://5.78.91.93:6006/    │
│                                                 │
╰─────────────────────────────────────────────────╯
```

Next

```ts
...

import { spawn, ChildProcess } from "child_process";
import fetch from "node-fetch";

...

let storybookProcess: ChildProcess | null = null;

const startStorybook = (): Promise<void> => {
  return new Promise((resolve) => {
    storybookProcess = spawn("yarn", ["storybook", "--ci"]);
    if (!storybookProcess) {
      throw new Error("Could not start storybook process");
    } else if (!storybookProcess.stdout || !storybookProcess.stderr) {
      throw new Error("Could not get storybook output");
    }

    const storybookStartupMessage =
      /Storybook \d+\.\d+\.\d+ for sveltekit started/;

    storybookProcess.stdout.on("data", (data) => {
      const strippedData = data.toString().replace(/\\x1B\[\d+m/g, "");
      if (storybookStartupMessage.test(strippedData)) {
        resolve();
      }
    });

    storybookProcess.stderr.on("data", (data) => {
      console.error(`Storybook error: ${data}`);
    });
  });
};

const checkIfStorybookIsRunning = async (): Promise<boolean> => {
  try {
    await fetch("http://localhost:6006");
    return true;
  } catch {
    return false;
  }
};

describe("Storybook visual tests", () => {
  ...

  beforeAll(async () => {
    const isStorybookRunning = await checkIfStorybookIsRunning();
    if (!isStorybookRunning) {
      await startStorybook();
    }

    ...
  });

  afterAll(async () => {
    await browser.close();

    if (storybookProcess) {
      storybookProcess.kill();
    }
  });

  ...
}
```

The `--ci` option passed to Storybook prevents it from automatically popping a browser window open, which can be annoying when testing repeatedly during development.

Make sure you now update `.github/workflows/tests.yaml` as well. Screenshots should now be uploaded from our new directory:

```yaml
  svelte:
    ...
    steps:
      - name: Upload final app
        if: always() # run even if tests fail
        uses: actions/upload-artifact@v3
        with:
          name: storybook-screenshots
          path: |
            src-svelte/screenshots/testing/**/*.png
          retention-days: 1
```

### Diff direction

In some cases, we want the diff to be vertical when the screenshot is wide, and we want the diff to be horizontal when the screenshot is tall. We install


```bash
$ yarn add -D image-size
```

and then do

```ts
import sizeOf from "image-size";

describe("Storybook visual tests", () => {
  ...
          const screenshotSize = sizeOf(screenshot);
          const diffDirection = (screenshotSize.width && screenshotSize.height && screenshotSize.width > screenshotSize.height) ? "vertical" : "horizontal";

          // @ts-ignore
          expect(screenshot).toMatchImageSnapshot({
            diffDirection,
            ...
          });
  ...
});
```

### Testing dynamic images

Let's say we want to also optionally check that static elements are static, whereas dynamic elements (e.g. animated ones) change. We wouldn't want to do exact screenshot tests because depending on the exact timing of the screenshot, the animation may be in a slightly different place each time. Instead, we take two screenshots and check that they're different. But first, we want to only *optionally* specify this for each test. We do this by defining an optional additional config for each test variant:

```ts
interface ComponentTestConfig {
  path: string[]; // Represents the Storybook hierarchy path
  variants: string[] | VariantConfig[];
}

interface VariantConfig {
  name: string;
  assertDynamic?: boolean;
}
```

Now we change the test components to:

```ts
const components: ComponentTestConfig[] = [
  {
    path: ["background"],
    variants: [{
      name: "static",
      assertDynamic: false,
    }, {
      name: "dynamic",
      assertDynamic: true,
    }],
  },
  ...
]
```

We refactor screenshot-taking to be able to reuse it:

```ts
  const takeScreenshot = (page: Page) => {
    const frame = page.frame({ name: "storybook-preview-iframe" });
          if (!frame) {
            throw new Error("Could not find Storybook iframe");
          }
    return frame
    .locator("#storybook-root > :first-child")
    .screenshot();
  }
```

Then we update our tests as such:

```ts
  for (const config of components) {
    const storybookUrl = config.path.join("-");
    const storybookPath = config.path.join("/");
    describe(storybookPath, () => {
      for (const variant of config.variants) {
        const variantConfig = typeof variant === "string" ? {
          name: variant,
        } : variant;
        const testName = variantConfig.name;
        test(`${testName} should render the same`, async () => {
          const variantPrefix = `--${variantConfig.name}`;

          await page.goto(
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );

          const screenshot = await takeScreenshot(page);

          const screenshotSize = sizeOf(screenshot);
          const diffDirection =
            screenshotSize.width &&
            screenshotSize.height &&
            screenshotSize.width > screenshotSize.height
              ? "vertical"
              : "horizontal";

          if (!variantConfig.assertDynamic) {
            // don't compare dynamic screenshots against baseline
            // @ts-ignore
            expect(screenshot).toMatchImageSnapshot({
              diffDirection,
              storeReceivedOnFailure: true,
              customSnapshotsDir: "screenshots/baseline",
              customSnapshotIdentifier: `${storybookPath}/${testName}`,
              customDiffDir: "screenshots/testing/diff",
              customReceivedDir: "screenshots/testing/actual",
              customReceivedPostfix: "",
            });
          }

          if (variantConfig.assertDynamic !== undefined) {
            await new Promise(r => setTimeout(r, 1000));
            const newScreenshot = await takeScreenshot(page);

            if (variantConfig.assertDynamic) {
              expect(newScreenshot).not.toEqual(screenshot);
            } else {
              // do the same assertion from before so that we can see what changed the
              // second time around if a static screenshot turns out to be dynamic
              //
              // @ts-ignore
              expect(newScreenshot).toMatchImageSnapshot({
                diffDirection,
                storeReceivedOnFailure: true,
                customSnapshotsDir: "screenshots/baseline",
                customSnapshotIdentifier: `${storybookPath}/${testName}`,
                customDiffDir: "screenshots/testing/diff",
                customReceivedDir: "screenshots/testing/actual",
                customReceivedPostfix: "",
              });
            }
          }
        });
      }
    });
  }
```

We get an error:

```
 FAIL  src/routes/storybook.test.ts > Storybook visual tests > background > dynamic should render the same
TypeError: this.customTesters is not iterable
 ❯ Proxy.<anonymous> ../node_modules/playwright/lib/matchers/expect.js:166:37
 ❯ src/routes/storybook.test.ts:159:41
    157| 
    158|             if (variantConfig.assertDynamic) {
    159|               expect(newScreenshot).not.toEqual(screenshot);
       |                                         ^
    160|             } else {
    161|               // do the same assertion from before so that we can see …
```

It appears we can't use our regular testers here. Even doing `expect(Buffer.compare(screenshot, newScreenshot)).not.toEqual(0);` doesn't help. We confirm that we literally can't use any of the usual matchers:

```ts
expect("one").not.toEqual("two");
```

fails as well.

We finally see that [others](https://github.com/microsoft/playwright/issues/20432) have run into this issue too. Our usage of `expect.extend({ toMatchImageSnapshot });` turns out to be a red herring, as the problem is that Playwright replaces the default `expect` with its own. We'll have to check from the list of assertions [here](https://playwright.dev/docs/test-assertions). This is the solution:


```ts
expect(Buffer.compare(screenshot, newScreenshot) !== 0).toBeTruthy();
```

Of course, we should also test that our tests are actually discriminatory between passing and failing states. If we disable the animation, does the test for dynamism now fail? It turns out that it does, so we are good here.

#### Refactoring duplicate config

Note that if we are to now add a new setting such as `allowSizeMismatch`, we now need to add it to two separate configs because it got duplicated now. To avoid this, refactor out the common parts:

```ts
import { toMatchImageSnapshot, type MatchImageSnapshotOptions } from "jest-image-snapshot";

const baseMatchOptions: MatchImageSnapshotOptions = {
    allowSizeMismatch: true,
    storeReceivedOnFailure: true,
    customSnapshotsDir: "screenshots/baseline",
    customDiffDir: "screenshots/testing/diff",
    customReceivedDir: "screenshots/testing/actual",
    customReceivedPostfix: "",
  };

  for (const config of components) {
    ...
            const matchOptions = {
              ...baseMatchOptions,
              diffDirection,
              customSnapshotIdentifier: `${storybookPath}/${testName}`,
            };

            if (!variantConfig.assertDynamic) {
              // don't compare dynamic screenshots against baseline
              // @ts-ignore
              expect(screenshot).toMatchImageSnapshot(matchOptions);
            }

            if (variantConfig.assertDynamic !== undefined) {
              ...
                expect(newScreenshot).toMatchImageSnapshot(matchOptions);
              ...
            }
          },
          ...
```

### Testing entire body

Sometimes, if we change the shadow of an element, the shadow is not part of the element's bounding box and therefore won't be captured in the screenshot. For these cases, we may want to zoom out to take a screenshot of the entire body instead of just the element itself. We can add an option:

```ts
interface ComponentTestConfig {
  ...
  screenshotEntireBody?: boolean;
}

const components: ComponentTestConfig[] = [
  ...
  {
    path: ["dashboard", "api-keys-display"],
    variants: ["loading", "unknown", "known"],
    screenshotEntireBody: true,
  },
  {
    path: ["dashboard", "metadata"],
    variants: ["metadata"],
    screenshotEntireBody: true,
  },
  ...
];

...

  const takeScreenshot = (page: Page, screenshotEntireBody?: boolean) => {
    const frame = page.frame({ name: "storybook-preview-iframe" });
    if (!frame) {
      throw new Error("Could not find Storybook iframe");
    }
    const locator = screenshotEntireBody ? "body" : "#storybook-root > :first-child";
    return frame.locator(locator).screenshot();
  };

...

          const screenshot = await takeScreenshot(page, config.screenshotEntireBody);

...
```

You may find some of these full-body tests to be flaky. In that case, specify a certain number of retries as explained [here](/zamm/resources/tutorials/setup/tauri/vitest.md).

## Parallelization

You can follow the instructions [here](https://vitest.dev/guide/test-context.html) to make tests concurrent. Note that this only affects tests within a single suite; separate suites of tests will still execute sequentially.

For example, to refactor `src-svelte/src/routes/storybook.test.ts` to run tests in parallel:

```ts
import {
  type Browser,
  chromium,
  type Page,
  type BrowserContext,
} from "@playwright/test";
import { afterAll, beforeAll, afterEach, beforeEach, describe, test, type TestContext } from "vitest";
...

interface StorybookTestContext {
  page: Page;
}

describe.concurrent("Storybook visual tests", () => {
  let storybookProcess: ChildProcess | undefined;
  let browser: Browser;
  let browserContext: BrowserContext;

  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    browserContext = await browser.newContext();
    storybookProcess = await ensureStorybookRunning();
  });

  afterAll(async () => {
    await browserContext.close();
    await browser.close();
    await killStorybook(storybookProcess);
  });

  beforeEach<StorybookTestContext>(
    async (context: TestContext & StorybookTestContext) => {
      context.page = await browserContext.newPage();
      context.expect.extend({ toMatchImageSnapshot });
    },
  );

  afterEach<StorybookTestContext>(
    async (context: TestContext & StorybookTestContext) => {
      await context.page.close();
    },
  );

  ...

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
      const testName = variantConfig.name;
      test(
        `${testName} should render the same`,
        async ({ expect, page }: TestContext & StorybookTestContext) => {
          const variantPrefix = `--${variantConfig.name}`;

          await page.goto(
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );

          ...
        },
        {
          retry: 4,
          timeout: 10_000,
        },
      );
    }
  }
});

```

Note that the nested `describe` has been stripped to allow full concurrency across all tests in this file. Because we have removed that context from which test is being run, we should add the information back into each test:

```ts
      const testName = `${storybookPath}/${variantConfig.name}.png`;
```

However, this means that we should also change `matchOptions` to use `variantConfig.name` instead of `testName`, or else none of the existing screenshot files will match because new ones will be created instead at paths like `src-svelte/screenshots/baseline/screens/dashboard/api-keys-display/screens/dashboard/api-keys-display/...`:

```ts
          const matchOptions = {
            ...baseMatchOptions,
            diffDirection,
            customSnapshotIdentifier: `${storybookPath}/${variantConfig.name}`,
          };
```

## Errors

### Test timeout

If you get an error such as

```
 FAIL  src/routes/storybook.test.ts > Storybook visual tests > navigation/sidebar > settings-selected should render the same
Error: Test timed out in 5000ms.
If this is a long-running test, pass a timeout value as the last argument or configure it globally with "testTimeout".
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/4]⎯
```

then you can increase the timeout like so:

```ts
...

        test(`${testName} should render the same`, async () => {
          ...
        }, 40_000);

...
```

If you're specifying other options such as `retry`, then add `timeout` like so:

```ts
...

        test(`${testName} should render the same`, async () => {
          ...
        }, {
          retry: 4,
          timeout: 40_000,
        });

...
```

### Element visibility timeout

If instead you get a test like this:

```
 FAIL  src/routes/storybook.test.ts > Storybook visual tests > navigation/sidebar > settings-selected should render the same
TimeoutError: locator.screenshot: Timeout 29859.952000000005ms exceeded.
=========================== logs ===========================
taking element screenshot
  waiting for element to be visible and stable
    element is not visible - waiting...
============================================================
```

and you are in the specific context of trying to take a Storybook screenshot, this may be because the `#storybook-root` element itself has zero height despite having child elements that are visible -- for example, if its only child is a header element. In this case, you can try to take a screenshot of the child element instead.

```ts
          const screenshot = await frame
            .locator("#storybook-root > :first-child")
            .screenshot();
```

This has the added benefit of making the screenshot more compact, since it will only be the size of the child element and not the entire Storybook root element.

Note that we do this again in [settings.md](/ui/settings.md), making it

```ts
  const takeScreenshot = async (page: Page, screenshotEntireBody?: boolean) => {
    const frame = page.frame({ name: "storybook-preview-iframe" });
    if (!frame) {
      throw new Error("Could not find Storybook iframe");
    }
    let locator = screenshotEntireBody
      ? "body"
      : "#storybook-root > :first-child";
    const elementClass = await frame.locator(locator).getAttribute("class");
    if (elementClass === "storybook-wrapper") {
      locator = "#storybook-root > :first-child > :first-child";
    }
    return await frame.locator(locator).screenshot();
  };
```

### Faster timeouts

To timeout faster so that your tests can run faster, see [this answer](https://stackoverflow.com/a/68107808).

### New browser download needed

If you get

```
 FAIL  src/routes/storybook.test.ts > Storybook visual tests
Error: browserType.launch: Executable doesn't exist at /root/.cache/ms-playwright/chromium-1080/chrome-linux/chrome
╔═════════════════════════════════════════════════════════════════════════╗
║ Looks like Playwright Test or Playwright was just installed or updated. ║
║ Please run the following command to download new browsers:              ║
║                                                                         ║
║     yarn playwright install                                             ║
║                                                                         ║
║ <3 Playwright Team                                                      ║
╚═════════════════════════════════════════════════════════════════════════╝
```

then do as it says and run

```bash
$ yarn playwright install     
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/playwright install
Removing unused browser at /root/.cache/ms-playwright/chromium-1076
Removing unused browser at /root/.cache/ms-playwright/firefox-1422
Removing unused browser at /root/.cache/ms-playwright/webkit-1883
Downloading Chromium 117.0.5938.62 (playwright build v1080) from https://playwright.azureedge.net/builds/chromium/1080/chromium-linux.zip
153.1 Mb [====================] 100% 0.0s
Chromium 117.0.5938.62 (playwright build v1080) downloaded to /root/.cache/ms-playwright/chromium-1080
Downloading Firefox 117.0 (playwright build v1424) from https://playwright.azureedge.net/builds/firefox/1424/firefox-ubuntu-22.04.zip
78.8 Mb [====================] 100% 0.0s
Firefox 117.0 (playwright build v1424) downloaded to /root/.cache/ms-playwright/firefox-1424
Downloading Webkit 17.0 (playwright build v1908) from https://playwright.azureedge.net/builds/webkit/1908/webkit-ubuntu-22.04.zip
82.5 Mb [====================] 100% 0.0s
Webkit 17.0 (playwright build v1908) downloaded to /root/.cache/ms-playwright/webkit-1908
Done in 10.96s.
```

**Note that when a new browser is used,** some effects such as SVG filters may be rendered ever so slightly differently. This may cause your screenshot tests to fail. Visually inspect the changes, and if they are minor as expected, then update the baseline screenshots. Alternatively, as mentioned [here](https://news.ycombinator.com/item?id=32908506), update screenshots for a build that is known to be good. You may want to combine this with visual inspection if the differences are large enough, in case the old build relied on some quirks of the old browser version to render correctly.

### Vitest error unhandled

If you get an unhandled error:

```
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯ Unhandled Errors ⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯

Vitest caught 1 unhandled error during the test run.
This might cause false positive tests. Resolve unhandled errors to make sure your tests are not affected.

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯ Unhandled Rejection ⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯
Error: expect(received).toHaveAttribute(expected)

Expected string: "true"
Received string: "false"
Call log:
  - locator._expect with timeout 5000ms
  - waiting for getByRole('switch')
  -   locator resolved to <button tabindex="0" type="button" role="switch" id="swi…>…</button>
  -   unexpected value "false"
```

that's because of [this problem](https://github.com/vitest-dev/vitest/discussions/3229#discussioncomment-5685717). Your assert is likely making a promise. Await on it, for example:

```ts
await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
```

Then you get a proper error:

```
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/lib/Switch.playwright.test.ts > Switch drag test > plays clicking sound when drag released
Error: Test timed out in 5000ms.
If this is a long-running test, pass a timeout value as the last argument or configure it globally with "testTimeout".
```

## Moving screenshot locations

Say you want to move dashboard screenshots to be under a more general screens directory. Then:

Rename entire folders like `src-svelte/screenshots/baseline/dashboard/metadata/` to `src-svelte/screenshots/baseline/screens/dashboard/metadata/`.

Edit `src-svelte/src/routes/ApiKeysDisplay.stories.ts` from

```ts
export default {
  ...
  title: "Dashboard/API Keys Display",
  ...
};
```

to

```ts
export default {
  ...
  title: "Screens/Dashboard/API Keys Display",
  ...
};
```

Do the same thing for the stories at `src-svelte/src/routes/Metadata.stories.ts`.

Then, edit the tests at `src-svelte/src/routes/storybook.test.ts` from

```ts
const components: ComponentTestConfig[] = [
  ...
    {
    path: ["dashboard", "api-keys-display"],
    variants: ["loading", "unknown", "known"],
    screenshotEntireBody: true,
  },
  {
    path: ["dashboard", "metadata"],
    variants: ["metadata"],
    screenshotEntireBody: true,
  },
  ...
```

to

```ts
const components: ComponentTestConfig[] = [
  ...
    {
    path: ["screens", "dashboard", "api-keys-display"],
    variants: ["loading", "unknown", "known"],
    screenshotEntireBody: true,
  },
  {
    path: ["screens", "dashboard", "metadata"],
    variants: ["metadata"],
    screenshotEntireBody: true,
  },
  ...
```

Rearrange the entries, for example in alphabetical order, so that the screen stories still appear next to each other.

## Approximate screenshots

To allow for approximate screenshots to pass, you can use

```ts
                expect(newScreenshot).toMatchImageSnapshot({
                  ...
                  allowSizeMismatch: true,
                  ...
                });
```
