# Setting up automated end-to-end testing for a Tauri project

First follow the instructions [here](https://tauri.app/v1/guides/testing/webdriver/introduction/).

Try seeing if `WebKitWebDriver` exists. If not:

```bash
$ which WebKitWebDriver
WebKitWebDriver not found
```

then install it:

```bash
$ apt-get install webkit2gtk-driver
```

Now it should show

```bash
$ which WebKitWebDriver
/usr/bin/WebKitWebDriver
```

Install `tauri-driver`

```bash
$ cargo install tauri-driver
```

Check that it can run

```bash
$ tauri-driver
```

If it can't, you may need to reshim with asdf:

```bash
$ asdf reshim rust
```

Set up Vitest using the instructions from [`vitest.md`](./vitest.md).

## Selenium

For Selenium setup, follow the examples on [this webpage](https://tauri.app/v1/guides/testing/webdriver/example/selenium).

```bash
$ yarn add --dev selenium-webdriver @types/selenium-webdriver
```

If the test somehow fails to kill the `tauri-driver` running on port 4444, run this command to manually kill it instead:

```bash
$ sudo kill -9 `sudo lsof -t -i:4444`
$ sudo kill -9 `sudo lsof -t -i:4445`
```

Port 4445 should be killed too, as it also runs for some reason.

Then edit `src/e2e.test.ts` using the example code in the demo. Particular changes made:

Import from `vitest` instead:

```ts
import { beforeAll, afterAll, describe, expect, test } from 'vitest'
```

Path must go to zamm binary.

```ts
// create the path to the expected application binary
const application = path.resolve(
  __dirname,
  '..',
  'src-tauri',
  'target',
  'release',
  'zamm'
)
```

`cargo build --release` must be run from inside `src-tauri` directory, and the `stdio` argument fails with the error. It appears to be related to [this](https://github.com/aws/aws-cdk/issues/20873).

```
TypeError: The argument 'stdio' is invalid. Received WritableWorkerStdio {
  _writableState: WritableState {
    objectMode: false,
    highWaterMark: 16384,
    finalCalled: false,...
```

```ts
beforeAll(async function () {
  // ensure the program has been built
  spawnSync('bash', ['-c', 'cd src-tauri && cargo build --release'], {
    stdio: "inherit"
  })
```

Wait for tauri-driver to come up, otherwise you get 

```
Error: ECONNREFUSED connect ECONNREFUSED 127.0.0.1:4444
```

```ts
await new Promise(resolve => setTimeout(resolve, 1000));
```

Add the timeout at the end for `vitest`:

```ts
  // start the webdriver client
  driver = await new Builder()
    .withCapabilities(capabilities)
    .usingServer('http://127.0.0.1:4444/')
    .build()
}, 10_000) // timeout in ms, for cargo build
```

Kill the Tauri driver first because otherwise it won't be cleaned up if the webdriver fails:

```ts
afterAll(async function () {
  // kill the tauri-driver process
  tauriDriver.kill()

  // stop the webdriver session
  await driver.quit()
})
```

Adapt the tests for our own sample app:

```ts
describe('Welcome screen', () => {
  test('should show greet button', async () => {
    const text = await driver.findElement(By.tagName('button')).getText()
    expect(text).to.match(/^Greet$/)
  })

  test('should greet user when button pressed', async () => {
    let name_field = await driver.findElement(By.id('greet-input'))
    name_field.sendKeys("me")
    let greet_button = await driver.findElement(By.tagName('button'))
    greet_button.click()
    const text = await driver.findElement(By.tagName('p')).getText()
    expect(text).to.match(/^Hello me/)
  })
})
```

Selenium appears to be rather brittle with Vitest, and tends to hang on driver connect.

## WebdriverIO

Put everything inside its own directory:

```bash
$ mkdir webdriver
$ cd webdriver
```

Now create a minimal `package.json` in this directory:

```bash
{
  "name": "webdriver",
  "private": true,
  "scripts": {
    "test": "wdio run ./wdio.conf.ts"
  }
}
```

Make it a workspace of the main `package.json`:

```bash
{
  "name": "zamm",
  "private": true,
  ...
  "workspaces": ["webdriver"],
  ...
}
```

Define `wdio.conf.ts` as such:

```ts
const { spawn, spawnSync } = require("child_process");

// keep track of the `tauri-driver` child process
let tauriDriver;

exports.config = {
  specs: ["./test/specs/**/*.js"],
  maxInstances: 1,
  capabilities: [
    {
      maxInstances: 1,
      "tauri:options": {
        application: "../src-tauri/target/release/zamm",
      },
    },
  ],
  reporters: ["spec"],
  framework: "mocha",
  mochaOpts: {
    ui: "bdd",
    timeout: 60000,
  },

  // ensure the rust project is built since we expect this binary to exist for the webdriver sessions
  onPrepare: () => spawnSync("cargo", ["build", "--release"]),

  // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests
  beforeSession: () =>
    (tauriDriver = spawn("tauri-driver", [], {
      stdio: [null, process.stdout, process.stderr],
    })),

  // clean up the `tauri-driver` process we spawned at the start of the session
  afterSession: () => tauriDriver.kill(),
};
```

Add the CLI tool:

```bash
$ yarn add --dev @wdio/cli@^7.32.3 @wdio/local-runner@^7.32.3 @wdio/mocha-framework@^7.30.2 @wdio/spec-reporter@^7.31.1
```

We are using version 7 due to [this issue](https://github.com/chippers/hello_tauri/issues/3).

Edit `src/lib/Greet.svelte` to allow for easy identification by the test, from

```svelte
  <p>{greetMsg}</p>
```

to

```svelte
  <p id="greet-message">{greetMsg}</p>
```

Now we use that for a test. Create a new file `webdriver/test/specs/e2e.test.js`:

```js
describe("Welcome screen", function () {
  it("should show greet button", async function () {
    const text = await $("button").getText();
    expect(text).toMatch(/^Greet$/);
  });

  it("should greet user when button pressed", async function () {
    const original = await $("p#greet-message").getText();
    expect(original).toMatch(/^$/);

    const greetInput = await $("#greet-input");
    // workaround for https://github.com/tauri-apps/tauri/issues/6541
    await browser.execute(`arguments[0].value="me"`, greetInput);
    await browser.execute(
      'arguments[0].dispatchEvent(new Event("input", { bubbles: true }))',
      greetInput,
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
    const inputText = await $("#greet-input").getValue();
    expect(inputText).toMatch(/^me$/);

    await browser.execute(() => {
      document.getElementsByTagName("button")[0].click();
    });

    await new Promise((resolve) => setTimeout(resolve, 1000));
    const text = await $("p#greet-message").getText();
    expect(text).toMatch(/^Hello, me! You have been greeted/);
  });
});
```

We are executing JS actions in the browser instead of using the webdriver because of [this issue](https://github.com/tauri-apps/tauri/issues/6541).

Back in the top-level `package.json`, we add this to `scripts`:

```json
{
  ...
  "scripts": {
    ...
    "e2e-test": "yarn workspace webdriver test"
  },
  ...
}
```

### Project tooling updates

If you've followed the project setup from before:

At this time, it's unknown how to successfully run the Typescript transpiler on the Webdriver files without also breaking the Webdriver test run. As such, we should avoid running `svelte-check` on the files until it is possible to run Typescript on WebdriverIO. First, exclude the new `webdriver` subdirectory. So if it currently looks like this:

```yaml
  - repo: local
    hooks:
      - id: typecheck-svelte
        name: svelte-check
        entry: yarn svelte-check --fail-on-warnings
        language: system
        types: [svelte]
```

change it to

```yaml
  - repo: local
    hooks:
      - id: typecheck-svelte
        name: svelte-check
        entry: yarn svelte-check --fail-on-warnings
        language: system
        types: [svelte]
        exclude: ^webdriver/
```

Because it's in a subdirectory, if at some point in the future we wanted to set up TS support for WebdriverIO, we will have to edit `tsconfig.json` to include that directory, changing from:

```json
...
  "include": ["src/**/*.d.ts", "src/**/*.ts", "src/**/*.js", "src/**/*.svelte"],
...
```

to:

```json
...
  "include": [
    "src/**/*.d.ts",
    "src/**/*.ts",
    "src/**/*.js",
    "src/**/*.svelte",
    "webdriver/**/*.ts",
    "webdriver/**/*.js",
  ],
...
```

Now to setup eslint for WebdriverIO, add these plugins:

```bash
$ yarn add --dev eslint-plugin-mocha eslint-plugin-wdio@^7.25.3
```

Create a new file `webdriver/.eslintrc.yaml` and set these settings:

```yaml
plugins:
  - wdio
  - mocha
extends:
  - plugin:wdio/recommended
  - plugin:mocha/recommended
rules:
  "@typescript-eslint/no-var-requires": off
```

`@typescript-eslint/no-var-requires` is required here because there also does not appear to be a way to change the import

```js
const { spawn, spawnSync } = require("child_process");
```

in `webdriver/wdio.conf.ts` without causing wdio to fail to run. If this changes in the future, then this rule can be turned on again.

## Execution in CI environments

If the tests fail because the only thing rendered on the screen is `Could not connect: Connection refused`, check to see if you have the `custom-protocol` feature enabled. In particular, check if you haven't defined Tauri features twice in `Cargo.toml`.

For example, this is **wrong**:

```toml
[dependencies]
tauri = { version = "1.4", features = [ "shell-sidecar", "shell-open", "process-command-api"] }
...

[features]
# this feature is used for production builds or when `devPath` points to the filesystem
# DO NOT REMOVE!!
custom-protocol = ["tauri/custom-protocol"]
...
```

If you have that, make sure the Tauri "custom-protocol" feature is always enabled for end-to-end testing, and edit the Tauri dependency to this:

```toml
[dependencies]
tauri = { version = "1.4", features = [ "shell-sidecar", "shell-open", "process-command-api", "custom-protocol"] }
...

[features]
# this is necessary to prevent the Tauri CLI from complaining about the lack of a
# custom-protocol feature during compilation
custom-protocol = []
```

Make sure to put in a comment explaining why the `custom-protocol` feature is defined for the local project even though it doesn't do anything. This is to prevent an error message from `cargo tauri build` such as:

```
error: none of the selected packages contains these features: custom-protocol
       Error failed to build app: failed to build app
```

Note also that the `custom-protocol` defined in `[features]` refers to the `custom-protocol` feature of the app we're building (which we are likely never going to refer to), whereas the `custom-protocol` in the `features` array of the `tauri` dependency refers to the `custom-protocol` feature of the Tauri library itself.

In addition to all the setup required for pre-commit hooks as described in [`pre-commit.md`](/zamm/resources/tutorials/setup/repo/workflows/pre-commit.md), you will likely need these additional setup steps as well:

```yaml
      - name: Install webdriver dependencies
        run: sudo apt-get install -y webkit2gtk-driver xvfb
      - name: Install tauri-driver
        uses: actions-rs/cargo@v1
        with:
          command: install
          args: tauri-driver
      - name: Try creating directories
        run: |
          mkdir -p /home/runner/.local/share/zamm/
          chmod -R 777 /home/runner/.local/share/zamm/
          chmod +x src-tauri/target/release/zamm
          chmod +x src-tauri/target/release/zamm-python
      - name: Run headless WebdriverIO tests
        run: xvfb-run yarn e2e-test
      - name: Upload test screenshots as artifacts
        if: always() # run even if tests fail
        uses: actions/upload-artifact@v3
        with:
          name: test-screenshots
          path: webdriver/screenshots/*.png
```

Alternatively, you can use [this](https://webdriver.io/docs/wdio-image-comparison-service/) and install

```bash
$ yarn add -D wdio-image-comparison-service
```

Then edit `webdriver/wdio.conf.ts` per instructions:

```ts
...
const { join } = require("path");

...

exports.config = {
  ...
  services: [
    [
      "image-comparison",
      {
        baselineFolder: join(process.cwd(), "./screenshots/baseline/"),
        formatImageName: "{tag}-{logName}-{width}x{height}",
        screenshotPath: join(process.cwd(), "./screenshots/testing/"),
        savePerInstance: true,
        autoSaveBaseline: true,
        blockOutStatusBar: true,
        blockOutToolBar: true,
        isHybridApp: true,
      },
    ],
  ],
  ...
};
```

Be sure to change the uploads at `.github/workflows/tests.yaml` if you've set that up:

```yaml
      - name: Upload test screenshots as artifacts
        if: always() # run even if tests fail
        uses: actions/upload-artifact@v3
        with:
          name: test-screenshots
          path: webdriver/screenshots/*.png

```

Edit `webdriver/screenshots/.gitignore` accordingly:

```gitignore
testing/

```

Now edit `webdriver/test/specs/e2e.test.js` as well:

```js
describe("Welcome screen", function () {
  it("should show unset OpenAI API key", async function () {
    console.log(browser.capabilities);
    await expect(
      await browser.checkElement(await $("table"), "apiKeys", {}),
    ).toEqual(0);
    ...
```

If you see the error

```
[wry 0.24.3 linux #0-0]    ✖ should show unset OpenAI API key
[wry 0.24.3 linux #0-0]
[wry 0.24.3 linux #0-0] 1 failing (203ms)
[wry 0.24.3 linux #0-0]
[wry 0.24.3 linux #0-0] 1) Welcome screen should show unset OpenAI API key
[wry 0.24.3 linux #0-0] browser.saveElement is not a function
[wry 0.24.3 linux #0-0] TypeError: browser.saveElement is not a function
[wry 0.24.3 linux #0-0]     at Context.<anonymous> (/root/zamm/webdriver/test/specs/e2e.test.js:4:21)
[wry 0.24.3 linux #0-0]     at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
```

Then per [this issue](https://github.com/wswebcreation/wdio-image-comparison-service/issues/53), try

```bash
$ yarn test --logLevel=debug
...
[0-0] 2023-08-29T04:43:42.515Z INFO @wdio/local-runner: Run worker command: run
[0-0] 2023-08-29T04:43:42.521Z DEBUG @wdio/config:ConfigParser: No compiler found, continue without compiling files
[0-0] 2023-08-29T04:43:42.524Z DEBUG @wdio/local-runner:utils: init remote session
[0-0] 2023-08-29T04:43:42.531Z DEBUG @wdio/utils:initialiseServices: initialise service "image-comparison" as NPM package
[0-0] 2023-08-29T04:43:42.549Z ERROR @wdio/utils:initialiseServices: Error: Couldn't initialise "wdio-image-comparison-service".
[0-0] Error: Cannot find module '../build/Release/canvas.node'
[0-0] Require stack:
[0-0] - /root/zamm/node_modules/canvas/lib/bindings.js
[0-0] - /root/zamm/node_modules/canvas/lib/canvas.js
[0-0] - /root/zamm/node_modules/canvas/index.js
[0-0] - /root/zamm/node_modules/webdriver-image-comparison/build/methods/images.js
[0-0] - /root/zamm/node_modules/webdriver-image-comparison/build/commands/saveScreen.js
[0-0] - /root/zamm/node_modules/wdio-image-comparison-service/build/service.js
[0-0] - /root/zamm/node_modules/wdio-image-comparison-service/build/index.js
[0-0] - /root/zamm/node_modules/@wdio/utils/build/utils.js
[0-0] - /root/zamm/node_modules/@wdio/utils/build/initialisePlugin.js
[0-0] - /root/zamm/node_modules/@wdio/utils/build/index.js
[0-0] - /root/zamm/node_modules/@wdio/runner/build/index.js
[0-0] - /root/zamm/node_modules/@wdio/local-runner/build/run.js
[0-0]     at Module._resolveFilename (node:internal/modules/cjs/loader:1048:15)
[0-0]     at Module._load (node:internal/modules/cjs/loader:901:27)
[0-0]     at Module.require (node:internal/modules/cjs/loader:1115:19)
[0-0]     at require (node:internal/modules/helpers:130:18)
[0-0]     at Object.<anonymous> (/root/zamm/node_modules/canvas/lib/bindings.js:3:18)
[0-0]     at Module._compile (node:internal/modules/cjs/loader:1233:14)
[0-0]     at Module._extensions..js (node:internal/modules/cjs/loader:1287:10)
[0-0]     at Module.load (node:internal/modules/cjs/loader:1091:32)
[0-0]     at Module._load (node:internal/modules/cjs/loader:938:12)
[0-0]     at Module.require (node:internal/modules/cjs/loader:1115:19)
[0-0]     at safeRequire (/root/zamm/node_modules/@wdio/utils/build/utils.js:192:15)
[0-0]     at initialisePlugin (/root/zamm/node_modules/@wdio/utils/build/initialisePlugin.js:37:44)
[0-0]     at initialiseServices (/root/zamm/node_modules/@wdio/utils/build/initialiseServices.js:59:56)
[0-0]     at initialiseWorkerService (/root/zamm/node_modules/@wdio/utils/build/initialiseServices.js:140:26)
[0-0]     at Runner.run (/root/zamm/node_modules/@wdio/runner/build/index.js:75:45)
[0-0]     at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
[0-0] 2023-08-29T04:43:42.553Z DEBUG @wdio/utils:shim: Finished to run "beforeSession" hook in 0ms
```

Now we see [here](https://stackoverflow.com/questions/14771781/nodejs-cannot-find-module-build-release-canvas) that we have to install `canvas`:

```bash
$ yarn add -D canvas
```

Next is the problem

```
[0-0] 2023-08-29T04:46:38.128Z INFO wdio-image-comparison-service: Adding commands to Multi Browser:  [ 'maxInstances', 'tauri:options' ]
[0-0] 2023-08-29T04:46:38.129Z ERROR @wdio/utils:shim: TypeError: Cannot read properties of undefined (reading 'browserName')
[0-0]     at getInstanceData (/root/zamm/node_modules/wdio-image-comparison-service/build/utils.js:43:37)
[0-0]     at WdioImageComparisonService.addCommandsToBrowser (/root/zamm/node_modules/wdio-image-comparison-service/build/service.js:66:53)
[0-0]     at WdioImageComparisonService.before (/root/zamm/node_modules/wdio-image-comparison-service/build/service.js:51:14)
[0-0]     at /root/zamm/node_modules/@wdio/utils/build/shim.js:88:27
[0-0]     at new Promise (<anonymous>)
[0-0]     at /root/zamm/node_modules/@wdio/utils/build/shim.js:85:47
[0-0]     at Array.map (<anonymous>)
[0-0]     at executeHooksWithArgsShim (/root/zamm/node_modules/@wdio/utils/build/shim.js:85:33)
[0-0]     at Runner.run (/root/zamm/node_modules/@wdio/runner/build/index.js:99:48)
[0-0]     at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
```

We jump to `/root/zamm/node_modules/wdio-image-comparison-service/build/utils.js:44:37` and add a `console.log`:

```js
function getInstanceData(capabilities, currentBrowser) {
  // Substract the needed data from the running instance
  const currentCapabilities = currentBrowser && currentBrowser.capabilities || browser.capabilities;
  console.log(capabilities);
  const browserName = (capabilities.browserName || currentCapabilities.browserName || 'not-known').toLowerCase();
  ...
```

We see that it is actually `undefined`. We go to `/root/zamm/node_modules/wdio-image-comparison-service/build/service.js:51:14`:

```js
  before(capabilities) {
    if (typeof capabilities['browserName'] !== 'undefined') {
      log.info('Adding commands to global browser');
      this.addCommandsToBrowser(capabilities, browser);
    } else {
      const browserNames = Object.keys(capabilities);
      log.info('Adding commands to Multi Browser: ', browserNames);
      console.log(capabilities)
      for (const browserName of browserNames) {
        this.addCommandsToBrowser(capabilities[browserName].capabilities, global[browserName]);
      }
    ...
```

We see the log:

```
[0-0] 2023-08-29T05:01:53.547Z INFO wdio-image-comparison-service: Adding commands to Multi Browser:  [ 'maxInstances', 'tauri:options' ]
[0-0] {
[0-0]   maxInstances: 1,
[0-0]   'tauri:options': { application: '../src-tauri/target/release/zamm' }
[0-0] }
```

Try editing `/root/zamm/node_modules/wdio-image-comparison-service/build/service.js` to change

```ts
    if (typeof capabilities['browserName'] !== 'undefined') {
      log.info('Adding commands to global browser');
```

to

```ts
    if (typeof capabilities['browserName'] === 'undefined') {
      log.info('Adding commands to global browser');
```

After all, a command that is added to the global browser should work on our regular browser too. Now try again:

```
 "spec" Reporter:
------------------------------------------------------------------
[wry 0.24.3 linux #0-0] Running: wry (v0.24.3) on linux
[wry 0.24.3 linux #0-0] Session ID: f6be3644-877f-4920-ab77-49bd9a7123ca
[wry 0.24.3 linux #0-0]
[wry 0.24.3 linux #0-0] » /test/specs/e2e.test.js
[wry 0.24.3 linux #0-0] Welcome screen
[wry 0.24.3 linux #0-0]    ✓ should show unset OpenAI API key
[wry 0.24.3 linux #0-0]
[wry 0.24.3 linux #0-0] 1 passing (1s)
```

Sure enough, this works! But could we fix it better?

We edit `e2e.test.js` to log the browser capabilities, including what `browserName` we're using:

```js
describe("Welcome screen", function () {
  it("should show unset OpenAI API key", async function () {
    console.log(browser.capabilities);
    ...
```

Then we see in the output:

```
[0-0] {
[0-0]   browserName: 'wry',
[0-0]   browserVersion: '0.24.3',
[0-0]   platformName: 'linux',
[0-0]   acceptInsecureCerts: false,
[0-0]   strictFileInteractability: false,
[0-0]   setWindowRect: true,
[0-0]   unhandledPromptBehavior: 'dismiss and notify',
[0-0]   pageLoadStrategy: 'normal',
[0-0]   proxy: {},
[0-0]   timeouts: { script: 30000, pageLoad: 300000, implicit: 0 }
[0-0] }
```

We go back to edit this into `webdriver/wdio.conf.ts`:

```ts
exports.config = {
  specs: ["./test/specs/**/*.js"],
  maxInstances: 1,
  capabilities: [
    {
      browserName: "wry",
      maxInstances: 1,
      "tauri:options": {
        application: "../src-tauri/target/release/zamm",
      },
    },
  ],
  ...
```

We are still getting

```
[0-0] 2023-08-29T05:16:41.016Z ERROR @wdio/utils:shim: TypeError: Cannot read properties of undefined (reading 'browserName')
[0-0]     at getInstanceData (/root/zamm/node_modules/wdio-image-comparison-service/build/utils.js:43:37)
[0-0]     at WdioImageComparisonService.addCommandsToBrowser (/root/zamm/node_modules/wdio-image-comparison-service/build/service.js:67:53)
[0-0]     at WdioImageComparisonService.before (/root/zamm/node_modules/wdio-image-comparison-service/build/service.js:52:14)
```

It appears it's expecting a dictionary of a different sort to be passed in. Let's check to see if it expects the latest version of wdio:

```bash
$ yarn add -D @wdio/cli @wdio/local-runner @wdio/mocha-framework @wdio/spec-reporter
$ yarn test
yarn run v1.22.19
$ wdio run ./wdio.conf.ts

node:internal/process/esm_loader:46
      internalBinding('errors').triggerUncaughtException(
                                ^
Error [ERR_MODULE_NOT_FOUND]: Cannot find package 'ts-node' imported from /root/zamm/webdriver/
    at new NodeError (node:internal/errors:405:5)
    at packageResolve (node:internal/modules/esm/resolve:782:9
    ...
```

Okay, fine, let's add that too.

```bash
$ yarn add -D ts-node
```

We are back at the old issues that caused us to use wdio version 7:

```
[0-0] 2023-08-29T05:37:04.343Z ERROR @wdio/runner: Error: Unknown browser name "wry". Make sure to pick from one of the following chrome,googlechrome,chromium,chromium-browser,firefox,ff,mozilla,mozilla firefox,edge,microsoftedge,msedge,safari,safari technology preview
[0-0]     at startWebDriver (file:///root/zamm/node_modules/@wdio/utils/build/driver/index.js:123:15)
```

and if we try leaving it out:

```
[0-0] 2023-08-11T04:33:32.841Z ERROR @wdio/runner: Error: No "browserName" defined in capabilities nor hostname or port found!
[0-0] If you like to run a mobile session with Appium, make sure to set "hostname" and "port" in your WebdriverIO options. If you like to run a local browser session make sure to pick from one of the following browser names: chrome,googlechrome,chromium,chromium-browser,firefox,ff,mozilla,mozilla firefox,edge,microsoftedge,msedge,safari,safari technology preview
```

However, now that we understand things better, we can try adding the hostname and port:

```ts
...
exports.config = {
  ...
  hostname: "localhost",
  port: 4444,
  capabilities: [
    {
      browserName: "wry",
      ...
    },
  ],
...
```

Now we have finally fixed https://github.com/chippers/hello_tauri/issues/3.

You can also add a

```js
    await expect(
      await browser.checkFullPageScreen("welcome-screen", {}),
    ).toEqual(0);
```

to take screenshots of entire pages as well.

You may want to separate the screenshot tests from the other assertions, so that in case of a massive UI change where you don't have time to update the screenshots, you can still run the other tests. Vice versa, failing paired non-screenshot tests could clue you in to why the screenshot tests are failing.

If the tests fail on the CI server due to a completely black screen, you may want to `await` an element you know will appear for sure.

If the tests fail due to slight pixel offsets on the CI server, you can update the baseline images to match the CI server's, and then edit the code to include a maximum offset:

```js
const maxMismatch = process.env.MISMATCH_TOLERANCE === undefined ? 0 : parseFloat(process.env.MISMATCH_TOLERANCE);

describe("Welcome screen", function () {
  it("should render the welcome screen correctly", async function () {
    await $("table"); // ensure page loads before taking screenshot
    await expect(
      await browser.checkFullPageScreen("welcome-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should render the API keys table correctly", async function () {
    await expect(
      await browser.checkElement(await $("table"), "api-keys", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  ...
});

```

Remember that WebdriverIO uses jest asserts.

Now run with

```bash
$ MISMATCH_TOLERANCE=0.07 yarn test
```

and the tests from before should pass. To make this permanent on your local development machine, export it in your `.zshrc`:

```sh
...
export MISMATCH_TOLERANCE=0.07
```

If tests still prove flaky on CI, you can try rerunning the suite a certain number of times, as noted [here](https://webdriver.io/docs/retry/):

```js
describe("Welcome screen", function () {
  this.retries(2);

  ...
```

Or, run just that one test multiple times, because it is the one that ensures the whole screen is ready:

```js
describe("Welcome screen", function () {
  it("should render the welcome screen correctly", async function () {
    this.retries(2);
    ...
  });

  ...
});
```

## Using local user directories

If you're doing something with local user data directories, you may have to create them first before the CI run:

```yaml
      - name: Try creating directories
        run: |
          mkdir -p /home/runner/.local/share/zamm/
          chmod -R 777 /home/runner/.local/share/zamm/
```

This is because the CI environment might not give the running process the permissions to create directories in the user's home directory. If that is the case, the best option is of course to handle such an edge case within the program itself.

## Supporting screenshots

Add this code to the beginning of the test:

```bash
  afterEach(async function () {
    const screenshotPath = `./screenshots/${this.currentTest.title.replace(/\s+/g, '_')}.png`;
    await browser.saveScreenshot(screenshotPath);
    console.log(`Screenshot saved to ${screenshotPath}`);
  });
```

Then create the file `webdriver/screenshots/.gitignore` with the following contents to avoid commiting any screenshots while still ensuring the directory exists:

```gitignore
*
!.gitignore
```

If you are running this in CI, then add this to the end of the workflow:

```yaml
      - name: Upload test screenshots as artifacts
        if: always() # run even if (especially if) tests fail
        uses: actions/upload-artifact@v3
        with:
          name: test-screenshots
          path: webdriver/screenshots/*.png
```

## Other errors

### Object returned instead of string

If you do

```ts
  it("should show unset OpenAI API key", async function () {
    const openAiCell = await $("tr*=OpenAI").$("td:nth-child(2)");
    expect(openAiCell.getText()).toMatch(/^not set$/);
  });
```

and get

```
[wry 0.24.3 linux #0-0] 1) Welcome screen should show unset OpenAI API key
[wry 0.24.3 linux #0-0] expect(received).toMatch(expected)

Matcher error: received value must be a string

Received has type:  object
Received has value: {}
```

it is because `getText` returns a promise. You should instead do

```ts
  it("should show unset OpenAI API key", async function () {
    const openAiCell = await $("tr*=OpenAI").$("td:nth-child(2)");
    expect(await openAiCell.getText()).toMatch(/^not set$/);
  });
```
