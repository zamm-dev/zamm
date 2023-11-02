# Adding a slider component to the app

We realize that we need the same labeling construct as used for the switch, so we first refactor that out while making sure everything still works. Create `src-svelte/src/lib/label-id.ts`:

```ts
import { customAlphabet } from "nanoid/non-secure";

const nanoid = customAlphabet("1234567890", 6);

export default function getComponentId(componentType?: string) {
  const prefix = componentType || "component";
  return `${prefix}-${nanoid()}`;
}

```

and then edit `src-svelte/src/lib/Switch.svelte`:

```svelte
<script lang="ts">
  ...
  import getComponentId from "./label-id";
  ...

  const switchId = getComponentId("switch");
  ...
</script>
```

We now create `src-svelte/src/lib/Slider.svelte` with the basics copied over from `Switch.svelte`:

```svelte
<script lang="ts">
  import getComponentId from "./label-id";

  const switchId = getComponentId("switch");

  export let label: string | undefined = undefined;
  export let min: number = 0;
  export let max: number;
  export let step: number = 1;
  export let value: number = min;
</script>

<div class="container">
  {#if label}
    <label for={switchId}>{label}</label>
  {/if}
  <input type="range" id={switchId} min="{min}" max="{max}" value="{value}" step="{step}" />
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  label {
    flex: 1;
  }

  @media (min-width: 52rem) {
    label {
      white-space: nowrap;
    }
  }
</style>

```

and create `src-svelte/src/lib/Slider.stories.ts` with different label lengths and screen sizes:

```ts
import Slider from "./Slider.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Slider,
  title: "Reusable/Slider",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Slider,
  props: args,
});

export const TinyPhoneScreen: StoryObj = Template.bind({}) as any;
TinyPhoneScreen.args = {
  label: "Simulation",
  max: 10,
  value: 5,
};
TinyPhoneScreen.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const TinyPhoneScreenWithLongLabel: StoryObj = Template.bind({}) as any;
TinyPhoneScreenWithLongLabel.args = {
  label: "Extra Large Simulation",
  max: 10,
  value: 5,
};
TinyPhoneScreenWithLongLabel.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const LargePhoneScreenWithLongLabel: StoryObj = Template.bind({}) as any;
LargePhoneScreenWithLongLabel.args = {
  label: "Extra Large Simulation",
  max: 10,
  value: 5,
};
LargePhoneScreenWithLongLabel.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.args = {
  label: "Simulation",
  max: 10,
  value: 5,
};
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

```

One way to get closer to what we want is to use the same strategy as the settings page:

```css
  .container {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  label {
    white-space: nowrap;
  }

  @media (min-width: 30rem) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }
```

However, this forces the slider to go onto the next row even when the label is short enough to accomodate everything on one line. Flex box appears to still be the best option for this. We simply enable wrapping, and make the input also grow as the width expands (unlike with the switch):

```css
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  label {
    white-space: nowrap;
    flex: 1;
  }

  input {
    flex: 1;
  }
```

The `LargePhoneScreenWithLongLabel` story no longer exposes any meaningful differences in display, so we get rid of it.

We look at [this page](https://css-tricks.com/styling-cross-browser-compatible-range-inputs-css/) and [this page](https://css-tricks.com/sliding-nightmare-understanding-range-input/) (accessed from [this trove](https://codepen.io/collection/DgYaMj/) of examples) for caveats on browser styling, and [this example](https://codepen.io/chupai/pen/jOqarGN) and [this example](https://codepen.io/chriscoyier/pen/vYoMzN) for inspiration on what we want to do: create a groove that's similar to the groove for the switch, with a thumb that is bigger than the groove it sits in.

We start off by resetting the slider display for Firefox and Chrome (we avoid messing with Edge or Safari until we get our hands on them):

```css
  input {
    -webkit-appearance: none; /* Hides the slider so that custom slider can be made */
    width: 100%; /* Specific width is required for Firefox. */
    background: transparent; /* Otherwise white in Chrome */
  }

  input::-webkit-slider-thumb {
    -webkit-appearance: none;
  }

  input:focus {
    outline: none;
  }

  input::-moz-range-thumb {
    height: 36px;
    width: 16px;
    background-color: #ddd;
    box-shadow:
      0.1em 0.1em 0.15em rgba(0, 0, 0, 0.1),
      inset -0.1em -0.1em 0.15em rgba(0, 0, 0, 0.3),
      inset 0.1em 0.1em 0.15em rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
    cursor: ew-resize;
  }
```

Now we made dimensions consistent with the switch:

```css
  .container {
    --label-height: 1.5em;
    --thumb-height: calc(1.2 * var(--label-height));
  }
```

We add in the skew to be consistent with the switch:

```css
  .container {
    --skew: -20deg;
  }

  input {
    transform: skew(var(--skew));
  }
```

And start styling the switch thumb:

```css
  .container {
    --thumb-width: 0.75rem;
  }

  input::-moz-range-thumb {
    height: var(--thumb-height);
    width: var(--thumb-width);
    background-color: #ddd;
    box-shadow:
      0.1em 0.1em 0.15em rgba(0, 0, 0, 0.1),
      inset -0.1em -0.1em 0.15em rgba(0, 0, 0, 0.3),
      inset 0.1em 0.1em 0.15em rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
    cursor: ew-resize;
  }

  input::-webkit-slider-thumb {
    -webkit-appearance: none;
    margin-top: 0;

    height: var(--thumb-height);
    width: var(--thumb-width);
    background-color: #ddd;
    box-shadow:
      0.1em 0.1em 0.15em rgba(0, 0, 0, 0.1),
      inset -0.1em -0.1em 0.15em rgba(0, 0, 0, 0.3),
      inset 0.1em 0.1em 0.15em rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
    cursor: ew-resize;
  }
```

We picked the `ew-resize` cursor from [here](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor) to make it more obvious how it's meant to be moved.

We style the track similarly as well, although it deserves a lower height than the switch's groove:

```css
  .container {
    --track-height: calc(0.5 * var(--label-height));
  }

  input::-webkit-slider-thumb {
    margin-top: calc(-0.5 * (var(--thumb-height) - var(--track-height)));
  }

  input::-moz-range-track {
    width: 100%;
    height: var(--track-height);
    border-radius: var(--corner-roundness);
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
  }

  input::-webkit-slider-runnable-track {
    width: 100%;
    height: var(--track-height);
    border-radius: var(--corner-roundness);
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
  }
```

We now see why the `margin-top` was needed for `input::-webkit-slider-thumb` on Chrome.

Now we style the progress indicator. This is where we have to look at another example such as [this](https://codepen.io/thebabydino/pen/JoOomG) from the trove. We have

```svelte
<script lang="ts">
  ...
  let percentageValue: number;
  
  $: percentageValue = (value - min) / (max - min) * 100.0;
</script>

<div class="container">
  ...
  <input type="range" ... bind:value style="--val: {percentageValue}" />
</div>

<style>
  ...

  input::-moz-range-progress {
    background: linear-gradient(to left, #00F, #BBBBFF);
    height: var(--track-height);
    border-radius: var(--corner-roundness);
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
  }

  input::-webkit-slider-container {
		/* Chrome tries really hard to make this read-only */
		-webkit-user-modify: read-write !important;
    --unit: 1%;
		background: linear-gradient(to left, #00F, #BBBBFF) 0 / calc(var(--val) * var(--unit)) no-repeat;
    height: var(--track-height);
    border-radius: var(--corner-roundness);
    width: 50%;
	}
</style>
```

Note that in Firefox, the progress indicator is on top of the track, whereas in Chrome, the progress indicator is behind the track, as indicated by the shadows. We decide that we like this look after all, and replicate the effect by adding the same box-shadow to the Firefox styling. The slash in the background property for the Webkit version is explained [here](https://teamtreehouse.com/community/what-is-the-purpose-of-the-forward-slash-in-background-shorthand).

Finally, we make the step optional and set it to `"any"` if it isn't defined:

```svelte
<script lang="ts">
  ...
  export let step: number | undefined = undefined;
  ...
  let stepAttr: string = step ? step.toString() : "any";
  
  ...
</script>

<div class="container">
  ...
  <input ... step={stepAttr} ... />
</div>
```

Now we edit `src-svelte/src/routes/storybook.test.ts` to test this new component in all its variants:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["reusable", "slider"],
    variants: ["tiny-phone-screen", "tiny-phone-screen-with-long-label", "tablet"],
    screenshotEntireBody: true,
  },
  ...
];
```

When committing, we get the warning

```
/root/zamm/src-svelte/src/lib/Slider.svelte:56:5
Warn: Also define the standard property 'appearance' for compatibility (css)
    transform: skew(var(--skew));
    -webkit-appearance: none; /* Hides the slider so that custom slider can be made */
    width: 100%; /* Specific width is required for Firefox. */
```

We edit `src-svelte/src/lib/Slider.svelte` accordingly to add this in:

```svelte
  input {
    ...
    appearance: none;
    ...
  }
```

All the tests still pass.

## Mimicking the switch toggle

Upon further consideration, we would like to further mimic the switch toggle for consistency when the controls are placed together on the settings page. We therefore edit `src-svelte/src/lib/Slider.svelte` as such:

```css
  input {
    ...
    margin-right: calc(-0.5 * var(--toggle-height) * sin(var(--skew)));
    ...
  }
```

This gives it the same right-side offset as the switch. This becomes visible on the settings page when many controls are placed next to each other.

We also change the thumb to be exactly the same size as the switch toggle:

```css
  .container {
    --label-width: 3rem;
    --label-height: 1.5rem;
    --thumb-height: calc(1.2 * var(--label-height));
    --thumb-width: calc(1.05 * var(--label-width));
  }
```

and change the track to be exactly the same height as the switch groove:

```css
  .container {
    --track-height: calc(1 * var(--label-height));
  }
```

This makes us notice that in Firefox, there's an extra border around the toggle. We remove it as such:

```css
  input::-moz-range-thumb {
    border: none;
  }
```

The thickness of the thumb now also makes the `ew-resize` cursor inappropriate. We change it to `grab`:

```css
  input::-moz-range-thumb {
    cursor: grab;
  }

  input::-webkit-slider-thumb {
    cursor: grab;
  }
```

But this now introduces the problem that the cursor doesn't change when the thumb is being grabbed. From the answers to [this question](https://stackoverflow.com/questions/4082195/changing-the-cursor-on-a-hold-down), it appears that we must use JavaScript to change the cursor:

```svelte
<script lang="ts">
  ...
  let grabbing: boolean = false;

  const startGrabbing = () => {
    grabbing = true;
  };

  const stopGrabbing = () => {
    grabbing = false;
  };

  ...
</script>

<div class="container">
  ...
  <input
    ...
    class={grabbing ? "grabbing" : ""}
    ...
    on:mousedown={startGrabbing}
    on:mouseup={stopGrabbing}
  />
</div>

<style>
  ...

  input.grabbing::-moz-range-thumb {
    cursor: grabbing;
  }

  ...

  input.grabbing::-webkit-slider-thumb {
    cursor: grabbing;
  }

  ...
</style>
```

Now we have re-implemented the switch toggle in the slider.

## Debugging

We see that the switch toggle no longer releases the drag state during the mouse up event. After doing a quick Git bisect, we find out that this regression started with commit `6928575`, when we first implemented the slider. The interesting thing is that this commit didn't even touch the switch. And yet it is reproducible -- no bug on the previous commit `e5be997`, bug on `6928575`.

We check out commit `e5be997`, and from here check out one compiling file at a time from `6928575` until the bug appears:

- `src-svelte/src/lib/Slider.svelte` is fine
- All screenshot files in `src-svelte/screenshots/baseline/` are of course fine
- `src-svelte/src/lib/Slider.stories.ts` causes a failure

Interestingly, taking out all the component code from `src-svelte/src/lib/Slider.svelte`, leaving it a completely blank file, still causes a failure. Removing all exported stories fixes the problem. Leaving even a single one in, any one at all, restores the problem. It turns out even changing the title to `"Non-reusable/Switch"`, for the switch, works. Or, changing the title of the slider to `"Non-reusable/Slider"` also works. Or, both. It appears that this is only a Storybook cache issue, as the issue does not appear on the actual app itself.

## Completely custom slider

After seeing Webkit bungle the custom slider progress bar in the locally built app, we try to instead build our own completely custom slider. This should not only allow us greater control over the look and functionality of our sliders, but also allow us potentially greater cross-browser compatibility.

We copy most of our code from `Switch.svelte`, with the main differences being:

- What we consider `dragging` is always, as soon as the mouse is down, not only after the mouse starts moving
- The width of the slider is variable, meaning that we need to listen to the component resize event. We find that we can use the ResizeObserver API for this, as described [here](https://blog.sethcorker.com/question/how-do-you-use-the-resize-observer-api-in-svelte/), but we can instead use the ready-made library [svelte-watch-resize](https://www.npmjs.com/package/svelte-watch-resize).
- Because the slider width is variable, we can't initialize the progress div and therefore the drag position until the component is mounted and the dimensions and positions are known. However, `neodrag` is going to check only once whether a `position` argument is provided. So we have to set `position: { x: 0, y: 0 },` from the beginning so that Neodrag will know from the get-go that this is a controlled component.
- Because the progress end-point is variable, it makes more sense to just position the toggle at the same spot where the progress ends, rather than recreating the progress layer.
- For some reason, `onDragEnd` gets called before the `onClick` event here, unlike in `Switch.svelte`, so we have to wait a little bit to clear the `dragging` flag.

This is the resulting code:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { watchResize } from "svelte-watch-resize";
  import getComponentId from "./label-id";
  import {
    draggable,
    type DragOptions,
    type DragEventData,
  } from "@neodrag/svelte";

  const rootFontSize = parseFloat(
    getComputedStyle(document.documentElement).fontSize,
  );
  const sliderId = getComponentId("slider");
  const transitionAnimation = `transition: left 0.1s ease-out;`;
  const overshoot = 0.4 * rootFontSize; // how much overshoot to allow per-side

  export let label: string | undefined = undefined;
  export let min = 0;
  export let max: number;
  export let step: number | undefined = undefined;
  export let value: number = min;
  export let onUpdate: (newValue: number) => void = () => undefined;
  const range = max - min;
  let track: HTMLDivElement;
  let toggleBound: HTMLDivElement;
  let toggleLabel: HTMLDivElement;
  let leeway = 0;
  let left = 0;
  let transition = transitionAnimation;
  // needed because unlike Neodrag, we want the class to apply as soon as mousedown
  // happens
  let dragging = false;

  let toggleDragOptions: DragOptions = {
    axis: "x",
    bounds: () => toggleBound,
    inverseScale: 1,
    // need to set this for neodrag to know that this is a controlled drag
    position: { x: 0, y: 0 },
    render: (data: DragEventData) => {
      left = data.offsetX;
    },
    onDragStart: () => {
      transition = "";
      dragging = true;
    },
    onDragEnd: (data: DragEventData) => {
      transition = transitionAnimation;
      const newValue = calculateValue(data.offsetX);
      updateValue(newValue);
      // for some reason, unlike in Switch.svelte, onClick runs after onDragEnd
      // so we need to wait a bit to stop the dragging
      setTimeout(() => (dragging = false), 100);
    },
  };

  function updateValue(newValue: number) {
    const minStep = step || 0.01;
    newValue = Math.round(newValue / minStep) * minStep;
    if (newValue < min) {
      newValue = min;
    } else if (newValue > max) {
      newValue = max;
    }

    if (newValue !== value) {
      value = newValue;
      try {
        onUpdate(newValue);
      } catch (e) {
        console.error(`Error in callback: ${e}`);
      }
    }

    // necessary in case we overshoot regular bounds
    toggleDragOptions = calculatePosition(value);
  }

  function toPercentage(value: number) {
    return (value - min) / range;
  }

  function calculateValue(position: number) {
    const percentageValue = position / leeway;
    return min + range * percentageValue;
  }

  function calculatePosition(value: number) {
    leeway = track.clientWidth - toggleLabel.clientWidth;
    const x = leeway * toPercentage(value);
    return {
      ...toggleDragOptions,
      position: { x, y: 0 },
    };
  }

  function handleResize() {
    toggleDragOptions = calculatePosition(value);
  }

  function onClick(e: MouseEvent) {
    if (dragging) {
      return;
    }

    // toggle midpoint should go where cursor is, at least as much as possible
    // what calculateValue expects though is the left edge of the toggle
    const toggleTargetLeft =
      e.clientX - toggleLabel.getBoundingClientRect().width / 2;
    const offsetX = toggleTargetLeft - track.getBoundingClientRect().left;
    const newValue = calculateValue(offsetX);
    updateValue(newValue);
  }

  function onKeyPress(e: KeyboardEvent) {
    const finalStepSize = step || range / 10.0;
    if (e.key === "ArrowLeft" || e.key === "ArrowDown") {
      updateValue(value - finalStepSize);
    } else if (e.key === "ArrowRight" || e.key === "ArrowUp") {
      updateValue(value + finalStepSize);
    }
  }

  onMount(() => {
    toggleDragOptions = calculatePosition(value);
  });

  $: left = toggleDragOptions.position?.x ?? 0;
</script>

<div class="container">
  {#if label}
    <div class="label" id={sliderId}>{label}</div>
  {/if}
  <div
    class="slider"
    role="slider"
    tabindex="0"
    aria-valuemin={min}
    aria-valuemax={max}
    aria-valuenow={value}
    aria-labelledby={sliderId}
    style="--overshoot: {overshoot}px;"
    on:click={onClick}
    on:keydown={onKeyPress}
  >
    <div
      class="groove-layer groove"
      bind:this={track}
      use:watchResize={handleResize}
    >
      <div class="groove-layer shadow"></div>
      <div
        class="groove-contents progress"
        style="--leeway: {leeway}px; --left: {left}px; {transition}"
      ></div>
    </div>
    <div class="groove-layer bounds" bind:this={toggleBound}></div>
    <div
      class="toggle-label"
      use:draggable={toggleDragOptions}
      style="--leeway: {leeway}px; --left: {left}px; {transition}"
      bind:this={toggleLabel}
    >
      <div class="toggle" class:grabbing={dragging}></div>
    </div>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
    width: 100%;

    --skew: -20deg;
    --label-width: 3rem;
    --label-height: 1.5rem;
    --toggle-height: calc(1.2 * var(--label-height));
    --toggle-width: calc(1.05 * var(--label-width));
    --track-height: calc(1 * var(--label-height));
  }

  .label {
    white-space: nowrap;
    flex: 1;
  }

  .slider {
    --groove-contents-layer: 1;
    --groove-layer: 2;
    --toggle-layer: 3;
    flex: 1;
    height: var(--track-height);
    min-width: 7rem;
    cursor: pointer;
    transform: skew(var(--skew));
    margin-right: calc(-0.5 * var(--toggle-height) * sin(var(--skew)));
    padding: 0;
  }

  .groove-layer {
    width: 100%;
    height: var(--label-height);
    border-radius: var(--corner-roundness);
    z-index: var(--groove-layer);
    position: relative;
  }

  .groove-layer.groove {
    overflow: hidden;
  }

  .groove-layer.shadow {
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
  }

  .groove-layer.bounds {
    width: calc(100% + 2 * var(--overshoot));
    margin-left: calc(-1 * var(--overshoot));
    background: transparent;
    position: float;
    top: calc(-1 * var(--track-height));
  }

  .groove-contents.progress {
    --total-width: calc(var(--leeway) + var(--overshoot));
    z-index: var(--groove-contents-layer);
    position: absolute;
    top: 0;
    left: calc(var(--left) - var(--total-width));
    width: var(--total-width);
    height: var(--track-height);
    background: linear-gradient(to left, #00f, #bbbbff);
  }

  .toggle-label {
    width: var(--label-width);
    height: var(--label-height);
    display: flex;
    flex-direction: row;
    align-items: center;
    z-index: var(--toggle-layer);
    position: absolute;
    left: var(--left);
    top: 0;
  }

  .toggle {
    position: absolute;
    width: var(--toggle-width);
    height: var(--toggle-height);
    background-color: #ddd;
    box-shadow:
      0.1em 0.1em 0.15em rgba(0, 0, 0, 0.1),
      inset -0.1em -0.1em 0.15em rgba(0, 0, 0, 0.3),
      inset 0.1em 0.1em 0.15em rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
  }

  .toggle:hover {
    cursor: grab;
  }

  :global(.toggle.grabbing),
  .toggle.grabbing:hover {
    cursor: grabbing;
  }
</style>

```

and the resulting tests at `src-svelte/src/lib/Slider.playwright.test.ts`:

```ts
import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Frame,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import type { ChildProcess } from "child_process";
import { ensureStorybookRunning, killStorybook } from "$lib/test-helpers";

const DEBUG_LOGGING = false;

describe("Switch drag test", () => {
  let storybookProcess: ChildProcess | undefined;
  let page: Page;
  let frame: Frame;
  let browser: Browser;
  let context: BrowserContext;
  let numSoundsPlayed: number;

  beforeAll(async () => {
    storybookProcess = await ensureStorybookRunning();

    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    await context.exposeFunction(
      "_testRecordSoundPlayed",
      () => numSoundsPlayed++,
    );
    page = await context.newPage();

    if (DEBUG_LOGGING) {
      page.on("console", (msg) => {
        console.log(msg);
      });
    }
  });

  afterAll(async () => {
    await browser.close();
    await killStorybook(storybookProcess);
  });

  beforeEach(() => {
    numSoundsPlayed = 0;
  });

  const getSliderAndThumb = async (variant = "tiny-phone-screen-with-long-label") => {
    await page.goto(
      `http://localhost:6006/?path=/story/reusable-slider--${variant}`,
    );

    const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
    if (!maybeFrame) {
      throw new Error("Could not find Storybook iframe");
    }
    frame = maybeFrame;
    const slider = frame.getByRole("slider");
    const thumb = slider.locator(".toggle");
    const sliderBounds = await slider.boundingBox();
    if (!sliderBounds) {
      throw new Error("Could not get slider bounding box");
    }

    return { slider, thumb, sliderBounds };
  };

  test(
    "goes to maximum even when thumb released past end",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expect(slider).toHaveAttribute("aria-valuenow", "5");

      await thumb.dragTo(slider, {
        targetPosition: { x: sliderBounds.width, y: sliderBounds.height / 2 },
      });
      await expect(slider).toHaveAttribute("aria-valuenow", "10");
    },
    { retry: 2 },
  );

  test(
    "goes to minimum even when thumb released past end",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expect(slider).toHaveAttribute("aria-valuenow", "5");

      await thumb.dragTo(slider, {
        targetPosition: { x: 0, y: sliderBounds.height / 2 },
      });
      await expect(slider).toHaveAttribute("aria-valuenow", "0");
    },
    { retry: 2 },
  );

  test(
    "goes to intermediate value when thumb released in-between",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expect(slider).toHaveAttribute("aria-valuenow", "5");

      await thumb.dragTo(slider, {
        targetPosition: { x: sliderBounds.width * 0.75, y: sliderBounds.height / 2 },
      });
      const valueString = await slider.getAttribute("aria-valuenow") as string;
      const value = parseFloat(valueString);
      expect(value).toBeGreaterThan(5);
      expect(value).toBeLessThan(10);
    },
    { retry: 2 },
  );

  test(
    "allows for arrow key use",
    async () => {
      const { slider } = await getSliderAndThumb();
      await expect(slider).toHaveAttribute("aria-valuenow", "5");

      await slider.press("ArrowRight");
      const valueString = await slider.getAttribute("aria-valuenow") as string;
      const value = parseFloat(valueString);
      expect(value === 6).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "allows for mouse click",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expect(slider).toHaveAttribute("aria-valuenow", "5");

      await page.mouse.click(sliderBounds.x + sliderBounds.width * 0.25, sliderBounds.y + sliderBounds.height / 2);
      const valueString = await slider.getAttribute("aria-valuenow") as string;
      const value = parseFloat(valueString);
      expect(value).toBeLessThan(5);
      expect(value).toBeGreaterThan(0);
    },
    { retry: 2 },
  );
});

```

Because we can no longer mock the input target event directly, we instead mock a key press event in `src-svelte/src/routes/settings/Settings.test.ts`:

```ts
  test("can persist changes to volume slider", async () => {
    ...
    volumeSlider.focus();
    const user = userEvent.setup();
    await user.keyboard("[ArrowLeft]");
    expect(get(volume)).toBe(0.8);
    ...
  });
```

But because each keypress will only move the values by 10%, we will change `src-tauri/api/sample-calls/set_preferences-volume-partial.yaml` to match our new API call, which means we will change `src-tauri/api/sample-settings/volume-override/preferences.toml` to match the result of the previous call, and `src-tauri/api/sample-calls/get_preferences-volume-override.yaml` to match the result of reading that preference file, and `src-svelte/src/routes/AppLayout.test.ts` to match the result of getting that preference API call.

Tauri tests now fail with

```
Test will use preference file at /tmp/zamm/tests/set_preferences-volume-partial/preferences.toml
thread 'commands::preferences::write::tests::test_set_preferences_volume_partial' panicked at 'assertion failed: `(left == right)`
  left: `"volume = 0.800000011920929"`,
 right: `"volume = 0.8"`', src/commands/preferences/write.rs:149:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

We see from here that this may be fixed if we edit `src-tauri/src/commands/preferences/models.rs` to use `f64`:

```rust
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    ...
    volume: Option<f64>,
}
```

## Hardware acceleration

As with the [switch](./switch.md), we use CSS transforms rather than positioning so as to enable hardware acceleration on the slider animations. We edit `src-svelte/src/lib/Slider.svelte`:

```svelte
<script lang="ts">
  ...
    const transitionAnimation =
    `transition: transform ` +
    `calc(0.1s / var(--base-animation-speed)) ` +
    `ease-out;`;
  ...
</script>

...

<style>
  ...

  groove-contents.progress {
    ...
    left: calc(-1 * var(--total-width));
    transform: translateX(var(--left));
    ...
  }

  .toggle-label {
    ...
    left: 0;
    ...
    transform: translateX(var(--left));
  }

  ...
</style>
```

Note that we can also go [further](https://blog.teamtreehouse.com/increase-your-sites-performance-with-hardware-accelerated-css) with hardware acceleration by asking for GPU usage, but as that article notes, that comes with a lot of caveats.
