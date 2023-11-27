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
