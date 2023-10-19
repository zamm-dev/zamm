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
