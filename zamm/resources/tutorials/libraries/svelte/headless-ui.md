# Using Headless UI for Svelte

If we want standard components with no styling, we can use [Headless UI](https://github.com/rgossiaux/svelte-headlessui), an unofficial port of the Headless UI library for Svelte. We can also look at the docs [here](https://svelte-headlessui.goss.io/docs/2.0).

First install it:

```bash
$ yarn add -D @rgossiaux/svelte-headlessui
```

Then follow the instructions [here](https://svelte-headlessui.goss.io/docs/2.0/switch) to make a custom checkbox, and follow the instructions [here](https://svelte-headlessui.goss.io/docs/2.0/general-concepts#component-styling) to style it. We first create `src-svelte/src/lib/Switch.svelte` as such:

```svelte
<script lang="ts">
  import { Switch, SwitchLabel, SwitchGroup } from "@rgossiaux/svelte-headlessui";

  export let label: string | undefined = undefined;
  export let toggledOn = false;
</script>

<div>
  <SwitchGroup class="switch-group">
    {#if label}
      <SwitchLabel>
        <div class="label">
          {label}
        </div>
      </SwitchLabel>
    {/if}
    <Switch
      bind:checked={toggledOn}
      class={toggledOn ? "button on" : "button off"}
    >
      <span class={toggledOn ? "toggle on" : "toggle off"}>ON / OFF</span>
    </Switch>
  </SwitchGroup>
</div>

<style>
  * :global(.switch-group) {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  * :global(.button) {
    border-color: pink;
  }

  * :global(.toggle.on) {
    color: green;
  }

  * :global(.toggle.off) {
    color: red;
  }
</style>

```

with the corresponding Storybook stories at `src-svelte/src/lib/Switch.stories.ts`:

```ts
import Switch from "./Switch.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Switch,
  title: "Reusable/Switch",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Switch,
  props: args,
});

export const On: StoryObj = Template.bind({}) as any;
On.args = {
  label: "Simulation",
  toggledOn: true,
};

export const Off: StoryObj = Template.bind({}) as any;
Off.args = {
  label: "Simulation",
  toggledOn: false,
};

```

Right now, all we want to do is to confirm that we are able to create a working switch and control the styling of both the switch button and its contents. Note that the SvelteKit HMR may not refresh the global selectors properly, so if your global selectors are not doing anything, you may want to try reloading the entire page.

Now we add a basic interaction test to `src-svelte/src/lib/Switch.test.ts`:

```ts
import { expect, test, vi } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Switch from "./Switch.svelte";

describe("Switch", () => {
  test("can be toggled on", async () => {
    render(Switch, {});

    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveClass("button off");
    await act(() => userEvent.click(onOffSwitch));
    expect(onOffSwitch).toHaveClass("button on");
  });

  test("can be toggled off", async () => {
    render(Switch, { toggledOn: true });

    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveClass("button on");
    await act(() => userEvent.click(onOffSwitch));
    expect(onOffSwitch).toHaveClass("button off");
  });
});

```

and we add this to `src-svelte/src/routes/storybook.test.ts` to the components we want to visually test:

```ts
...

const components: ComponentTestConfig[] = [
  {
    path: ["reusable", "switch"],
    variants: ["on", "off"],
  },
  ...
];
```

## Styling the switch

We then look at the CSS toggles [here](https://alvarotrigo.com/blog/toggle-switch-css/) and [here](https://freefrontend.com/css-toggle-switches/) for inspiration. We want:

- the springiness of [this toggle](https://codepen.io/team/keyframers/pen/JVdxzz)
- the engraved and animated nature of the first 3D toggle [here](https://codepen.io/rgg/pen/waEYye)
- the slide and the skew of the middle toggle [here](https://codepen.io/alvarotrigo/pen/RwjEZeJ)

Some toggles are absolute works of art, and deserve to be called out despite not being used in this project yet:

- [This](https://codepen.io/alvarotrigo/pen/PoOXJpM) day and night toggle by Alvaro
- [This](https://codepen.io/milanraring/pen/KKwRBQp) power switch by Milan Raring

We start off with emulating the skew effect. We get rid of all other CSS and do this:

```css
  * :global(.button) {
    --skew: -20deg;
    overflow: hidden;
    transform: skew(var(--skew));
  }

  * :global(.toggle) {
    display: inline-block;
    transform: skew(calc(-1 * var(--skew)));
  }
```

The toggle is skewed in the opposite direction to ensure that the text looks okay.

We strip away all the regular button styling:

```css
  * :global(.button) {
    padding: 0;
    border: none;
    background: transparent;
  }
```

but we ensure that the divs we're going to render are obviously clickable:

```css
  * :global(.button) {
    cursor: pointer;
  }
```

We create the layout for the bottom layer, which sits below the groove, and consists of three equal-sized segments:

```svelte
<div class={toggledOn ? "groove-contents on" : "groove-contents off"}>
  <div class="toggle-label on"><span>On</span></div>
  <div class="toggle-label"></div>
  <div class="toggle-label off"><span>Off</span></div>
</div>

<style>
  * :global(.button) {
    --label-width: 3rem;
    --label-height: 1.5rem;
  }

  * :global(.groove-contents) {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  * :global(.toggle-label) {
    width: var(--label-width);
    height: var(--label-height);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  * :global(.toggle-label.on) {
    background: green;
  }

  * :global(.toggle-label.off) {
    background: red;
  }
</style>
```

The middle compartment is where the toggle button will go, but because it sits above the groove, we leave it empty here for now. Note that we can also define conditional classes in this alternative way with Svelte:

```svelte
        <div class="groove-contents" class:on={toggledOn} class:off={!toggledOn}>
          <div class="toggle-label on"><span>On</span></div>
          <div class="toggle-label"></div>
          <div class="toggle-label off"><span>Off</span></div>
        </div>
```

Taking inspiration from [this answer](https://stackoverflow.com/a/39573216) and [this question](https://stackoverflow.com/questions/35247529/css-embossed-inset-text), we style the text inside to look like it's inset into the label:

```css
  * :global(.toggle-label span) {
    --shadow-offset: 0.05rem;
    --shadow-intensity: 0.3;
    transform: skew(calc(-1 * var(--skew)));
    color: white;
    font-size: 0.9rem;
    font-family: Nasalization, sans-serif;
    text-transform: uppercase;
    text-shadow:
      calc(-1 * var(--shadow-offset)) calc(-1 * var(--shadow-offset)) 0 rgba(0, 0, 0, var(--shadow-intensity)),
      var(--shadow-offset) var(--shadow-offset) 0 rgba(255, 255, 255, var(--shadow-intensity));
  }
```

We now place this inside of the groove:

```svelte
      <div class="groove-layer groove">
        <div class="groove-layer shadow"></div>
        <div class="groove-contents" class:on={toggledOn} class:off={!toggledOn}>
          ...
        </div>
      </div>

<style>
  * :global(.button) {
    --groove-contents-layer: 1;
    --groove-layer: 2;
  }

  * :global(.groove-layer) {
    width: calc(2 * var(--label-width));
    height: var(--label-height);
    border-radius: var(--corner-roundness);
    z-index: var(--groove-layer);
    position: relative;
  }

  * :global(.groove-layer.groove) {
    overflow: hidden;
  }

  * :global(.groove-layer.shadow) {
    box-shadow: inset 0.05rem 0.05rem 0.3rem rgba(0, 0, 0, 0.4);
  }

  * :global(.groove-contents) {
    z-index: var(--groove-contents-layer);
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
```

This ensures that the groove covers the contents of the layer below it, and is only twice the regular label length so that it always only shows one single label, plus the toggle. We use the shadow to make this inset effect obvious to the user.

Next, we create the toggle button, which is a button that sits on top of the groove:

```svelte
      <div class="groove-layer groove">
        ...
      </div>
      <div class="groove-contents toggle-layer" class:on={toggledOn} class:off={!toggledOn}>
        <div class="toggle-label"></div>
        <div class="toggle-label"><div class="toggle"></div></div>
        <div class="toggle-label"></div>
      </div>

<style>
  * :global(.button) {
    --toggle-layer: 3;
  }

  * :global(.groove-contents.toggle-layer) {
    z-index: var(--toggle-layer);
  }

  * :global(.toggle) {
    position: absolute;
    width: calc(1.05 * var(--label-width));
    height: calc(1.2 * var(--label-height));
    background-color: #ddd;
    box-shadow:
      0.1rem 0.1rem 0.15rem rgba(0, 0, 0, 0.1),
      inset -0.1rem -0.1rem 0.15rem rgba(0, 0, 0, 0.3),
      inset 0.1rem 0.1rem 0.15rem rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
  }
</style>
```

We make it slightly bigger than the usual groove label so that it functions as a slider that sits on top of the groove. We also add box shadows to give the appearance that it's a physical slider, and we add a border radius to make it look consistent with the rest of the rounded switch. We put this inside another `groove-contents` div so that it can be positioned in exactly the same way as the groove contents layer. We can't place it in the original layer, or else its edges will also be clipped by the groove.

Finally, we need to also show the off state for the toggle:

```css
  * :global(.groove-contents) {
    transition: left 0.05s ease-out;
  }

  * :global(.groove-contents.off) {
    left: calc(-1 * var(--label-width));
  }
```

Finally, we make the container div for all this an `inline-block`, so that the div is sized to just the size of the child elements, as mentioned [here](https://stackoverflow.com/questions/15102480/how-to-make-a-parent-div-auto-size-to-the-width-of-its-children-divs).

```svelte
<div class="container">
  <SwitchGroup class="switch-group">
    ...
  </SwitchGroup>
</div>

<style>
  .container {
    display: inline-block;
  }

  ...
</style>
```

We edit our tests. For the DOM tests at `src-svelte/src/lib/Switch.test.ts`, we now realize that the switch's CSS "on" and "off" classes are nested inside the switch because such differentiation is not needed at the top-level. Continuing to tests for these would tightly couple our tests to the HTML implementation. Instead, we make use of the accessibility attributes, which also makes it easy for us to test because they're meant to be machine-readable:

```ts
  test("can be toggled on", async () => {
    render(Switch, {});

    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
    await act(() => userEvent.click(onOffSwitch));
    expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
  });
```

For the screenshot tests at `src-svelte/src/lib/Switch.stories.ts`, we once again realize that the CSS styling is going to be cropped if we take a screenshot of just the switch. Instead, we take a screenshot of the entire body, and specify a smaller screen size to avoid having too much whitespace:

```ts
export const On: StoryObj = Template.bind({}) as any;
On.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};
```

Now we add in the springiness in the two examples we noted. It turns out that springiness in the first example is implemented by having two nested transitions that run in opposite directions:

```html
    <div class="checkmark">
      <svg viewBox="0 0 100 100">
        ...
      </svg>
    </div>
```

```scss
    ~ .checkmark {
      transform: translate(1.9em);

      svg {
        ...
        transform: translate(-.4em);
        ...
      }
    }
```

Springiness in the second example is achieved via the cubic bezier function:

```scss
            @include transition-timing-function(cubic-bezier(0.52,-0.41, 0.55, 1.46));
```

To understand what is happening, we can go to [this website](https://cubic-bezier.com/#.52,-0.41,.55,1.46). The first vector results in undershoot, the second in overshoot before the attribute treads back to its final value. We play around with these values in our own switch before arriving at:

```css
    transition: left 0.1s;
    transition-timing-function: cubic-bezier(0, 0, 0, 1.3);
```

The undershoot complicates the animation too much, so we remove it. The third number determines how much time is spent on the initial slide versus how much time is spent correcting the overshoot. We want the switch to feel fast and snappy; by setting the third number to zero, we see from the cubic bezier graph that the progress is just around 100% at the 50% time mark. This means that half the time is spent getting from beginning to end, and then the other half of the time is spent overshooting and correcting the overshoot. The first half is what makes the switch feels snappy; the second half is just a flourish. As such, to make this just as snappy as our previous value of 0.05s, we double the transition timing to 0.1s so that we spend 0.05s getting from beginning to end.

All our tests should pass unchanged because we have only modified the animation, not the start and end states that we are testing for.
