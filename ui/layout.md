# App Layout

## Rounding out the left corners of the main content

It turns out that we need to do a major refactor of our app layout to achieve this effect.

In `src-svelte/src/routes/styles.css`, rename `--color-background` to `--color-foreground` and make a new `--color-background` that's equivalent to the sidebar's background color. Now the sidebar can be colorless. However, now the body should be made explicitly white to keep all the Storybook screenshots consistent:

```css
body {
  background-color: white;
}
```

Instead, we set the background in the `.app` div. Create `src-svelte/src/routes/AppLayout.svelte` so that we can also visualize it in a Storybook story:

```svelte
<script>
  import Sidebar from "./Sidebar.svelte";
  import Background from "./Background.svelte";
  import "./styles.css";
</script>

<div class="app">
  <Sidebar />

  <div class="main-container">
    <div class="background-layout">
      <Background />
    </div>
    <main>
      <slot />
    </main>
  </div>
</div>

<style>
  .app {
    box-sizing: border-box;
    height: 100vh;
    width: 100vw;
    position: absolute;
    top: 0;
    left: 0;
    background-color: var(--color-background);
    --main-corners: var(--corner-roundness) 0 0 var(--corner-roundness);
  }

  .main-container {
    height: 100vh;
    box-sizing: border-box;
    margin-left: var(--sidebar-width);
    overflow: scroll;
    border-radius: var(--main-corners);
    background-color: var(--color-foreground);
    box-shadow: calc(-1 * var(--shadow-offset)) 0 var(--shadow-blur) 0 #ccc;
  }

  .background-layout {
    z-index: 0;
    border-radius: var(--main-corners);
    position: absolute;
    top: 0;
    bottom: 0;
    left: var(--sidebar-width);
    right: 0;
  }

  main {
    position: relative;
    z-index: 1;
    padding: 1em;
  }
</style>

```

The main things to keep in mind here are:

- We want the main content to have rounded left corners
- We don't want the main content casting any shadows on the selected icon of the sidebar, or for the selected icon to cast any shadows on the main content either. We will need to understand the [stacking context](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_positioned_layout/Understanding_z-index/Stacking_context) to achieve this.
- We do want the main content to cast a shadow on the rest of the sidebar
- We don't want the animated background of the main content to bleed over into the sidebar or general page background (potentially visible through the bottom left corner)
- We want the main content to lay on top of the animated background
- We want the sidebar, the round corners of the main content, and the background to stay fixed while the contents scroll
- We want the sidebar and rounded corners to span the entire height of the window, without producing any scrollbars of their own

The corresponding story at `src-svelte/src/routes/AppLayout.stories.ts`:

```ts
import AppLayout from "./AppLayout.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: AppLayout,
  title: "Layout/App",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: AppLayout,
  props: args,
});

export const Dynamic: StoryObj = Template.bind({}) as any;
Dynamic.parameters = {
  preferences: {
    unceasingAnimations: true,
  },
};

export const Static: StoryObj = Template.bind({}) as any;
Static.parameters = {
  preferences: {
    unceasingAnimations: false,
  },
};

```

and every time we create a new story, we should add it to the list at `src-svelte/src/routes/storybook.test.ts`. Now that we have decided to use the `Layout/` folder, we should also move the existing sidebar stories to be under that instead. In doing so, we notice that the `dashboard-selected` variant of the sidebar story is somehow no longer listed in the test, but clearly it was at some point because the screenshot is there. We do a git blame on main, which is at commit `6e94a38`, and see that the last time this changed was `d25aac3` when dashboard stories were moved under general screen stories. However, this change was just shifting the lines around and not meaningful on its own. We go to its parent, `05b4241`, and git blame again. To our surprise, this time it is `4e494a0`, where the screenshot was first defined. So when did the dashboard screenshot get added?

We go to `src-svelte/screenshots/baseline/navigation/sidebar/dashboard-selected.png` and see that it was created with 79b6c53. It appears the file got copied without us remembering to update the Storybook test. This is why automation is important.

Now the original `src-svelte/src/routes/+layout.svelte` can be simplified to:

```svelte
<script>
  import AppLayout from "./AppLayout.svelte";
  import "./styles.css";
</script>

<AppLayout>
  <slot />
</AppLayout>

```

Meanwhile in `src-svelte/src/routes/BackgroundUI.svelte`, to clean up scrollbars in the Storybook story:

```css
  .background {
    ...
    overflow: hidden;
    ...
  }

  .bg {
    ...
    position: absolute;
    ...
  }
```

In `src-svelte/src/routes/Sidebar.svelte`, to ensure rendering succeeds in Storybook despite the use of `$app/stores`:

```svelte
<script lang="ts">
  import { page } from "$app/stores";
  ...

  let currentRoute: string;
  $: currentRoute = $page.url?.pathname || "/";
</script>
```

In `src-svelte/src/routes/SidebarUI.svelte`:

```css
  header {
    float: left;
    box-sizing: border-box;
  }
```

We remove z-index, width, background-color, position, top, and left from the header because they're no longer needed.

We remove `header::before` entirely because now the shadow is cast properly by the main element and captured in the screenshot for the overall layout instead of the screenshot for the sidebar itself.

## Centering content horizontally

Edit `src-svelte/src/routes/AppLayout.svelte` to constrain the maximum width of the app layout and center it horizontally:

```css
  main {
    ...
    max-width: 70rem;
    margin: 0 auto;
  }
```

## Updating base animation speed with standard duration

We had previously introduced `--base-animation-speed` to the layout in [`settings.md`](/ui/settings.md). Now we introduce the standard duration to the layout as well, because it is in practice used more often.

We first create `src-svelte/src/routes/AnimationControl.svelte` to consolidate the animation control logic out of `AppLayout.svelte` so that it can be reused across "prod" and testing:

```svelte
<script lang="ts">
  import {
    animationSpeed,
    animationsOn,
    standardDuration,
  } from "$lib/preferences";

  $: standardDurationMs = $standardDuration.toFixed(2) + "ms";
</script>

<div
  class="container"
  class:animations-disabled={!$animationsOn}
  style="--base-animation-speed: {$animationSpeed}; --standard-duration: {standardDurationMs};"
>
  <slot />
</div>

<style>
  .container.animations-disabled :global(*) {
    animation-play-state: paused !important;
    transition: none !important;
  }
</style>

```

We create `src-svelte/src/routes/AnimationControl.test.ts` as well, containing much test logic from the original `AppLayout.svelte`:

```ts
import { expect, test } from "vitest";
import "@testing-library/jest-dom";
import { render } from "@testing-library/svelte";
import AnimationControl from "./AnimationControl.svelte";
import {
  animationsOn,
  animationSpeed,
} from "$lib/preferences";

describe("AnimationControl", () => {
  beforeEach(() => {
    animationsOn.set(true);
    animationSpeed.set(1);
  });

  test("will enable animations by default", async () => {
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).not.toContainEqual("animations-disabled");
    expect(animationControl.getAttribute("style")).toEqual("--base-animation-speed: 1; --standard-duration: 100.00ms;");
  });

  test("will disable animations if preference overridden", async () => {
    animationsOn.set(false);
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).toContainEqual("animations-disabled");
    expect(animationControl.getAttribute("style")).toEqual("--base-animation-speed: 1; --standard-duration: 0.00ms;");
  });

  test("will slow down animations if preference overridden", async () => {
    animationSpeed.set(0.9);
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).not.toContainEqual("animations-disabled");
    expect(animationControl.getAttribute("style")).toEqual("--base-animation-speed: 0.9; --standard-duration: 111.11ms;");
  });
});

```

Now we can refactor `src-svelte/src/routes/AppLayout.svelte` to use this logic instead, and remove the parts that we have now consolidated into `AnimationControl.svelte`:

```svelte
<script lang="ts">
  ...
  import AnimationControl from "./AnimationControl.svelte";
  ...
</script>

<div id="app">
  <AnimationControl>
    <Sidebar />
    ...
  </AnimationControl>
</div>

<style>
  ...
</style>
```

We edit `src-svelte/src/routes/AppLayout.test.ts` as well to make the preference-setting tests more consistent, and to remove the test logic that we've since moved into `AnimationControl`:

```ts
describe("AppLayout", () => {
  ...

  test("will set animation if animation preference overridden", async () => {
    expect(get(animationsOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-animations-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await tickFor(3);
    expect(get(animationsOn)).toBe(false);
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });

  test("will set animation speed if speed preference overridden", async () => {
    expect(get(animationSpeed)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-animation-speed-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await tickFor(3);
    expect(get(animationSpeed)).toBe(0.9);
    expect(tauriInvokeMock).toBeCalledTimes(1);
  });
});

```

We port these changes to `src-svelte/src/lib/__mocks__/MockAppLayout.svelte`, which had previously done its own imitation of `--base-animation-speed`:

```svelte
<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
</script>

<div class="storybook-wrapper">
  <AnimationControl>
    <slot />
    <Snackbar />
  </AnimationControl>
</div>

<style>
  .storybook-wrapper {
    max-width: 50rem;
    position: relative;
  }
</style>

```

We edit `src-svelte/src/lib/__mocks__/MockPageTransitions.svelte` to use `MockAppLayout`:

```svelte
<script lang="ts">
  import MockAppLayout from "./MockAppLayout.svelte";
  ...
</script>

<MockAppLayout>
  <PageTransition ...>
    ...
  </PageTransition>
</MockAppLayout>

```

We manually test the `Dashboard/Full Page` story because animations are not covered by our tests. Next, we do the same for `src-svelte/src/lib/__mocks__/MockTransitions.svelte`, where we get rid of the reset button because Storybook's reset button works just fine for our purposes:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import MockAppLayout from "./MockAppLayout.svelte";

  let visible = false;

  onMount(() => {
    setTimeout(() => {
      visible = true;
    }, 50);
  });
</script>

<MockAppLayout>
  {#if visible}
    <slot />
  {/if}
</MockAppLayout>

```

We manually test the InfoBox stories as well to confirm that they're still working as expected.

For one of the last test-specific mock layouts, we edit `src-svelte/src/routes/PageTransitionView.svelte` to use the new mock setup instead of its own custom ones:

```svelte
<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
  ...
</script>

<MockAppLayout>
  ...
  <PageTransition ...>
    ...
  </PageTransition>
</MockAppLayout>

<style>
  ...
</style>
```

Next, we start replacing all custom calculations with the new standard duration variables and making sure to manually test their transitions, starting with `src-svelte/src/lib/controls/Button.svelte`:

```css
  .outer,
  .inner {
    ...
    transition-property: filter, transform;
    transition: calc(0.5 * var(--standard-duration)) ease-out;
  }
```

and `src-svelte/src/lib/controls/TextInput.svelte`:

```css
  input[type="text"] + .focus-border {
    ...
    transition: width calc(0.5 * var(--standard-duration)) ease-out;
  }
```

and `src-svelte/src/lib/snackbar/Snackbar.svelte`:

```svelte
<script lang="ts">
  import { standardDuration } from "$lib/preferences";
  ...

  $: setBaseAnimationDurationMs($standardDuration);
</script>

<div class="snackbars">
  {#each ...}
    <div
      in:fly|global={{ y: "1rem", duration: $standardDuration }}
      out:fade|global={{ duration: $standardDuration }}
      ...
    >
      ...
    </div>
  {/each}
</div>

```

and `src-svelte/src/lib/Slider.svelte`:

```ts
  const transitionAnimation =
    `transition: transform var(--standard-duration) ease-out;`;
```

and `src-svelte/src/lib/Switch.svelte`:

```ts
  const transitionAnimation = `
    transition: transform var(--standard-duration);
    transition-timing-function: cubic-bezier(0, 0, 0, 1.3);
  `;
```

and `src-svelte/src/routes/components/api-keys/Service.svelte`:

```css
  .api-key {
    ...
    transition: var(--standard-duration) ease-in;
  }
```

and `src-svelte/src/routes/PageTransition.svelte`:

```ts
  ...
  import { standardDuration } from "$lib/preferences";
  ...

  // twice the speed of sidebar UI slider
  $: totalDurationMs = 2 * $standardDuration;
  ...
```

and `src-svelte/src/routes/SidebarUI.svelte`:

```css
  header {
    --animation-duration: calc(2 * var(--standard-duration));
    ...
  }
```

We add a default to `src-svelte/src/routes/styles.css` in case anything goes wrong and the CSS variable ends up undefined:

```css
  :root {
    ...
    --standard-duration: 100ms;
    ...
  }
```

Finally, we come to `src-svelte/src/routes/BackgroundUI.svelte`:

```css
  .background {
    ...
    --base-duration: calc(150 * var(--standard-duration));
    ...
  }
```

With this new setup, the background no longer appears. Because this works already in "prod," we create a new view just for the background element so that test code does not impact regular code. We create `src-svelte/src/routes/BackgroundUIView.svelte`:

```svelte
<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
</script>

<MockAppLayout>
  <div class="background-container">
    <slot />
  </div>
</MockAppLayout>

<style>
  .background-container {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
</style>

```

We edit `src-svelte/src/routes/BackgroundUI.stories.ts` accordingly to use this new view instead:

```ts
...
import BackgroundUIView from "./BackgroundUIView.svelte";

export default {
  ...
  decorators: [
    ...,
    (story: StoryFn) => {
      return {
        Component: BackgroundUIView,
        slot: story,
      };
    },
  ],
};

...
```

Finally, we have to edit `src-svelte/src/routes/storybook.test.ts` because now that we're using the new `MockLayout`, the actual components being tested are nested more deeply than before:

```ts
  const takeScreenshot = async (...) => {
    ...
    if (elementClass?.includes("storybook-wrapper")) {
      locator = ".storybook-wrapper > :first-child > :first-child";
    }
    ...
  };
```

We find that two screenshots have now changed: `src-svelte/screenshots/baseline/layout/sidebar/dashboard-selected.png` and `src-svelte/screenshots/baseline/layout/sidebar/settings-selected.png` now no longer have rounded corners at the bottom left corner, which is what we want after all but never realized was the case, so we update those screenshots as well.

While committing, we find that eslint complains

```
/root/zamm/src-svelte/src/lib/Slider.svelte
  14:1  error  This line has a length of 89. Maximum allowed is 88  max-len
```

The line in question is

```ts
  const transitionAnimation = `transition: transform var(--standard-duration) ease-out;`;
```

If we simply put a newline after the `=`, `prettier` will reformat it back to its current state, so we have to split it more vigorously:

```ts
  const transitionAnimation =
    `transition: ` + `transform var(--standard-duration) ease-out;`;
```

We do something similar with `src-svelte/src/routes/AnimationControl.svelte`, where there is the line

```svelte
<div
  ...
  style="--base-animation-speed: {$animationSpeed}; --standard-duration: {standardDurationMs};"
>
```

We turn this into

```svelte
<script lang="ts">
  ...
  $: style =
    `--base-animation-speed: ${$animationSpeed}; ` +
    `--standard-duration: ${standardDurationMs};`;
</script>

<div ... {style}>
  ...
</div>

```

### Debugging reveal animation regression

At some point, we realize that the reveal animation for the Active/Inactive indicator starts too late. The indicator appears long after everything else has appeared, such that the fade-in to green doen't even appear anymore because the fade has completed by the time the element appears. After digging through our Git history to find the last time this worked, we see that things are still fine on `88fa2e9` but are failing by `4c1ccaf`. We do a git bisect:

```bash
$ git bisect start 4c1ccaf 88fa2e9
Bisecting: 2 revisions left to test after this (roughly 1 step)
[7cc5050a60758f12f64305684ce0fddad75a29e5] Allow sample calls to represent API call failures as well
$ git bisect good                 
Bisecting: 0 revisions left to test after this (roughly 1 step)
[bd22202637df28192a199a451320c2afc53fc177] Merge pull request #41 from amosjyng/refactor/sample-call-failures
$ git bisect good
4c1ccaf4dc5a3eacb3f43e8f9db04fbfa5e6ee7f is the first bad commit
commit 4c1ccaf4dc5a3eacb3f43e8f9db04fbfa5e6ee7f
Author: Amos Jun-yeung Ng <me@amos.ng>
Date:   Sat Jan 6 11:15:36 2024 +1100

    Refactor animation settings to use standardDuration
    ...
```

Back then we didn't edit `src-svelte/src/lib/InfoBox.svelte` because it uses its own

```ts
  $: timingScaleFactor = shouldAnimate ? 1 / $animationSpeed : 0;
```

instead of the usual standardDuration. It appears that file is not the culprit, as the transition timing returned is the same for both elements of the API keys display.

Instead, reverting the files one by one, we see that reverting `src-svelte/src/routes/components/api-keys/Service.svelte` fixes the bug. The change in question is going from

```css
    transition-property: background-color, box-shadow;
    transition-duration: calc(0.1s / var(--base-animation-speed));
    transition-timing-function: ease-in;
```

to

```css
    transition-property: background-color, box-shadow;
    transition: var(--standard-duration) ease-in;
```

without realizing that the `transition` applies to all properties, not just the properties defiend in `transition-property`. We fix this while keeping the spirit of the refactor by changing the lines to

```css
    transition-property: background-color, box-shadow;
    transition-duration: var(--standard-duration);
    transition-timing-function: ease-in;
```

This would be a good example of an animation regression that you would ideally be able to write a test for with an animation testing framework. Even without it, this is where the infrastructure we've built, including our Git history, comes through for us.

## Positioning the snackbar

We notice that sometimes the snackbar is covered by other things. We edit `src-svelte/src/lib/snackbar/Snackbar.svelte`:

```css
  .snackbars {
    z-index: 100;
    ...
  }
```

Because `src-svelte/src/lib/__mocks__/MockAppLayout.svelte` looks like this:

```svelte
...

<div class="storybook-wrapper">
  <AnimationControl>
    <slot />
    <Snackbar />
  </AnimationControl>
</div>

...
```

we make sure to mark `src-svelte/src/routes/AnimationControl.svelte` as a positioned element:

```css
  .container {
    ...
    position: relative;
  }
```

We check the same for prod, and we see that `src-svelte/src/routes/AppLayout.svelte` looks like this:

```svelte
<div id="app">
  <AnimationControl>
    <Sidebar />

    <div class="main-container">
      <div class="background-layout">
        <Background />
      </div>
      <Snackbar />

      <main>
        ...
      </main>
    </div>
  </AnimationControl>
</div>

```

We edit the CSS for the parent element accordingly:

```css
  .main-container {
    ...
    position: relative;
    ...
  }
```

However, we find out from our end-to-end tests that the sidebar now appears under the main content. We realize we should undo this last change, because the `div` created by `AnimationControl` will now be the one that the snackbar's `z-index` applies to.
