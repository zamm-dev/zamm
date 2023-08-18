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
