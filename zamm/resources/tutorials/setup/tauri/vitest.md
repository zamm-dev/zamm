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
