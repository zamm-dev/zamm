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

## Adding sliders

We add these dummy preferences to `src-svelte/src/lib/preferences.ts`:

```ts
export const animationsOn = writable(true);
export const animationSpeed = writable(4);
export const volume = writable(100);
```

We want to expose these new preferences in the settings page, but first we create slider components styled specifically for the settings page, by creating `src-svelte/src/routes/settings/SettingsSlider.svelte`:

```svelte
<script lang="ts">
  import Slider from "$lib/Slider.svelte";

  export let label: string;
  export let min = 0;
  export let max: number;
  export let step: number | undefined = undefined;
  export let value: number = min;
</script>

<div class="settings-slider container">
  <Slider {label} {min} {max} {step} bind:value />
</div>

<style>
  .container {
    padding: calc(0.5 * var(--side-padding)) var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
  }

  .container:hover {
    background: hsla(60, 100%, 50%, 0.2);
  }
</style>

```

This is of course copied straight from `src-svelte/src/routes/settings/SettingsSwitch.svelte`.

We now add the new settings to `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  ...
  import SettingsSlider from "./SettingsSlider.svelte";
  import {
    animationsOn,
    animationSpeed,
    ...
    volume,
    ...
  } from "$lib/preferences";
  ...
</script>

<InfoBox title="Settings">
  <h3>Animation</h3>
  <div class="container">
    <SettingsSwitch
      label="Enabled"
      bind:toggledOn={$animationsOn}
    />
    <SettingsSwitch
      label="Background"
      bind:toggledOn={$unceasingAnimations}
      onToggle={onUnceasingAnimationsToggle}
    />
    <SettingsSlider label="General speed" min={0} max={4} bind:value={$animationSpeed} />
  </div>

  <h3>Sound</h3>
  <div class="container">
    <SettingsSwitch
      label="Enabled"
      bind:toggledOn={$soundOn}
      onToggle={onSoundToggle}
    />
    <SettingsSlider label="Volume" min={0} max={200} bind:value={$volume} />
  </div>
</InfoBox>

...
```

We notice that the slider takes up less vertical space than the switch, causing the slider to not be aligned vertically with the switch. This means we have to edit `src-svelte/src/routes/settings/SettingsSlider.svelte` to center the slider vertically:

```css
  .container {
    --label-height: 1.5em;
    --toggle-height: calc(1.2 * var(--label-height));
    min-height: var(--toggle-height);
    display: flex;
    align-items: center;
  }
```

and then edit `src-svelte/src/lib/Slider.svelte` to make sure that the resulting slider flex box still expands to fill 100% of its parent width:

```css
  .container {
    width: 100%;
  }
```

The settings page is definitely getting a bit crowded now, but we don't want to sacrifice too much information density if it can be helped. The settings page is now starting to mimic the crowded 1950’s control panel aesthetic. The only problem is that the Nasalization font for the "Sounds" subcategory looks off, so we edit `src-svelte/src/routes/styles.css`, making sure to keep `h3` and `th` consistent still:

```css
h3, th {
  font-family: var(--font-header);
  font-size: 1.1rem;
  margin: 0;
  font-weight: 400;
  filter: drop-shadow(0px 1px 1px rgba(0, 0, 0, 0.7));
}

h3 {
  margin-top: 1rem;
}
```

Now that we have it looking the way we want, we have to make sure the tests pass as well. Because we have now separated the settings into different categories, where the toggle for enabling a setting is now called "Enabled" rather than "Sounds", we have to update `src-svelte/src/routes/settings/Settings.test.ts` accordingly. We need an accessible way of finding the sound enablement toggle when there are actually now two such toggles with the name "Enabled", so we encapsulate the categories into a `<section>` tag that automatically has the [`region` ARIA role](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/region_role). We create `src-svelte/src/lib/SubInfoBox.svelte` as such:

```svelte
<script lang="ts">
  import getComponentId from "./label-id";

  export let subheading: string;
  const subinfoboxId = getComponentId("subinfobox");
</script>

<section class="sub-info-box" aria-labelledby={subinfoboxId}>
  <h3 id={subinfoboxId}>{subheading}</h3>
  <slot />
</section>

```

and add the same accessibility feature to `src-svelte/src/lib/InfoBox.svelte`:

```svelte
<script lang="ts">
  import getComponentId from "./label-id";

  export let title = "";
  const infoboxId = getComponentId("infobox");
</script>

<section class="container" aria-labelledby={infoboxId}>
  ...
  <div class="border-container">
    <div class="border-box"></div>
    <div class="info-box">
      <h2 id={infoboxId}>{title}</h2>
      <slot />
    </div>
  </div>
</section>
```

We edit `src-svelte/src/routes/settings/Settings.svelte` to make use of this new `SubInfoBox`, which still needs to be wrapped in a `container` div to make use of the component-specific CSS:

```svelte
<script lang="ts">
  ...
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  ...
</script>

<InfoBox title="Settings">
  <div class="container">
    <SubInfoBox subheading="Animation">
      <SettingsSwitch
        label="Enabled"
        bind:toggledOn={$animationsOn}
      />
      <SettingsSwitch
        label="Background"
        bind:toggledOn={$unceasingAnimations}
        onToggle={onUnceasingAnimationsToggle}
      />
      <SettingsSlider label="General speed" min={0} max={4} bind:value={$animationSpeed} />
    </SubInfoBox>
  </div>

  <div class="container">
    <SubInfoBox subheading="Sound">
      <SettingsSwitch
        label="Enabled"
        bind:toggledOn={$soundOn}
        onToggle={onSoundToggle}
      />
      <SettingsSlider label="Volume" min={0} max={200} bind:value={$volume} />
    </SubInfoBox>
  </div>
</InfoBox>

...
```

We now finally get to edit `src-svelte/src/routes/settings/Settings.test.ts`. As described [here](https://testing-library.com/docs/queries/bylabeltext), for nested ARIA queries, don't start the call with `screen.`.

```ts
...
import { act, getByLabelText, render, screen } from "@testing-library/svelte";
...

  test("can toggle sound on and off while saving setting", async () => {
    ...
    const soundRegion = screen.getByRole("region", { name: "Sound" });
    const soundSwitch = getByLabelText(soundRegion, "Enabled");
    ...
  });
```

Now, looking at the component screenshot tests, we realize the change to `SubInfoBox` has messed up the h3 placement. Requirements:

- The h3 subheading should be flush with the rest of the content
- When the viewport is wide enough, the content should be visible in two columns, but the subheading should not be part of those columns
- Every subheading after the first one should have a 1 rem spacing from the previous element

To do all this, we edit `src-svelte/src/lib/SubInfoBox.svelte` as such:

```svelte
  ...
  <div class="content">
    <slot />
  </div>
  ...
```

and `src-svelte/src/routes/settings/Settings.svelte` as such (adding 0.5rem to the top margin as well to better separate the controls from the subheading):

```css
  .container {
    margin-top: 1rem;
  }

  .container:first-of-type {
    margin-top: 0;
  }

  .container :global(.sub-info-box .content) {
    --side-padding: 0.8rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.1rem;
    margin: 0.5rem calc(-1 * var(--side-padding));
  }

  .container :global(h3) {
    margin-left: var(--side-padding);
  }

  /* this takes sidebar width into account */
  @media (min-width: 52rem) {
    .container :global(.sub-info-box .content) {
      grid-template-columns: 1fr 1fr;
    }
  }
```

and we remove this from `src-svelte/src/routes/styles.css`:

```css
...
h3 {
  margin-top: 1rem;
}
...
```

We now update the screenshot `src-svelte/screenshots/baseline/screens/dashboard/metadata/metadata.png` because our changes to the h3 and th displays also affected the metadata info box, and we update the screenshot `src-svelte/screenshots/baseline/screens/settings/tablet.png` to reflect our extensive changes to the settings page. However, we notice that for other screen sizes such as the large phone screen, portions of the Storybook UI are caught in view. We fix this by editing `src-svelte/src/routes/storybook.test.ts` to click the close button on the bottom Storybook panel:

```ts
          await page.goto(
            `http://localhost:6006/?path=/story/${storybookUrl}${variantPrefix}`,
          );
          await page.locator("button[title='Hide addons [A]']").click();
```

After running the tests one more time, we can now add the fixed settings screen screenshots. However, we must now also add the following screenshots, because the increased height of the frame has also changed these screenshots:

- `src-svelte/screenshots/baseline/background/static.png`
- `src-svelte/screenshots/baseline/layout/app/static.png`
- `src-svelte/screenshots/baseline/layout/sidebar/dashboard-selected.png`
- `src-svelte/screenshots/baseline/layout/sidebar/settings-selected.png`

We have finally finished adding sliders to the settings page.

### Volume control

We implement the functionality on the backend first by editing `src-tauri/src/commands/sounds.rs`:

```rust
#[tauri::command]
#[specta]
pub fn play_sound(..., volume: f32) {
    thread::spawn(move || {
        if let Err(e) = play_sound_async(..., volume) {
            ...
        }
    });
}

fn play_sound_async(..., volume: f32) -> ZammResult<()> {
    ...
    let source = Decoder::new(cursor)?.amplify(volume);
    ...
}

#[cfg(test)]
mod tests {
    ...

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        ...
        volume: f32,
    }

    ...

    fn check_play_sound_sample(file_prefix: &str) {
        ...
        #[allow(clippy::let_unit_value)]
        let actual_result = play_sound(..., request.volume);
        ...
    }

    ...
}
```

We then also edit the YAML files to play sound at different volumes. For example, `src-tauri/api/sample-calls/play_sound-whoosh.yaml` changes to:

```yaml
request:
  - play_sound
  - >
    {
      "sound": "Whoosh",
      "volume": 0.5
    }
response: "null"

```

Because this is an internal API, we don't have to worry about backwards compatibility.

`src-svelte/src/lib/bindings.ts` gets automatically updated thanks to Specta:


```ts
...

export function playSound(..., volume: number) {
    return invoke()<null>("play_sound", { ...,volume })
}

...
```

Because sound playing now involves more complicated logic (both check if sound is enabled, and if so, its volume), we refactor this logic out into `src-svelte/src/lib/sound.ts`:

```ts
import { get } from "svelte/store";
import { playSound, type Sound } from "./bindings";
import { soundOn, volume } from "./preferences";

export function playSoundEffect(sound: Sound) {
  if (get(soundOn)) {
    const soundEffectVolume = get(volume) / 100.0;
    playSound(sound, soundEffectVolume);
    if (window._testRecordSoundPlayed !== undefined) {
      window._testRecordSoundPlayed();
    }
  }
}

```

and edit the callers at `src-svelte/src/routes/SidebarUI.svelte`:

```ts
  import { playSoundEffect } from "$lib/sound";

  ...

  function playWhooshSound() {
    playSoundEffect("Whoosh");
  }

  ...
```

and `src-svelte/src/lib/Switch.svelte`:

```ts
  import { playSoundEffect } from "./sound";
  ...

  function playClick() {
    playSoundEffect("Switch");
  }

  ...
```

Because we changed the volume in the whoosh sample call from the default, we edit `src-svelte/src/routes/SidebarUI.test.ts` accordingly to make sure that we're still making the right call as expected:

```ts
import { soundOn, volume } from "$lib/preferences";

...

  test("plays whoosh sound with right volume during page path change", async () => {
    volume.update(() => 50);
    await act(() => userEvent.click(settingsLink));
    expect(spy).toHaveBeenLastCalledWith(...whooshRequest);
  });

...
```

Now we notice that we get the old error `TypeError: this.customTesters is not iterable` (mentioned [here](/zamm/resources/tutorials/setup/dev/playwright-test-components.md)) from `src-svelte/src/lib/Switch.playwright.test.ts`. It turns out that the error message is a red herring, and just an indication that the test is failing but the matcher is also failing to produce output in the way that Vitest expects.

We want to try print debugging, but we find that no console output is being shown. We see [this answer](https://stackoverflow.com/a/75823426) for a solution, and edit the test file accordingly:

```ts
describe("Switch drag test", () => {
  ...

  beforeAll(async () => {
    ...

    page.on('console', async (msg) => {
      const msgArgs = msg.args();
      const logValues = await Promise.all(msgArgs.map(async arg => await arg.jsonValue()));
      console.log(...logValues);
    });
  });

  ...
});
```

Now we see that somehow, `window._testRecordSoundPlayed` is no longer being defined inside the Playwright tests in this file. We commit the results first, and then try to see when exactly the tests break here.

We start by checking out the previous commit `3b096b6`, and see that `yarn test src/lib/Switch.playwright.test.ts` does pass successfully. We then check out our latest commit again, running

```bash
$ yarn vitest src/lib/Switch.playwright.test.ts -t "at end"
```

to repeated run one single indicative test as soon as we make changes. We try

```bash
$ git checkout 3b096b6 -- src/lib/Switch.svelte
```

and, after manually adding a number to the second argument of `playSound`, find that the test now passes. Interestingly, `console.log(window._testRecordSoundPlayed);` still outputs `undefined`, but the test does finally fail when the lines

```ts
    if (window._testRecordSoundPlayed !== undefined) {
      window._testRecordSoundPlayed();
    }
```

are removed. Console logging something else shows that console logging is indeed working as expected. `console.log(typeof window._testRecordSoundPlayed);` finally shows `function` as expected. Testing further in a browser, it turns out that `console.log((() => {console.log("asdf")}))` *returns* `undefined`. but actually logs the message. We edit `src-svelte/src/lib/Switch.playwright.test.ts` to use the first answer instead:

```ts
    page.on("console", (msg) => {
      console.log(msg);
    });
```

Given that changing the call in this one file fails the test, clearly the problem somehow lies in calling the function in `sound.ts`. We edit `src-svelte/src/lib/sound.ts` to always call the function if it exists, and it still never gets called.

We finally realize that the test actually fails now because it's being called *twice*. We were somehow chasing a ghost, debugging a problem caused by our own debugging.

We finally note that the debug logging pollutes the output even when tests succeed, so we add a final guard for it:

```ts
const DEBUG_LOGGING = false;

describe("Switch drag test", () => {
  ...

  beforeAll(async () => {
    ...

    if (DEBUG_LOGGING) {
      page.on("console", (msg) => {
        console.log(msg);
      });
    }
  });

  ...
});
```
