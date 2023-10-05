# Settings page

We create a new top-level page. In this case, the link to it is already in `src-svelte/src/routes/Sidebar.svelte`, so we don't need to do anything there. Instead, we create `src-svelte/src/routes/settings/+page.svelte`:

```svelte
<script lang="ts">
  import Settings from "./Settings.svelte";
</script>

<Settings />

```

It is relatively barebones because we want to be able to display most of the page contents in Storybook. We then create the imported file `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SettingsSwitch from "./SettingsSwitch.svelte";
</script>

<InfoBox title="Settings">
  <div class="container">
    <SettingsSwitch label="Unceasing animations" />
    <SettingsSwitch label="Sounds" toggledOn={true} />
  </div>
</InfoBox>

<style>
  .container {
    --side-padding: 0.8rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.1rem;
    margin: 0 calc(-1 * var(--side-padding)) 0.5rem;
  }

  /* this takes sidebar width into account */
  @media (min-width: 52rem) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>

```

As noted [here](https://css-tricks.com/equal-columns-with-flexbox-its-more-complicated-than-you-might-think/), it's probably better to use CSS grids for equally spaced grid layouts, rathern than flexbox because that's better suited for flexible layouts.

The media query is so that when the page gets wide enough, we can display two columns of switches rather than one. The `--side-padding`, as we will see and as the name implies, controls the spacing on the sides between the switches. However, we don't want much spacing between the switches and the edge of the parent container, so we set `margin` to a negative value.

We add the same media query to `src-svelte/src/lib/Switch.svelte`:

```css
  @media (min-width: 52rem) {
    label {
      white-space: nowrap;
    }
  }
```

This is so that we avoid the situation where labels wrap instead of forcing the entire switch element to elongate. This is mostly really only useful if we do an alternative layout option like flexbox.

We also create `src-svelte/src/routes/settings/SettingsSwitch.svelte`:

```svelte
<script lang="ts">
  import Switch from "$lib/Switch.svelte";

  export let label: string;
  export let toggledOn = false;
</script>

<div class="container">
  <Switch label={label} toggledOn={toggledOn} />
</div>

<style>
  .container {
    padding: calc(0.5 * var(--side-padding)) var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
  }

  .container:hover {
    background: hsla(60, 100%, 50%, 0.20);
  }
</style>

```

This is just a wrapper around our existing switch element that adds some padding and a highlight when the user mouses over. We use a wrapper because we are as of yet unsure how likely this particular pattern is to spread, and it will be easier to merge two disparate files together than to refactor one out into two.

Finally, we add this page to Storybook at `src-svelte/src/routes/settings/Settings.stories.ts`. We set a different default viewport for each story so that we can see how it renders across different screen sizes:

```ts
import SettingsComponent from "./Settings.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: SettingsComponent,
  title: "Screens/Settings",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: SettingsComponent,
  props: args,
});

export const TinyPhoneScreen: StoryObj = Template.bind({}) as any;
TinyPhoneScreen.parameters = {
  viewport: {
      defaultViewport: "mobile1"
  }
}

export const LargePhoneScreen: StoryObj = Template.bind({}) as any;
LargePhoneScreen.parameters = {
  viewport: {
      defaultViewport: "mobile2"
  }
}

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
      defaultViewport: "tablet"
  }
}

```

Finally, we add a new entry to `src-svelte/src/routes/storybook.test.ts` to ensure that we keep track of how this page renders from now on:

```ts
...

const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "settings"],
    variants: ["tiny-phone-screen", "large-phone-screen", "tablet"],
    screenshotEntireBody: true,
  },
];

...
```

Check that the tests now pass.

## Hooking up stores

In `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  ...
  import { soundOn } from "../../preferences";
</script>

<InfoBox title="Settings">
  <div class="container">
    ...
    <SettingsSwitch label="Sounds" bind:toggledOn={$soundOn} />
  </div>
</InfoBox>
```

Because we have a wrapper around the regular switch, we have to update the wrapper at `src-svelte/src/routes/settings/SettingsSwitch.svelte` too:

```svelte
<div class="container">
  <Switch {label} bind:toggledOn />
</div>
```

Finally, we test that this option takes effect immediately by creating `src-svelte/src/routes/settings/Settings.test.ts`:

```ts
import { expect, test, vi } from "vitest";
import { get } from 'svelte/store';
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Settings from "./Settings.svelte";
import { soundOn } from "../../preferences";

const mockAudio = {
  pause: vi.fn(),
  play: vi.fn(),
};

global.Audio = vi.fn().mockImplementation(() => mockAudio);

describe("Switch", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("can toggle sound on and off", async () => {
    render(Settings, {});
    expect(get(soundOn)).toBe(true);
    expect(mockAudio.play).not.toHaveBeenCalled();

    const soundSwitch = screen.getByLabelText("Sounds");
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(false);
    expect(mockAudio.play).not.toHaveBeenCalled();

    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(true);
    expect(mockAudio.play).toBeCalledTimes(1);
  });
});

```

## Full container clickability

We want the entire container, including padding, to be clickable. We'll do this by calling the child switch's toggle from the parent. There are two main ways of doing this: doing a [module export](https://learn.svelte.dev/tutorial/module-exports), or [binding](https://stackoverflow.com/a/61334528) the child function to a parent variable.

If we try the first way, we get:

```
/root/zamm/src-svelte/src/lib/Switch.svelte:29:7 Cannot reference store value inside <script context="module">
3:53:58 AM [vite] Internal server error: /root/zamm/src-svelte/src/lib/Switch.svelte:29:7 Cannot reference store value inside <script context="module">
  Plugin: vite-plugin-svelte
  File: /root/zamm/src-svelte/src/lib/Switch.svelte:29:7
   27 |  let dragPositionOnLeft = false;
   28 |  function playClick() {
   29 |    if (!$soundOn) {
                ^
   30 |      return;
   31 |    }
```

Instead, we do it the other way by editing `src-svelte/src/routes/settings/SettingsSwitch.svelte`:

```svelte
<script lang="ts">
  ...
  let switch: Switch;
</script>

<div class="container" on:click|preventDefault={switch.toggle} role="none">
  <Switch {label} bind:this={switch} bind:toggledOn />
</div>

<style>
  .container {
    ...
    cursor: pointer;
  }

  ...
</style>
```

and in `src-svelte/src/lib/Switch.svelte`:

```ts
  export function toggle() {
    ...
  }
```

Now we get

```
Error while preprocessing /root/zamm/src-svelte/src/routes/settings/SettingsSwitch.svelte - Transform failed with 1 error:
/root/zamm/src-svelte/src/routes/settings/SettingsSwitch.svelte:6:6: ERROR: Expected ";" but found "switch"
3:59:30 AM [vite] Internal server error: Error while preprocessing /root/zamm/src-svelte/src/routes/settings/SettingsSwitch.svelte - Transform failed with 1 error:
/root/zamm/src-svelte/src/routes/settings/SettingsSwitch.svelte:6:6: ERROR: Expected ";" but found "switch"
  Plugin: vite-plugin-svelte
  File: /root/zamm/src-svelte/src/routes/settings/SettingsSwitch.svelte

   Expected ";" but found "switch"
   4  |    export let label |  string;
   5  |    export let toggledOn = false;
   6  |    let switch;
      |        ^
   7  |
```

We realize that this is because `switch` is a keyword and cannot be used as a variable name, so instead we rename it to `switchChild`. Now we discover that even with `on:click|preventDefault`, button clicking doesn't work. This is because of the `on:click` on the `button`. We add a new property to the switch:

```svelte
<script lang="ts">
  ...
  export let parentToggle = false;
  ...

  function buttonClicked() {
    if (!parentToggle) {
      toggle();
    }
  }

  ...
</script>

<div class="container">
  {#if label}
    <label for={switchId}>{label}</label>
  {/if}
  <button
    ...
    on:click={buttonClicked}
    ...
  >
    ...
  </button>
</div>
```

and edit the settings switch as well to turn the parent toggle functionality on:

```svelte
<div ...>
  <Switch ... parentToggle={true} />
</div>
```

Now we test with a new file `src-svelte/src/routes/settings/SettingsSwitch.test.ts`:

```ts
import { assert, expect, test, vi } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import SettingsSwitch from "./SettingsSwitch.svelte";

const mockAudio = {
  pause: vi.fn(),
  play: vi.fn(),
};

global.Audio = vi.fn().mockImplementation(() => mockAudio);

describe("Settings switch", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("can be toggled on from clicking the container", async () => {
    const {container} = render(SettingsSwitch, { label: "Test" });

    const switchContainer = container.querySelector(".settings-switch");
    assert(switchContainer);
    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
    await act(() => userEvent.click(switchContainer));
    expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
  });
});

```

## WebdriverIO E2E testing

Edit `webdriver/test/specs/e2e.test.js` to add this:

```js
...

async function findAndClick(selector, timeout) {
  const button = await $(selector);
  await button.waitForClickable({
    timeout,
  });
  await browser.execute("arguments[0].click();", button);
}

describe("Welcome screen", function () {
  ...

  it("should allow navigation to the settings page", async function () {
    findAndClick('a[title="Settings"]');
    findAndClick("aria/Sounds");
    await browser.pause(500); // for CSS transitions to finish
    expect(
      await browser.checkFullPageScreen("settings-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });
});
```

## Adding a new setting

We add a new setting to `src-svelte/src/preferences.ts`:

```ts
export const unceasingAnimations = writable(false);
```

We add a new switch to `src-svelte/src/routes/settings/Settings.svelte`. In this case the switch already exists, so we just hook it up:

```svelte
<script lang="ts">
  import { unceasingAnimations, soundOn } from "../../preferences";
</script>

<InfoBox title="Settings">
  <div class="container">
    <SettingsSwitch label="Unceasing animations" bind:toggledOn={$unceasingAnimations} />
    ...
  </div>
</InfoBox>
```

We now use it where it's needed. We rename `src-svelte/src/routes/Background.svelte` to `src-svelte/src/routes/BackgroundUI.svelte`, edit the corresponding story to import from the new file, and create the original file with:

```svelte
<script lang="ts">
  import BackgroundUI from "./BackgroundUI.svelte";
  import { unceasingAnimations } from "../preferences";
</script>

<BackgroundUI bind:animated={$unceasingAnimations} />

```

and edit the `BackgroundUI` to change the duration reactively:

```svelte
<script lang="ts">
  export let animated = false;
  let duration: number;
  
  $: duration = animated ? 15 : 0;
</script>

...
```
