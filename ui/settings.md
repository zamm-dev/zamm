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

Now we make this setting persistent by editing `src-tauri/src/commands/preferences/models.rs`:

```rust
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    ...
    volume: Option<f32>,
}
```

Note that we have removed `Eq` from `Preferences` because `f32` doesn't implement it. We have to edit `src-tauri/src/commands/preferences/write.rs` as well for this to compile. As expected, Tauri tests now fail. We now edit both get_* and set_* preference sample calls, such as `src-tauri/api/sample-calls/set_preferences-sound-on.yaml`:

```yaml
request:
  - set_preferences
  - >
    {
      "preferences": {
        "unceasing_animations": null,
        "sound_on": true,
        "volume": null
      }
    }
response: "null"

```

Now the Tauri tests pass, but the Svelte tests fail, as expected. We edit the frontend as well at `src-svelte/src/lib/preferences.ts`:

```ts
export const NullPreferences: Preferences = {
  ...
  volume: null,
};
```

Now the tests pass. Ideally, we should create a sample preferences file to check against. We briefly consider deferring this until the creation of these tests can be automated, but notice that the volume slider settings update API call is never actually triggered. Because this will involve a whole non-trivial change that involves piping the callback function from the Settings page parent to the child slider, we will create a new sample file and test it fully anyways. We create `src-tauri/api/sample-calls/set_preferences-volume-partial.yaml`:

```yaml
request:
  - set_preferences
  - >
    {
      "preferences": {
        "unceasing_animations": null,
        "sound_on": null,
        "volume": 0.5
      }
    }
response: "null"

```

and the expected output `src-tauri/api/sample-settings/volume-override/preferences.toml`:

```toml
volume = 0.5

```

and edit `src-tauri/src/commands/preferences/write.rs` to add this test:

```rust
    #[test]
    fn test_set_preferences_volume_partial() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-volume-partial.yaml",
            None,
            "./api/sample-settings/volume-override/preferences.toml",
        );
    }
```

We check that the Tauri tests pass, as expected. We then edit `src-svelte/src/routes/settings/Settings.test.ts` to add a reference to this sample call:

```ts
...
import { fireEvent } from '@testing-library/dom';
...
import { soundOn, volume } from "$lib/preferences";
...

describe("Switch", () => {
  ...
  let setVolumePartialCall: ParsedCall;

  beforeAll(() => {
    ...
    setVolumePartialCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-volume-partial.yaml",
      true,
    );
  });

  ...

  test("can persist changes to volume slider", async () => {
    render(Settings, {});
    expect(get(volume)).toBe(100);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const soundRegion = screen.getByRole("region", { name: "Sound" });
    const volumeSlider = getByLabelText(soundRegion, "Volume");
    playback.addCalls(setVolumePartialCall);
    await fireEvent.change(volumeSlider, { target: { value: 50 } });
    expect(get(volume)).toBe(50);
    expect(tauriInvokeMock).toBeCalledTimes(1);
    expect(playback.unmatchedCalls.length).toBe(0);
  });
});
```

Note that because there's no explicit method for dragging the input range slider in `testing-library` just yet, we are using [this workaround](https://github.com/testing-library/user-event/issues/871#issuecomment-1059317998).

As expected, the test fails because the functionality is still unimplemented. We edit `src-svelte/src/lib/Slider.svelte`:

```svelte
<script lang="ts">
  ...
  export let onUpdate: (newValue: number) => void = () => undefined;
  ...

  function onChange(e: Event) {
    const target = e.target as HTMLInputElement;
    onUpdate(parseFloat(target.value));
  }

  ...
</script>

<div class="container">
  ...
  <input
    ...
    on:change={onChange}
  />
</div>
```

and `src-svelte/src/routes/settings/SettingsSlider.svelte`:

```svelte
<script lang="ts">
  ...
  export let onUpdate: (newValue: number) => void = () => undefined;
</script>

<div class="settings-slider container">
  <Slider ... {onUpdate} ... />
</div>
```

and `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  ...

  const onVolumeUpdate = (newValue: number) => {
    setPreferences({
      ...NullPreferences,
      volume: newValue,
    });
  };
</script>

...

      <SettingsSlider label="Volume" min={0} max={200} onUpdate={onVolumeUpdate} bind:value={$volume} />
```

We see that the tests still fail, albeit for a different reason now because no matching call has been found. We add some more logging to `src-svelte/src/lib/sample-call-testing.ts` to shed light on the problem:

```ts
    assert(matchingCallIndex !== -1, `No matching call found for ${jsonArgs}.\nCandidates are ${this.unmatchedCalls.map((call) => JSON.stringify(call.request)).join("\n")}`);
```

Now we see that the problem is:

```
AssertionError: No matching call found for ["set_preferences",{"preferences":{"unceasing_animations":null,"sound_on":null,"volume":50}}].
Candidates are ["set_preferences",{"preferences":{"unceasing_animations":null,"sound_on":null,"volume":0.5}}]
```

We see that the problem is that we've been setting the volume setting as a percentage on the frontend, but playing it as a float on the backend. We change them to be consistent at `src-svelte/src/lib/preferences.ts`:

```ts
export const volume = writable(1);
```

and `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
      <SettingsSlider label="Volume" min={0} max={2} onUpdate={onVolumeUpdate} bind:value={$volume} />
```

and `src-svelte/src/routes/settings/Settings.test.ts`:

```ts
  test("can persist changes to volume slider", async () => {
    ...
    expect(get(volume)).toBe(1);
    ...
    await fireEvent.change(volumeSlider, { target: { value: 0.5 } });
    expect(get(volume)).toBe(0.5);
    ...
  });
```

and `src-svelte/src/lib/sound.ts`:

```ts
    const soundEffectVolume = get(volume);
```

and `src-svelte/src/routes/SidebarUI.test.ts`:

```ts
  test("plays whoosh sound with right volume during page path change", async () => {
    volume.update(() => 0.5);
    ...
  });
```

Now all the tests finally pass. However, we notice that the sound is still not set on app start. We add a `src-tauri/api/sample-calls/get_preferences-volume-override.yaml` to ensure that the sound file is indeed getting correctly parsed by the backend, and edit `src-tauri/src/commands/preferences/read.rs` to reference it:

```rust
    #[test]
    fn test_get_preferences_with_volume_override() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-volume-override.yaml",
            "./api/sample-settings/volume-override",
        );
    }
```

The tests still pass. We realize the problem is because we never actually update the setting after getting the latest value on app startup. We edit `src-svelte/src/routes/AppLayout.svelte`:

```ts
  import { soundOn, unceasingAnimations, volume } from "$lib/preferences";

  onMount(async () => {
    ...

    if (prefs.volume !== null) {
      volume.set(prefs.volume);
    }

    ...
  });
```

We double-check on `src-svelte/src/routes/AppLayout.test.ts`:

```ts
  ...
  import { soundOn, volume } from "$lib/preferences";
  ...

  test("will set volume if volume preference overridden", async () => {
    expect(get(volume)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const getPreferencesCall = parseSampleCall(
      "../src-tauri/api/sample-calls/get_preferences-volume-override.yaml",
      false,
    );
    playback.addCalls(getPreferencesCall);

    render(AppLayout, {});
    await tickFor(3);
    expect(get(volume)).toBe(0.5);
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });
```

Now the sound gets persisted, but the slider [looks off](./screenshots/6c04f56.png) in the final built app. The progress bar vertical height is proportional to the progress. We therefore completely implement the slider ourselves from scratch in [`slider.md`](./slider.md), so as to not be beholden to the whims of various browsers.

### General animations control

We see that we can disable all animations like [this](https://stackoverflow.com/a/11132887). As such, we edit `src-svelte/src/routes/AppLayout.svelte`:

```svelte
<script>
  ...
  import { soundOn, unceasingAnimations, volume, animationsOn } from "$lib/preferences";

  ...
</script>

<div class="app" class:animations-disabled={animationsOn}>
  ...
</div>

<style>
  ...

  .app.animations-disabled :global(*) {
    animation-play-state: paused !important;
    transition: none !important;
  }
  
  ...
</style>
```

Now we need to actually control this new setting via preferences. We edit `src-tauri/src/commands/preferences/models.rs` to add it:

```rust
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    animations_on: Option<bool>,
    ...
}
```

The Tauri app runs, and Specta updates `src-svelte/src/lib/bindings.ts` to include this new field in the preferences read. Now we edit `src-svelte/src/routes/AppLayout.svelte` again to actuall set animationsOn when preferences are read:

```ts
  onMount(async () => {
    ...

    if (prefs.animations_on !== null) {
      animationsOn.set(prefs.animations_on);
    }

    ...
  });
```

The UI for the setting is already there, so we wire it up in `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts>
  ...

    const onAnimationsToggle = (newValue: boolean) => {
    setPreferences({
      ...NullPreferences,
      animations_on: newValue,
    });
  };

  ...

</script>

<InfoBox title="Settings">
  ...
      <SettingsSwitch label="Enabled" bind:toggledOn={$animationsOn} onToggle={onAnimationsToggle} />
  ...
</InfoBox>
```

We see that `svelte-check` gives an error:

```
/root/zamm/src-svelte/src/lib/preferences.ts:10:14
Error: Property 'animations_on' is missing in type '{ unceasing_animations: null; sound_on: null; volume: null; }' but required in type 'Preferences'. 

export const NullPreferences: Preferences = {
  unceasing_animations: null,
```

We fix `src-svelte/src/lib/preferences.ts` accordingly:

```ts
export const NullPreferences: Preferences = {
  animations_on: null,
  ...
};
```

This reminds us that we should update all existing preference-related API calls. We add this new element to files such as `src-tauri/api/sample-calls/get_preferences-extra-settings.yaml`, and then make sure that all tests pass.

However, we notice that the tests don't pass. We add some tests after all, first by creating `src-tauri/api/sample-calls/get_preferences-animations-off.yaml`:

```yaml
request: ["get_preferences"]
response: >
  {
    "animations_on": false,
    "unceasing_animations": null,
    "sound_on": null,
    "volume": null
  }

```

Because this setting does not involve any interesting new read/write functionality, we can skip writing new Rust tests. Instead, we update `AppLayout` to use an "app" ID instead of a class for the semantics, and create new tests in `src-svelte/src/routes/AppLayout.test.ts`:

```ts
...
import { soundOn, volume, animationsOn } from "$lib/preferences";
...

    test("will enable animations by default", async () => {
    expect(get(animationsOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const getPreferencesCall = parseSampleCall(
      "../src-tauri/api/sample-calls/get_preferences-no-file.yaml",
      false,
    );
    playback.addCalls(getPreferencesCall);

    render(AppLayout, {});
    await tickFor(3);
    expect(get(animationsOn)).toBe(true);
    expect(tauriInvokeMock).toBeCalledTimes(1);
    const app = document.querySelector("#app") as Element;
    expect(app.classList).not.toContainEqual("animations-disabled");
  });

  test("will disable animations if settings set", async () => {
    expect(get(animationsOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const getPreferencesCall = parseSampleCall(
      "../src-tauri/api/sample-calls/get_preferences-animations-off.yaml",
      false,
    );
    playback.addCalls(getPreferencesCall);

    render(AppLayout, {});
    await tickFor(3);
    expect(get(animationsOn)).toBe(false);
    expect(tauriInvokeMock).toBeCalledTimes(1);
    const app = document.querySelector("#app") as Element;
    expect(app.classList).toContainEqual("animations-disabled");
  });
```

These tests fail, and we find that it is because we got the class name condition wrong, and we're also not actually accessing the value of the store with the `$`. We fix it in `src-svelte/src/routes/AppLayout.svelte`:

```svelte
<div id="app" class:animations-disabled={!$animationsOn}>
  ...
</div>
```

### Animation speed control

We follow the route just taken by editing `src-tauri/src/commands/preferences/models.rs`:

```rust
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    ...
    animation_speed: Option<f64>,
    ...
}
```

We update all the preference API call files, and create a new one at `src-tauri/api/sample-calls/get_preferences-animation-speed-override.yaml`:

```yaml
request: ["get_preferences"]
response: >
  {
    "animations_on": null,
    "unceasing_animations": null,
    "animation_speed": 0.9,
    "sound_on": null,
    "volume": null
  }

```

We handle this API call at `src-svelte/src/routes/AppLayout.svelte`:

```ts
  import {
    ...
    animationSpeed,
    ...
  } from "$lib/preferences";

  ...

  onMount(async () => {
    ...
    if (prefs.animation_speed !== null) {
      animationSpeed.set(prefs.animation_speed);
    }
  });
```

and write the settings back out at `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  ...

  const onAnimationSpeedUpdate = (newValue: number) => {
    setPreferences({
      ...NullPreferences,
      animation_speed: newValue,
    });
  };

  ...
</script>

<InfoBox title="Settings">
  ...
        <SettingsSlider
        label="General speed"
        min={0.25}
        max={1}
        ...
        onUpdate={onAnimationSpeedUpdate}
      />
  ...
</div>
```

We make it a minimum of 0.25 so as to not make animations take infinitely long. We are going to make this setting an inverse multiplier on animation durations.

We edit `src-svelte/src/lib/preferences.ts` accordingly as well, to not only do this but also add a null animation speed to the set of null preferences:

```ts
...
export const animationSpeed = writable(1);
...

export const NullPreferences: Preferences = {
  ...
  animation_speed: null,
  ...
};
```

Now for the actual functionality, we update `src-svelte/src/routes/styles.css`:

```css
:root {
  ...
  --base-animation-speed: 1;
  ...
}
```

and we update `src-svelte/src/routes/AppLayout.svelte` again to override this everywhere:

```svelte
<div id="app" class:animations-disabled={!$animationsOn} style="--base-animation-speed: {$animationSpeed};">
  ...
</div>
```

We test this override in `src-svelte/src/routes/AppLayout.test.ts`:

```ts
...
import { soundOn, volume, animationsOn, animationSpeed } from "$lib/preferences";
...

  test("will slow down animations if preference overridden", async () => {
    expect(get(animationSpeed)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const getPreferencesCall = parseSampleCall(
      "../src-tauri/api/sample-calls/get_preferences-animation-speed-override.yaml",
      false,
    );
    playback.addCalls(getPreferencesCall);

    render(AppLayout, {});
    await tickFor(3);
    expect(get(animationSpeed)).toBe(0.9);
    expect(tauriInvokeMock).toBeCalledTimes(1);
    const app = document.querySelector("#app") as Element;
    expect(app.getAttribute("style")).toEqual("--base-animation-speed: 0.9;");
  });
```

Now in `src-svelte/src/routes/SidebarUI.svelte`:

```css
  header {
    --animation-duration: calc(0.1s / var(--base-animation-speed));
    ...
  }
```

We test this out by adding a new story to `src-svelte/src/routes/SidebarUI.stories.ts`:

```ts
import SvelteStoresDecorator from "$lib/__mocks__/stores";
...

export default {
  ...
  decorators: [SvelteStoresDecorator],
};


export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  currentRoute: "/",
  dummyLinks: true,
};
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};

```

We handle this additional preference at `src-svelte/src/lib/__mocks__/stores.ts`:

```ts
...
import { unceasingAnimations, animationSpeed } from "$lib/preferences";

interface Preferences {
  ...
  animationSpeed?: number;
}

...

const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  ...
  if (preferences?.animationSpeed !== undefined) {
    animationSpeed.set(preferences.animationSpeed);
  }

  ...
};
```

However, our new setting has no effect whatsoever. This is because the `base-animation-speed` is not actually being overwritten in the Storybook story because the `AppLayout` is not included here. As such, we'll have to create a mock `AppLayout` at `src-svelte/src/lib/__mocks__/MockAppLayout.svelte`:

```svelte
<script lang="ts">
  import { animationSpeed } from "$lib/preferences";
</script>

<div style="--base-animation-speed: {$animationSpeed};">
  <slot />
</div>

```

We add in this new mock wrapper decorator to `src-svelte/src/routes/SidebarUI.stories.ts`, noting that the [story type](https://stackoverflow.com/a/66861907) is `StoryFn`:

```ts
...
import type { StoryFn, StoryObj } from "@storybook/svelte";
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
...

export default {
  ...
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: MockAppLayout,
        slot: story,
      };
    },
  ],
};

...
```

and we see that our animations finally do take longer to happen.

We finally update `src-svelte/src/routes/settings/Settings.svelte` one last time to lower the minimum further, now that we know 1/4th the speed still feels reasonably fast:

```svelte
      <SettingsSlider
        label="General speed"
        min={0.1}
        ...
      />
```

Finally, we add this setting to the other places with animations, such `src-svelte/src/lib/Slider.svelte`:

```ts
  const transitionAnimation = `transition: left calc(0.1s / var(--base-animation-speed)) ease-out;`;
```

and then edit `src-svelte/src/lib/Slider.stories.ts` in the same way:

```ts
...
import type { StoryFn, StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";

export default {
  ...
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: MockAppLayout,
        slot: story,
      };
    },
  ],
};

...

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  label: "Extra Large Simulation",
  max: 10,
  value: 5,
};
SlowMotion.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
  preferences: {
    animationSpeed: 0.1,
  },
};
```

We notice that the animation speed does not reset to the default values for the stores when going back to the other stories, so we edit `src-svelte/src/lib/__mocks__/stores.ts` to do this for us:

```ts
const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  ...
  if (preferences?.animationSpeed === undefined) {
    animationSpeed.set(1);
  } else {
    animationSpeed.set(preferences.animationSpeed);
  }

  ...
};
```

We do this to `src-svelte/src/lib/Switch.svelte` and its stories as well, and `src-svelte/src/routes/BackgroundUI.svelte` and its stories, and we are now done with initial implementation. However, we find that the sidebar tests are failing because we're running into the same problem around Playwright screenshot timeouts mentioned [here](/zamm/zamm/resources/tutorials/setup/dev/playwright-test-components.md), this time because the sidebar is absolutely positioned and therefore does not give the wrapper div any dimensions. We edit `src-svelte/src/lib/__mocks__/MockAppLayout.svelte` to make it obvious that this is a test wrapper:

```svelte
<div class="storybook-wrapper" ...>
  ...
</div>

```

and now we edit `src-svelte/src/routes/storybook.test.ts` to take this into account too:

```ts
  const takeScreenshot = async (page: Page, screenshotEntireBody?: boolean) => {
    const frame = page.frame({ name: "storybook-preview-iframe" });
    if (!frame) {
      throw new Error("Could not find Storybook iframe");
    }
    let locator = screenshotEntireBody
      ? "body"
      : "#storybook-root > :first-child";
    const elementClass = await frame.locator(locator).getAttribute("class");
    if (elementClass === "storybook-wrapper") {
      locator = "#storybook-root > :first-child > :first-child";
    }
    return await frame.locator(locator).screenshot();
  };
```

#### CI

Now our tests are failing on CI because a bit of the "Storybook update" [popup](/ui/screenshots/c72b7ff.png) ("Click to learn what's new in Storybook") sneaks into our screenshots. Even the `--no-version-updates` [option](https://stackoverflow.com/a/67279259) does anything to hide this. We try upgrading Storybook:

```ts
$ npx storybook@latest upgrade
```

and say "yes" when asked whether or not we want to do an auto-migration.

This still doesn't get rid of the popup. We do a little digging and find that the string is located [here](https://github.com/storybookjs/storybook/blob/3e098fd/code/lib/manager-api/src/modules/whatsnew.ts#L95). It turns out the `whatsnew` module is controlled by [this option](https://storybook.js.org/docs/7.4/react/api/main-config-core#disablewhatsnewnotifications). We edit `src-svelte/.storybook/main.ts` as requested:

```ts
const config: StorybookConfig = {
  ...
  core: {
    disableWhatsNewNotifications: true,
  },
  ...
};
```

Somehow, the `svelte-check` pre-commit hook is failing now too:

```
Loading svelte-check in workspace: /home/runner/work/zamm-ui/zamm-ui/src-svelte
Getting Svelte diagnostics...

/home/runner/work/zamm-ui/zamm-ui/src-svelte/src/lib/Slider.svelte:8:10
Error: Cannot find module '@neodrag/svelte' or its corresponding type declarations. (ts)
    type DragEventData,
  } from "@neodrag/svelte";

```

And we had somehow accidentally deleted `src-svelte/screenshots/baseline/reusable/slider/tablet.png` when upgrading Storybook, which means we just need to put that back in.

#### Sounds

Some sounds, like the sidebar whoosh, should be slower if the animation speed is slower. We'll start off by enabling that on the backend at `src-tauri/src/commands/sounds.rs`:

```rust
...

#[tauri::command]
#[specta]
pub fn play_sound(..., speed: f32) {
    thread::spawn(move || {
        if let Err(e) = play_sound_async(..., speed) {
            ...
        }
    });
}

fn play_sound_async(..., speed: f32) -> ZammResult<()> {
    ...
    let source = Decoder::new(cursor)?.amplify(volume).speed(speed);
    ...
}

#[cfg(test)]
mod tests {
    ...

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        ...
        speed: f32,
    }

    ...

    fn check_play_sound_sample(file_prefix: &str) {
        ...
        #[allow(clippy::let_unit_value)]
        let actual_result = play_sound(request.sound, request.volume, request.speed);
        ...
    }

    ...
}
```

`src-svelte/src/lib/bindings.ts` should get updated automatically. Meanwhile, we make sure nothing changes in `src-tauri/api/sample-calls/play_sound-switch.yaml`:

```yaml
request:
  - play_sound
  - >
    {
      "sound": "Switch",
      "volume": 1,
      "speed": 1
    }
response: "null"

```

and that custom sound volumes are specified in `src-tauri/api/sample-calls/play_sound-whoosh.yaml`:

```yaml
request:
  - play_sound
  - >
    {
      "sound": "Whoosh",
      "volume": 0.5,
      "speed": 0.25
    }
response: "null"

```

After confirming that backend tests still pass, we edit `src-svelte/src/lib/sound.ts` to enable optionally specifying that option on the frontend:

```ts
...

export function playSoundEffect(sound: Sound, speed?: number) {
  if (get(soundOn)) {
    ...
    const soundEffectSpeed = speed || 1;
    try {
      playSound(sound, ..., soundEffectSpeed);
    }
    ...
  }
}

```

Then we edit `src-svelte/src/routes/SidebarUI.svelte`, the one place where this new feature will be used:

```ts
  import { animationSpeed } from "$lib/preferences";
  ...

  function playWhooshSound() {
    playSoundEffect("Whoosh", $animationSpeed);
  }

  ...
```

We update the test `src-svelte/src/routes/SidebarUI.test.ts` to make sure this new feature is actually used:

```ts
...
import { soundOn, volume, animationSpeed } from "$lib/preferences";
...

  test("plays whoosh sound with right speed and volume", async () => {
    volume.update(() => 0.5);
    animationSpeed.update(() => 0.25);
    await act(() => userEvent.click(settingsLink));
    expect(spy).toHaveBeenLastCalledWith(...whooshRequest);
  });
```

We notice when we do this that the volume is low, so we compensate for that by boosting it in `src-svelte/src/lib/sound.ts`:

```ts
export function playSoundEffect(sound: Sound, speed?: number) {
  if (get(soundOn)) {
    const soundEffectSpeed = speed || 1;
    // boost volume to compensate for lowered volume from lower speed
    const soundEffectVolume = get(volume) / soundEffectSpeed;
    ...
  }
}
```

Now `src/routes/SidebarUI.test.ts` fails because the actual call to the API has changed and no longer matches the sample API file. We edit it and document the reason why:

```ts
  test("plays whoosh sound with right speed and volume", async () => {
    // volume is at 0.125 so that when it's boosted 4x to compensate for the 4x
    // reduction in playback speed, the net volume will be at 0.5 as specified in the
    // sample file
    volume.update(() => 0.125);
    ...
  });
```

The pitch is low as well. Browsers themselves have [an option](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preservesPitch) to preserve pitch, but [pitch-corrected audio timestretch](https://en.wikipedia.org/wiki/Audio_time_stretching_and_pitch_scaling) is not available for Rodio. There is the [`pitch_shift`](https://docs.rs/pitch_shift/1.0.0/pitch_shift/struct.PitchShifter.html#method.shift_pitch) library, and we find that the [formula](https://forum.cockos.com/showthread.php?t=251580) for converting speed changes to pitch is "Starting rate * 2^(1/12) = up one semitone". However, the pitch_shift library doesn't take in Rodio data structures, so there will need to be a conversion between different data structures, and most likely caching in order to avoid having to do an expensive pitch correction on each audio playthrough. Pitch correction looks to be more trouble than it is worth at the moment.

The switch, on the other hand, is meant to have a crisp clicking sound when the toggle reaches the other side. As such, this means a sound delay rather than a slowdown. We edit `src-svelte/src/lib/Switch.svelte`:

```svelte
<script lang="ts" context="module">
  export function getClickDelayMs(animationSpeed: number) {
    // time taken to reach other end of switch, meaning that this doesn't account
    // for the time it takes to bounce back from the overshoot
    const usualAnimationDurationMs = 50;
    const presentAnimationDuration = usualAnimationDurationMs / animationSpeed;
    return Math.max(0, presentAnimationDuration - usualAnimationDurationMs);
  }
</script>

<script lang="ts">
  ...
  import { animationSpeed } from "./preferences";
  ...

  function playDelayedClick() {
    setTimeout(() => playClick(), getClickDelayMs($animationSpeed));
  }

  ...

  export function toggle() {
    if (!dragging) {
      ...
      playDelayedClick();
    }
    ...
  }
</script>
```

We only want to delay the click when the switch is not being dragged, because the distance from releasing the mouse drag to the end would be much shorter and not deserving of a delay.

We edit `src-svelte/src/lib/Switch.test.ts` as well to ensure that our logic is working as expected:

```ts
...
import Switch, { getClickDelayMs } from "./Switch.svelte";
...

describe("Switch delay", () => {
  test("is 0 under regular animation speeds", () => {
    expect(getClickDelayMs(1)).toBe(0);
  });

  test("increases a little when animation is twice as slow", () => {
    expect(getClickDelayMs(0.5)).toBe(50);
  });

  test("increases a lot when animation is 10x as slow", () => {
    expect(getClickDelayMs(0.1)).toBe(450);
  });
});
```

Upon trying this out, we realize that we do want to always delay the click, because when the user has dragged it closer, the switch also goes towards its final position slower. We merge the two functions into one with `playClick`:

```ts
  function playClick() {
    setTimeout(() => playSoundEffect("Switch"), getClickDelayMs($animationSpeed));
  }
```

and replaced the reference to `playDelayedClick` with `playClick`. This is an example of how different degrees of splitting abstractions make sense given different requirements.

Upon even further testing, we realize that we do want a delayed click in all instances *except* for the one where the user has manually dragged it past the edge. There, a delayed click would sound weird because it's already reached the position where the click is supposed to happen. That means that this function, which decides whether or not a click should be made during a drag:

```ts
  function playDragClick(offsetX: number) {
    if (dragging) {
      if (dragPositionOnLeft && offsetX >= onLeft) {
        playClick();
        dragPositionOnLeft = false;
      } else if (!dragPositionOnLeft && offsetX <= offLeft) {
        playClick();
        dragPositionOnLeft = true;
      }
    }
  }
```

must take in a new argument specifying whether the click is to be delayed. It would be awkward to have to specify if-else statements twice, so we should either set the function variable like `const clickFn = delayed ? playClick : playDelayedClick;`, or else have `playClick` take the argument and perform the logic there, leaving `playDragClick` to simply pass the argument on. `playClick` seems like a more natural place for this logic, and this way we won't have to define two functions:

```ts
  function playClick(delayed: boolean = true) {
    const delay = delayed ? getClickDelayMs($animationSpeed) : 0;
    setTimeout(() => playSoundEffect("Switch"), delay);
  }

  ...

  function playDragClick(..., delayed: boolean) {
    if (dragging) {
      if (...) {
        playClick(delayed);
        ...
      } else if (...) {
        playClick(delayed);
        ...
      }
    }
  }

  let toggleDragOptions: DragOptions = {
    ...
    onDrag: (data: DragEventData) => {
      ...
      playDragClick(data.offsetX, false);
    },
    onDragEnd: (data: DragEventData) => {
      ...
      playDragClick(..., true);
      ...
    },
  };

  ...
```

This is an example of yet a third way of abstracting these functions given different requirements. Given the non-triviality of these requirements, we should test them. We first modify the fast `jest-dom` tests at `src-svelte/src/lib/Switch.test.ts`, renaming `spy` to `tauriInvokeSpy` because there are now multiple spies:

```ts
...
import { soundOn, animationSpeed } from "$lib/preferences";
...

const tauriInvokeMock = vi.fn();
const recordSoundDelay = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
vi.stubGlobal("_testRecordSoundDelay", recordSoundDelay);

...

describe("Switch", () => {
  ...
  let tauriInvokeSpy: SpyInstance;
  let recordSoundDelaySpy: SpyInstance;

  ...

  beforeEach(() => {
    tauriInvokeSpy = vi.spyOn(window, "__TAURI_INVOKE__");
    recordSoundDelaySpy = vi.spyOn(window, "_testRecordSoundDelay");
    ...
  });

  ...

  test("does not usually delay click", async () => {
    render(Switch, {});
    expect(recordSoundDelaySpy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(recordSoundDelaySpy).toHaveBeenLastCalledWith(0);
  });

  test("delays click when animation speed is slow", async () => {
    animationSpeed.set(0.1);
    render(Switch, {});
    expect(recordSoundDelaySpy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(recordSoundDelaySpy).toHaveBeenLastCalledWith(450);
  });

  ...
});
```

We edit `src-svelte/src/lib/Switch.svelte` to record this value for testing:

```ts
  function playClick(delayed: boolean = true) {
    const delay = ...;
    ...
    if (window._testRecordSoundDelay !== undefined) {
      window._testRecordSoundDelay(delay);
    }
  }
```

Then we add tests to `src-svelte/src/lib/Switch.playwright.test.ts`, comparing via `JSON.stringify` because JS [doesn't support](https://stackoverflow.com/questions/7837456/how-to-compare-arrays-in-javascript) these kinds of array equality by default:

```ts
...
import { animationSpeed } from "./preferences";

...

describe("Switch drag test", () => {
  ...
  let soundDelays: number[];

  beforeAll(async () => {
    ...
    await context.exposeFunction(
      "_testRecordSoundDelay",
      (delay: number) => soundDelays.push(delay),
    );
    ...
  });

  ...

  beforeEach(() => {
    ...
    soundDelays = [];
  });

  test(
    "switches state when drag released more than halfway to end",
    async () => {
      ...
      const expectedDelays = [0];
      expect(JSON.stringify(soundDelays) === JSON.stringify(expectedDelays)).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "delays click sound when animation speed slow",
    async () => {
      animationSpeed.set(0.1);
      // ===== same as previous test =====
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      ...
      // ===== end similarity block =====
      const expectedDelays = [450];
      expect(JSON.stringify(soundDelays) === JSON.stringify(expectedDelays)).toBeTruthy();
    },
    { retry: 2 },
  );

  ...

  test(
    "clicks twice when dragged to end and back",
    async () => {
      animationSpeed.set(0.1);
      ...
      // does not play sound when released
      await page.mouse.up();
      expect(numSoundsPlayed === 2).toBeTruthy();

      // should have no delay for both sounds played despite slower animation
      const expectedDelays = [0, 0];
      expect(JSON.stringify(soundDelays) === JSON.stringify(expectedDelays)).toBeTruthy();
    },
    { retry: 2 },
  );
});
```

Our tests fail. We realize that `animationSpeed` is being set on the local test runner and has nothing to do with `animationSpeed` on the Playwright browser page. Instead, we do

```ts
  test(
    "delays click sound when animation speed slow",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle("slow-motion");
      // ===== same as previous test =====
      ...
    },
    { retry: 2 },
  );

  ...

  test(
    "clicks twice when dragged to end and back",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle("slow-motion");
      ...
    },
    { retry: 2 },
  );
```

The "delays click sound" test is still failing. Upon more debugging, we realize this is actually because it has gotten dragged so far that it triggers the immediate click. We fix this by dragging it more than halfway, but not all the way:

```ts
  test(
    "switches state when drag released more than halfway to end",
    async () => {
      ...

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      ...
    },
    { retry: 2 },
  );

  test(
    "delays click sound when animation speed slow",
    async () => {
      ...

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      ...
    },
    { retry: 2 },
  );
```

#### Standard duration

We eventually notice that we are using logic such as

```ts
const duration = $animationsOn ? 100 / $animationSpeed : 0;
```

a lot. To simplify this, we add this to `src-svelte/src/lib/preferences.ts`:

```ts
import { ..., derived } from "svelte/store";
...

export const standardDuration = derived(
  [animationsOn, animationSpeed],
  ([$animationsOn, $animationSpeed]) =>
    $animationsOn ? 100 / $animationSpeed : 0,
);
```

To do this properly, we should define a CSS variable based on this value as well, and propagate this change throughout the app. However, that is a more extensive refactor that will be left as a TODO instead.

## Sidebar mock navigation

We notice that the mocked sidebar navigation no longer works in Storybook due to the console error

```
TypeError: invoke() is not a function
```

We edit `src-svelte/src/lib/sound.ts` to make sure that things are ok even if the backend is not mocked:

```ts
export function playSoundEffect(sound: Sound) {
  ...
    try {
      playSound(sound, soundEffectVolume);
    } catch (e) {
      console.error(`Problem playing ${sound}: ${e}`);
    }
  ...
}
```

## Settings artifacts

We see [artifacts](./screenshots/222fe89.png) on the settings page on the `.deb` (but not the AppImage). We find that this turns out to be because of the `erd_scroll_detection_container` div that `svelte-watch-resize` inserts into the page, because deleting that div also removes the artifacts.

We try using `ResizeObserver` [directly](https://stackoverflow.com/a/39312522) instead. We edit `src-svelte/src/lib/Slider.svelte`:

```ts
  ...

  function handleResize() {
    // disable transition temporarily to avoid progress bar and thumb going out of sync
    transition = "";
    toggleDragOptions = calculatePosition(value);
    setTimeout(() => (transition = transitionAnimation), 100);
  }

  ...

  onMount(() => {
    toggleDragOptions = calculatePosition(value);
    new ResizeObserver(handleResize).observe(track);
  });
```

We find that `src-svelte/src/routes/settings/Settings.test.ts` fails as well due to the `ResizeObserver` not being defined in `jest-dom`, so we mock it as that appears to be the [standard solution](https://github.com/jsdom/jsdom/issues/3368#issuecomment-1147970817) to this problem.

Finally, we remove `svelte-watch-resize` from `src-svelte/package.json` and run `yarn` again to update the lockfile.

We now see the eslint errors

```
/root/zamm/src-svelte/src/routes/settings/Settings.test.ts
  25:17  error  Unexpected empty method 'observe'     @typescript-eslint/no-empty-function
  26:19  error  Unexpected empty method 'unobserve'   @typescript-eslint/no-empty-function
  27:20  error  Unexpected empty method 'disconnect'  @typescript-eslint/no-empty-function
```

It appears that they want us to [add explicit comments](https://eslint.org/docs/latest/rules/no-empty-function) saying that the empty functions are not an accident, so we do just that:

```ts
    global.ResizeObserver = class ResizeObserver {
      observe() {
        // do nothing
      }
      unobserve() {
        // do nothing
      }
      disconnect() {
        // do nothing
      }
    };
```
