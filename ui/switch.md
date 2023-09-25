# Adding a Switch component to the app

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

### Draggable switch

We'll want the user to be able to drag the switch to set it. This is especially true if they're going to be eventually using this on mobile.

[This pen](https://codepen.io/stevenfabre/pen/OJgoOp) shows exactly what we want: the switch can be dragged along the x-axis, and will snap to the closest end when the mouse is released. However, it is using jQuery, which is not generally recommended for use with frameworks such as Svelte. Let's see if we can find alternatives that don't require installing a different DOM manipulator.

There is also [Shopify's draggable](https://shopify.github.io/draggable/). This is, however, not tailored to Svelte.

There is [this Svelte example](https://svelte.dev/repl/7d674cc78a3a44beb2c5a9381c7eb1a9?version=4.2.0). However, it is very low-level. Let's see if we can find any proper libraries.

There is [Svelte Dnd](https://github.com/isaacHagoel/svelte-dnd-action), similar to the DnD library for React. However, the API doesn't appear to allow us to easily specify which axis to constrain ourselves to.

We come across [Neodrag](https://www.neodrag.dev/docs/svelte), which according to [this list](https://www.reddit.com/r/sveltejs/comments/1181gte/drag_drop_in_svelte_hunting_for_libs/) is by far the smallest library, and also offers axis snapping. Let's try it out.

Upon trying it out, we find that it is not quite flexible enough for our needs, so we [fork it](/zamm/resources/tutorials/setup/repo/yarn-fork.md). We create [this pull request](https://github.com/PuruVJ/neodrag/pull/135) with the changes we need:

- We need to use the groove layer as the bounds, but use the toggle as the element to be dragged within those bounds. Of the documented options, there is only the choice to specify the parent element as the bounds. It turns out there is an undocumented option to specify any element, but due to how such references [will be undefined](https://learn.svelte.dev/tutorial/bind-this) until the component mounts, we need to either update the configuration in an `onMount` (we see from Neodrag's code that it does support [updates](https://svelte.dev/docs/svelte-action) in the `draggable` action), or else add an option to return a function, which is what we go for in the fork.
- We are using the `left` property to simultaneously position both the toggle and the groove contents, whereas Neodrag always uses the `transform` property to position only one element at a time. We add a custom `render` function to get full control over how we display the transformation during the drag.
- We notice that the bounds change between when the switch is off versus when the switch is on. It turns out this is because of Neodrag's `inverseScale` calculations, where it adjusts for scaling effects based on the rendered screen size and the element's logical size, as explained [here](https://stackoverflow.com/a/45419412). However, because we are manually calculating the rest positions for the `left` property, we would have to either get that scaling information passed in as part of the drag event data so that we can undo the scaling, or else we would have to substitute our own scaling of 1. We go for the latter option as it is simpler on our calculations.

As for our calculations of the resting positions for the toggle handle, since we're using `rem` in the CSS, we might as well use it in JavaScript as well. As such, we get the current rem value as mentioned [here](https://stackoverflow.com/a/42769683).

After all these changes to our own fork of the Neodrag library, we are still left with the switch always being toggled, no matter how much or how little the user drags it. It appears that there is no documented way to override this functionality in the Headless UI package. We look at [the code](https://github.com/rgossiaux/svelte-headlessui/blob/v2.0.0/src/lib/components/switch/Switch.svelte) for the switch component and we see that it's simple enough that there is actually no need to fork it -- we can just implement all the functionality ourselves.

The only problem is that we want the label to be explicitly tied to the button. We could put the button inside the label, but as noted [here](https://www.w3.org/WAI/tutorials/forms/labels/#associating-labels-implicitly):

> Generally, explicit labels are better supported by assistive technology.

So, we will label it explicitly. But this requires referring to the ID of the label. Since we may have multiple switches on the same page, we can't hardcode this. We see that there are [already](https://github.com/sveltejs/svelte/issues/6932) [issues](https://github.com/sveltejs/svelte/issues/7517) opened about supporting this natively in Svelte. But since Svelte does not want to support it natively, we'll have to try other methods. We could try to use a package such as `locally-unique-id-generator`, which guarantees uniqueness at the cost of potentially encountering hydration issues across client/server contexts, or use a package such as `nanoid`, which makes uniqueness super probable.

For [cross-compilation purposes](/zamm/resources/tutorials/setup/dev/cross.md), we may want to use nanoid version 3. Version 5 requires NodeJS 18, which in turn requires GLIBC 2.28 when you try to install it:

```
2.618 node: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.28' not found (required by node)
```

That in turn requires Ubuntu 20.04. We'll try our best to support as old an installation of Ubuntu as practical. We could try building our frontend specifically in a Docker image with a newer version of Ubuntu, as the generated artifacts are all we need and should be compatible with any modern browser. However, this will require refactoring the Docker build image, which is too much work to do for one single package.

We'll want to test that the unique IDs actually work to differentiate multiple instances of the switches in the same document:

```ts
  test("can have multiple unique labels", async () => {
    render(Switch, { label: "One" });
    render(Switch, { label: "Two" });

    const switchOne = screen.getByLabelText("One");
    expect(switchOne).toHaveAttribute("aria-checked", "false");
    const switchTwo = screen.getByLabelText("Two");
    expect(switchTwo).toHaveAttribute("aria-checked", "false");

    await act(() => userEvent.click(switchOne));
    expect(switchOne).toHaveAttribute("aria-checked", "true");
    expect(switchTwo).toHaveAttribute("aria-checked", "false");

    await act(() => userEvent.click(switchTwo));
    expect(switchOne).toHaveAttribute("aria-checked", "true");
    expect(switchTwo).toHaveAttribute("aria-checked", "true");
  });
```

The final implementation of `src-svelte/src/lib/Switch.svelte`:

```svelte
<script lang="ts">
  import { customAlphabet } from "nanoid/non-secure";
  import {
    draggable,
    type DragOptions,
    type DragEventData,
  } from "@neodrag/svelte";

  const rem = parseFloat(getComputedStyle(document.documentElement).fontSize);
  const labelWidth = 3 * rem;
  const offLeft = -labelWidth;
  const onLeft = 0;
  const transitionAnimation = `
    transition: left 0.1s;
    transition-timing-function: cubic-bezier(0, 0, 0, 1.3);
  `;
  const nanoid = customAlphabet("1234567890", 6);
  const switchId = `switch-${nanoid()}`;

  export let label: string | undefined = undefined;
  export let toggledOn = false;
  let toggleBound: HTMLElement;
  let left = 0;
  let transition = transitionAnimation;
  let startingOffset = 0;
  let dragging = false;

  let toggleDragOptions: DragOptions = {
    axis: "x",
    bounds: () => toggleBound,
    inverseScale: 1,
    render: (data: DragEventData) => {
      left = data.offsetX;
    },
    onDragStart: (data: DragEventData) => {
      transition = "";
      dragging = false;
      startingOffset = data.offsetX;
    },
    onDrag: (data: DragEventData) => {
      // if we ever start dragging, then the toggle state will depend on the final
      // resting position, even if it gets returned back to the very beginning.
      // On the other hand, if we never drag at all, then thet toggle state will simply
      // flip because it's just a click.
      //
      // offsetX starts based on the current position of the switch, not at 0, so we
      // have to keep track of the starting offset to determine if we've actually
      // moved
      dragging = dragging || data.offsetX !== startingOffset;
    },
    onDragEnd: (data: DragEventData) => {
      transition = transitionAnimation;
      if (dragging) {
        toggledOn = data.offsetX > offLeft / 2;
      }
      // even if toggle state didn't change, reset back to resting position
      toggleDragOptions = updatePosition(toggledOn);
    },
  };

  function updatePosition(toggledOn: boolean) {
    return {
      ...toggleDragOptions,
      position: toggledOn ? { x: onLeft, y: 0 } : { x: offLeft, y: 0 },
    };
  }

  function toggle() {
    if (!dragging) {
      toggledOn = !toggledOn;
    }
    dragging = false; // subsequent clicks should register
  }

  $: toggleDragOptions = updatePosition(toggledOn);
  $: left = toggleDragOptions.position?.x ?? 0;
</script>

<div class="container">
  {#if label}
    <label for={switchId}>{label}</label>
  {/if}
  <button
    type="button"
    role="switch"
    tabIndex="0"
    aria-checked={toggledOn}
    id={switchId}
    on:click={toggle}
  >
    <div class="groove-layer groove">
      <div class="groove-layer shadow"></div>
      <div
        class="groove-contents"
        class:on={toggledOn}
        class:off={!toggledOn}
        style="--left: {left}px; {transition}"
      >
        <div class="toggle-label on"><span>On</span></div>
        <div class="toggle-label"></div>
        <div class="toggle-label off"><span>Off</span></div>
      </div>
    </div>
    <div class="groove-layer bounds" bind:this={toggleBound}></div>
    <div
      class="groove-contents toggle-layer"
      class:on={toggledOn}
      class:off={!toggledOn}
      style="--left: {left}px; {transition}"
    >
      <div class="toggle-label"></div>
      <div class="toggle-label" use:draggable={toggleDragOptions}>
        <div class="toggle"></div>
      </div>
      <div class="toggle-label"></div>
    </div>
  </button>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  button {
    --skew: -20deg;
    --label-width: 3rem;
    --label-height: 1.5rem;
    --groove-contents-layer: 1;
    --groove-layer: 2;
    --toggle-layer: 3;
    cursor: pointer;
    transform: skew(var(--skew));
    padding: 0;
    border: none;
    background: transparent;
  }

  .groove-layer {
    --groove-width: calc(2 * var(--label-width));
    width: var(--groove-width);
    height: var(--label-height);
    border-radius: var(--corner-roundness);
    z-index: var(--groove-layer);
    position: relative;
  }

  .groove-layer.groove {
    overflow: hidden;
  }

  .groove-layer.shadow {
    box-shadow: inset 0.05rem 0.05rem 0.3rem rgba(0, 0, 0, 0.4);
  }

  .groove-layer.bounds {
    /* How much overshoot to allow */
    --overshoot: 0.2;
    /* unskew bounds to make reasoning easier */
    transform: skew(calc(-1 * var(--skew)));
    width: calc((1 + var(--overshoot)) * var(--groove-width));
    margin-left: calc(var(--overshoot) / -2 * var(--groove-width));
    background: transparent;
    position: absolute;
    top: 0;
  }

  .groove-contents {
    --left: 0;
    z-index: var(--groove-contents-layer);
    display: flex;
    flex-direction: row;
    align-items: center;
    position: absolute;
    top: 0;
    left: var(--left);
  }

  .toggle-label {
    width: var(--label-width);
    height: var(--label-height);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toggle-label.on {
    background: green;
    padding-left: var(--label-width);
    margin-left: calc(-1 * var(--label-width));
  }

  .toggle-label.off {
    background: red;
    padding-right: var(--label-width);
    margin-right: calc(-1 * var(--label-width));
  }

  .toggle-label span {
    --shadow-offset: 0.05rem;
    --shadow-intensity: 0.3;
    transform: skew(calc(-1 * var(--skew)));
    color: white;
    font-size: 0.9rem;
    font-family: Nasalization, sans-serif;
    text-transform: uppercase;
    text-shadow:
      calc(-1 * var(--shadow-offset)) calc(-1 * var(--shadow-offset)) 0
        rgba(0, 0, 0, var(--shadow-intensity)),
      var(--shadow-offset) var(--shadow-offset) 0
        rgba(255, 255, 255, var(--shadow-intensity));
  }

  .groove-contents.toggle-layer {
    z-index: var(--toggle-layer);
  }

  .toggle {
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

Since we didn't end up using Headless UI after all, we should remove it from the dependencies.

## Adding sound to the switch

We'll use [this sound](https://pixabay.com/sound-effects/light-switch-156813/), strong and succinct, for the switch. We download it to `src-svelte/static/sounds/switch.mp3`.

As [this answer](https://superuser.com/a/63440) mentions, we can find out the duration of the sound with `ffmpeg`:

```bash
$ ffmpeg -i src-svelte/static/sounds/switch.mp3 2>&1 | grep Duration 
  Duration: 00:00:00.46, start: 0.000000, bitrate: 256 kb/s
```

It appears to take nearly 500ms, which is about 5x longer than our entire switch animation from beginning to end. Perhaps we can speed it up. According to [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate):

> The audio is muted when the fast forward or slow motion is outside a useful range (for example, Gecko mutes the sound outside the range 0.25 to 4.0).

It appears that we'll have to change the speed ourselves. We see from [this answer](https://superuser.com/a/90349) that we can speed it up using `sox`:

sox

```bash
$ sox --show-progress src-svelte/static/sounds/switch.mp3 src-svelte/static/sounds/switch2.mp3 tempo 10
sox FAIL formats: no handler for file extension `mp3'
```

As [this answer](https://superuser.com/a/421168) shows, we must first install:

```bash
$ sudo apt install -y libsox-fmt-mp3
$ sox --show-progress src-svelte/static/sounds/switch.mp3 src-svelte/static/sounds/switch2.mp3 tempo 10

Input File     : 'src-svelte/static/sounds/switch.mp3'
Channels       : 2
Sample Rate    : 48000
Precision      : 16-bit
Duration       : 00:00:00.43 = 20736 samples ~ 32.4 CDDA sectors
File Size      : 14.6k
Bit Rate       : 270k
Sample Encoding: MPEG audio (layer I, II or III)

In:100%  00:00:00.43 [00:00:00.00] Out:2.07k [      |      ]        Clip:0    
Done.
```

The resulting file does not make any sound at all. Let's try to see if we can manually edit the sound file ourselves. From [this answer](https://askubuntu.com/a/927316), we see that Audacity is an option, with a smaller download size than Kdenlive.

We install Audacity and open the sound file up. It turns out that the file is mostly silence, and that the main part of the sound is already pretty snappy and runs its course inside of tens of milliseconds. We delete the first empty part of the sound so that the entire main part of the sound wave begins and ends within 50 ms. This is already quite a snappy sound, the sound file just contains a lot of silence after the beginning. We strip this silence out, and find out that sounds that are too short don't produce any output at all. We undo this last delete, and export the file with just the front part removed. We export as `ogg` since it produces a smaller size than `mp3`, although given that the original mp3 itself is only 14 kB, this does not actually matter much.

We now actually place the new sound file under `src-svelte/src/lib/sounds/switch.ogg`, because as described in the documentation [here](https://vitejs.dev/guide/assets.html):

> Assets in `public` cannot be imported from JavaScript.

However, we absolutely do need to access and play this sound from JS. As such, we instead follow the example [here](https://kit.svelte.dev/docs/assets).

Then, in `src-svelte/src/lib/Switch.svelte`:

```ts
  ...
  import clickSound from "$lib/sounds/switch.ogg";

  ...

  let dragPositionOnLeft = false;

  function playClick() {
    const audio = new Audio(clickSound);
    audio.volume = 0.05;
    audio.play();
  }

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

  let toggleDragOptions: DragOptions = {
    ...
    onDragStart: (data: DragEventData) => {
      ...
      dragPositionOnLeft = !toggledOn;
    },
    onDrag: (data: DragEventData) => {
      ...
      playDragClick(data.offsetX);
    },
    onDragEnd: (data: DragEventData) => {
      ...
      playDragClick(toggledOn ? onLeft : offLeft);
    },
  };

  ...

  function toggle() {
    if (!dragging) {
      toggledOn = !toggledOn;
      playClick();
    }
    dragging = false; // subsequent clicks should register
  }
```

We want the sound to play whenever we click on the switch. If we're dragging the switch, however, then the sound should only play when the switch is successfully dragged all the way to the other end. Even then, we would want to play the sound again if it is dragged back to the starting end -- basically, whenever it switches ends. Finally, if the user releases the switch more than halfway through, then the switch should snap to the other end, and the sound should play, but only if it hasn't already been played yet for that end.

We also want the sound to be played twice if the user quickly double-clicks on the switch. As we see from [this answer](https://stackoverflow.com/a/66991558), we'll need to create a fresh audio object every time to achieve this effect. The default sound is also too loud, so we edit the [volume](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume).

We see that our tests still pass, but now while outputting the following warning message:

```
stderr | src/lib/Switch.test.ts > Switch > can have multiple unique labels
Error: Not implemented: HTMLMediaElement.prototype.play
    at module.exports (/root/zamm/node_modules/jsdom/lib/jsdom/browser/not-implemented.js:9:17)
    at HTMLAudioElementImpl.play (/root/zamm/node_modules/jsdom/lib/jsdom/living/nodes/HTMLMediaElement-impl.js:118:5)
    at HTMLAudioElement.play (/root/zamm/node_modules/jsdom/lib/jsdom/living/generated/HTMLMediaElement.js:148:60)
    at playClick (/root/zamm/src-svelte/src/lib/Switch.svelte:33:11)
    ...
```

We look at [this answer](https://stackoverflow.com/a/71320137) and see that we can just mock the audio API as usual. We edit `src-svelte/src/lib/Switch.test.ts`:

```ts
import { expect, test, vi } from "vitest";
...

const mockAudio = {
  pause: vi.fn(),
  play: vi.fn(),
}

global.Audio = vi.fn().mockImplementation(() => mockAudio);

describe("Switch", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  })

  ...

  test("plays clicking sound during toggle", async () => {
    render(Switch, {});
    expect(mockAudio.play).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(mockAudio.play).toHaveBeenCalledTimes(1);
  });
});
```

Now everything passes, including the new test. However, we want to test the drag interactions more thoroughly. We see that we'll have to emulate the individual mouse events ourselves for drag testing, much as described in the example documentation [here](https://testing-library.com/docs/example-drag/). The Neodrag library also already has drag tests at `src-svelte/forks/neodrag/packages/svelte/tests/Draggable.spec.ts`. We take inspiration from those.
