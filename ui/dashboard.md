# Homepage

## Storybook

To test out the homepage display in Storybook, we move all the code from `src-svelte/src/routes/+page.svelte` to `src-svelte/src/routes/Homepage.svelte`, and then import the new homepage component in `src-svelte/src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import Homepage from "./Homepage.svelte";
</script>

<Homepage />

```

In the process of setting up a Storybook story, we find that we need yet another mock page transition element if we want `src-svelte/src/lib/__mocks__/MockPageTransitions.svelte`:

```ts
<script lang="ts">
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed} from "$lib/preferences";

  firstAppLoad.set(true);
  firstPageLoad.set(true);
  animationSpeed.set(0.1);
</script>

<div class="storybook-wrapper">
  <PageTransition currentRoute="/storybook-demo">
    <slot />
  </PageTransition>
</div>

<style>
  .storybook-wrapper {
    --base-animation-speed: 0.1;
  }
</style>

```

Now the page will finally animate as if we're navigating to it. We can now create a Storybook story for the homepage at `src-svelte/src/routes/Homepage.stories.ts`:

```ts
import HompageComponent from "./Homepage.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";

export default {
  component: HompageComponent,
  title: "Screens/Homepage",
  argTypes: {},
  decorators: [
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockPageTransitions,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: HompageComponent,
  props: args,
});

export const Homepage: StoryObj = Template.bind({}) as any;
Homepage.parameters = {
  resolution: {
    openai: null,
  },
};
```

## Renaming to Dashboard

Upon realizing that this was already called the "Dashboard" for Storybook purposes, we rename both files and their imports.

## API keys display

We refactor `src-svelte/src/routes/ApiKeysDisplay.svelte` to contain just loading text, until the API call comes in:

```svelte
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";
  import InfoBox from "$lib/InfoBox.svelte";
  import Service from "./Service.svelte";

  let api_keys = getApiKeys();
</script>

<InfoBox title="API Keys" {...$$restProps}>
  {#await api_keys}
    <span class="loading">...loading</span>
  {:then keys}
    <div class="api-keys">
      <Service name="OpenAI" apiKey={keys.openai?.value} />
    </div>
  {:catch error}
    error: {error}
  {/await}
</InfoBox>

<style>
  span.loading {
    color: var(--color-faded);
  }
</style>

```

We refactor the per-row display code to `src-svelte/src/routes/Service.svelte`:

```svelte
<script lang="ts">
  export let name: string;
  export let apiKey: string | undefined;

  $: active = apiKey !== undefined;
  $: label = active ? "Active" : "Inactive";
</script>

<div class="container">
  <div class="service">{name}</div>
  <div class="api-key" class:active>{label}</div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  .service {
    text-align: left;
    font-family: var(--font-body);
    flex: 1;
  }

  .api-key {
    text-align: center;
    text-transform: uppercase;
    font-family: var(--font-body);
    background-color: gray;
    color: white;
    flex: 1;
    border-radius: var(--corner-roundness);
  }

  .api-key.active {
    background-color: green;
  }
</style>

```

Next, we notice that the new table contents appear immediately instead of fading in slowly. This is because they are no longer subject to the same animations as before. We update `src-svelte/src/lib/InfoBox.svelte` to account for this, first by making `getNodeAnimations` free of side-effects to make it easier to reason about, and then by adding a new `MutationObserver` to the node for the duration of the animation effect:

```ts
    const getNodeAnimations = (currentNode: Element): RevealContent[] => {
      ...
      if (
        ...
      ) {
        return [
          new RevealContent({
            node: currentNode,
            timing: getChildKickoffFraction(currentNode),
          }),
        ];
      } else {
        const revealAnimations: RevealContent[] = [];
        for (const child of currentNode.children) {
          revealAnimations.push(...getNodeAnimations(child));
        }
        return revealAnimations;
      }
    };

    let revealAnimations = getNodeAnimations(node);

    const config = { childList: true, subtree: true };
    const mutationCallback: MutationCallback = () => {
      revealAnimations = getNodeAnimations(node);
      // hide all new nodes immediately
      revealAnimations.forEach((anim) => {
        anim.tickForGlobalTime(0);
      });
    };
    const observer = new MutationObserver(mutationCallback);
    observer.observe(node, config);

    return {
      ...,
      tick: (tGlobalFraction: number) => {
        ...

        if (tGlobalFraction === 1) {
          observer.disconnect();
        }
      },
    };
```

Finally, as a nit, we edit `src-svelte/src/routes/Dashboard.svelte` to change the spacing on the main page by removing `flex: 1;` from `.metadata-contanier` and adding

```css
  .homepage-banner {
    ...
    justify-content: space-evenly;
  }
```

Next, we allow it to wrap on smaller screens, and remove `margin-left: 1rem;` from `.metadata-container` because the spacing should be 1rem even when wrapped vertically:

```css
  .homepage-banner {
    flex-wrap: wrap;
    gap: 1rem;
  }
```

Since there are now multiple Svelte files for this one component, we move `src-svelte/src/routes/ApiKeysDisplay.svelte` to `src-svelte/src/routes/components/api-keys/Display.svelte`, along with associated Storybook stories and Vitest tests. We move it inside `components` to distinguish complicated components that span multiple files from sub-paths in the app URLs. Storybook may need to be restarted due to indexing problems.

We originally renamed `ApiKeysDisplay.test.ts` to `Display.ts`, which doesn't trigger Vitest tests. Once we fix this by renaming it again to `Display.test.ts`, we find that the tests fail because of the changed HTML structure. First, we refactor the `tickFor` function out of `src-svelte/src/routes/AppLayout.test.ts` and into `src-svelte/src/lib/test-helpers.ts`:

```ts
...
import { tick } from "svelte";

...
export async function tickFor(ticks: number) {
  for (let i = 0; i < ticks; i++) {
    await tick();
  }
}
```

Then in `src-svelte/src/routes/AppLayout.test.ts`:

```ts
import { tickFor } from "$lib/test-helpers";
```

We make `src-svelte/src/routes/components/api-keys/Display.svelte` more accessible (and by extension, testable) by adding the "status" role to the relevant elements:

```svelte
<InfoBox ...>
  {#await api_keys}
    <span ... role="status">...loading</span>
  {:then keys}
    ...
  {:catch error}
    <span role="status">error: {error}</span>
  {/await}
</InfoBox>

```

We add "cell" roles to `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
...
    <div class="service" role="cell">{name}</div>
    <div class="api-key" class:active role="cell">{label}</div>
```

Finally, we use all this to fix the tests in `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
...
import { tickFor } from "$lib/test-helpers";
...

async function checkSampleCall(...) {
  ...
  render(ApiKeysDisplay, {});
  await tickFor(3);
}

test("loading by default", async () => {
  ...
  render(ApiKeysDisplay, {});

  const status = screen.getByRole("status");
  expect(status).toHaveTextContent(/^...loading$/);
});

...

test("API key error", async () => {
  ...
  render(ApiKeysDisplay, {});
  expect(spy).toHaveBeenLastCalledWith("get_api_keys");

  await waitFor(() => {
    const status = screen.getByRole("status");
    expect(status).toHaveTextContent(/^error: testing$/);
  });
});
```

The API tests are still failing, so we refactor to use the `TauriInvokePlayback` as seen in `src-svelte/src/routes/AppLayout.test.ts`:

```ts
import { ..., type Mock } from "vitest";
...
import { parseSampleCall, TauriInvokePlayback } from "$lib/sample-call-testing";

describe("API Keys Display", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
  });

  async function checkSampleCall(filename: string, expected_display: string) {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    const getApiKeysCall = parseSampleCall(filename, false);
    playback.addCalls(getApiKeysCall);

    render(ApiKeysDisplay, {});
    await tickFor(3);
    expect(tauriInvokeMock).toBeCalledTimes(1);

    const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
    const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
    await waitFor(() =>
      expect(openAiKeyCell).toHaveTextContent(expected_display),
    );
  }

  ...

  test("no API key set", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
  });

  test("some API key set", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );
  });

  ...
});
```

### Adding a form

We create a mock form at `src-svelte/src/routes/components/api-keys/Form.svelte`, with the requisite CSS transitions:

```svelte
<script lang="ts">
  import { cubicInOut } from "svelte/easing";
  import { animationSpeed, animationsOn } from "$lib/preferences";

  let saveKey = true;

  function growY(node: HTMLElement) {
    const height = node.offsetHeight;
    const duration = $animationsOn ? 100 / $animationSpeed : 0;
    return {
      duration,
      easing: cubicInOut,
      css: (t: number) => {
        const value = height * t;
        return `height: ${value}px;`;
      },
    }
  }
</script>

<div class="container" transition:growY>
  <form>
    <div class="form-row">
      <label for="apiKey">API key:</label>
      <input type="text" id="apiKey" name="apiKey">
    </div>

    <div class="form-row">
      <input type="checkbox" id="saveKey" name="saveKey" checked={saveKey}>
      <label for="saveKey">Save key to:</label>
      <input type="text" id="saveKeyInput" name="saveKeyInput">
    </div>

    <input type="submit" value="Save">
  </form>
</div>

<style>
  .container {
    --horizontal-overshoot: 1rem;
    overflow: hidden;
    box-sizing: border-box;
    margin: 0 calc(-1 * var(--horizontal-overshoot));
  }

  form {
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
    margin: 0.5rem 0;
    padding: 0.5rem var(--horizontal-overshoot);
    background-color: var(--color-background);
    margin-bottom: 0.5rem;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex-wrap: nowrap;
  }

  label {
    white-space: nowrap;
  }

  .form-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  input[type=text] {
    flex: 1;
    min-width: 1rem;
    border: none;
    border-bottom: 1px solid var(--color-border);
    background-color: var(--color-background);
    font-family: var(--font-mono);
    font-weight: bold;
    font-size: 1rem;
    transition: border-bottom calc(0.05s / var(--base-animation-speed)) linear;
  }

  input[type=text]:focus {
    outline: none;
    border-bottom: 1px solid blue;
  }

  input[type=submit] {
    align-self: flex-start;
  }
</style>
```

We add this to `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  import Form from "./Form.svelte";

  ...
  let editing = false;

  function toggleEditing() {
    editing = !editing;
  }

  ...
</script>

<div class="container">
  <div class="row" on:click={toggleEditing} on:keypress={toggleEditing} role="row" tabindex="0">
    <div class="service">{name}</div>
    <div class="api-key" class:active>{label}</div>
  </div>

  {#if editing}
    <Form />
  {/if}
</div>

<style>
  .row {
    ...
    cursor: pointer;
  }

  ...
</style>
```

We should test this as part of our screenshot snapshots. To avoid having to interact with the component as part of our screenshot tests, we'll allow editing to be controlled externally in `src-svelte/src/routes/components/api-keys/Service.svelte`:

```ts
  export let editing = false;
```

Next, we add a demo prop to `src-svelte/src/routes/components/api-keys/Display.svelte` to control the editing:

```svelte
<script lang="ts">
  ...
  export let editDemo = false;
  ...
</script>

...
      <Service ... editing={editDemo} />
```

and a new story to `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
export const Editing: StoryObj = Template.bind({}) as any;
Editing.args = {
  editDemo: true,
};
Editing.parameters = {
  resolution: knownKeys,
  viewport: {
    defaultViewport: "mobile2",
  },
};
```

and a test to `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "dashboard", "api-keys-display"],
    variants: [..., "editing"],
    ...
  },
  ...
];
```

We realize we should display the existing key if it is available, so we edit `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  export let apiKey = "";
  ...
</script>

...
      <input ... value={apiKey}>
```

and pass it in through `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
    <Form {apiKey} />
```

#### Styling the text input

We follow [this example](https://codepen.io/maheshambure21/pen/EozKKy) from [here](https://freefrontend.com/css-input-text/), and refactor and create `src-svelte/src/lib/controls/TextInput.svelte` to style the text input:

```svelte
<script lang="ts">
  export let name: string;
  export let value: string;
</script>

<div class="fancy-input">
  <input type="text" id={name} {name} {value} />
  <span class="focus-border"></span>
</div>

<style>
  .fancy-input {
    position: relative;
    flex: 1;
  }

  input[type="text"] {
    min-width: 1rem;
    width: 100%;
    border: none;
    border-bottom: 1px solid var(--color-border);
    background-color: var(--color-background);
    font-family: var(--font-mono);
    font-weight: bold;
    font-size: 1rem;
  }

  input[type="text"]:focus {
    outline: none;
  }

  input[type="text"] + .focus-border {
    position: absolute;
    bottom: -1px;
    left: 0;
    width: 0;
    height: 2px;
    background-color: blue;
    transition: width calc(0.05s / var(--base-animation-speed)) ease-out;
  }

  input[type="text"]:focus + .focus-border {
    width: 100%;
  }
</style>
```

Then, we use this new input in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  import TextInput from "$lib/controls/TextInput.svelte";

  ...
  export let saveKeyLocation = "";
  ...
</script>

...
  <form>
    <div ...>
      ...
      <TextInput name="apiKey" value={apiKey} />
    </div>

    <div ...>
      ...
      <label for="saveKeyLocation">Save key to:</label>
      <TextInput name="saveKeyLocation" value={saveKeyLocation} />
    </div>

    ...
  </form>
```

#### Styling the button

We create a button element at `src-svelte/src/lib/controls/Button.svelte`:

```svelte
<script lang="ts">
  export let text: string;
</script>

<button class="outer">
  <div class="inner">{text}</div>
</button>

<style>
  .outer, .inner {
    --cut: 7px;
    --background-color: var(--color-background);
    --border-color: #ccc;
    --border: 0.15rem;
    --diagonal-border: calc(var(--border) * 0.8);
    font-size: 0.9rem;
    font-family: var(--font-body);
    border: var(--border) solid var(--border-color);
    text-transform: uppercase;
    background:
      linear-gradient(-45deg, var(--border-color) 0 calc(var(--cut) + var(--diagonal-border)), var(--background-color) 0) bottom right / 50% 100%,
      linear-gradient(135deg, var(--border-color) 0 calc(var(--cut) + var(--diagonal-border)), var(--background-color) 0) top left / 50% 100%;
    background-origin:border-box;
    background-repeat: no-repeat;
    -webkit-mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    -webkit-mask-size: 51% 100%;
    -webkit-mask-repeat: no-repeat;
    mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
    transition: all calc(0.05s / var(--base-animation-speed)) ease-out;
  }

  .inner {
    padding: 5px 10px;
  }

  .inner:hover {
    filter: brightness(1.05);
  }

  .inner:active {
    transform: translateY(0.08rem) scale(0.98);
  }

  .outer {
    --background-color: #eee;
    --border: 2px;
    --diagonal-border: 2.5px;
    --cut: 8px;
    padding: 1px;
    display: inline-block;
  }
</style>

```

Note that we use a brightness filter on hover because changes to the background gradient colors don't get animated. The brightness filter lights up the entire element, including borders, but that is an acceptable look for our purposes.

We also create `src-svelte/src/lib/controls/Button.stories.ts`:

```ts
import Button from "./Button.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";

export default {
  component: Button,
  title: "Reusable/Button",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Button,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  text: "Simulate",
};

```

We also add this as a test to `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["reusable", "button"],
    variants: ["regular"],
  },
  ...
];
```

Finally, we use this new element in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  import Button from "$lib/controls/Button.svelte";
  ...
</script>

...
      <div class="save-button">
        <Button text="Save" />
      </div>
...

<style>
  ...
  .save-button {
    align-self: flex-start;
  }
</style>
```

#### Fixing corner overlap and bottom shadow

If we play the animation slowly, we see that 1) the corner of the form overlaps the supposed border of the div for a while until the animation finishes, and 2) there is no shadow on the bottom of the inset until the animation finishes playing. We fix this by moving the box shadow from the form to a containing element, and by first growing the padding before growing the container for the form:

```svelte
<script lang="ts">
  ...
  function growY(node: HTMLElement) {
    const rem = 18;
    const totalFinalPadding = 1 * rem;
    ...
    return {
      ...
      css: (t: number) => {
        const totalHeight = height * t;
        const totalCurrentPadding = Math.min(totalFinalPadding, totalHeight);
        const contentHeight = totalHeight - totalCurrentPadding;
        return `
          --vertical-padding: ${totalCurrentPadding / 2}px;
          --form-height: ${contentHeight}px;
        `;
      },
    };
  }
</script>

<div class="container" transition:growY>
  <div class="inset-container">
    <form>
      ...
    </form>
  </div>
</div>

<style>
  .container {
    --form-height: 100%;
    --vertical-padding: 0.5rem;
    --horizontal-overshoot: 1rem;
    overflow: hidden;
    margin: 0 calc(-1 * var(--horizontal-overshoot));
    padding: var(--vertical-padding) 0;
  }

  .inset-container {
    height: var(--form-height);
    overflow: hidden;
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
    background-color: var(--color-background);
  }

  form {
    padding: 0.5rem var(--horizontal-overshoot);
    ...
  }
  ...
</style>
```

#### Fixing animation on Firefox

We notice that the animation isn't running for some reason on Firefox. As such, we implement it by manually changing the style:

```ts
  function growY(node: HTMLElement) {
    ...
    const duration = $animationsOn ? 200 / $animationSpeed : 0;
    return {
      ...
      tick: (t: number) => {
        ...
        node.style.setProperty("--vertical-padding", `${totalCurrentPadding / 2}px`);
        node.style.setProperty("--form-height", `${contentHeight}px`);
      }
    };
  }
```

Note that we're now using the `tick` function instead of the `css` function, and we doubled the duration to make its speed feel consistent with the rest of the page.

#### Determining the shell

The shell will determine where the API key is saved. There are a number of ways to do this, such as by checking [`SHELL`](https://stackoverflow.com/a/3327022) or by checking more specific environment variables such as [`ZSH_NAME`](https://unix.stackexchange.com/a/522909).

In any case, we start off by adding the package

```bash
$ cargo add shellexpand
```

We create a new file at `src-tauri/src/commands/system.rs`:

```rust

use serde::{Deserialize, Serialize};
use specta::specta;
use specta::Type;

use std::env;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct SystemInfo {
    shell: Option<Shell>,
    shell_init_file: Option<String>,
}

fn get_shell() -> Option<Shell> {
  if let Ok(shell) = env::var("SHELL") {
      if shell.ends_with("/zsh") {
          return Some(Shell::Zsh);
      }
      if shell.ends_with("/bash") {
          return Some(Shell::Bash);
      }
  }

  None
}

fn get_shell_init_file(shell: &Shell) -> String {
    let relative_file = match shell {
        Shell::Bash => "~/.bashrc",
        Shell::Zsh => "~/.zshrc",
    };
    shellexpand::tilde(relative_file).to_string()
}

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    let shell = get_shell();
    let shell_init_file = shell.as_ref().map(|s| get_shell_init_file(s));

    SystemInfo {
        shell,
        shell_init_file,
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_determine_shell() {
      let shell = get_shell();
      println!("Determined shell to be {:?}", shell);
      assert!(shell.is_some());
  }

  #[test]
  fn test_can_predict_shell_init() {
      let shell = Shell::Zsh;
      let shell_init_file = get_shell_init_file(&shell);
      println!("Shell init file is {}", shell_init_file);
      assert!(shell_init_file.starts_with("/"));
      assert!(shell_init_file.ends_with("/.zshrc"));
  }
}

```

Note that we want to test that the predicted shell init file path has successfully had the tilde replaced by the absolute path, but we want the test to pass even on different machines, so we use two separate assertions.

And as usual, edit `src-tauri/src/commands/mod.rs`:

```rust
...
mod system;

...
pub use system::get_system_info;
```

And add it to the main list of commands at `src-tauri/src/main.rs`:

```rust
...
use commands::{..., get_system_info};

...

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            ...,
            get_system_info
        ],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![
            ...,
            get_system_info
        ])
        ...
```

As usual, `src-svelte/src/lib/bindings.ts` is automatically updated with the next time the updated development version of the app is run.

We should test this as usual, to ensure that we are generating the exact response we're expecting. Due to the amount of system-dependent calls made, we'll avoid mocking as done in `src-tauri/src/commands/greet.rs`, but instead simply check that serialization of an example system info struct will result in the expected API call response.

As such, we create `src-tauri/api/sample-calls/get_system_info-linux.yaml`:

```yaml
request: ["get_system_info"]
response: >
  {
    "shell": "Zsh",
    "shell_init_file": "/root/.zshrc"
  }

```

and test this in `src-tauri/src/commands/system.rs`:

```rust
...

#[cfg(test)]
mod tests {
  ...
  use crate::sample_call::SampleCall;
    use std::fs;

    fn parse_system_info(response_str: &str) -> SystemInfo {
        serde_json::from_str(response_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_get_system_info_sample(file_prefix: &str, actual_info: &SystemInfo) {
        let system_info_sample = read_sample(file_prefix);
        assert_eq!(system_info_sample.request, vec!["get_system_info"]);

        let expected_info = parse_system_info(&system_info_sample.response);
        assert_eq!(actual_info, &expected_info);
    }

    ...

    #[test]
    fn test_get_linux_system_info() {
        let system_info = SystemInfo {
            shell: Some(Shell::Zsh),
            shell_init_file: Some("/root/.zshrc".to_string()),
        };

        check_get_system_info_sample(
            "./api/sample-calls/get_system_info-linux.yaml",
            &system_info,
        );
    }
}
```

#### Simplifying the ApiKeys data structure

In an act of overengineering, we had defined the `ApiKeys` to take in an `ApiKey` struct that included information about the provenance of that API key. Instead, we now edit `src-tauri/src/setup/api_keys.rs` to define it more simply:

```rs
pub struct ApiKeys {
    pub openai: Option<String>,
}

pub fn setup_api_keys() -> ApiKeys {
    ...
        api_keys.openai = Some(openai_api_key);
    ...
}

#[cfg(test)]
mod tests {
  ...
    #[test]
    fn test_get_present_api_keys() {
        temp_env::with_var("OPENAI_API_KEY", Some("dummy"), || {
            ...
            assert_eq!(api_keys.openai, Some("dummy".to_string()));
        });
    }
}
```

We'll have to also edit `src-tauri/src/commands/keys.rs`:

```rs
    #[test]
    fn test_get_openai_key() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some("0p3n41-4p1-k3y".to_string()),
        }));

        ...
    }
```

We have to update our sample API call at `src-tauri/api/sample-calls/get_api_keys-openai.yaml`:

```yaml
request: ["get_api_keys"]
response: >
  {
    "openai": "0p3n41-4p1-k3y"
  }

```

`src-svelte/src/lib/bindings.ts` should automatically be updated by Specta.

Now on the frontend, we update `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
const knownKeys: ApiKeys = {
  openai: "sk-1234567890",
};
```

and `src-svelte/src/routes/components/api-keys/Display.svelte`:

```svelte
    <div ... role="table">
      <Service ... apiKey={keys.openai} ... />
    </div>
```

and `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  ...
  export let apiKey: string | null;
  ...
  $: active = apiKey !== null;
  ...
</script>

...
    <Form apiKey={apiKey ?? ""} />
```

We realize that tests are still failing because we have the wrong sample API call. We take inspiration from the latest incarnation of API calling tests done in `src-tauri/src/commands/preferences/read.rs` (the updates to which we should've propagated to other API calling tests), and edit `src-tauri/src/commands/keys.rs` accordingly:

```rs
    fn check_get_preferences_sample(file_prefix: &str, preferences_dir: &str) {
        ...

        let actual_result = get_preferences_helper(&Some(preferences_dir.into()));
        let actual_json = serde_json::to_string_pretty(&actual_result).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);
    }
```

and we edit `src-tauri/api/sample-calls/get_api_keys-empty.yaml` accordingly to match:

```yaml
request: ["get_api_keys"]
response: >
  {
    "openai": null
  }

```

Now even the frontend tests pass as well.

#### Getting temporary test directory

In preparation for the next step, we refactor temporary test directory logic out of `src-tauri/src/commands/preferences/write.rs`. We put this logic into `src-tauri/src/test_helpers.rs`:

```rs
use std::path::PathBuf;
use std::env;
use std::fs;

pub fn get_temp_test_dir(test_name: &str) -> PathBuf {
    let mut test_dir = env::temp_dir();
    test_dir.push("zamm/tests");
    test_dir.push(test_name);
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)
            .expect("Can't reset test preferences dir");
    }
    test_dir
}

```

and refer to the module in `src-tauri/src/main.rs`:

```rs
...
#[cfg(test)]
mod test_helpers;
...
```

Now we edit `src-tauri/src/commands/preferences/write.rs` to import the new logic:

```rs
...

#[cfg(test)]
mod tests {
    ...
    use crate::test_helpers::get_temp_test_dir;
    ...

    fn check_set_preferences_sample(
        file_prefix: &str,
        ...
    ) {
        ...

        let test_preferences_dir = get_temp_test_dir(
            PathBuf::from(file_prefix)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        ...
    }
}
```

#### Updating the API keys

First, we refactor the `command/keys.rs` module by making it into a proper folder. We move `src-tauri/src/commands/keys.rs` to `src-tauri/src/commands/keys/get.rs` like so:

```rs
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use tauri::State;

fn get_api_keys_helper(zamm_api_keys: &ZammApiKeys) -> ApiKeys {
    zamm_api_keys.0.lock().unwrap().clone()
}

#[tauri::command(async)]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    get_api_keys_helper(&api_keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use std::sync::Mutex;

    use std::fs;

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_get_api_keys_sample(file_prefix: &str, rust_input: &ZammApiKeys) {
        let greet_sample = read_sample(file_prefix);
        assert_eq!(greet_sample.request, vec!["get_api_keys"]);

        let actual_result = get_api_keys_helper(rust_input);
        let actual_json = serde_json::to_string_pretty(&actual_result).unwrap();
        let expected_json = greet_sample.response.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_get_empty_keys() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-empty.yaml",
            &api_keys,
        );
    }

    #[test]
    fn test_get_openai_key() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some("0p3n41-4p1-k3y".to_string()),
        }));

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-openai.yaml",
            &api_keys,
        );
    }
}

```

Now we make it a module by creating `src-tauri/src/commands/keys/mod.rs`:

```rs
mod get;
mod set;

pub use get::get_api_keys;
pub use set::set_api_key;

```

and we create `src-tauri/src/commands/keys/set.rs` to export the corresponding `set_api_key` function:

```rs
use crate::commands::errors::ZammResult;
use crate::setup::api_keys::{ApiKeys, Service};
use crate::ZammApiKeys;
use specta::specta;
use tauri::State;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

fn set_api_key_helper(
    api_keys: &mut ApiKeys,
    filename: Option<&str>,
    service: &Service,
    api_key: String,
) -> ZammResult<()> {
    // write new API key to disk before we can no longer borrow it
    let init_update_result = || -> ZammResult<()> {
        if let Some(filename) = filename {
            let ends_in_newline = {
                if Path::new(filename).exists() {
                    let mut file = OpenOptions::new().read(true).open(filename)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    contents.ends_with('\n')
                } else {
                    true // no need to start the file with a newline later
                }
            };

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)?;
            if !ends_in_newline {
                writeln!(file)?;
            }
            writeln!(file, "export OPENAI_API_KEY=\"{}\"", api_key)?;
        }
        Ok(())
    }();
    // assign ownership of new API key string to in-memory API keys
    api_keys.update(service, api_key);
    init_update_result
}

#[tauri::command(async)]
#[specta]
pub fn set_api_key(
    api_keys: State<ZammApiKeys>,
    filename: Option<&str>,
    service: Service,
    api_key: String,
) -> ZammResult<()> {
    let mut api_keys = api_keys.0.lock().unwrap();
    set_api_key_helper(&mut api_keys, filename, &service, api_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::get_temp_test_dir;
    use serde::{Deserialize, Serialize};

    use std::fs;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetApiKeyRequest {
        filename: Option<String>,
        service: Service,
        api_key: String,
    }

    fn parse_request(request_str: &str) -> SetApiKeyRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_set_api_key_sample(sample_file: &str, mut existing_api_keys: ApiKeys) {
        let sample = read_sample(sample_file);
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "set_api_key");

        let request = parse_request(&sample.request[1]);
        let request_path = request.filename.map(|f| PathBuf::from(&f));
        let test_init_file = request_path.as_ref().map(|p| {
            let sample_file_directory = p.parent().unwrap().to_str().unwrap();
            let test_name = format!("set_api_key/{}", sample_file_directory);
            let temp_init_dir = get_temp_test_dir(&test_name);
            let init_file = temp_init_dir.join(p.file_name().unwrap());
            println!(
                "Test will be performed on shell init file at {}",
                init_file.display()
            );

            let starting_init_file = Path::new("api/sample-init-files").join(p);
            if PathBuf::from(&starting_init_file).exists() {
                fs::copy(&starting_init_file, &init_file).unwrap();
            }

            init_file
        });

        let actual_result = set_api_key_helper(
            &mut existing_api_keys,
            test_init_file.as_ref().map(|f| f.to_str().unwrap()),
            &request.service,
            request.api_key.clone(),
        );
        // check that the API call returns a success signal
        assert!(
            actual_result.is_ok(),
            "API call failed: {:?}",
            actual_result
        );

        // check that the API call returns the expected JSON
        let actual_json =
            serde_json::to_string_pretty(&actual_result.unwrap()).unwrap();
        let expected_json = sample.response.trim();
        assert_eq!(actual_json, expected_json);

        // check that the API call actually modified the in-memory API keys
        assert_eq!(existing_api_keys.openai, Some(request.api_key));

        // check that the API call successfully wrote the API keys to disk, if asked to
        if let Some(p) = request_path {
            let expected_init_file = Path::new("api/sample-init-files")
                .join(p)
                .with_file_name("expected.bashrc");

            let resulting_contents = fs::read_to_string(test_init_file.unwrap())
                .expect("Test shell init file doesn't exist");
            let expected_contents = fs::read_to_string(&expected_init_file)
                .unwrap_or_else(|_| {
                    panic!(
                        "No gold init file found at {}",
                        expected_init_file.display()
                    )
                });
            assert_eq!(resulting_contents.trim(), expected_contents.trim());
        }
    }

    #[test]
    fn test_write_new_init_file() {
        check_set_api_key_sample(
            "api/sample-calls/set_api_key-no-file.yaml",
            ApiKeys::default(),
        );
    }

    #[test]
    fn test_overwrite_existing_init_file_with_newline() {
        check_set_api_key_sample(
            "api/sample-calls/set_api_key-existing-with-newline.yaml",
            ApiKeys::default(),
        );
    }

    #[test]
    fn test_overwrite_existing_init_file_no_newline() {
        check_set_api_key_sample(
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            ApiKeys::default(),
        );
    }

    #[test]
    fn test_no_disk_write() {
        check_set_api_key_sample(
            "api/sample-calls/set_api_key-no-disk-write.yaml",
            ApiKeys::default(),
        );
    }
}

```

Note that we take much inspiration from `src-tauri/src/commands/preferences/write.rs` for also using a similar pattern of copying files over to a temporary test directory and then comparing file results. Two changes does need to be made to `src-tauri/src/test_helpers.rs` to complete the above refactor:

```rs
pub fn get_temp_test_dir(test_name: &str) -> PathBuf {
    ...
    if ... {
        ...expect("Can't reset temp test dir");
    }
    fs::create_dir_all(&test_dir).expect("Can't create temp test dir");
    test_dir
}
```

Namely, the error message strings need to be updated, and the logic to actually create the test directory after first checking if it's removed should be shared. We remove the corresponding logic from `src-tauri/src/commands/preferences/write.rs` now that we have moved it here.

Note that we also need to modify `src-tauri/src/setup/api_keys.rs` to allow us to actually update the API keys based on the string key:

```rs
...
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Service {
    OpenAI,
}

...

impl ApiKeys {
    pub fn update(&mut self, service: &Service, key: String) {
        match service {
            Service::OpenAI => self.openai = Some(key),
        }
    }
}

```

##### Test cases

Note that we also have to define the test cases mentioned in the file above.

###### Update API key without writing to disk

This one is simple. We create `src-tauri/api/sample-calls/set_api_key-no-disk-write.yaml` like so:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": null,
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null"

```

###### Update API key while creating a new file

We create `src-tauri/api/sample-calls/set_api_key-no-file.yaml` like so:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "no-file/.bashrc",
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null"

```

This time, we should define what we expect the result to look like by creating `src-tauri/api/sample-init-files/no-file/expected.bashrc`:

```bash
export OPENAI_API_KEY="0p3n41-4p1-k3y"

```

###### Update API key while updating an existing file with a newline at the end

We create `src-tauri/api/sample-calls/set_api_key-existing-with-newline.yaml`:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "with-newline/.bashrc",
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null"

```

and create the initial file as such at `src-tauri/api/sample-init-files/with-newline/.bashrc`:

```bash
# dummy initial bashrc file
# check that newline at end of file doesn't result in ugly whitespace
export SOME_ENV_VAR="some value"

```

That API call should result in `src-tauri/api/sample-init-files/with-newline/expected.bashrc`:

```bash
# dummy initial bashrc file
# check that newline at end of file doesn't result in ugly whitespace
export SOME_ENV_VAR="some value"
export OPENAI_API_KEY="0p3n41-4p1-k3y"

```

###### Update API key while updating an existing file without a newline at the end

We create `src-tauri/api/sample-calls/set_api_key-existing-no-newline.yaml`:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "no-newline/.bashrc",
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null"

```

As described, the sample file at `src-tauri/api/sample-init-files/no-newline/.bashrc` will start off with no newline:

```bash
# dummy initial bashrc file
export SOME_ENV_VAR="some value"
# no newline at end of file to check that it still works
```

and the API call should result in this example file we define at `src-tauri/api/sample-init-files/no-newline/expected.bashrc`:

```bash
# dummy initial bashrc file
export SOME_ENV_VAR="some value"
# no newline at end of file to check that it still works
export OPENAI_API_KEY="0p3n41-4p1-k3y"

```

##### Wrapping up the command

We add the new command to `src-tauri/src/commands/mod.rs`:

```rust
...
pub use keys::{get_api_keys, set_api_key};
...
```

and then to `src-tauri/src/main.rs`:

```rs
...
use commands::{
    ..., set_api_key,
    ...,
};

...

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            ...
            set_api_key,
            ...
        ],
        ...
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![
            ...,
            set_api_key,
            ...
        ])
        ...
}

```

As usual, `src-svelte/src/lib/bindings.ts` should be updated automatically by Specta.

#### Loading shell info

We want to load shell info on app startup. Let's do this in the metadata component. We first move it to the `src-svelte/src/routes/components` folder, just like the API keys display was.

Then, we refactor out the loading part of `Display.svelte` into `src-svelte/src/lib/Loading.svelte`:

```svelte
<span class="loading" role="status">...loading</span>

<style>
  span.loading {
    color: var(--color-faded);
  }
</style>

```

and then we import this new definition in the original file at `src-svelte/src/routes/components/api-keys/Display.svelte`:

```svelte
<script lang="ts">
  ...
  import Loading from "$lib/Loading.svelte";
  ...
</script>

<InfoBox ...>
  {#await apiKeys}
    <Loading />
  {:then keys}
    ...
  {/await}
</InfoBox>
```

Now we use this same pattern in `src-svelte/src/routes/components/Metadata.svelte`:

```svelte
<script lang="ts">
  ...
  import Loading from "$lib/Loading.svelte";
  import { getSystemInfo } from "$lib/bindings";

  let systemInfoCall = getSystemInfo();
</script>

<InfoBox title="System Info" {...$$restProps}>
  {#await systemInfoCall}
    <Loading />
  {:then systemInfo}
    <table>
      ...
    </table>
  {:catch error}
    <span role="status">error: {error}</span>
  {/await}
</InfoBox>

...
```

It appears there is not really a better way to cut down on the boilerplate for the pattern here, as evidence by [this question](https://stackoverflow.com/q/64023421).

Next, we update the stories at `src-svelte/src/routes/components/Metadata.stories.ts` to also display the loading page:

```ts
...
import type { SystemInfo } from "$lib/bindings";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";

export default {
  ...,
  decorators: [TauriInvokeDecorator],
};

...

const linuxInfo: SystemInfo = {
  shell: "Zsh",
  shell_init_file: "/home/john.smith/.zshrc",
};

export const Loaded: StoryObj = Template.bind({}) as any;
Loaded.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
  resolution: linuxInfo,
};

export const Loading: StoryObj = Template.bind({}) as any;
Loading.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
  resolution: linuxInfo,
  shouldWait: true,
};
```

Finally, we'll update the tests at `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "dashboard", "metadata"],
    variants: ["loading", "loaded"],
    screenshotEntireBody: true,
  },
  ...
];
```

Now, to make sure the diffs work correctly, we rename `src-svelte/screenshots/baseline/screens/dashboard/metadata/metadata.png` to `src-svelte/screenshots/baseline/screens/dashboard/metadata/loaded.png`.

Next, we'll want to make sure that the API call is being triggered correctly on page load, and updating the HTML as expected. We copy and modify the code from `src-svelte/src/routes/components/api-keys/Display.test.ts`, and end up creating `src-svelte/src/routes/components/Metadata.test.ts` as such:

```ts
import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import Metadata from "./Metadata.svelte";
import { within, waitFor } from "@testing-library/dom";
import { parseSampleCall, TauriInvokePlayback } from "$lib/sample-call-testing";
import { tickFor } from "$lib/test-helpers";

describe("Metadata", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
  });

  test("loading by default", async () => {
    const getSystemInfoCall = parseSampleCall("../src-tauri/api/sample-calls/get_system_info-linux.yaml", false);
    playback.addCalls(getSystemInfoCall);

    render(Metadata, {});

    const status = screen.getByRole("status");
    expect(status).toHaveTextContent(/^...loading$/);
  });

  test("linux system info returned", async () => {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    const getSystemInfoCall = parseSampleCall("../src-tauri/api/sample-calls/get_system_info-linux.yaml", false);
    playback.addCalls(getSystemInfoCall);

    render(Metadata, {});
    await tickFor(3);
    expect(tauriInvokeMock).toBeCalledTimes(1);

    const shellRow = screen.getByRole("row", { name: /Shell/ });
    const shellValueCell = within(shellRow).getAllByRole("cell")[1];
    await waitFor(() =>
      expect(shellValueCell).toHaveTextContent("Zsh"),
    );
  });

  test("API key error", async () => {
    const spy = vi.spyOn(window, "__TAURI_INVOKE__");
    expect(spy).not.toHaveBeenCalled();
    tauriInvokeMock.mockRejectedValueOnce("testing");

    render(Metadata, {});
    expect(spy).toHaveBeenLastCalledWith("get_system_info");

    await waitFor(() => {
      const status = screen.getByRole("status");
      expect(status).toHaveTextContent(/^error: testing$/);
    });
  });
});

```

In the course of adapting the old test file to the new case, we made some changes to the mocking functionality. We port these changes back into `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
...

  test("loading by default", async () => {
    const getApiKeysCall = parseSampleCall("../src-tauri/api/sample-calls/get_api_keys-empty.yaml", false);
    playback.addCalls(getApiKeysCall);

    render(ApiKeysDisplay, {});

    ...
  });

...
```

#### Refactoring playback tests

Before we go further, we observe an opportunity to simplify sample call parsing in `src-svelte/src/lib/sample-call-testing.ts` by getting rid of the `argumentsExpected` argument:

```ts
export function parseSampleCall(sampleFile: string): ParsedCall {
  ...
  assert(rawSample.request.length <= 2);
  const parsedRequest = rawSample.request.length === 2
    ...;
  ...
```

After all, the arguments (or lack thereof) to the API call will be checked later, so there is no need to make this check at sample parsing time. Now, the resulting calls at `src-svelte/src/routes/AppLayout.test.ts`, `src-svelte/src/routes/components/Metadata.test.ts`, `src-svelte/src/routes/components/api-keys/Display.test.ts`, and `src-svelte/src/routes/settings/Settings.test.ts` will also have to be modified.

Now we can edit `src-svelte/src/lib/sample-call-testing.ts` yet again to add a simpler function for registering sample call files:

```ts
export class TauriInvokePlayback {
  ...
  addSamples(...sampleFiles: string[]): void {
    const calls = sampleFiles.map((filename) => parseSampleCall(filename));
    this.addCalls(...calls);
  }
}
```

Now we can make refactors such as changing this:

```ts
    const getSystemInfoCall = parseSampleCall(
      "../src-tauri/api/sample-calls/get_system_info-linux.yaml",
    );
    playback.addCalls(getSystemInfoCall);
```

into this:

```ts
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_system_info-linux.yaml",
    );
```

We once again make these changes in every one of the above files except for `Settings.test.ts`. In that particular file, the shorter names for the function calls make the test more interpretable, and therefore the existing call to `playback.addCalls` can be kept.

#### Sample API playback in stories

Now that we have performed the above refactor, we can use the test playback mechanism in the Storybook stories as well. We modify `src-svelte/src/lib/__mocks__/invoke.ts`:

```ts
import { TauriInvokePlayback } from "$lib/sample-call-testing";

let playback = new TauriInvokePlayback();
let nextShouldWait = false;

function mockInvokeFn<T>(command: string, args?: Record<string, string>): Promise<T> {
  if (nextShouldWait) {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(null as T);
      }, 1_000_000); // the re-render never happens, so any timeout is fine
   });
  } else {
    let allArgs = args === undefined ? [command] : [command, args];
    return playback.mockCall(...allArgs) as Promise<T>;
  }
};

window.__TAURI_INVOKE__ = mockInvokeFn;

interface TauriInvokeArgs {
  sampleCallFiles?: string[];
  ...
}

const TauriInvokeDecorator: Decorator = (
  ...
) => {
  ...
  const { sampleCallFiles, shouldWait } = parameters as TauriInvokeArgs;
  if (sampleCallFiles !== undefined) {
    playback.addSamples(...sampleCallFiles);
  }
  ...
};

...
```

and modify `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
...

const unknownKeys = [
  "src-tauri/api/sample-calls/get_api_keys-empty.yaml",
];

const knownKeys = [
  "src-tauri/api/sample-calls/get_api_keys-openai.yaml",
];

export const Loading: StoryObj = Template.bind({}) as any;
Loading.parameters = {
  shouldWait: true,
  viewport: {
    defaultViewport: "mobile2",
  },
};

export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  sampleCallFiles: unknownKeys,
  ...
};

...
```

Our first attempt at this gives us the error

```ts
Vitest failed to access its internal state.


One of the following is possible:
- "vitest" is imported directly without running "vitest" command
- "vitest" is imported inside "globalSetup" (to fix this, use "setupFiles" instead, because "globalSetup" runs in a different context)
- Otherwise, it might be a Vitest bug. Please report it to https://github.com/vitest-dev/vitest/issues
```

This is because `parseSampleCall` and `TauriInvokePlayback` both make use of Vitest's `assert` function, but that is not available in Storybook. We try to copy over Vitest's `Assert` type definition:

```ts
interface Assert {
  (expression: any, message?: string): asserts expression;
}
```

but this only results in the error

```
Assertions require every name in the call target to be declared with an explicit type annotation.
```

We see that [this](https://stackoverflow.com/a/71617709) is a proposed workaround, but for simplicity we define the assert ourselves in `src-svelte/src/lib/sample-call-testing.ts`:

```ts
function customAssert(condition: boolean, message?: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

...

export function parseSampleCall(sampleFile: string): ParsedCall {
  ...
  customAssert(rawSample.request.length <= 2);
  ...
}

export class TauriInvokePlayback {
  ...

  mockCall(
    ...
  ): Promise<Record<string, string>> {
    ...
    customAssert(
      matchingCallIndex !== -1,
      `No matching call found ...`
    );
    ...
  }

  ...
}
```

Unfortunately, we now run into the problem

```
Module "fs" has been externalized for browser compatibility. Cannot access "fs.readFileSync" in client code.  See http://vitejs.dev/guide/troubleshooting.html#module-externalized-for-browser-compatibility for more details.
```

Even initializing `playback` in the story file instead of the decorator doesn't work. Therefore, we'll have to make a network request instead when loading sample call files in Storybook. We see that we can determine whether or not we're running in the browser context [here](https://stackoverflow.com/a/34550964), and avoid changing too much of our code by using `XMLHttpRequest` synchronously as described [here](https://stackoverflow.com/a/72561702). We now make the corresponding changes to `src-svelte/src/lib/sample-call-testing.ts`:

```ts
...

function loadYamlFromNetwork(url: string): string {
  const request = new XMLHttpRequest();
  request.open("GET", url, false);
  request.send(null);
  return request.responseText;
}

export function parseSampleCall(sampleFile: string): ParsedCall {
  const sample_call_yaml = typeof process === "object"
    ? fs.readFileSync(sampleFile, "utf-8")
    : loadYamlFromNetwork(sampleFile);
  ...
}
```

and tell Storybook how to find these API calls by editing `src-svelte/.storybook/main.ts`:

```ts
const config: StorybookConfig = {
  ...
  staticDirs: [..., "../../src-tauri"],
  ...
};
```

We edit `src-svelte/src/routes/components/api-keys/Display.stories.ts` again to point to the new URL:

```ts
...

const unknownKeys = [
  "/api/sample-calls/get_api_keys-empty.yaml",
];

const knownKeys = [
  "/api/sample-calls/get_api_keys-openai.yaml",
];

...
```

After restarting Storybook, we notice that the sample calls don't work as expected when navigating between pages, so we edit `src-svelte/src/lib/__mocks__/invoke.ts`:

```ts
...
let playback: TauriInvokePlayback;
...

const TauriInvokeDecorator: Decorator = (
  ...
) => {
  ...
  playback = new TauriInvokePlayback();
  if (...) {
    playback.addSamples(...sampleCallFiles);
  }
  ...
};

```

If we want a "soft" failure where our own UI (instead of the Storybook UI) displays the error around not finding a matching call, then we can edit `src-svelte/src/lib/sample-call-testing.ts`:

```ts
export class TauriInvokePlayback {
  ...
  mockCall(
    ...
  ): Promise<Record<string, string>> {
    ...
    if (matchingCallIndex === -1) {
      const candidates = this.unmatchedCalls
        .map((call) => JSON.stringify(call.request))
        .join("\n");
      const errorMessage = `No matching call found for ${jsonArgs}.\nCandidates are ${candidates}`;
      if (typeof process === "object") {
        throw new Error(errorMessage);
      } else {
        return Promise.reject(errorMessage);
      }
    }
    ...
  }
}
```

We continue editing files where `TauriInvokeDecorator` appears, such as `src-svelte/src/routes/Dashboard.stories.ts` (which can now finally support two different kinds of API calls, which was the whole point of this refactor):

```ts
export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_keys-empty.yaml",
    "/api/sample-calls/get_system_info-linux.yaml",
  ],
};
```

#### Fixing the info box grow animation on API call resolution

When the API call resolves, the info box grows in size because the loading indicator is replaced by a visualization of the API call data. The border box growth animation should dynamically update to reflect the change in the size of the info box's child nodes. To do this, we first make it possible to dynamically update the final stopping value of `PropertyAnimation` in `src-svelte/src/lib/animation-timing.ts`:

```ts
...

export class PropertyAnimation extends SubAnimation<string> {
  max: number;

  constructor(anim: {
    ...
  }) {
    ...
    const css = (t: number) => {
      ...
      const growth = this.max - anim.min;
      ...
    };
    ...

    this.max = anim.max;
  }
}
```

Next, we'll make use of this in `src-svelte/src/lib/InfoBox.svelte`. First, let's handle the border box growth:

1. We use the parent node instead of the border box node because the parent node dimensions will get updated when its children change, but the border box's dimensions cannot be trusted while it is in the middle of its animation.
2. We copy the mutation observer code from the existing logic for the content reveal.
3. We finally realize that the reason the `css` animation function doesn't work in Firefox is that it gets precomputed beforehand, which means that the mutation observer gets deregistered early before any mutations occur. Even if it didn't get deregistered early, the fact that it gets precomputed means that the old growth values never get updated. This is why we change the returned object to use `tick` instead of `css` to set the border box style.

```ts
  function revealOutline(
    ...
  ): TransitionConfig {
    const parentNode = node.parentNode as Element;
    const actualWidth = parentNode.clientWidth;
    const actualHeight = parentNode.clientHeight;
    ...
    const contentNode = parentNode.querySelector(".info-content") as Element;
    const observer = new MutationObserver(() => {
      growWidth.max = parentNode.clientWidth;
      growHeight.max = parentNode.clientHeight;
    });
    observer.observe(contentNode, { childList: true, subtree: true });

    return {
      ...
      tick: (tGlobalFraction: number) => {
        const width = growWidth.tickForGlobalTime(tGlobalFraction);
        const height = growHeight.tickForGlobalTime(tGlobalFraction);
        node.setAttribute(
          "style",
          width + height,
        );

        if (tGlobalFraction === 1) {
          observer.disconnect();
        }
      },
    };
  }
```

While this dynamically fixes the border box growth, the content still waits until the very end to reveal itself, as if the mutation observer for the content reveal effect isn't doing anything. This is because the updates made there are still based on the old dimensions of the border box. We get rid of `infoBoxHeight` and `infoBoxTop`, and instead dynamically feed that information as required:

```ts
  function revealInfoBox(node: Element, timing: InfoBoxTiming) {
    ...
    const getChildKickoffFraction = (child: Element, border: DOMRect) => {
      const childRect = child.getBoundingClientRect();
      const childBottomYRelativeToInfoBox =
        childRect.top + childRect.height - border.top;
      const equivalentYProgress = inverseCubicInOut(
        childBottomYRelativeToInfoBox / border.height,
      );
      ...
    };

    const getNodeAnimations = (currentNode: Element, root?: DOMRect): RevealContent[] => {
      if (root === undefined) {
        root = currentNode.getBoundingClientRect();
      }
      ...
      if (
        ...
      ) {
        return [
          new RevealContent({
            ...,
            timing: getChildKickoffFraction(currentNode, root),
          }),
        ];
      } else {
        ...
        for (const child of currentNode.children) {
          revealAnimations.push(...getNodeAnimations(child, root));
        }
        ...
      }
    };
    ...
  }
```

Note that both `MutationObserver`s now are listening for the exact same changes to the exact same node.

Unfortunately we see that the width growth is not fixed on Chrome, and only when refreshing the page for the first time. We find that we might as well remove the observer code altogether:

```ts
  function revealOutline(
    node: Element,
    timing: BorderBoxTiming,
  ): TransitionConfig {
    ...
    const growWidth = new PropertyAnimation({
      ...
    });

    const growHeight = new PropertyAnimation({
      ...
    });

    return {
      ...
      tick: (tGlobalFraction: number) => {
        growWidth.max = parentNode.clientWidth;
        growHeight.max = parentNode.clientHeight;
        ...

        if (tGlobalFraction === 1) {
          node.removeAttribute("style");
        }
      },
    };
  }
```

#### Accessing the loaded metadata from the edit form

We want the loaded metadata to be available for use in the API keys edit form. As such, we create a store at `src-svelte/src/lib/system-info.ts`:

```ts
import { writable, type Writable } from "svelte/store";
import type { SystemInfo } from "./bindings";

export const systemInfo: Writable<SystemInfo | undefined> = writable(undefined);

```

and then we use this store in `src-svelte/src/routes/components/Metadata.svelte`:

```ts
  ...
  import { systemInfo } from "$lib/system-info";

  let systemInfoCall = getSystemInfo();
  systemInfoCall.then((result) => {
    systemInfo.set(result);
  }).catch((error) => {
    console.error(`Could not retrieve system info: ${error}`);
  });
```

and we test that this is set in `src-svelte/src/routes/components/Metadata.test.ts`:

```ts
...
import { systemInfo } from "$lib/system-info";
import { get } from "svelte/store";
...

  test("linux system info returned", async () => {
    ...
    expect(get(systemInfo)?.shell_init_file).toEqual("/root/.zshrc");
  });
```

Note that if we don't include the `.catch(...)` statement in the Svelte file, then we'll get this error for the `API key error` test case:

```
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯ Unhandled Rejection ⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯
Unknown Error: testing
This error originated in "src/routes/components/Metadata.test.ts" test file. It doesn't mean the error was thrown inside the file itself, but while it was running.
```

Finally, we read this store in when creating the form at `src-svelte/src/routes/components/api-keys/Form.svelte`:

```ts
  ...
  import { systemInfo } from "$lib/system-info";
  ...
  export let saveKeyLocation = $systemInfo?.shell_init_file ?? "";
  ...
```

and we test this at `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
...
import userEvent from "@testing-library/user-event";
import { systemInfo } from "$lib/system-info";
...

  test("some API key set", async () => {
    systemInfo.set({
      shell: "Zsh",
      shell_init_file: "/home/rando/.zshrc",
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    const openAiCell = screen.getByRole("cell", { name: "OpenAI" });
    await userEvent.click(openAiCell);
    const apiKeyInput = screen.getByLabelText("API key:");
    expect(apiKeyInput).toHaveValue("0p3n41-4p1-k3y");
    const saveFileInput = screen.getByLabelText("Save key to:");
    expect(saveFileInput).toHaveValue("/home/rando/.zshrc");
  });
```

#### Calling the API from the form

Now that we have all the elements in place, let's actually make a call to the API when the form gets submitted. We edit `src-svelte/src/lib/controls/Button.svelte` to trigger form submission when it's clicked:

```svelte
<button ... type="submit">
  ...
</button>
```

We edit `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  import { setApiKey, type Service } from "$lib/bindings";
  ...

  export let service: Service;
  ...

  function submitApiKey(e: SubmitEvent) {
    setApiKey(
      saveKey ? saveKeyLocation : null,
      service,
      apiKey,
    )
  }
</script>

<div ...>
  <div ...>
    <form on:submit|preventDefault={submitApiKey}>
      ...
    </form>
  </div>
</div>

```

Note that we added a `service` argument for the API call. We'll have to pass it in to the form through `src-svelte/src/routes/components/api-keys/Service.svelte`, where we type `name` as `Service` instead of `string` for type checking purposes:

```svelte
<script lang="ts">
  ...
  import type { Service } from "$lib/bindings";

  export let name: Service;
  ...
</script>

<div ...>
  <div ...>
    <div ...>{service}</div>
    ...
  </div>

  {#if editing}
    <Form service={name} ... />
  {/if}
</div>
```

Now we'll add a new test for this in `src-svelte/src/routes/components/api-keys/Display.test.ts`. We copy most of the setup for the "no API key set" test, except that the shell init file field will be pre-filled with the expected value, and the simulated user will go on to fill out the API key editing form and trigger the expected API call to save the API key.

```ts
describe("API Keys Display", () => {
  ...

  test("can edit API key", async () => {
    systemInfo.set({
      shell: "Zsh",
      shell_init_file: "no-newline/.bashrc",
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-existing-no-newline.yaml"
    );

    const openAiCell = screen.getByRole("cell", { name: "OpenAI" });
    await userEvent.click(openAiCell);
    const apiKeyInput = screen.getByLabelText("API key:");
    expect(apiKeyInput).toHaveValue("");
    await userEvent.type(apiKeyInput, "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });
});
```

We get the error

```
Error: No matching call found for ["set_api_key",{"filename":"no-newline/.bashrc","service":"OpenAI","apiKey":"0p3n41-4p1-k3y"}].
Candidates are ["set_api_key",{"filename":"no-newline/.bashrc","service":"OpenAI","api_key":"0p3n41-4p1-k3y"}]
 ❯ TauriInvokePlayback.mockCall src/lib/sample-call-testing.ts:65:15
     63|         `Candidates are ${candidates}`;
     64|       if (typeof process === "object") {
     65|         throw new Error(errorMessage);
       |               ^
     66|       } else {
     67|         return Promise.reject(errorMessage);      
```

We see that Tauri automatically adapts the naming convention of each language, which is good, but means that we must now manually adapt how our tests read from the sample file. This also means that the sample files themselves are no longer a completely verbatim reflection of the API calls being made.

We could do this on either the frontend side, by converting snake case to camelcase when reading the sample file, or on the backend side, by converting camelcase to snake case. We choose the former, and install `lodash`:

```bash
$ yarn add -D lodash
```

and edit `src-svelte/src/lib/sample-call-testing.ts` to do the camel case conversion for us by replacing the `JSON.parse` with a custom function:

```ts
...
import { camelCase } from "lodash";

...

function parseJsonRequest(request: string): Record<string, string> {
  const jsonRequest = JSON.parse(request);
  for (const key in jsonRequest) {
    const camelKey = camelCase(key);
    if (camelKey !== key) {
      jsonRequest[camelKey] = jsonRequest[key];
      delete jsonRequest[key];
    }
  }
  return jsonRequest;
}

export function parseSampleCall(sampleFile: string): ParsedCall {
  ...
  const parsedRequest =
    ...
      ? [..., parseJsonRequest(rawSample.request[1])]
      : ...;
  ...
}

...
```

This test passes, so we add a new one to `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
  test("can submit with custom file", async () => {
    const defaultInitFile = "/home/rando/.bashrc";
    systemInfo.set({
      shell: "Zsh",
      shell_init_file: defaultInitFile,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-existing-no-newline.yaml"
    );

    await userEvent.click(screen.getByRole("cell", { name: "OpenAI" }));
    const fileInput = screen.getByLabelText("Save key to:");
    defaultInitFile.split("").forEach(() => userEvent.type(fileInput, "{backspace}"));
    await userEvent.type(fileInput, "no-newline/.bashrc");
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });
```

Now we find that the text input update doesn't work. This is because we have to bind values. We edit `src-svelte/src/routes/components/api-keys/Form.svelte` again to bind the text inputs and the checkbox to their respective variables:

```svelte
<div ...>
  <div ...>
    <form ...>
      <div ...>
        ...
        <TextInput name="apiKey" bind:value={apiKey} />
      </div>

      <div ...>
        ...
        <input type="checkbox" ... bind:checked={saveKey} />
        ...
        <TextInput name="saveKeyLocation" bind:value={saveKeyLocation} />
      </div>

      ...
    </form>
  </div>
</div>
```

and then we edit `src-svelte/src/lib/controls/TextInput.svelte` to bind the value to the actual HTML input element:

```svelte
<div ...>
  <input ... bind:value={value} />
  ...
</div>
```

Now this test passes as well, and we add a final one to `src-svelte/src/routes/components/api-keys/Display.test.ts` to check that we can also set the API key without persisting it to disk at all:

```ts
  test("can submit with no file", async () => {
    const defaultInitFile = "/home/rando/.bashrc";
    systemInfo.set({
      shell: "Zsh",
      shell_init_file: defaultInitFile,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-no-disk-write.yaml"
    );

    await userEvent.click(screen.getByRole("cell", { name: "OpenAI" }));
    await userEvent.click(screen.getByLabelText("Save key to disk?"));
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });
```

Note that we want a way to refer to the checkbox in an accessible manner without actually changing anything about how we display the component as a whole. We edit `src-svelte/src/routes/components/api-keys/Form.svelte` one more time to allow our test to make use of the hidden "Save key to disk?" label:

```svelte
<div ...>
  <div ...>
    <form ...>
      ...

      <div ...>
        <label for="saveKey" class="accessibility-only">Save key to disk?</label>
        <input type="checkbox" id="saveKey" ... />
        ...
      </div>

      ...
    </form>
  </div>
</div>

<style>
  ...

  .accessibility-only {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    border: 0;
  }

  ...
</style>
```

We double check that the component renders exactly the same, and then check to see that the test passes. We could choose to instead put this inside `src-svelte/src/routes/styles.css`, so as to make it accessible from anywhere else in the app.

Now, we make it possible to trigger this API call in Storybook by editing `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
...
const writeToFile = "/api/sample-calls/set_api_key-existing-no-newline.yaml";
const unknownKeys = "/api/sample-calls/get_api_keys-empty.yaml";
const knownKeys = "/api/sample-calls/get_api_keys-openai.yaml";
...

Unknown.parameters = {
  sampleCallFiles: [unknownKeys],
  ...
};

...
Known.parameters = {
  sampleCallFiles: [knownKeys],
  ...
};

...
Editing.parameters = {
  sampleCallFiles: [knownKeys, writeToFile],
  ...
};
```

This way, we can confirm with manual testing as well.

#### Refactoring the form open action in tests

Refactor all the form-related tests in `src-svelte/src/routes/components/api-keys/Display.test.ts` to handle the form opening action as a unit:

```ts
  async function toggleOpenAIForm() {
    const openAiCell = screen.getByRole("cell", { name: "OpenAI" });
    await userEvent.click(openAiCell);
  }

  ...

  test("some API key set", async () => {
    ...

    await toggleOpenAIForm();
    const apiKeyInput = screen.getByLabelText("API key:");
    ...
  });

  ...

  test("can edit API key", async () => {
    ...
    await toggleOpenAIForm();
    ...
  });

  test("can submit with custom file", async () => {
    ...
    await toggleOpenAIForm();
    ...
  });

  test("can submit with no file", async () => {
    ...
    await toggleOpenAIForm();
    ...
  });
```

Now we can test that the form can be toggled opened and closed:

```ts
  test("can open and close form", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    await toggleOpenAIForm();
    const apiKeyInput = screen.getByLabelText("API key:");
    expect(apiKeyInput).toBeInTheDocument();
    await toggleOpenAIForm();
    expect(apiKeyInput).not.toBeInTheDocument();
  });
```

Unfortunately, despite the functionality working in Storybook, the test itself fails:

```
Error: expect(element).not.toBeInTheDocument()

expected document not to contain element, found <input
  class="svelte-tgv6wr"
  id="apiKey"
  name="apiKey"
  type="text"
/> instead
```

This is because of the `transition:growY` animation that we introduced in `Form.svelte`, as evidenced by the fact that the test passes if we remove the animation. We see that there is [this thread](https://github.com/testing-library/svelte-testing-library/issues/99) and [this thread](https://github.com/testing-library/svelte-testing-library/issues/206) on the issue. We find that the solution [here](https://github.com/testing-library/svelte-testing-library/issues/206#issuecomment-1470158576) works. Adapted to our code, we edit `src-svelte/src/routes/components/api-keys/Display.test.ts` as such:

```ts
  beforeEach(() => {
    ...

    vi.stubGlobal('requestAnimationFrame', (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  ...

  test("can open and close form", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    // closed by default
    const formExistenceCheck = () => screen.getByLabelText("API key:");
    expect(formExistenceCheck).toThrow()

    // opens on click
    await toggleOpenAIForm();
    expect(formExistenceCheck).not.toThrow();

    // closes again on click
    await toggleOpenAIForm();
    await waitFor(() => expect(formExistenceCheck).toThrow());
  });
```

#### Hiding the form on API success

If all goes well, we should hide the form again if the call succeeds. We add a callback to the form at `src-svelte/src/routes/components/api-keys/Form.svelte`.

```ts
  ...
  export let formClose = () => {};
  ...

  function submitApiKey() {
    setApiKey(...).finally(() => {
      formClose();
    });
  }
```

We set it to always close first, and handle the callback in `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  ...
  function formClose() {
    editing = false;
  }
  ...
</script>

...
    <Form {formClose} ... />
```

We test this by checking in `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
  test("can edit API key", async () => {
    ...
    await waitFor(() => expect(apiKeyInput).not.toBeInTheDocument());
  });
```

When committing, we change

```ts
export let formClose = () => {};
```

to

```ts
export let formClose: () => void = () => undefined;
```

to avoid both the error

```
/root/zamm/src-svelte/src/routes/components/api-keys/Form.svelte
  12:32  error  Unexpected empty arrow function  @typescript-eslint/no-empty-function
```

and the error

```
/root/zamm/src-svelte/src/routes/components/api-keys/Service.svelte:34:12
Error: Type '() => void' is not assignable to type '() => undefined'.
  Type 'void' is not assignable to type 'undefined'. (ts)
  {#if editing}
    <Form {formClose} service={name} apiKey={apiKey ?? ""} />
  {/if}
```

#### Showing error in snackbar

If there's an error, we'll want to inform the user and show it in the snackbar. To do so, we'll first have to create a snackbar component. There does exist pre-existing snackbar elements from [Svelte Material UI](https://sveltematerialui.com/demo/snackbar/) and [SmelteJS](https://smeltejs.com/components/snackbars/), but neither of these appear to offer the flexibility we want as these components visibly do not support displaying multiple messages simultaneously.

We create `src-svelte/src/lib/Snackbar.svelte`:

```svelte
<script lang="ts" context="module">
  import { writable, type Writable } from 'svelte/store';
  import { fly, fade } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import IconClose from "~icons/ep/close-bold";

  interface SnackbarMessage {
    id: number;
    msg: string;
  }

  export const snackbars: Writable<SnackbarMessage[]> = writable([]);
  export let durationMs = 5_000;
  let animateDurationMs = 1_000;

  let nextId = 0;

  // Function to show a new snackbar message
  export function snackbarError(msg: string) {
    animateDurationMs = 1_000;
    const id = nextId++;
    snackbars.update(current => [...current, { id, msg }]);

    // Auto-dismiss after 'duration'
    setTimeout(() => {
      dismiss(id);
    }, durationMs);
  }

  // Function to manually dismiss a snackbar
  function dismiss(id: number) {
    animateDurationMs = 1_000 * 2;
    snackbars.update(current =>
      current.filter(snackbar => snackbar.id !== id)
    );
  }
</script>

<div class="snackbars">
  {#each $snackbars as snackbar (snackbar.id)}
    <div class="snackbar"
      in:fly|global={{ y: "1rem", duration: 1000 }}
      out:fade|global={{ duration: 1000 }}
      animate:flip={{ duration: animateDurationMs  }}
    >
      {snackbar.msg}
      <button on:click={() => dismiss(snackbar.id)}>
        <IconClose />
      </button>
    </div>
  {/each}
</div>

<style>
  .snackbars {
    width: 100%;
    position: fixed;
    bottom: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .snackbar {
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    background-color: var(--color-error);
    color: white;
    border-radius: 4px;
    filter: drop-shadow(0px 1px 4px #CC0000);
    width: fit-content;
    margin: 0 auto;
  }

  button {
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.5rem;
    margin: -0.5rem;
    align-self: flex-end;
  }
</style>

```

and edit `src-svelte/src/routes/styles.css` to define the new CSS variable:

```css
:root {
  ...
  --color-error: #FF0000;
  ...
}
```

We create a component at `src-svelte/src/lib/SnackbarView.svelte` to display this in Storybook with:

```ts
<script lang="ts">
  import Snackbar, { snackbarError } from "./Snackbar.svelte";

  let count = 0;

  function showError() {
    count++;
    const noun = count === 1 ? "thing" : "things";
    snackbarError(`${count} ${noun} went wrong!`);
  }
</script>

<Snackbar />

<button on:click={showError}>Show Error</button>

```

and we create the stories at `src-svelte/src/lib/Snackbar.stories.ts`:

```ts
import SnackbarView from "./SnackbarView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: SnackbarView,
  title: "Layout/Snackbar",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: SnackbarView,
  props: args,
});

export const Default: StoryObj = Template.bind({}) as any;

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};

export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.parameters = {
  preferences: {
    animationsOn: false,
  },
};

```

The Storybook stories are where we initially encounter what is a [known problem](https://stackoverflow.com/questions/68273921/svelte-animation-blocks-transition) around doing both Svelte animations and transitions. The problem with the collapsing overlays exists with the proposed solutions as well; it's just less obvious due to the shorter time intervals involved. We take a page from the proposed solutions and mitigate the problem as follows:

- We introduce animations of different durations for incoming and outgoing animations
- We make the transitions different between incoming and outgoing, as the weirdness only becomes apparent with outgoing animations.

We make the animations and transitions dependent on the animation speed setting. We edit `src-svelte/src/lib/Snackbar.svelte`, keeping in mind that we can't directly access stores from modules, so we have to go about this in a roundabout manner:

```svelte
<script lang="ts" context="module">
  ...
  let baseAnimationDurationMs = 100;
  let animateDurationMs = baseAnimationDurationMs;

  function setBaseAnimationDurationMs(newDurationMs: number) {
    baseAnimationDurationMs = newDurationMs;
  }
  ...
  // Function to show a new snackbar message
  export function snackbarError(msg: string) {
    animateDurationMs = baseAnimationDurationMs;
    ...
  }

  // Function to manually dismiss a snackbar
  function dismiss(id: number) {
    animateDurationMs = 2 * baseAnimationDurationMs;
    ...
  }
</script>

<script lang="ts">
  import { animationSpeed } from "$lib/preferences";

  $: baseDurationMs = 100 / $animationSpeed;
  $: setBaseAnimationDurationMs(baseDurationMs);
</script>

...
    <div
      ...
      in:fly|global={{ y: "1rem", duration: baseDurationMs }}
      out:fade|global={{ duration: baseDurationMs }}
      ...
    >
      ...
    </div>
...
```

We also rename `durationMs` to `messageDurationMs` and avoid making it dependent on animation speed because the length of time to show a message is not a quantity that should be affected by the speed of animations.

Now we refactor out a single message into its own component so that it can be displayed in a Storybook story without needing to be triggered as part of the overall snackbar story. We move everything into the `src-svelte/src/lib/snackbar` folder, and then create `src-svelte/src/lib/snackbar/Message.svelte`:

```svelte
<script lang="ts">
  import IconClose from "~icons/ep/close-bold";

  export let dismiss: () => void;
  export let message: string;
</script>

<div class="snackbar">
  {message}
  <button on:click={dismiss}>
    <IconClose />
  </button>
</div>

<style>
  .snackbar {
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    background-color: var(--color-error);
    color: white;
    border-radius: 4px;
    filter: drop-shadow(0px 1px 4px #cc0000);
    width: fit-content;
    margin: 0 auto;
  }

  button {
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.5rem;
    margin: -0.5rem;
    margin-top: -0.3rem;
    align-self: flex-center;
  }
</style>

```

The only consequential change we've made here is to change `align-self` to center, because now we realize that if the message content spans multiple lines on a small screen, the close button will no longer be centered properly. To compensate for the vertical offset, we now also override `margin-top` to be slightly smaller than the bottom.

Now we make use of this refactored component in `src-svelte/src/lib/snackbar/Snackbar.svelte`, removing the relevant styles and moving the imports down to the non-module portion of the script:

```svelte
<script lang="ts">
  ...
  import { fly, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import Message from "./Message.svelte";
</script>

<div class="snackbars">
  {#each $snackbars as snackbar (snackbar.id)}
    <div
      in:fly|global={{ y: "1rem", duration: baseDurationMs }}
      out:fade|global={{ duration: baseDurationMs }}
      animate:flip={{ duration: animateDurationMs }}
    >
      <Message
        dismiss={() => dismiss(snackbar.id)}
        message={snackbar.msg}
      />
    </div>
  {/each}
</div>
```

Note that we can't set the `animate` directive as part of the refactored Message component because then we get:

```
An element that uses the animate directive must be the immediate child of a keyed each block(invalid-animation)
```

Finally, we create the single-message story next at `src-svelte/src/lib/snackbar/Message.stories.ts`, constraining it to a small screen for better screenshot comparisons. This is how we discovered the issue with the word wrapping and close button alignment.

```ts
import MessageComponent from "./Message.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: MessageComponent,
  title: "Layout/Snackbar",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: MessageComponent,
  props: args,
});

export const Message: StoryObj = Template.bind({}) as any;
Message.args = {
  message: "Something is wrong.",
  dismiss: () => { console.log("Dismiss button clicked.") },
};
Message.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

```

and record this as a new screenshot to be taken in `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["layout", "snackbar"],
    variants: ["message"],
  },
  ...
];
```

We realize from our manual testing that the "motionless" story isn't actually working as intended, so we fix that in `src-svelte/src/lib/snackbar/Snackbar.svelte`:

```ts
  import { animationSpeed, animationsOn } from "$lib/preferences";
  ...

  $: baseDurationMs = $animationsOn ? 100 / $animationSpeed : 0;
  ...
```

Now, we edit `src-svelte/src/routes/components/api-keys/Form.svelte`:

```ts
  ...
  import { snackbarError } from "$lib/Snackbar.svelte";
  ...

  function submitApiKey() {
    setApiKey(...).then(() => {
      formClose();
    }).catch((error) => {
      snackbarError(`Error: ${error}`);
    });
  }
```

##### Testing the snackbar

We add a test at `src-svelte/src/lib/snackbar/Snackbar.test.ts`:

```ts
import Snackbar, { snackbarError, clearAllMessages } from "./Snackbar.svelte";
import "@testing-library/jest-dom";
import { within, waitFor } from "@testing-library/dom";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { expect, vi } from "vitest";
import { tickFor } from "$lib/test-helpers";

describe("Snackbar", () => {
  beforeEach(() => {
    clearAllMessages();

    vi.stubGlobal("requestAnimationFrame", (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
  });

  it("should not display any messages by default", () => {
    render(Snackbar, {});

    const alerts = screen.queryAllByRole("alert");
    expect(alerts).toHaveLength(0);
  });

  it("should display a message after an alert is triggered", async () => {
    const message = "This is a test message";
    render(Snackbar, {});
    snackbarError(message);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(message);
  });

  it("should be able to display multiple messages", async () => {
    const message1 = "This is a test message";
    const message2 = "This is another test message";
    render(Snackbar, {});
    snackbarError(message1);
    snackbarError(message2);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(2);
    expect(alerts[0]).toHaveTextContent(message1);
    expect(alerts[1]).toHaveTextContent(message2);
  });

  it("should hide a message if the dismiss button is clicked", async () => {
    const message = "This is a test message";
    render(Snackbar, {});
    snackbarError(message);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(message);

    const dismissButton = within(alerts[0]).getByRole("button", {
      name: "Dismiss",
    });
    await userEvent.click(dismissButton);
    await waitFor(() => expect(alerts[0]).not.toBeInTheDocument());
    const alertsAfterDismiss = screen.queryAllByRole("alertdialog");
    expect(alertsAfterDismiss).toHaveLength(0);
  });
});

```

We need to use the `requestAnimationFrame` mock fix, and export a function to clear all outstanding messages in `src-svelte/src/lib/snackbar/Snackbar.svelte`:

```svelte
<script lang="ts" context="module">
  ...

  export function clearAllMessages() {
    snackbars.set([]);
  }

  ...
</script>
```

and to make each message more accessible in `src-svelte/src/lib/snackbar/Message.svelte`:

```svelte
<div ... role="alertdialog">
  ...
  <button ... title="Dismiss">
    ...
  </button>
</div>
```

#### Triggering snackbar error from form

Now we actually make use of the snackbar we've just created. First, we have to render the snackbar component with the app layout at `src-svelte/src/routes/AppLayout.svelte`, inside the main container but before all of the main content:

```svelte
<script lang="ts">
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  ...
</script>

<div
  id="app"
  ...
>
  ...

  <div class="main-container">
    ...
    <Snackbar />

    <main>
      ...
    </main>
  </div>
</div>
```

Then, we render the snackbar component in the test app layout as well, at `src-svelte/src/lib/__mocks__/MockAppLayout.svelte`:

```svelte
<script lang="ts">
  ...
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
</script>

<div
  class="storybook-wrapper"
  ...
>
  <Snackbar />
  <slot />
</div>

```

We add it to the existing app layout mock component so that we don't need a different component for every single app-wide feature. Now, we edit the Storybook story at `src-svelte/src/routes/components/api-keys/Display.stories.ts` to make use of this new mock functionality:

```ts
...
import type { StoryFn, StoryObj } from "@storybook/svelte";
...
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";

...

export default {
  ...,
  decorators: [
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockAppLayout,
        slot: story,
      };
    },
  ],
};

export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  sampleCallFiles: [..., writeToFile],
  ...
};

export const Known: StoryObj = Template.bind({}) as any;
Known.parameters = {
  sampleCallFiles: [..., writeToFile],
  ...
};
```

Next, we add a line to persist and debug failures in `src-svelte/src/lib/snackbar/Snackbar.svelte`:

```ts
  export function snackbarError(msg: string) {
    console.log(`Error reported: ${msg}`);
    ...
  }
```

Finally, we change `src-svelte/src/routes/components/api-keys/Form.svelte` to only trigger the form to close if the API call was successful. If it failed, we leave it open for the user to fix the issue:

```ts
  ...
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  ...

  function submitApiKey() {
    setApiKey(...)
    .then(() => {
      formClose();
    })
    .catch((err) => {
      snackbarError(err);
    });
  }
```
