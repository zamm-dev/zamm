# Information boxes

## Storybook

We follow the strategy recommended [here](https://stackoverflow.com/a/63710793) to simply create a custom view just for the story. We create `src-svelte/src/lib/InfoBoxView.svelte`, making use of [`$$restProps`](https://stackoverflow.com/a/62900378):

```svelte
<script lang="ts">
  import InfoBox from "./InfoBox.svelte";
</script>

<InfoBox {...$$restProps}>
  <p>How do we know that even the realest of realities wouldn't be subjective, in the final analysis? Nobody can prove his existence, can he? &mdash; <em>Simulacron 3</em></p>
</InfoBox>

```

and then create `src-svelte/src/lib/InfoBox.stories.ts`:

```ts
import InfoBox from "./InfoBoxView.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";

export default {
  component: InfoBox,
  title: "Reusable/InfoBox",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: InfoBox,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  title: "Simulation",
}
Regular.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};
```

We edit `src-svelte/src/routes/storybook.test.ts` to add this to the screenshots to take:

```ts
const components: ComponentTestConfig[] = [
  ...
    {
    path: ["reusable", "infobox"],
    variants: [
      "regular",
    ],
    screenshotEntireBody: true,
  },
  ...
];
```

## Sub-information

After implementing the `SubInfoBox` as described in [`settings.md`](/ui/settings.md), we tweak it to center the h3 subheadings:

```svelte
<section ...>
  <div class="subheading">
    <h3 id={subinfoboxId}>{subheading}</h3>
  </div>
  ...
</section>

<style>
  .subheading {
    width: 100%;
    text-align: center;
  }
</style>
```

We can now remove this from `src-svelte/src/routes/settings/Settings.svelte`:

```css
  .container :global(h3) {
    margin-left: var(--side-padding);
  }
```

## Animation

We want to reveal the info box in stages:

0. Start the container box at a small but non-zero size
1. Expand the container horizontally
2. Expand the container vertically
3. Fade in the content

We need to break up the animation into sub-animations and stagger them. We start off by defining the data we need for each sub-animation in `src-svelte/src/lib/InfoBox.svelte`:

```ts
  import { cubicInOut } from "svelte/easing";

  ...

  class SubAnimation {
    delayFraction: number;
    durationFraction: number;
    css: (t: number) => string;

    constructor(anim: {
      delayFraction: number;
      durationFraction: number;
      css: (t: number) => string;
    }) {
      this.delayFraction = anim.delayFraction;
      this.durationFraction = anim.durationFraction;
      this.css = anim.css;
    }

    cssForGlobalTime(t: number) {
      if (t < this.delayFraction) {
        return this.css(0);
      } else if (t > this.delayFraction + this.durationFraction) {
        return this.css(1);
      }

      const subAnimationTime = (t - this.delayFraction) / this.durationFraction;
      return this.css(subAnimationTime);
    }
  }

  class ProperyAnimation extends SubAnimation {
    constructor(anim: {
      delayFraction: number;
      durationFraction: number;
      property: string;
      min: number;
      max: number;
      unit: string;
      easingFunction?: (t: number) => number;
    }) {
      const growth = anim.max - anim.min;
      const easingFunction = anim.easingFunction ?? cubicInOut;
      const css = (t: number) => {
        const easing = easingFunction(t);
        const value = anim.min + growth * easing;
        return `${anim.property}: ${value}${anim.unit};`;
      };
      super({
        delayFraction: anim.delayFraction,
        durationFraction: anim.durationFraction,
        css,
      });
    }
  }
```

Now we define the reveal animation itself:

```ts
  interface RevealParams {
    delay: number;
    duration: number;
  }

  ...

  function reveal(node: Element, { delay, duration }: RevealParams) {
    const actualWidth = node.clientWidth;
    const actualHeight = node.clientHeight;
    const minDimensions = 3 * 18; // 3 rem

    const growWidth = new ProperyAnimation({
      delayFraction: 0,
      durationFraction: 0.5,
      property: "width",
      min: minDimensions,
      max: actualWidth,
      unit: "px",
    });

    const growHeight = new ProperyAnimation({
      delayFraction: 0.4,
      durationFraction: 0.6,
      property: "height",
      min: minDimensions,
      max: actualHeight,
      unit: "px",
    });

    return {
      delay,
      duration,
      css: (t: number) => {
        const width = growWidth.cssForGlobalTime(t);
        const height = growHeight.cssForGlobalTime(t);
        return width + height;
      },
    };
  }
```

Note that we can't simply do a scale transformation, because that would also transform the notches. Also note the animation overlap, where the height starts growing before the width stops completely, because this looks more aesthetically pleasing.

Now we hook the HTML up and manually stagger the fade, which also overlaps with the growth of the height:

```svelte
<script lang="ts>
  import { fade } from "svelte/transition";

  ...

  // let the first half of page transition play before starting
  $: borderBoxDelay = 100 / $animationSpeed;
  $: borderBoxDuration = 200 / $animationSpeed;
  $: infoBoxDelay = 260 / $animationSpeed;
  $: infoBoxDuration = 100 / $animationSpeed;
</script>

...
  <div class="border-container">
    <div
      class="border-box"
      in:reveal|global={{ delay: borderBoxDelay, duration: borderBoxDuration }}
    ></div>
    <div
      class="info-box"
      in:fade|global={{ delay: infoBoxDelay, duration: infoBoxDuration }}
    >
      ...
    </div>
  </div>
...
```

We use `|global` here because otherwise, we find that we aren't seeing any animations in the reveal. This is because we need to use [global transitions](https://svelte.dev/tutorial/global-transitions) when the element is being hidden or revealed as part of a parent element.

### Persistence

Because the animation takes a bit of time, we only show it on the first load of a page. To do so, we'll need to keep track of which pages we've seen, and then signal to the `InfoBox` that it should not animate. As such, we first implement the signaling mechanism in `src-svelte/src/lib/firstPageLoad.ts`:

```ts
import { writable } from "svelte/store";

export const firstPageLoad = writable(false);

```

Now we get InfoBox to listen to this signal in `src-svelte/src/lib/InfoBox.svelte`:

```ts
  ...
  import {firstPageLoad} from "./firstPageLoad";
  ...

  $: borderBoxDelay = $firstPageLoad ? 100 / $animationSpeed : 0;
  $: borderBoxDuration = $firstPageLoad ? 200 / $animationSpeed : 0;
  $: infoBoxDelay = $firstPageLoad ? 260 / $animationSpeed : 0;
  $: infoBoxDuration = $firstPageLoad ? 100 / $animationSpeed : 0;
```

And now we get PageTransition to set this signal in `src-svelte/src/routes/PageTransition.svelte`:

```ts
  ...
  import { firstPageLoad } from "$lib/firstPageLoad";

  ...

  const visitedKeys = new Set<string>();
  
  function checkFirstPageLoad(key: string) {
    if (visitedKeys.has(key)) {
      firstPageLoad.set(false);
    } else {
      visitedKeys.add(key);
      firstPageLoad.set(true);
    }
  }

  ...
  $: checkFirstPageLoad(currentRoute);
```

We make the function take in `currentRoute` as an explicit argument instead of accessing its value directly, because Svelte's reactivity won't know to trigger if `currentRoute` isn't a [direct dependency](https://sveltesociety.dev/recipes/svelte-language-fundamentals/reactivity).

Finally, we create a test for the page transition component successfully signaling whether or not the animations should be disabled. We edit `src-svelte/src/routes/PageTransition.test.ts` to produce a test using the testing library's [rerender](https://testing-library.com/docs/react-testing-library/api/#rerender) function:

```ts
  it("should unset first page load on visit to old page", async () => {
    const pageTransition = render(PageTransitionControl, { currentRoute: "/" });
    pageTransition.rerender({ currentRoute: "/settings" });
    pageTransition.rerender({ currentRoute: "/" });
    expect(get(firstPageLoad)).toEqual(false);
  });
```

This does not work because the component gets completely remounted every time. If we want to avoid using a store just to persist this data for testing purposes, then it appears we'll have to [mock](https://stackoverflow.com/q/66072846) a wrapper component from scratch.

We create `src-svelte/src/routes/PageTransitionControl.svelte`:

```svelte
<script lang="ts">
  import PageTransition from "./PageTransition.svelte";

  export let currentRoute = "/";
</script>

<input type="text" aria-label="Route" bind:value={currentRoute} />
<PageTransition {currentRoute} />

```

However, this does not work:

```ts
  it("should unset first page load on visit to old page", async () => {
    const pageTransition = render(PageTransitionControl, { currentRoute: "/" });
    const routeInput = screen.getByLabelText("Route");
    await act(() => userEvent.type(routeInput, "/settings"));
    await act(() => userEvent.type(routeInput, "/"));
    expect(get(firstPageLoad)).toEqual(false);
  });
```

because the user's typed text simply appends to the existing URL. Instead of wrangling with user events to backspace and delete all existing text, we instead edit the mock Svelte page to use a button that navigates to the new page while clearing all old input:

```svelte
<script lang="ts">
  import PageTransition from "./PageTransition.svelte";

  export let currentRoute = "/";
  let newRoute = "";

  function navigate() {
    currentRoute = newRoute;
    newRoute = "";
  }
</script>

<input type="text" aria-label="Route" bind:value={newRoute} />
<button on:click={navigate}>Navigate</button>
<PageTransition {currentRoute} />

```

Finally, we have a working `src-svelte/src/routes/PageTransition.test.ts`, with two groups of tests, the first one of which we rename to distinguish them:

```ts
...
import PageTransitionControl from "./PageTransitionControl.svelte";
import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { get } from "svelte/store";
import { firstPageLoad } from "$lib/firstPageLoad";

describe("PageTransition durations", () => {
  ...
});

describe("PageTransition", () => {
  let routeInput: HTMLElement;
  let navigateButton: HTMLElement;

  const navigateTo = async (url: string) => {
    await act(() => userEvent.type(routeInput, url));
    await act(() => userEvent.click(navigateButton));
  };

  beforeEach(() => {
    render(PageTransitionControl, { currentRoute: "/" });
    routeInput = screen.getByLabelText("Route");
    navigateButton = screen.getByText("Navigate");
  });

  it("should set first page load on initial visit", async () => {
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should set first page load on visit to new page", async () => {
    await navigateTo("/settings");
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should unset first page load on visit to old page", async () => {
    await navigateTo("/settings");
    await navigateTo("/");
    expect(get(firstPageLoad)).toEqual(false);
  });
});

```

### AnimeJS

We find out that it is [possible](https://dev.to/manyeya/custom-transitions-and-staggered-transitions-in-svelte-with-animejs-plm) to use AnimeJS with Svelte. We leave this for the future.
