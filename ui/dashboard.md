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

We no longer need `metadata-container`, so we remove that and put the `InfoBox` directly inside the row:

```svelte
<section class="homepage-banner">
  ...
  <Metadata ... />
</section>
```

Now we update `src-svelte/src/routes/components/Metadata.svelte` as well to make the component screenshot there consistent. We wrap things in a new `inline-block` container as noted in the responses to [this question](https://stackoverflow.com/q/5827272):

```svelte
<div class="container">
  <InfoBox title="System Info" ...>
    ...
  </InfoBox>
</div>

<style>
  .container {
    display: inline-block;
  }

  ...
</style>
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

We realize when this test fails in CI that we want more information, so we edit the print debug statement to:

```rs
        println!(
            "Determined shell to be {:?} from env var {:?}",
            shell,
            env::var("SHELL")
        );
```

We find that this fails in CI because the environment variable is not set there, so we simply `#[ignore]` the test for CI purposes. However, we still edit `src-tauri/Makefile` to make the default test command test everything, as documented [here](https://doc.rust-lang.org/book/ch11-02-running-tests.html):

```Makefile
test: tests
tests:
	cargo test -- --include-ignored
```

There's no need to edit the GitHub test workflow because it already runs `cargo test` directly instead of through the Makefile.

As usual, edit `src-tauri/src/commands/mod.rs`:

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

We add a fallback in case `SHELL` isn't set:

```rs
fn get_shell() -> Option<Shell> {
    if let Ok(shell) = env::var("SHELL") {
        ...
    }

    if env::var("ZSH_NAME").is_ok() {
        return Some(Shell::Zsh);
    }
    if env::var("BASH").is_ok() {
        return Some(Shell::Bash);
    }

    None
}
```

It turns out this does not work either because those environment variables are special ones for the shell that are not passed onto the running program.

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

Note that we take much inspiration from `src-tauri/src/commands/preferences/write.rs` for also using a similar pattern of copying files over to a temporary test directory and then comparing file results. Two changes do need to be made to `src-tauri/src/test_helpers.rs` to complete the above refactor:

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

Now we use this same pattern in `src-svelte/src/routes/components/Metadata.svelte`, now with the actual shell info instead of the mocked value:

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

    <table class="less-space">
      ...
      <tr>
        <td>Shell</td>
        <td>{systemInfo.shell}</td>
      </tr>
    </table>
  {:catch error}
    <span role="status">error: {error}</span>
  {/await}
</InfoBox>

...
```

It appears there is not really a better way to cut down on the boilerplate for the pattern here, as evidence by [this question](https://stackoverflow.com/q/64023421).

We notice that the shell displays as "null" when the backend is unable to detect what shell it is, so we set it instead as

```svelte
      <tr>
        <td>Shell</td>
        <td>{systemInfo.shell ?? "Unknown"}</td>
      </tr>
```

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
    screenshotEntireBody: true,
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

We add it to the existing app layout mock component so that we don't need a different component for every single app-wide feature. However, we find that some of our screenshot tests are failing because the first item being screenshotted is now the empty snackbar, which causes Playwright to timeout waiting for the empty snackbar to become visible. As such, we move the snackbar under the `slot` instead.

Now, we edit the Storybook story at `src-svelte/src/routes/components/api-keys/Display.stories.ts` to make use of this new mock functionality:

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

We end up taking this out because Vitest does not hide stdout even when the tests pass.

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

#### Persisting form input through open and close

To preserve form state through the showing and hiding of the form, we coalesce all form field data into one convenient data structure and base form controls off of that in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts" context="module">
  export interface FormFields {
    apiKey: string;
    saveKey: boolean;
    saveKeyLocation: string;
  }
</script>

<script lang="ts">
  ...
  export let fields: FormFields;
  ...

  function submitApiKey() {
    setApiKey(fields.saveKey ? fields.saveKeyLocation : null, service, fields.apiKey)
      ...;
  }
</script>

<div ...>
  <div ...>
    <form ...>
      <div class="form-row">
        ...
        <TextInput name="apiKey" bind:value={fields.apiKey} />
      </div>

      <div class="form-row">
        ...
        <input
          ...
          name="saveKey"
          bind:checked={fields.saveKey}
        />
        ...
        <TextInput name="saveKeyLocation" bind:value={fields.saveKeyLocation} />
      </div>

      ...
    </form>
  </div>
</div>

```

We then instantiate a form with this new field, and also persist it in `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  ...
  import { systemInfo } from "$lib/system-info";

  ...
  let formFields: FormFields = {
    apiKey: "",
    saveKey: true,
    saveKeyLocation: "",
  };

  function toggleEditing() {
    ...

    if (formFields.apiKey === "") {
      formFields.apiKey = apiKey ?? "";
    }
    if (formFields.saveKeyLocation === "") {
      formFields.saveKeyLocation = $systemInfo?.shell_init_file ?? "";
    }
  }

  ...
</script>

<div class="container">
  ...

  {#if editing}
    <Form ... bind:fields={formFields} />
  {/if}
</div>
```

Finally, we write a new test in `src-svelte/src/routes/components/api-keys/Display.test.ts` that tests this functionality:

```ts
  test("preserves unsubmitted changes after opening and closing form", async () => {
    const defaultInitFile = "/home/rando/.bashrc";
    systemInfo.set({
      shell: "Zsh",
      shell_init_file: defaultInitFile,
    });
    const customInitFile = "/home/different/.bashrc";
    const customApiKey = "0p3n41-4p1-k3y";

    // setup largely copied from "can submit with custom file" test
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
      "../src-tauri/api/sample-calls/set_api_key-existing-no-newline.yaml",
    );

    // open form and type in API key
    await toggleOpenAIForm();
    let apiKeyInput = screen.getByLabelText("API key:");
    let saveKeyCheckbox = screen.getByLabelText("Save key to disk?");
    let fileInput = screen.getByLabelText("Save key to:");

    expect(apiKeyInput).toHaveValue("");
    expect(saveKeyCheckbox).toBeChecked();
    expect(fileInput).toHaveValue(defaultInitFile);

    await userEvent.type(apiKeyInput, customApiKey);
    await userEvent.click(saveKeyCheckbox);
    defaultInitFile
      .split("")
      .forEach(() => userEvent.type(fileInput, "{backspace}"));
    await userEvent.type(fileInput, customInitFile);
    
    expect(apiKeyInput).toHaveValue(customApiKey);
    expect(saveKeyCheckbox).not.toBeChecked();
    expect(fileInput).toHaveValue(customInitFile);

    // close and reopen form
    await toggleOpenAIForm();
    await waitFor(() => expect(apiKeyInput).not.toBeInTheDocument());
    await toggleOpenAIForm();
    await waitFor(() => {
      const formExistenceCheck = () => screen.getByLabelText("API key:");
      expect(formExistenceCheck).not.toThrow();
    });

    // check that changes to form fields persist
    // need to obtain references to new form fields
    apiKeyInput = screen.getByLabelText("API key:");
    saveKeyCheckbox = screen.getByLabelText("Save key to disk?");
    fileInput = screen.getByLabelText("Save key to:");
    expect(apiKeyInput).toHaveValue(customApiKey);
    expect(saveKeyCheckbox).not.toBeChecked();
    expect(fileInput).toHaveValue(customInitFile);
  });
```

#### Refreshing API key on form close

First, we make sure that things will work on the backend. We edit `src-tauri/src/commands/keys/get.rs` to make the test functions public:

```rs
...

#[cfg(test)]
pub mod tests {
  ...
  pub fn check_get_api_keys_sample(..., rust_input: &ZammApiKeys) {
    ...
  }
  ...
}
```

and then we edit `src-tauri/src/commands/keys/set.rs` to make the test functions public and have matching signatures with a helper function that takes in `ZammApiKeys` instead of `ApiKeys`:

```rs
...
use crate::setup::api_keys::Service;
...

fn set_api_key_helper(
    zamm_api_keys: &ZammApiKeys,
    ...
) -> ZammResult<()> {
    let api_keys = &mut zamm_api_keys.0.lock().unwrap();
    ...
}

#[tauri::command(async)]
#[specta]
pub fn set_api_key(
    api_keys: State<ZammApiKeys>,
    ...
) -> ZammResult<()> {
    set_api_key_helper(&api_keys, ...)
}

#[cfg(test)]
pub mod tests {
    ...
    use crate::setup::api_keys::ApiKeys;
    ...
    use std::sync::Mutex;

    ...
    pub fn check_set_api_key_sample(
        ...,
        existing_zamm_api_keys: &ZammApiKeys,
    ) {
      ...
      let actual_result = set_api_key_helper(
          existing_zamm_api_keys,
          ...,
      );
      ...
      // check that the API call actually modified the in-memory API keys
      let existing_api_keys = existing_zamm_api_keys.0.lock().unwrap();
      ...
    }

    #[test]
    fn test_write_new_init_file() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample(
            "api/sample-calls/set_api_key-no-file.yaml",
            &api_keys,
        );
    }

    ...
}

```

We do the same for the rest of the tests in that file. We finally put all this together in `src-tauri/src/commands/keys/mod.rs`, making sure that one API call followed by the other will result in the expected API responses:

```rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::api_keys::ApiKeys;
    use crate::ZammApiKeys;
    use get::tests::check_get_api_keys_sample;
    use set::tests::check_set_api_key_sample;
    use std::sync::Mutex;

    #[test]
    fn test_get_after_set() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_set_api_key_sample(
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            &api_keys,
        );

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-openai.yaml",
            &api_keys,
        );
    }
}

```

The tests initially fail, so we add debugging to `src-tauri/src/test_helpers.rs` for greater clarity:

```rs
pub fn get_temp_test_dir(...) -> PathBuf {
    ...
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).unwrap_or_else(|_| {
            panic!("Can't remove temp test dir at {}", test_dir.display())
        });
    }
    fs::create_dir_all(&test_dir).unwrap_or_else(|_| {
        panic!("Can't create temp test dir at {}", test_dir.display())
    });
    ...
}
```

The test failures are non-deterministic, and turn out to be caused by cargo's test parallelism. We refactor `src-tauri/src/commands/keys/set.rs` again to allow an external function to set the test name:

```rs
    pub fn check_set_api_key_sample(
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
        test_dir_name: &str,
    ) {
      ...
                  let test_name = format!("{}/{}", test_dir_name, sample_file_directory);
    }

    fn check_set_api_key_sample_unit(
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
    ) {
        check_set_api_key_sample(sample_file, existing_zamm_api_keys, "set_api_key");
    }
```

We replace all calls to `check_set_api_key_sample` in this file with calls to `check_set_api_key_sample_unit` instead. Then, we add the new argument in `src-tauri/src/commands/keys/mod.rs`:

```rs
    #[test]
    fn test_get_after_set() {
        ...

        check_set_api_key_sample(
            ...,
            "api_keys_integration_tests",
        );

        ...
    }
```

Only `check_set_api_key_sample` needs this because `check_get_api_keys_sample` does not involve any disk IO.

Now, we have the confidence to make use of this on the frontend. We notice that we need some way of storing and updating the API keys for display on the frontend, so we add a new store to `src-svelte/src/lib/system-info.ts`:

```ts
...
import type { SystemInfo, ApiKeys } from "./bindings";

...
export const apiKeys: Writable<ApiKeys> = writable({
  openai: null,
});
```

We then edit `src-svelte/src/routes/components/api-keys/Form.svelte` to update this store with a call to our source of truth, the backend, whenever the callback finishes, no matter if it failed or not (e.g. it could've failed because there was no way to write the update to disk, even if the API key got successfully set in memory):

```ts
  ...
  import { getApiKeys, setApiKey, type Service } from "$lib/bindings";
  ...
  import { apiKeys } from "$lib/system-info";
  ...

  function submitApiKey() {
    setApiKey(
      ...
    )
      ...
      .finally(async () => {
        apiKeys.set(await getApiKeys());
      });
  }
```

We update `src-svelte/src/routes/components/api-keys/Display.svelte` to make use of this store instead of the promise. We preserve the loading display with custom code, and replace the error message with a snackbar notification:

```svelte
<script lang="ts">
  ...
  import { apiKeys as apiKeysStore } from "$lib/system-info";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  ...
  import { onMount } from "svelte";

  let isLoading = true;
  ...

  onMount(() => {
    getApiKeys()
      .then((keys) => {
        apiKeysStore.set(keys);
      })
      .catch((error) => {
        snackbarError(error);
      })
      .finally(() => {
        isLoading = false;
      });
  });

  $: apiKeys = $apiKeysStore;
</script>

<InfoBox ...>
  {#if isLoading}
    <Loading />
  {:else}
    <div ...>
      <Service name="OpenAI" apiKey={apiKeys.openai} editing={editDemo} />
    </div>
  {/if}
</InfoBox>

```

We update the tests at `src-svelte/src/routes/components/api-keys/Display.test.ts` accordingly:

```ts
...
import Snackbar from "$lib/snackbar/Snackbar.svelte";
...

  async function checkSampleCall(filename: string, expected_display: string) {
    ...

    render(ApiKeysDisplay, {});
    await waitFor(() =>
      expect(screen.getByRole("row", { name: /OpenAI/ })).toBeInTheDocument(),
    );
    ...
  }

  ...

  test("API key error", async () => {
    const errorMessage = "Testing error message";
    ...
    tauriInvokeMock.mockRejectedValueOnce(errorMessage);

    render(ApiKeysDisplay, {});
    await waitFor(() =>
      expect(screen.getByRole("row", { name: /OpenAI/ })).toBeInTheDocument(),
    );
    expect(spy).toHaveBeenLastCalledWith("get_api_keys");

    render(Snackbar, {});
    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(errorMessage);
  });

  ...

  test("can edit API key", async () => {
    ...
    playback.addSamples(
      ...,
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );
    ...
    expect(tauriInvokeMock).toBeCalledTimes(2);
    ...
  });

  ...
```

We add similar playback samples to the "can submit with no file" and "can submit with custom file" tests.

Finally, we add the second callback to the stories at `src-svelte/src/routes/components/api-keys/Display.stories.ts` to simulate the effect of successfully submitting an API call:

```ts
export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile, knownKeys],
  ...
};
```

We notice that the story for the form edit demo is no longer showing the API key in the form. This is because the form field doesn't get updated when new information comes in about the API key. We can try to fix this in `src-svelte/src/routes/components/api-keys/Service.svelte` by moving the default logic out of `toggleEditing`:

```ts
  function toggleEditing() {
    editing = !editing;
  }

  ...
  $: formFields.apiKey = apiKey ?? "";
  $: formFields.saveKeyLocation = $systemInfo?.shell_init_file ?? "";
  ...
```

However, this prevents the form contents from being edited. We wrap it in a function instead and only trigger it when editing:

```ts
  function updateFormFields(trigger: boolean) {
    if (!trigger) {
      return;
    }

    if (formFields.apiKey === "") {
      formFields.apiKey = apiKey ?? "";
    }
    if (formFields.saveKeyLocation === "") {
      formFields.saveKeyLocation = $systemInfo?.shell_init_file ?? "";
    }
  }

  ...
  $: updateFormFields(editing);
```

#### Styling active button

To emphasize the change in activation status, we add a glow effect and CSS transition to `src-svelte/src/routes/components/api-keys/Service.svelte`:

```css
  .api-key {
    ...
    transition: all calc(0.1s / var(--base-animation-speed)) ease-in;
  }

  .api-key.active {
    box-shadow: 0 0 var(--shadow-blur) 0 green;
    ...
  }
```

Then, we add a story with slow-motion in `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
...
import SvelteStoresDecorator from "$lib/__mocks__/stores";
...

export default {
  ...,
  decorators: [
    SvelteStoresDecorator,
    ...
  ],
};

...

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile, knownKeys],
  preferences: {
    animationSpeed: 0.1,
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};

```

We notice that the status now transitions incorrectly on page load. It appears, then disappears, then appears again with a heavy delay after everything else has already appeared during the info box reveal animation. We realize that this is due to the `all` transition property, and we change it to be more specific:

```css
  .api-key {
    ...
    transition-property: background-color, box-shadow;
    transition-duration: calc(0.1s / var(--base-animation-speed));
    transition-timing-function: ease-in;
  }
```

Finally, we want to repeat this effect on initial page load, so that the eye candy isn't confined to just the moments when the API key is set. We add a class to mark elements as transitioning inside `src-svelte/src/lib/InfoBox.svelte`:

```ts
  class RevealContent extends SubAnimation<void> {
    constructor(anim: { node: Element; ... }) {
      ...
      super({
        ...,
        tick: (tLocalFraction: number) => {
          ...

          if (tLocalFraction < 0.9) {
            anim.node.classList.add("wait-for-infobox");
          } else {
            anim.node.classList.remove("wait-for-infobox");
          }
        },
      });
    }
  }
```

Note that we can instead do

```ts
          if (tLocalFraction === 0) {
            anim.node.classList.add("wait-for-infobox");
          } else if (tLocalFraction >= 0.9) {
            anim.node.classList.remove("wait-for-infobox");
          }
```

to save on the number of times we add the class.

We then make use of this new marker class in `src-svelte/src/routes/components/api-keys/Service.svelte`:

```css
.api-key {
    --inactive-color: gray;
    ...
    background-color: var(--inactive-color);
    ...
  }

  ...

  .container :global(.api-key.active.wait-for-infobox) {
    background-color: var(--inactive-color);
    box-shadow: none;
  }
```

Finally, we edit `src-svelte/src/routes/Dashboard.stories.ts` to provide a default screenshot where the API key is indeed provided and the OpenAI service activated, so that we can see this in effect:

```ts
export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_keys-openai.yaml",
    ...
  ],
};
```

After looking further at this, it appears desirable for the CSS transition to also be delayed during form submission, as it is during the info box reveal. We will try delaying it so that it looks like the visual closure of the form causes the service activation to be processed. We avoid doing this with the CSS `transition-delay` attribute because it is a bit weird for the text to change while the transition waits, so we instead try editing `src-svelte/src/routes/components/api-keys/Form.svelte` to delay the entire update:

```ts
  ...
  import { standardDuration } from "$lib/preferences";
  ...

  $: growDuration = 2 * $standardDuration;

  function growY(node: HTMLElement) {
    ...
    return {
      duration: growDuration,
      ...
    };
  }

  function submitApiKey() {
    setApiKey(
      ...
    )
      ...
      .finally(() => {
        setTimeout(async () => {
          // delay here instead of in CSS transition so that the text updates
          // simultaneously with the transition
          apiKeys.set(await getApiKeys());
        }, 0.75 * growDuration);
      });
  }
```

Some tests fail now because the API update is not called immediately, so we fix them by having them wait until the API call completes. We also shorten the animation duration to facilitate testing. We edit `src-svelte/src/routes/components/api-keys/Display.test.ts` for the fix:

```ts
...
import { animationSpeed } from "$lib/preferences";

describe("API Keys Display", () => {
  ...

  beforeAll(() => {
    animationSpeed.set(10);
  });

  ...

  test("can edit API key", async () => {
    ...
    await waitFor(() => expect(tauriInvokeMock).toBeCalledTimes(2));
    ...
  });

  ...

  test("can submit with custom file", async () => {
    ...
    await waitFor(() => expect(tauriInvokeMock).toBeCalledTimes(2));
  });

  test("can submit with no file", async () => {
    ...
    await waitFor(() => expect(tauriInvokeMock).toBeCalledTimes(2));
  });
});
```

#### Adding example placeholder text

To help the user understand what type of file to specify in case we're unable to determine the user's init file ourselves, we will set a placeholder in `src-svelte/src/lib/controls/TextInput.svelte`:

```svelte
<script lang="ts">
  ...
  export let placeholder: string | undefined = undefined;
  ...
</script>

<div class="fancy-input">
  <input type="text" ... {placeholder} ... />
  ...
</div>

<style>
  ...

  input[type="text"]::placeholder {
    font-style: italic;
  }

  ...
</style>
```

We can now specify the placeholder in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<div class="container" ...>
  <div ...>
    <form ...>
      ...

      <div class="form-row">
        ...
        <TextInput
          name="saveKeyLocation"
          placeholder="e.g. /home/user/.bashrc"
          bind:value={fields.saveKeyLocation}
        />
      </div>

      ...
    </form>
  </div>
</div>
```

### Handling weird form submissions

We'll want to make sure we can handle various form submission edge cases on the backend. One obvious one is to not return an error if we cannot suggest a default init file for the user to save their API key to, and therefore the init file field is non-null but empty. Another is if the user enters in an invalid file path (for example, `"/"`) that we cannot write to.

We create `src-tauri/api/sample-calls/set_api_key-empty-filename.yaml` to cover the first case:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "",
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null"

```

and `src-tauri/api/sample-calls/set_api_key-invalid-filename.yaml` to cover the second:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "/",
      "service": "OpenAI",
      "api_key": "0p3n41-4p1-k3y"
    }
response: "null

```

We add new tests for this in `src-tauri/src/commands/keys/set.rs`:

```rs
    #[test]
    fn test_empty_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            "api/sample-calls/set_api_key-empty-filename.yaml",
            &api_keys,
        );
    }

    #[test]
    fn test_invalid_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            "api/sample-calls/set_api_key-invalid-filename.yaml",
            &api_keys,
        );
    }
```

These suspiciously pass even though we don't currently have any functionality that would take care of them. Upon further debugging, we find that somehow the invalid path never actually gets passed into the function that we are trying to test. We fix this, including the code needed to filter out performing any actions on invalid filenames:

```rs

fn set_api_key_helper(
    ...
) -> ZammResult<()> {
    ...
    let init_update_result = || -> ZammResult<()> {
        if let Some(untrimmed_filename) = filename {
            let f = untrimmed_filename.trim();
            if !f.is_empty() {
                let ends_in_newline = {
                    if Path::new(f).exists() {
                        let mut file = OpenOptions::new().read(true).open(f)?;
                        ...
                    } ...
                };

                let mut file = OpenOptions::new()
                    ...
                    .open(f)?;
                ...
            }
        }
        Ok(())
    }();
    ...
}

...

#[cfg(test)]
pub mod tests {
  ...

    pub fn check_set_api_key_sample(
        ...,
        should_fail: bool,
        ...
    ) {
        ...
        let request = parse_request(&sample.request[1]);
        let valid_request_path_specified = request
            .filename
            .as_ref()
            .map(|f| !f.is_empty() && !f.ends_with('/'))
            .unwrap_or(false);
        let request_path = request.filename.as_ref().map(|f| PathBuf::from(&f));
        let test_init_file = if valid_request_path_specified {
            let p = request_path.as_ref().unwrap();
            ...
        } else {
            request.filename
        };

        let actual_result = set_api_key_helper(
            ...,
            test_init_file.as_deref(),
            ...,
        );

        // check that the API call returns the expected success or failure signal
        if should_fail {
            assert!(actual_result.is_err(), "API call should have thrown error");
        } else {
            assert!(
                actual_result.is_ok(),
                "API call failed: {:?}",
                actual_result
            );
        }

        // check that the API call returns the expected JSON
        let actual_json = match actual_result {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap(),
            Err(e) => serde_json::to_string_pretty(&e).unwrap(),
        };
        ...

        // check that the API call actually modified the in-memory API keys,
        // regardless of success or failure
        ...

        // check that the API call successfully wrote the API keys to disk, if asked to
        if valid_request_path_specified {
            let p = request_path.unwrap();
            ...
            let resulting_contents =
                fs::read_to_string(test_init_file.unwrap().as_str())
                    .expect("Test shell init file doesn't exist");
            ...
        }
    }

    fn check_set_api_key_sample_unit(
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
    ) {
        check_set_api_key_sample(
            sample_file,
            existing_zamm_api_keys,
            false,
            "set_api_key",
        );
    }

    fn check_set_api_key_sample_unit_fails(
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
    ) {
        check_set_api_key_sample(
            sample_file,
            existing_zamm_api_keys,
            true,
            "set_api_key",
        );
    }

    ...

    #[test]
    fn test_invalid_filename() {
        ...
        check_set_api_key_sample_unit_fails(
            ...,
        );
    }
}
```

Note that we have also added a `should_fail` argument to specify that this API call is expected to fail with the given message. As such, we also edit the integration test in `src-tauri/src/commands/keys/mod.rs`:

```rs
    #[test]
    fn test_get_after_set() {
        ...

        check_set_api_key_sample(
            ...,
            false,
            ...,
        );

        ...
    }
```

The test finally fails because the error message doesn't match our expectations. We edit `src-tauri/api/sample-calls/set_api_key-invalid-filename.yaml` again:

```yaml
request:
  ...
response: >
  "Is a directory (os error 21)"

```

and now get the tests to pass properly.

Ideally, we encode the `should_fail` flag as part of the test itself. However, that will require a more extensive refactor to the sample call infrastructure, and as such will be tackled in the `Allowing failed responses` section of [`api-boundary-type-safety.md`](/zamm/resources/tutorials/setup/tauri/api-boundary-type-safety.md).

#### Unsetting the API key

We should also allow the user to unset the API key. If the submitted API key is empty, then it should be unset. We create `src-tauri/api/sample-calls/set_api_key-unset.yaml` to demonstrate:

```yaml
request:
  - set_api_key
  - >
    {
      "filename": "unset/.bashrc",
      "service": "OpenAI",
      "api_key": ""
    }
response: "null"

```

We create `src-tauri/api/sample-init-files/unset/.bashrc`:

```bash
# check that if we unset the API key in the app, the local file doesn't change
export OPENAI_API_KEY="0p3n41-4p1-k3y"

```

We also create `src-tauri/api/sample-init-files/unset/expected.bashrc` to verify that the file should be untouched:

```bash
# check that if we unset the API key in the app, the local file doesn't change
export OPENAI_API_KEY="0p3n41-4p1-k3y"

```

We add functionality to unset API keys at `src-tauri/src/setup/api_keys.rs`:

```rs
impl ApiKeys {
    ...

    pub fn remove(&mut self, service: &Service) {
        match service {
            Service::OpenAI => self.openai = None,
        }
    }
}
```

Finally, we implement the actual desired functionality and the test for it at `src-tauri/src/commands/keys/set.rs`:

```rs
fn set_api_key_helper(
    ...
) -> ZammResult<()> {
    ...
    let init_update_result = || -> ZammResult<()> {
        if api_key.is_empty() {
            return Ok(());
        }

        ...
    }();
    // assign ownership of new API key string to in-memory API keys
    if api_key.is_empty() {
        api_keys.remove(service);
    } else {
        api_keys.update(service, api_key);
    }
    ...
}

...

#[cfg(test)]
pub mod tests {
    ...

    pub fn check_set_api_key_sample(
        ...
    ) {
        ...
        if request.api_key.is_empty() {
            assert_eq!(existing_api_keys.openai, None);
        } else {
            assert_eq!(existing_api_keys.openai, Some(request.api_key));
        }
        ...
    }

    ...

    #[test]
    fn test_unset() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some("0p3n41-4p1-k3y".to_owned()),
            ..ApiKeys::default()
        }));
        check_set_api_key_sample_unit(
            "api/sample-calls/set_api_key-unset.yaml",
            &api_keys,
        );
        assert!(api_keys.0.lock().unwrap().openai.is_none());
    }

    ...
}
```

We check that all tests pass on the backend, and then add a Storybook story to the frontend to see the transition for ourselves at `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
...
const unsetKey = "/api/sample-calls/set_api_key-unset.yaml";
...

export const Unset: StoryObj = Template.bind({}) as any;
Unset.parameters = {
  sampleCallFiles: [knownKeys, unsetKey, unknownKeys],
  preferences: {
    animationSpeed: 1,
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};

...
```

### End-to-end tests

Finally, we get our end-to-end tests to pass by:

- updating the screenshot for the welcome page, which has now changed
- removing the test for the presence of the OpenAI row, which is no longer a table row, and should be covered alternately by the screenshot and by the jest-dom component tests
- adding a wait for the homepage to be updated

We edit `webdriver/test/specs/e2e.test.js` accordingly:

```js
describe("Welcome screen", function () {
  it("should render the welcome screen correctly", async function () {
    ...
    await $("table"); // ensure page loads before taking screenshot
    await browser.pause(500); // for CSS transitions to finish
    expect(
      ...
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should allow navigation to the settings page", async function () {
    ...
  });
});
```

### Using login shell init file

We realize that `.bashrc` just doesn't cut it in the regular case, so we edit the Rust code at `src-tauri/src/commands/system.rs` to return the login shell script:

```rs
...

fn get_relative_shell_init_file() -> Option<String> {
    #[cfg(target_os = "linux")]
    return Some("~/.profile".to_string());
    #[cfg(target_os = "macos")]
    return Some("~/.bash_profile".to_string());
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    return None;
}

fn get_shell_init_file() -> Option<String> {
    get_relative_shell_init_file().map(|f| shellexpand::tilde(&f).to_string())
}

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    ...
    let shell_init_file = get_shell_init_file();

    SystemInfo {
        ...,
        shell_init_file,
    }
}

#[cfg(test)]
mod tests {
    ...

    #[test]
    fn test_can_predict_shell_init() {
        let shell_init_file = get_shell_init_file().unwrap();
        println!("Shell init file is {}", shell_init_file);
        assert!(shell_init_file.starts_with('/'));
        assert!(shell_init_file.ends_with("profile"));
    }

    #[test]
    fn test_get_linux_system_info() {
        let system_info = SystemInfo {
            shell: Some(Shell::Zsh),
            shell_init_file: Some("/root/.profile".to_string()),
        };

        check_get_system_info_sample(
            "./api/sample-calls/get_system_info-linux.yaml",
            &system_info,
        );
    }
}
```

Next, we edit the sample response file at `src-tauri/api/sample-calls/get_system_info-linux.yaml` to maintain a proper example of the sort of return value we expect:

```yaml
request: ["get_system_info"]
response: >
  {
    "shell": "Zsh",
    "shell_init_file": "/root/.profile"
  }

```

This means we'll have to edit the corresponding test at `src-svelte/src/routes/components/Metadata.test.ts` as well:

```ts
  test("linux system info returned", async () => {
    ...
    expect(get(systemInfo)?.shell_init_file).toEqual("/root/.profile");
  });
```

We should really reflect this change in the screenshots as well, because as it turns out, there is no visual test for whether this gets properly displayed onscreen or not. But to do that, we'll first have to edit `src-svelte/src/lib/__mocks__/stores.ts` to set the store:

```ts
...
import { systemInfo } from "$lib/system-info";
import type { SystemInfo } from "$lib/bindings";
...

interface Stores {
  systemInfo?: SystemInfo;
}

interface StoreArgs {
  ...
  stores?: Stores;
  ...
}

const SvelteStoresDecorator: Decorator = (
  ...
) => {
  ...
  const { ..., stores } = parameters as StoreArgs;

  ...

  systemInfo.set(stores?.systemInfo);

  ...
};

...
```

We now edit `src-svelte/src/routes/components/api-keys/Display.stories.ts` to more clearly distinguish between an empty and a pre-filled form:

```ts
export const Editing: StoryObj = Template.bind({}) as any;
...
Editing.parameters = {
  sampleCallFiles: [unknownKeys, ...],
  ...
};

export const EditingPreFilled: StoryObj = Template.bind({}) as any;
EditingPreFilled.args = {
  editDemo: true,
};
EditingPreFilled.parameters = {
  sampleCallFiles: [knownKeys, writeToFile],
  stores: {
    systemInfo: {
      shell_init_file: "/root/.profile",
    },
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};

```

We add this new story to `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "dashboard", "api-keys-display"],
    variants: [..., "editing-pre-filled"],
    ...
  },
  ...
];
```

and update the screenshots with new ones from newly failing tests.

### Linking to OpenAI API key page

To make things easier for our users, let's link directly to our API key page from the app. This not only saves them a search, but also allow us to provide more up-to-date information, as OpenAI's [own help page](https://help.openai.com/en/articles/4936850-where-do-i-find-my-api-key) refers to the outdated location of `https://beta.openai.com/account/api-keys`.

But before we do that, let's make sure we can indicate external links properly.

#### Adding external link indicator

We follow [this guide](https://christianoliff.com/blog/styling-external-links-with-an-icon-in-css/) and add this to `src-svelte/src/routes/styles.css`:

```css

a {
  color: #3333FF;
  text-decoration: none;
}

a[href^="http://"]::after,
a[href^="https://"]::after {
  --size: 1rem;
  content: "";
  width: var(--size);
  height: var(--size);
  margin-left: 0.25rem;
  margin-bottom: -0.15rem;
  /* equivalent of ~icons/tabler/external-link */
  background-image: url('data:image/svg+xml,%3Csvg xmlns="http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg" width="24" height="24" viewBox="0 0 24 24"%3E%3Cpath fill="none" stroke="%233333FF" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6H6a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6m-7 1l9-9m-5 0h5v5"%2F%3E%3C%2Fsvg%3E');
  background-position: bottom;
  background-repeat: no-repeat;
  background-size: contain;
  display: inline-block;
}
```

We use the external link icon from the Tabler set, but since there doesn't appear to be a way to use unplugin icons in CSS, we break from the existing usage and use the data URI mentioned under "SVG as data: URI" on [the Iconify page](https://icon-sets.iconify.design/tabler/external-link/). We move the position of the icon down a bit to make the bottom stroke of the box flush with the baseline of the text.

Note that to make the color of the icon the same, we have to use `%23` in the `stroke` property because it is the equivalent of the `#` symbol as encoded in a URL, as demonstrated in [this table](https://www.w3schools.com/tags/ref_urlencode.ASP).

Next, we create `src-svelte/src/lib/ExternalLinkView.svelte` to demonstrate this:

```svelte
<p>This is a link to <a href="https://www.google.com" target="_blank">Google</a>. Clicking on it will open a new page.</p>
```

and `src-svelte/src/lib/ExternalLink.stories.ts` to display this in Storybook:

```ts
import ExternalLinkView from "./ExternalLinkView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ExternalLinkView,
  title: "Reusable/External Link",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ExternalLinkView,
  props: args,
});

export const ExternalLink: StoryObj = Template.bind({}) as any;

```

For once, we don't need to create our own custom component for this functionality, but we do need to still create a custom story and view component for it.

We add this to the Storybook tests at `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["reusable", "external-link"],
    variants: ["external-link"],
  },
  ...
];

async function findVariantFiles(
  ...
): Promise<string[]> {
  try {
    await fs.access(directoryPath);
  } catch (_) {
    return [];
  }

  ...
}
```

Note that we edit `findVariantFiles` here using the directory checks mentioned [here](https://nodejs.org/en/learn/manipulating-files/working-with-folders-in-nodejs) because otherwise, the test errors out because the directory doesn't even exist yet. If it doesn't exist yet, there are no variants to speak of, and therefore we simply return an empty array instead of having it error out.

We then add the new screenshot at `src-svelte/screenshots/baseline/reusable/external-link/external-link.png`.

#### Linking to OpenAI API key page

Now we can do the actual linking. We edit `src-svelte/src/routes/components/api-keys/Form.svelte` to allow for an optional link to a settings page:

```svelte
<script lang="ts">
  ...
  export let apiKeyUrl: string | undefined = undefined;
  ...
  </script>

<div class="container" ...>
  <div class="inset-container">
    <form ...>
      {#if apiKeyUrl}
        <p>
          Tip: Get your {service} key <a href={apiKeyUrl} target="_blank">here</a>.
        </p>
      {/if}

      ...
    </form>
  </div>
</div>

<style>
  ...

  form p {
    margin: 0 0 0.25rem;
    color: #666666;
    text-align: center;
  }

  ...
</style>
```

We propagate this up to `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  ...
  export let apiKeyUrl: string | undefined = undefined;
  ...
</script>

<div class="container">
  ...

  {#if editing}
    <Form ... {apiKeyUrl} ... />
  {/if}
</div>
```

and put the source for the URL data in `src-svelte/src/routes/components/api-keys/Display.svelte`:

```svelte
<InfoBox title="API Keys" ...>
  ...
    <div ...>
      <Service
        name="OpenAI"
        apiKeyUrl="https://platform.openai.com/api-keys"
        ...
      />
    </div>
  ...
</InfoBox>
```

We add a test for this propagation in `src-svelte/src/routes/components/api-keys/Display.test.ts`:

```ts
  test("shows link to API key", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    await toggleOpenAIForm();
    const apiKeyLink = screen.getByRole("link", { name: "here" });
    expect(apiKeyLink).toHaveAttribute(
      "href",
      "https://platform.openai.com/api-keys",
    );
  });
```

As expected, two of our screenshots have changed: `editing-pre-filled.png` and `editing.png`. We update `src-svelte/Makefile` to also update the gold screenshots when the local ones have been marked as changed:

```Makefile
update-local-screenshots:
	...
	cp -r screenshots/local/testing/actual/* screenshots/baseline/

```

They may need to be updated again anyways after the CI run, but at least they'll be explicitly marked as needing an update.

We now add our new screenshots:

```bash
$ make update-local-screenshots
```

and commit everything.

### Local DB read

We change our minds yet again and decide to use the profile init file only as a fallback in case we can't determine the user's current shell. This is so that the user will have access to their new API keys in a new terminal session without having to log out and back in again. This does mean we'll have to also save the API keys to the local database.

#### Shell init fallback

We edit `src-tauri/src/commands/system.rs`, renaming `get_relative_shell_init_file` to `get_relative_profile_init_file` and modifying the tests to match:

```rs
...

fn get_relative_profile_init_file() -> Option<String> {
    #[cfg(target_os = "linux")]
    return Some("~/.profile".to_string());
    ...
}

fn get_shell_init_file(shell: &Option<Shell>) -> Option<String> {
    let relative_file = match shell {
        Some(Shell::Bash) => Some("~/.bashrc".to_string()),
        Some(Shell::Zsh) => Some("~/.zshrc".to_string()),
        None => get_relative_profile_init_file(),
    };
    relative_file.as_ref().map(|f| shellexpand::tilde(f).to_string())
}

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    let shell = get_shell();
    let shell_init_file = get_shell_init_file(&shell);

    SystemInfo {
        ...,
        shell,
        shell_init_file,
    }
}

#[cfg(test)]
mod tests {
  ...

  #[test]
    fn test_can_predict_shell_init() {
        let shell = Shell::Zsh;
        let shell_init_file = get_shell_init_file(&Some(shell));
        println!("Shell init file is {:?}", shell_init_file);
        assert!(shell_init_file.is_some());
        let file_path = shell_init_file.unwrap();
        assert!(file_path.starts_with('/'));
        assert!(file_path.ends_with(".zshrc"));
    }

    #[test]
    fn test_can_predict_profile_init() {
        let shell_init_file = get_shell_init_file(&None).unwrap();
        println!("Shell init file is {}", shell_init_file);
        assert!(shell_init_file.starts_with('/'));
        assert!(shell_init_file.ends_with(".profile"));
    }

    #[test]
    fn test_get_linux_system_info() {
        let system_info = SystemInfo {
            ...,
            shell_init_file: Some("/root/.zshrc".to_string()),
        };

        ...
    }
}
```

We edit `src-tauri/api/sample-calls/get_system_info-linux.yaml` as well:

```yaml
request: ["get_system_info"]
response:
  message: >
    {
      ...
      "shell": "Zsh",
      "shell_init_file": "/root/.zshrc"
    }

```

and edit the corresponding frontend test call at `src-svelte/src/routes/components/Metadata.test.ts`:

```ts
  test("linux system info returned", async () => {
    ...
    await waitFor(() => expect(shellValueCell).toHaveTextContent("Zsh"));
    expect(get(systemInfo)?.shell_init_file).toEqual("/root/.zshrc");
    ...
  }
```

#### Setting up API key persistence in DB

We create a new migration for the files:

```bash
$ diesel migration generate create_api_keys
```

Then we edit `src-tauri/migrations/2024-01-07-035038_create_api_keys/up.sql`:

```sql
CREATE TABLE api_keys (
  service VARCHAR PRIMARY KEY NOT NULL,
  api_key VARCHAR NOT NULL
)

```

and `src-tauri/migrations/2024-01-07-035038_create_api_keys/down.sql`:

```sql
DROP TABLE api_keys

```

We delete the previous migrations at `src-tauri/migrations/2023-08-17-054802_create_executions`, which were only created for demo purposes, and run these new migrations to regenerate our `schema.rs`:

```bash
$ rm /root/.local/share/zamm/zamm.sqlite3
$ diesel migration run --database-url /root/.local/share/zamm/zamm.sqlite3
Running migration 2024-01-07-035038_create_api_keys
```

`src-tauri/src/schema.rs` has been automatically edited.

Now we install `strum`, which is mentioned in [this answer](https://stackoverflow.com/a/62711168):

```bash
$ cargo add strum strum_macros
```

and edit `src-tauri/src/setup/api_keys.rs` to add a few new derivations to the Service enum (in particular, `EnumString` and `Display` for easy string interop, and `AsExpression` and `FromSqlRow` for Diesel interop):

```rs
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::sql_types::Text;
...
use strum_macros::{Display, EnumString};

#[derive(
    ...
    EnumString,
    Display,
    AsExpression,
    FromSqlRow,
)]
#[diesel(sql_type = Text)]
#[strum(serialize_all = "snake_case")]
pub enum Service {
    OpenAI,
}

...
```

Finally, we rewrite `src-tauri/src/models.rs` like so:

```rs
use crate::schema::api_keys;
use crate::setup::api_keys::Service;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::str::FromStr;

#[derive(Queryable, Selectable, Debug)]
pub struct ApiKey {
    pub service: Service,
    pub api_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey<'a> {
    pub service: Service,
    pub api_key: &'a str,
}

impl ToSql<Text, Sqlite> for Service
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let service_str = self.to_string();
        out.set_value(service_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Service
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let service_str = String::from_sql(bytes)?;
        let parsed_service = Service::from_str(&service_str)?;
        Ok(parsed_service)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::db::MIGRATIONS;

    use diesel_migrations::MigrationHarness;

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    #[test]
    fn test_uuid_serialization_and_deserialization() {
        let mut conn = setup_database();
        let dummy_api_key = "0p3n41-4p1-k3y";

        let openai_api_key = NewApiKey {
            service: Service::OpenAI,
            api_key: dummy_api_key,
        };

        // Insert
        diesel::insert_into(api_keys::table)
            .values(&openai_api_key)
            .execute(&mut conn)
            .unwrap();

        // Query
        let results: Vec<ApiKey> = api_keys::table.load(&mut conn).unwrap();
        assert_eq!(results.len(), 1);

        let retrieved_api_key = &results[0];
        assert_eq!(retrieved_api_key.service, Service::OpenAI);
        assert_eq!(retrieved_api_key.api_key.as_str(), dummy_api_key);
    }
}

```

All the Tauri tests still pass. We haven't edited the sample API files, so we don't need to worry about Svelte because the API boundary hasn't changed.

#### Using DB for API keys

We'll now use the DB as a fallback if API keys aren't available via the environment. We edit `src-tauri/src/main.rs` to pass the DB connection to the API keys setup function:

```rs
    let mut possible_db = setup::get_db();
    let api_keys = setup_api_keys(&mut possible_db);

    tauri::Builder::default()
        .manage(ZammDatabase(Mutex::new(possible_db)))
        .manage(ZammApiKeys(Mutex::new(api_keys)))
        ...;
```

Now we edit `src-tauri/src/setup/api_keys.rs`:

```rs
use crate::models::ApiKey;
use crate::schema::api_keys;
use diesel;
...
use diesel::prelude::*;
...

pub fn setup_api_keys(possible_db: &mut Option<SqliteConnection>) -> ApiKeys {
    let mut api_keys = ApiKeys { openai: None };

    if let Some(conn) = possible_db.as_mut() {
        let load_result: Result<Vec<ApiKey>, diesel::result::Error> =
            api_keys::table.load(conn);
        if let Ok(api_keys_rows) = load_result {
            for api_key in api_keys_rows {
                api_keys.update(&api_key.service, api_key.api_key);
            }
        }
    }

    // database keys will get overridden by environment keys
    if let Ok(openai_api_key) = env::var("OPENAI_API_KEY") {
        api_keys.openai = Some(openai_api_key);
    }

    api_keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewApiKey;
    use crate::setup::db::MIGRATIONS;
    use diesel_migrations::MigrationHarness;
    use temp_env;

    const DUMMY_API_KEY: &str = "0p3n41-4p1-k3y";

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    #[test]
    fn test_get_empty_api_keys_no_db() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let api_keys = setup_api_keys(&mut None);
            assert!(api_keys.openai.is_none());
        });
    }

    #[test]
    fn test_get_present_api_keys_no_db() {
        temp_env::with_var("OPENAI_API_KEY", Some(DUMMY_API_KEY), || {
            let api_keys = setup_api_keys(&mut None);
            assert_eq!(api_keys.openai, Some(DUMMY_API_KEY.to_string()));
        });
    }

    #[test]
    fn test_get_api_keys_from_db() {
        let mut conn = setup_database();
        diesel::insert_into(api_keys::table)
            .values(&NewApiKey {
                service: Service::OpenAI,
                api_key: DUMMY_API_KEY,
            })
            .execute(&mut conn)
            .unwrap();

        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let some_conn = Some(conn);
            let api_keys = setup_api_keys(&mut some_conn);
            assert!(api_keys.openai.is_none());
        });
    }
}
```

The last function produces the error

```
error[E0507]: cannot move out of `conn`, a captured variable in an `Fn` closure
   --> src/setup/api_keys.rs:118:53
    |
108 | ...et mut conn = setup_database();
    |       -------- captured outer variable
...
117 | ...emp_env::with_var("OPENAI_API_KEY", None::<String>, || {
    |                                                        -- captured by this `Fn` closure
118 | ...   let api_keys = setup_api_keys(&mut Some(conn));
    |                                               ^^^^ move occurs because `conn` has type `diesel::SqliteConnection`, which does not implement the `Copy` trait
```

Turns out `temp_env::with_var` expects an Fn instead of an FnOnce, so we change the test to

```rs
    #[test]
    fn test_get_api_keys_from_db() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let mut conn = setup_database();
            diesel::insert_into(api_keys::table)
                .values(&NewApiKey {
                    service: Service::OpenAI,
                    api_key: DUMMY_API_KEY,
                })
                .execute(&mut conn)
                .unwrap();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, Some(DUMMY_API_KEY.to_string()));
        });
    }
```

We continue on with a couple more tests for edge cases:

```rs
    #[test]
    fn test_env_key_overrides_db_key() {
        let custom_api_key = "c0st0m-4p1-k3y";

        temp_env::with_var("OPENAI_API_KEY", Some(custom_api_key.to_string()), || {
            let mut conn = setup_database();
            diesel::insert_into(api_keys::table)
                .values(&NewApiKey {
                    service: Service::OpenAI,
                    api_key: DUMMY_API_KEY,
                })
                .execute(&mut conn)
                .unwrap();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, Some(custom_api_key.to_string()));
        });
    }

    #[test]
    fn test_empty_db_doesnt_crash() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let conn = setup_database();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, None);
        });
    }
```

#### Updating DB on API key change

Now we need to actually write to the DB so that we can read from it later. Let's add a new error type to `src-tauri/src/commands/errors.rs`:

```rs
#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error(transparent)]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },
    ...
}
```

Now we can edit `src-tauri/src/commands/keys/set.rs` to do the saving while capturing any errors during the save. We copy and modify the logic from writing the API key to disk, and modify our tests to verify the result of the database operation after the fact.

```rs
...
use crate::{ZammApiKeys, ZammDatabase};
...
use crate::schema::api_keys;
use diesel::RunQueryDsl;

...

fn set_api_key_helper(
    ...,
    zamm_db: &ZammDatabase,
    ...
) -> ZammResult<()> {
    ...
    let db = &mut zamm_db.0.lock().unwrap();
    ...

    // write new API key to database before we can no longer borrow it
    let db_update_result = || -> ZammResult<()> {
        if api_key.is_empty() {
            return Ok(());
        }

        if let Some(conn) = db.as_mut() {
            diesel::replace_into(api_keys::table)
                .values(crate::models::NewApiKey {
                    service: service.clone(),
                    api_key: &api_key,
                })
                .execute(conn)?;
        }
        Ok(())
    }();

    ...

    init_update_result?;
    db_update_result
}

#[tauri::command(async)]
#[specta]
pub fn set_api_key(
    ...,
    database: State<ZammDatabase>,
    ...
) -> ZammResult<()> {
    set_api_key_helper(..., &database, ...)
}

#[cfg(test)]
pub mod tests {
    ...
    use diesel::prelude::*;
    use crate::setup::db::MIGRATIONS;
    use crate::schema;
    use diesel_migrations::MigrationHarness;
    ...

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    ...

    fn get_openai_api_key_from_db(db: &ZammDatabase) -> Option<String> {
        use schema::api_keys::dsl::*;
        let mut conn_mutex = db.0.lock().unwrap();
        let conn = conn_mutex.as_mut().unwrap();
        api_keys
            .filter(service.eq(Service::OpenAI))
            .select(api_key)
            .first::<String>(conn)
            .ok()
    }

    pub fn check_set_api_key_sample(
        ...
    ) {
        let conn = setup_database();
        let db = ZammDatabase(Mutex::new(Some(conn)));

        ...

        let actual_result = set_api_key_helper(
            ...,
            &db,
            ...
        );

        ...

        if request.api_key.is_empty() {
            ...
            assert_eq!(get_openai_api_key_from_db(&db), None);
        } else {
            assert_eq!(..., Some(request.api_key.clone()));
            assert_eq!(
                get_openai_api_key_from_db(&db),
                Some(request.api_key.clone())
            );
        }

        ...
    }

    ...
}
```

We realize that we need to also remove API keys from the DB when they are unset, so we modify the file again to do that and to also test that the database removal actually works:

```rs
...
use diesel::{ExpressionMethods, ...};
...

fn set_api_key_helper(
    ...
) -> ZammResult<()> {
    ...

    // write new API key to database before we can no longer borrow it
    let db_update_result = || -> ZammResult<()> {
        if let Some(conn) = db.as_mut() {
            if api_key.is_empty() {
                // delete from db
                diesel::delete(api_keys::table)
                    .filter(api_keys::service.eq(service))
                    .execute(conn)?;
            } else {
                diesel::replace_into(api_keys::table)
                    .values(crate::models::NewApiKey {
                        service: service.clone(),
                        api_key: &api_key,
                    })
                    .execute(conn)?;
            }
        }
        Ok(())
    }();

    ...
}

...

#[cfg(test)]
pub mod tests {
    ...
    use crate::models::NewApiKey;
    ...

    pub fn setup_zamm_db() -> ZammDatabase {
        ZammDatabase(Mutex::new(Some(setup_database())))
    }

    pub fn check_set_api_key_sample(
        db: &ZammDatabase,
        ...
    ) {
      ...
    }

    fn check_set_api_key_sample_unit(
        db: &ZammDatabase,
        ...
    ) {
        check_set_api_key_sample(db, ...);
    }

    #[test]
    fn test_write_new_init_file() {
        ...
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            ...
        );
    }

    ...

    #[test]
    fn test_unset() {
        let dummy_key = "0p3n41-4p1-k3y";
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some(dummy_key.to_string()),
        }));
        let mut conn = setup_database();
        diesel::insert_into(api_keys::table)
            .values(&NewApiKey {
                service: Service::OpenAI,
                api_key: dummy_key,
            })
            .execute(&mut conn)
            .unwrap();

        check_set_api_key_sample_unit(
            &ZammDatabase(Mutex::new(Some(conn))),
            ...
        );
        assert!(api_keys.0.lock().unwrap().openai.is_none());
    }

    ...
}
```

All the other tests other than `test_unset` are modified in the same manner as `test_write_new_init_file`. We need to also edit the test at `src-tauri/src/commands/keys/mod.rs`:

```rs
...

#[cfg(test)]
mod tests {
    ...
    use set::tests::{..., setup_zamm_db};
    ...

    #[test]
    fn test_get_after_set() {
        ...

        check_set_api_key_sample(
            &setup_zamm_db(),
            ...
        );

        ...
    }
```

#### Updating the front-end text

Now that we are saving to and reading from the DB by default, the environment variable export is more optional than before. We should edit the labels, and also explain them for the non-technical user.

We create a component at `src-svelte/src/lib/Explanation.svelte` for standardized tooltips:

```svelte
<script lang="ts">
  export let text: string;
</script>

<sup>
  <span class="link-like" title={text}>[?]</span>
</sup>

<style>
  .link-like {
    cursor: help;
  }
</style>

```

We use a span instead of a link to avoid the Svelte error

```
A11y: '#' is not a valid href attribute
```

The solutions [here](https://stackoverflow.com/q/52801051) don't prevent Firefox from actually trying to navigate to the new page from inside the Storybook frame, so we use a span instead. We still make it look like a link by editing `src-svelte/src/routes/styles.css`:

```css
a, .link-like {
  ...
}
```

We edit `src-svelte/src/routes/components/api-keys/Form.svelte`, splitting `exportExplanation` across two lines to avoid eslint errors and using [this trick](https://stackoverflow.com/a/22561351) to get the newline to show up in the tooltip:

```svelte
<script lang="ts">
  ...
  import Explanation from "$lib/Explanation.svelte";

  ...
  const exportExplanation =
    `Exports this API key for use in other programs on your computer.&#10;&#13;` +
    `Don't worry about this option if you're not a programmer.`;
  
  ...
</script>

<div ...>
  <div ...>
    <form ...>
      ...

      <div class="form-row">
        <label for="saveKey" class="accessibility-only"
          >Export as environment variable?</label
        >
        <input
          type="checkbox"
          ...
        />
        <div>
          <label for="saveKeyLocation">Export from:</label>
          <Explanation text={exportExplanation} />
        </div>
        <TextInput ... />
      </div>

      ...
    </form>
  </div>
</div>
```

We realize that interestingly enough, setting the title attribute directly works fine, but when passed through a component, Svelte appears to encode the text for us, and the users see the `&#10;&#13;` text as well instead of a newline. We therefore change the line to:

```ts
  const exportExplanation =
    `Exports this API key for use in other programs on your computer.\n` +
    `Don't worry about this option if you're not a programmer.`;
```

We do a string replace on all the corresponding tests in `src-svelte/src/routes/components/api-keys/Display.test.ts`. For example:

```ts
  test("preserves unsubmitted changes after opening and closing form", async () => {
    ...
    let saveKeyCheckbox = screen.getByLabelText(
      "Export as environment variable?",
    );
    let fileInput = screen.getByLabelText("Export from:");
    ...
    saveKeyCheckbox = screen.getByLabelText("Export as environment variable?");
    fileInput = screen.getByLabelText("Export from:");
    ...
  });
```

As expected, we have to once again update the `editing-pre-filled.png` and `editing.png` screenshots.

## Metadata display

We should now make the rest of the metadata display use real values instead of mocked ones.

### Adding OS

We'll start by add the OS to our desired return value at `src-tauri/api/sample-calls/get_system_info-linux.yaml`:

```yaml
request: ["get_system_info"]
response: >
  {
    "os": "Linux",
    ...
  }

```

We get the backend tests passing again by producing the desired output in `src-tauri/src/commands/system.rs`:

```rs
...

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum OS {
    MacOS,
    Linux,
}

...

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct SystemInfo {
    os: Option<OS>,
    ...
}

fn get_os() -> Option<OS> {
    #[cfg(target_os = "linux")]
    return Some(OS::Linux);
    #[cfg(target_os = "macos")]
    return Some(OS::MacOS);
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    return None;
}

...

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: get_os(),
        shell: get_shell(),
        shell_init_file: get_shell_init_file(),
    }
}

#[cfg(test)]
mod tests {
    ...

    #[test]
    fn test_can_determine_os() {
        let os = get_os();
        println!("Determined OS to be {:?}", os);
        assert!(os.is_some());
    }

    ...

    #[test]
    fn test_get_linux_system_info() {
        let system_info = SystemInfo {
            os: Some(OS::Linux),
            ...
        };

        check_get_system_info_sample(
            "./api/sample-calls/get_system_info-linux.yaml",
            &system_info,
        );
    }
}
```

We make sure that `src-svelte/src/lib/bindings.ts` gets updated automatically, as usual.

Next, we get `src-svelte/src/routes/components/Metadata.svelte` to display the dynamic value instead of the hard-coded mock value:

```svelte
<InfoBox title="System Info" {...$$restProps}>
  ...
      <tr>
        <td>OS</td>
        <td>{systemInfo.os ?? "Unknown"}</td>
      </tr>
  ...
</InfoBox>
```

We test that this works in `src-svelte/src/routes/components/Metadata.test.ts`:

```ts
  test("linux system info returned", async () => {
    ...
    const osRow = screen.getByRole("row", { name: /OS/ });
    const osValueCell = within(osRow).getAllByRole("cell")[1];
    expect(osValueCell).toHaveTextContent("Linux");
  });
```

The screenshot tests are expected to continue passing as before because we're rendering the same thing, just with a real value instead of a dummy one now.

Finally, we edit `src-svelte/src/routes/components/api-keys/Display.test.ts` to provide a properly mocked value:

```ts
  test("some API key set", async () => {
    systemInfo.set({
      os: null,
      ...
    });
    ...
  });
```

We do this for each of the tests in that file.

### Adding version

We follow the previous steps and edit `src-tauri/api/sample-calls/get_system_info-linux.yaml` again:

```yaml
request: ["get_system_info"]
response: >
  {
    "zamm_version": "0.0.0",
    ...
  }

```

We'll edit `src-tauri/src/commands/system.rs` as well using an environment variable mentioned [here](https://doc.rust-lang.org/cargo/reference/environment-variables.html):

```rs
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct SystemInfo {
    zamm_version: String,
    ...
}

fn get_zamm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

...

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        zamm_version: get_zamm_version(),
        ...
    }
}
```

#[cfg(test)]
mod tests {
    ...

    #[test]
    fn test_can_determine_zamm_version() {
        let zamm_version = get_zamm_version();
        println!("Determined Zamm version to be {}", zamm_version);
        assert!(!zamm_version.is_empty());
    }

    ...

    #[test]
    fn test_get_linux_system_info() {
        let system_info = SystemInfo {
            zamm_version: "0.0.0".to_string(),
            ...
        };

        ...
    }
}
```

And `src-svelte/src/routes/components/Metadata.svelte`:

```svelte
<InfoBox title="System Info" {...$$restProps}>
  ...
      <tr>
        <td>Version</td>
        <td class="version-value">{systemInfo.zamm_version}</td>
      </tr>
  ...
</InfoBox>
```

And `src-svelte/src/routes/components/Metadata.test.ts`:

```ts
  test("linux system info returned", async () => {
    ...
    const versionRow = screen.getByRole("row", { name: /Version/ });
    const versionValueCell = within(versionRow).getAllByRole("cell")[1];
    expect(versionValueCell).toHaveTextContent("0.0.0");
    ...
  });
```

Before we go on to edit `Display.test.ts`, we first edit `src-svelte/src/lib/system-info.ts` to provide a default test value just like we did in `src-svelte/src/lib/preferences.ts` for user preferences:

```ts
export const NullSystemInfo: SystemInfo = {
  zamm_version: "dummy",
  os: null,
  shell: null,
  shell_init_file: null,
};
```

Now edit `src-svelte/src/routes/components/api-keys/Display.test.ts` to use this new dummy value in each test:

```ts
...
import { systemInfo, NullSystemInfo } from "$lib/system-info";
...

  test("some API key set", async () => {
    systemInfo.set({
      ...NullSystemInfo,
      shell_init_file: "/home/rando/.zshrc",
    });
    ...
  });
```
