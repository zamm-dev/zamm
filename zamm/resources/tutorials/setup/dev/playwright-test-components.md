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
