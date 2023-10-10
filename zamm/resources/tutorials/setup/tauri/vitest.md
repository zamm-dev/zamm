# Setting up Vitest for Tauri

If you are using Vite, then Vitest is a native test framework designed specifically for Vite. First install Vitest:

```bash
$ yarn add --dev vitest
```

Then add `test` and `test-watch` commands by editing `package.json` from

```json
...
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "tauri": "tauri"
  },
...
```

to

```json
...
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "tauri": "tauri",
    "test": "vitest run",
    "test-watch": "vitest",
  },
...
```

`test` will run tests and exit. `test-watch` will watch for files to change and then automatically rerun tests.

Add `ZixuanChen.vitest-explorer` to `.vscode/extensions.json`

## Creating Tauri frontend tests

Assuming you are using Svelte with Tauri, we will follow the guides at https://blog.logrocket.com/testing-svelte-app-vitest/ and https://testing-library.com/docs/svelte-testing-library/setup. First install these Svelte test helper libraries:

```bash
$ yarn add --dev @testing-library/svelte jsdom @testing-library/jest-dom @testing-library/user-event @testing-library/dom
```

Now define `vitest.config.ts`:

```ts
import {defineConfig} from 'vitest/config'
import {svelte} from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte({hot: !process.env.VITEST})],
  test: {
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    globals: true,
    environment: 'jsdom',
  },
})
```

We'll also follow the guides at https://tauri.app/v1/guides/testing/mocking/ and https://testing-library.com/docs/example-input-event/. Suppose you have a frontend component like this at `src/lib/Greet.svelte`:

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p id="greet-message">{greetMsg}</p>
</div>
```

and the corresponding backend function

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

First change the frontend component to be more accessible:

```html
  <p id="greet-message" role="paragraph">{greetMsg}</p>
```

We'll mock the call to the backend function with this code:

```ts
  mockIPC((cmd, args) => {
    // simulated rust command called "add" that just adds two numbers
    if(cmd === "greet") {
      return `Hello, ${args.name}! You've been greeted from Rust!`
    }
  });
```

And we'll interact with the UI by rendering it, entering some text into the textbox, and then clicking the button. Two things of note:

- It's best to keep your app accessible to blind users by ensuring elements are accessible by role. See [this](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles) for the list of available roles.
- Due to [issues with](https://stackoverflow.com/a/69589583) Svelte components not updating, we'll have to make sure to use `act` first. We also use the `userEvent` library for greater realism, though that is not strictly needed for this test to pass in the end.

```ts
  render(Greet, {});
  const greet_input = screen.getByRole("textbox");
  await act(() => userEvent.type(greet_input, "Vitest"));
  const greet_button = screen.getByRole("button");
  await act(() => userEvent.click(greet_button));
```

Let's setup a spy to make sure we're successfully invoking the mocked function:

```ts
  const spy = vi.spyOn(window, "__TAURI_IPC__");
  expect(spy).not.toHaveBeenCalled();
  
  // ... <UI interaction code here> ...

  expect(spy).toHaveBeenCalled();
```

Finally, let's make sure it works as expected:

```ts
  const message = screen.getByRole('paragraph')
  expect(message).toHaveTextContent(`Hello, Vitest! You've been greeted`)
```

Let's put this all together in a new file at `src/lib/Greet.test.ts`:

```ts
import { expect, test, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Greet from "./Greet.svelte";

test("invoke simple", async () => {
  mockIPC((cmd, args) => {
    if (cmd === "greet") {
      return `Hello, ${args.name}! You've been greeted from Rust!`;
    }
  });

  const spy = vi.spyOn(window, "__TAURI_IPC__");
  expect(spy).not.toHaveBeenCalled();

  render(Greet, {});
  const greet_input = screen.getByRole("textbox");
  await act(() => userEvent.type(greet_input, "Vitest"));
  const greet_button = screen.getByRole("button");
  await act(() => userEvent.click(greet_button));

  expect(spy).toHaveBeenCalled();

  const message = screen.getByRole("paragraph");
  expect(message).toHaveTextContent(`Hello, Vitest! You've been greeted`);
});
```

## Querying tables

Various options are described [here](https://github.com/testing-library/dom-testing-library/issues/583).

If you want to use [the first option](https://github.com/lexanth/testing-library-table-queries), you will have to first install it:

```bash
$ yarn add -D testing-library-table-queries
```

Then, if you have a table such as:

```svelte
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
</script>

<section>
  <table>
    <tr>
      <th class="header-text" colspan="2">API Keys</th>
    </tr>
    <tr>
      <td>OpenAI</td>
      <td class="key">
        {#await api_keys}
          ...loading
        {:then keys}
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        {:catch error}
          error: {error}
        {/await}
      </td>
    </tr>
  </table>
</section>
```

do something like

```ts
import { expect, test, vi } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import ApiKeysDisplay from "./+page.svelte";
import type { ApiKeys } from "$lib/bindings";
import {within, waitFor} from '@testing-library/dom'
import {getRowByFirstCellText} from 'testing-library-table-queries'

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);

test("loading by default", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const mockApiKeys: ApiKeys = {
    openai: null,
  }
  tauriInvokeMock.mockResolvedValueOnce(
    mockApiKeys
  );

  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = getRowByFirstCellText(screen.getByRole("table"), "OpenAI");
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  expect(openAiKeyCell).toHaveTextContent(/^...loading$/);
});
```

Alternatively, if you just want to get a row with certain specified text, you can use the second option and avoid adding a new dependency:

```ts
test("loading by default", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const mockApiKeys: ApiKeys = {
    openai: null,
  }
  tauriInvokeMock.mockResolvedValueOnce(
    mockApiKeys
  );

  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = screen.getByRole('row', { name: /OpenAI/ })
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  expect(openAiKeyCell).toHaveTextContent(/^...loading$/);
});
```

Note that we define `const mockApiKeys: ApiKeys` first instead of passing it directly into `tauriInvokeMock.mockResolvedValueOnce`, because the direct call to `tauriInvokeMock.mockResolvedValueOnce` won't trigger any type-checks. You can easily test this yourself.

## Waiting on promise re-renders

Given the same above table, you'll want to see it render after the promise is resolved:

```ts
test("no API key set", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  const mockApiKeys: ApiKeys = {
    openai: null,
  }
  tauriInvokeMock.mockResolvedValueOnce(
    mockApiKeys
  );

  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = getRowByFirstCellText(screen.getByRole("table"), "OpenAI");
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  await waitFor(() => expect(openAiKeyCell).toHaveTextContent(/^not set$/));
});
```

Alternatively, to test that it fails:

```ts
test("API key error", async () => {
  const spy = vi.spyOn(window, "__TAURI_INVOKE__");
  expect(spy).not.toHaveBeenCalled();
  tauriInvokeMock.mockRejectedValueOnce("testing");

  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  const openAiRow = getRowByFirstCellText(screen.getByRole("table"), "OpenAI");
  const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
  await waitFor(() => expect(openAiKeyCell).toHaveTextContent(/^error: testing$/));
});
```

## Mocking sidecar calls

Suppose that the above was instead

```ts
  import { Command } from '@tauri-apps/api/shell'

  let name = "";
  let greetMsg = ""

  async function greet(){
    const command = Command.sidecar('binaries/zamm-python', [
      name,
    ])
    const result = await command.execute()
    greetMsg = result.stdout + " via JavaScript!"
  }
```

and the corresponding Python sidecar function is:

```python
def greet(name: str) -> None:
    """Say hello-world."""
    print(f"Hello, {name}! You have been greeted from Python")
```

Then change the test to this:

```ts
import { expect, test, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Greet from "./Greet.svelte";

test("invoke simple", async () => {
  mockIPC((cmd, args) => {
    if (args.message.cmd === 'execute') {
      const eventCallbackId = `_${args.message.onEventFn}`;
      const eventEmitter = window[eventCallbackId];
  
      // 'Stdout' event can be called multiple times
      eventEmitter({
        event: 'Stdout',
        payload: `Hello, ${args.message.args[0]}! You've been greeted from Python`,
      });
  
      // 'Terminated' event must be called at the end to resolve the promise
      eventEmitter({
        event: 'Terminated',
        payload: {
          code: 0,
          signal: 'kill',
        },
      });
    }
  });

  const spy = vi.spyOn(window, "__TAURI_IPC__");
  expect(spy).not.toHaveBeenCalled();

  render(Greet, {});
  const greet_input = screen.getByRole("textbox");
  await act(() => userEvent.type(greet_input, "Vitest"));
  const greet_button = screen.getByRole("button");
  await act(() => userEvent.click(greet_button));

  expect(spy).toHaveBeenCalled();

  const message = screen.getByRole("paragraph");
  expect(message).toHaveTextContent(/^Hello, Vitest! You've been greeted from Python via JavaScript!$/);
});
```

However, `svelte-check` complains with TypeScript errors:

```
Loading svelte-check in workspace: /root/zamm
Getting Svelte diagnostics...

/root/zamm/src/lib/Greet.test.ts:11:22
Error: Property 'cmd' does not exist on type 'unknown'. 
  mockIPC((cmd, args) => {
    if (args.message.cmd === "execute") {
      const eventCallbackId = `_${args.message.onEventFn}`;


/root/zamm/src/lib/Greet.test.ts:12:48
Error: Property 'onEventFn' does not exist on type 'unknown'. 
    if (args.message.cmd === "execute") {
      const eventCallbackId = `_${args.message.onEventFn}`;
      const eventEmitter = window[eventCallbackId];


/root/zamm/src/lib/Greet.test.ts:18:41
Error: Property 'args' does not exist on type 'unknown'. 
        event: "Stdout",
        payload: `Hello, ${args.message.args[0]}! You've been greeted from Python`,
      });


/root/zamm/src/lib/Greet.test.ts:32:32
Error: Argument of type '"__TAURI_IPC__"' is not assignable to parameter of type '"undefined" | "ondevicemotion" | "ondeviceorientation" | "onorientationchange" | "opener" | "alert" | "blur" | "cancelIdleCallback" | "captureEvents" | "close" | "confirm" | ... 847 more ... | "clearImmediate"'. 

  const spy = vi.spyOn(window, "__TAURI_IPC__");
  expect(spy).not.toHaveBeenCalled();
```

If we do a `console.log`, we see that `args` looks like this:

```json
{
  __tauriModule: 'Shell',
  message: {
    cmd: 'execute',
    program: 'binaries/zamm-python',
    args: [ 'Vitest' ],
    options: { sidecar: true },
    onEventFn: 794958308
  }
}
```

Based on this information, define a `src/types/tauri.ts` that looks like this:

```ts
export interface SidecarMessageOptions {
  sidecar: boolean;
}

export interface SidecarMessage {
  cmd: string;
  program: string;
  args: string[];
  options: SidecarMessageOptions;
  onEventFn: number;
}

export interface SidecarArgs {
  message: SidecarMessage;
}
```

As for the other error, we can try to simply ignore it:

```ts
  // @ts-ignore
  const spy = vi.spyOn(window, "__TAURI_IPC__");
```

If you do this, you have to edit your `.eslintrc.yaml` from

```yaml
rules:
  no-inner-declarations: off
```

to

```yaml
rules:
  no-inner-declarations: off
  "@typescript-eslint/ban-ts-comment": off
```

Or we can fix it by creating ```src/types/tauri-env.d.ts``` as such:

```ts
interface Window {
  __TAURI_IPC__?: () => void;
}
```

### Specta

If instead you [set up Specta](/zamm/resources/tutorials/libraries/specta.md), you should follow the mocking instructions in that file.

## Tips

### Specifying specific test

Do

```bash
$ yarn vitest -t sidebar
```

to specify a specific test

### Retrying flaky tests

To specify a certain number of retries, simply add the options to the end of the test as mentioned [here](https://vitest.dev/api/):

```ts
        test(
          `${testName} should render the same`,
          async () => {
            // ... actual testing code ...
          },
          {
            retry: 3,
          },
        );
```

## Errors

### expect is not defined

If you see an error such as

```
ReferenceError: expect is not defined
 ❯ node_modules/@testing-library/jest-dom/dist/index.mjs:12:1
 ❯ src/Demo.test.ts:2:31
      1| import { expect, it } from "vitest";
      2| import "@testing-library/jest-dom";
       |                               ^
      3| 
      4| it("can render demo", async () => {

```

that's because you haven't set `globals: true` in the default test config:

```ts
export default defineConfig({
  ...
  test: {
    ...
    globals: true,
    environment: "jsdom",
  },
});
```

### screen.querySelector is not a function

If you see an error such as

```
TypeError: screen.querySelector is not a function
 ❯ src/Demo.test.ts:8:28
      6| it("can render demo", async () => {
      7|   render(Demo);
      8|   const paragraph = screen.querySelector("p");
       |                            ^
      9|   expect(paragraph).toHaveTextContent("Demo URL: ")
     10| });

```

it is because for querySelector, you have to use the returned `container` variable instead of the screen:

```ts
it("can render demo", async () => {
  const { container } = render(Demo);
  const paragraph = container.querySelector("p");
  ...
});
```

### Cannot subscribe to store on server

If you do

```svelte
<script lang="ts">
  import { page } from "$app/stores";
</script>

<p>Demo URL: {$page.url.pathname}</p>

```

and get an error like

```
Error: Cannot subscribe to 'page' store on the server outside of a Svelte component, as it is bound to the current request via component context. This prevents state from leaking between users.For more information, see https://kit.svelte.dev/docs/state-management#avoid-shared-state-on-the-server
 ❯ get_store node_modules/@sveltejs/kit/src/runtime/app/stores.js:96:9
 ❯ Object.subscribe node_modules/@sveltejs/kit/src/runtime/app/stores.js:39:37                                                                      
```

this is because of an [outstanding issue](https://github.com/sveltejs/kit/issues/1485) with a [minimal repro](https://github.com/amosjyng/sveltekit-vitest-minimal-repro). It was mentioned before with [a workaround](https://github.com/sveltejs/kit/issues/5525#issuecomment-1186390654).

## Minimal SvelteKit setup

Create a new SvelteKit project with `yarn create svelte`. Then

```bash
$ yarn add --dev @testing-library/jest-dom @testing-library/svelte jsdom
```

Create `src/Demo.svelte`:

```svelte
<p>Demo URL: </p>
```

Create `src/Demo.test.ts`:

```ts
import { expect, it } from "vitest";
import "@testing-library/jest-dom";
import { render, screen } from "@testing-library/svelte";
import Demo from "./Demo.svelte";

it("can render demo", async () => {
  const { container } = render(Demo);
  const paragraph = container.querySelector("p");
  expect(paragraph).toHaveTextContent("Demo URL:")
});

```

Edit `vite.config.ts`:

```ts
        test: {
               include: ['src/**/*.{test,spec}.{js,ts}'],
               globals: true,
               environment: 'jsdom',
        }
```


