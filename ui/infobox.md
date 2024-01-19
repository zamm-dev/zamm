# Information boxes

## Layout

### Dimensions

On a large screen, the text inside of the info box becomes too sparse and unreadable. We edit `src-svelte/src/lib/InfoBox.svelte` to constrain the maximum width:

```svelte
<script lang="ts">
  ...
  export let maxWidth = "50rem";
  ...
</script>

<section class="container" aria-labelledby={infoboxId} style="max-width: {maxWidth};">
  ...
</section>
```

We find that this is a comfortable maximum width except for the settings screen. We edit `src-svelte/src/routes/settings/Settings.svelte` accordingly:

```svelte
<InfoBox title="Settings" maxWidth="70rem">
  ...
</InfoBox>
```

However, upon visiting the homepage when the window is maximized, we realize that the info boxes are no longer aligned. We therefore constrain the max width as indicated in [`layout.md`](/ui/layout.md), and then edit `src-svelte/src/lib/InfoBox.svelte` to make the max width constraint optional:

```svelte
<script lang="ts">
  ...
  export let maxWidth: string | undefined = undefined;
  let maxWidthStyle = maxWidth === undefined ? "" : `max-width: ${maxWidth};`;
  ...
</script>

<section
  ...
  style={maxWidthStyle}
  ...
>
</section>
```

and remove the constraint from `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<InfoBox ...>
  ...
</InfoBox>
```

Finally, we constrain the size in our Storybook stories in order to make them look good even when the browser window is maximized. We edit `src-svelte/src/routes/PageTransitionView.svelte`:

```css
  .storybook-wrapper {
    ...
    max-width: 50rem;
    ...
  }
```

and `src-svelte/src/lib/InfoBox.stories.ts`:

```ts
...
Regular.args = {
  ...
  maxWidth: "50rem",
};

...
MountTransition.args = {
  ...
  maxWidth: "50rem",
};

...
SlowMotion.args = {
  ...
  maxWidth: "50rem",
};

...
Motionless.args = {
  ...
  maxWidth: "50rem",
};
```

We could also do this by introducing a Storybook wrapper that constrains the size of the content within.

### Bottom padding

We notice that at some point, the bottom padding from the content to the border is much thicker than it is between the title and the top border. We find that this is because of the default 1 rem margin around `<p>` elements. We disable this in `src-svelte/src/lib/InfoBox.svelte`:

```css
  .info-box :global(p:last-child) {
    margin-bottom: 0;
  }
```

but find that the subheadings are now too close to the text from the previous section. We fix this in `src-svelte/src/lib/SubInfoBox.svelte`:

```css
  .subheading {
    ...
    margin: 0.5rem 0;
  }
```

Now the subheadings are too far from the text that follows them. We disable the margins from the first paragraph as well:

```css
  .content :global(p:first-child) {
    margin-top: 0;
  }
```

It is not recommended to update `src-svelte/src/routes/settings/Settings.svelte` to remove the `.container` and `.container:first-of-type` styles, because it's still useful to add a little extra spacing in cases of dense non-text elements such as the settings page.

### Text justification

We justify all text except for the h2:

```css
  .info-box {
    ...
    text-align: justify;
  }

  .info-box h2 {
    text-align: left;
  }
```

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

### Alternative implementations

#### Using CSS instead

Note that Svelte transitions [are not](https://old.reddit.com/r/sveltejs/comments/12ldjbf/comment/jgd5zni/) the most performant, and we could achieve this with CSS animations instead as mentioned in the example in the thread:

```svelte
<script>
  import { onMount, onDestroy} from 'svelte/lifecycle'

  let visible = false;
  
  onMount(() => {visible = true})
  onDestroy(() => {visible = false})
  // Could also use beforeNavigate, depending on the use-case.
</script>

<div class:visible />

<style lang="scss">
  @mixin fly($direction: 'right', $distance: 70%, $duration: 400ms, $ease: ease-out) {
    transition: translate $duration $ease, opacity $duration $ease;
    opacity: 0;
    pointer-events: none;
    @if ($direction == 'right') {
      translate: $distance 0;
    } @else if($direction == 'left') {
      translate: (-$distance) 0;
    } @else if($direction == 'up') {
      translate: 0 (-$distance);
    } @else if($direction == 'down') {
      translate: 0 $distance;
    }
    &.visible {
      translate: 0 0;
      opacity: 1;
      pointer-events: auto;
    }
  }

  div {
    @include fly;
  }
</style>
```

#### Using AutoAnimate

The [`AutoAnimate` library](https://auto-animate.formkit.com/) appears to be yet another way of animating transitions, in a way that's compatible across different web frameworks.

#### AnimeJS

We find out that it is also [possible](https://dev.to/manyeya/custom-transitions-and-staggered-transitions-in-svelte-with-animejs-plm) to use AnimeJS with Svelte.

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

Now we have to also update `src-svelte/src/lib/InfoBoxView.svelte` to always treat it as a first page load:

```ts
  ...
  import { firstPageLoad } from "$lib/firstPageLoad";

  firstPageLoad.set(true);
```

We can also update `src-svelte/src/lib/firstPageLoad.ts` to set it to true by default:

```ts
export const firstPageLoad = writable(true);
```

Since nothing will overwrite this value for `InfoBoxView`, it will always be true so long as we're inside that component. However, as soon as we go to another component, such as `PageTransition`, and navigate back, we'll find that this will be always false instead. As such, we still keep setting the first page load to true before viewing the InfoBox story.

### Typewriter effect

We first see if there is an existing code we can reuse. We see that there is [this example](https://svelte.dev/repl/feb73de2a74544d2afa8e8fffc2ed29c?version=3.29.7), but it is not actually what we want.

We try using [svelte-typewriter](https://git.sr.ht/~henriquehbr/svelte-typewriter):

```bash
$ yarn add svelte-typewriter
```

Then in `src-svelte/src/lib/InfoBox.svelte`:

```svelte
<script lang="ts">
  ...
  import Typewriter from 'svelte-typewriter';

  ...
  $: entireDuration = infoBoxDelay + infoBoxDuration - borderBoxDelay;
  $: titleDelay = $firstPageLoad ? 100 / $animationSpeed : 0;
  $: titleInterval = $firstPageLoad ? entireDuration / title.length / 1.3 : 0;
</script>

...
    <div class="info-box">
        <Typewriter mode="cascade" delay={titleDelay} interval={titleInterval} cursor={false}>
          <h2 id={infoboxId}>{title}</h2>
        </Typewriter>
        ...
    </div>
...
```

We wrap only the h2 in the Typewriter component because wrapping it around the entire body produces an excessive and unpredictable animation duration. The titleInterval somehow works slower than expected and therefore require s the additional division by 1.3. There's also the problem of the final height of the box shifting at the end, which even disabling the cursor doesn't appear to help with. This might be because the h2 being empty in the beginning messes with the layout.

To fix these issues, we might as well implement it from scratch ourselves. We follow the example [here](https://svelte.dev/tutorial/custom-js-transitions). We edit `src-svelte/src/lib/InfoBox.svelte` to first rename `RevealParams` into `AnimationParams` because it fits both the existing and the new animation we're about to define:

```ts
  interface AnimationParams {
    delay: number;
    duration: number;
  }

  function reveal(node: Element, { delay, duration }: AnimationParams) {
    ...
  }
```

Next, we slightly modify what they gave us in that example, and use it here:

```ts
<script lang="ts">
  ...

  function typewriter(node: Element, { delay, duration }: AnimationParams) {
    const valid =
      node.childNodes.length === 1 &&
      node.childNodes[0].nodeType === Node.TEXT_NODE;

    if (!valid) {
      throw new Error(
        `This transition only works on elements with a single text node child`,
      );
    }

    const text = node.textContent ?? "";

    return {
      delay,
      duration,
      tick: (t: number) => {
        const i = Math.trunc(text.length * t);
        node.textContent = text.slice(0, i);
      },
    };
  }

  ...
  $: titleDelay = $firstPageLoad ? 110 / $animationSpeed : 0;
  $: titleDuration = $firstPageLoad ? borderBoxDuration * 0.3 : 0;
</script>

...
    <div class="info-box">
      <h2
        in:typewriter|global={{ delay: titleDelay, duration: titleDuration }}
        id={infoboxId}
      >
        {title}
      </h2>
      <div
        class="info-content"
        in:fade|global={{ delay: infoBoxDelay, duration: infoBoxDuration }}
      >
        <slot />
      </div>
    </div>
...
```

We give the border box horizontal expansion a bit of a head start to avoid making the first text too crowded, and we ensure that the title text ends by the time the horizontal expansion finishes.

#### Cursor block

Even though we're not using the svelte-typewriter package, we still want to replicate its cursor block effect, subject to the following requirements:

- Unlike svelte-typewriter, our cursor block should not blink. Instead, it should stay solid the whole time throughout
- The animation progression will be as follows:
  - start off with just the cursor block, without any text
  - update with the next fractional part of the text each time the function is called
  - end with the entire text, plus the cursor block
- Afterwards, the cursor block will slowly fade out, ending its transition at the same time as the vertical expansion

```svelte
<script lang="ts">
  ...
  const heightDelayFraction = 0.4;
  ...

  function reveal(node: Element, { delay, duration }: AnimationParams) {
    ...

    const growHeight = new ProperyAnimation({
      delayFraction: heightDelayFraction,
      durationFraction: 1 - heightDelayFraction,
      ...
    });

    ...
  }

  function typewriter(node: Element, { delay, duration }: AnimationParams) {
    ...
    const length = text.length + 1;

    return {
      delay,
      duration,
      tick: (t: number) => {
        const i = Math.trunc(length * t);
        node.textContent = text.slice(0, i - 1);
        if (t == 0) {
          node.classList.add("typewriting");
          node.classList.remove("done");
          node.classList.remove("started");
        } else if (t == 1) {
          node.classList.add("done");
        } else if (i > 0) {
          node.classList.add("started");
        }
      },
    };
  }

  ...
  $: heightStart = borderBoxDelay + heightDelayFraction * borderBoxDuration;
  $: titleDelay = $firstPageLoad ? 120 / $animationSpeed : 0;
  // title typing should end when height starts growing
  $: titleDuration = heightStart - titleDelay;
  // cursor fade should end when height stops growing
  $: cursorFadeDuration = (1 - heightDelayFraction) * borderBoxDuration;
</script>

...
      <h2
        ...
        style="--fade-duration: {cursorFadeDuration}ms;"
      >
...

<style>
  ...

  .info-box :global(h2.typewriting) {
    opacity: 0;
  }

  .info-box :global(h2.typewriting.started), .info-box :global(h2.typewriting.done) {
    opacity: 1;
  }

  .info-box :global(h2.typewriting.started::after) {
    content: "█";
    transition: opacity var(--fade-duration) ease-out;
  }

  .info-box :global(h2.typewriting.done::after) {
    opacity: 0;
  }
</style>
```

Note that:

- We refactor `heightDelayFraction` to make use of it later
- We do the `node.classList.remove` in order to reset the div state in between stories on Storybook, or else the transition would no longer run when we visit the Page Transitions story.
- Because we're now starting with the empty cursor block rather than the first letter of the text, it's more obvious that the empty block overlaps with the border box at the very beginning of the animation. As such, we add an extra 10ms to the title delay effect to keep the two from colliding.
- Svelte calls our `tick` function with the value `0` at the very beginning of animation setup, then waits for the delay, and then incrementally increases `t` until it reaches `1`. We take advantage of this to change the classes of the h2 accordingly at each stage of the animation.
- Because we use CSS transitions in conjunction with the Svelte ones, we have to edit `src-svelte/src/lib/InfoBoxView.svelte` as well:

```svelte
<script lang="ts">
  ...
  import { animationSpeed } from "$lib/preferences";

  ...
</script>

<div style="--base-animation-speed: {$animationSpeed};">
  ...
</div>
```

We realize that the opacity styling on the main h2 element (as opposed to the pseudo-element) is not actually necessary, and the only reason it shows (most of) the h2 element at the beginning is because `i` is 0 and therefore `text.slice(0, i - 1)` underflows. We fix the tick function accordingly:

```ts
      tick: (t: number) => {
        const i = Math.trunc(length * t);
        node.textContent = i === 0 ? "" : text.slice(0, i - 1);
        if (t == 0) {
          node.classList.remove("typewriting");
          node.classList.remove("done");
        } else if (t == 1) {
          node.classList.add("done");
        } else {
          node.classList.add("typewriting");
        }
      },
```

and remove this CSS:

```css
  .info-box :global(h2.typewriting) {
    opacity: 0;
  }

  .info-box :global(h2.typewriting.started), .info-box :global(h2.typewriting.done) {
    opacity: 1;
  }
```

#### Disabled animations

We still have to make sure to disable these animation effects when animation is disabled. So, we add a story to `src-svelte/src/lib/InfoBox.stories.ts` to test:

```ts
export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.args = {
  title: "Simulation",
};
Motionless.parameters = {
  preferences: {
    animationsOn: false,
  },
};
Motionless.decorators = [
  SvelteStoresDecorator,
  (story: StoryFn) => {
    return {
      Component: MockTransitions,
      slot: story,
    };
  },
];
```

and then we fix it in `src-svelte/src/lib/InfoBox.svelte`:

```ts
  ...
  import { animationSpeed, animationsOn } from "./preferences";
  ...

  ...
  $: shouldAnimate = $animationsOn && $firstPageLoad;
  // let the first half of page transition play before starting
  $: borderBoxDelay = shouldAnimate ? 100 / $animationSpeed : 0;
  $: borderBoxDuration = shouldAnimate ? 200 / $animationSpeed : 0;
  $: infoBoxDelay = shouldAnimate ? 260 / $animationSpeed : 0;
  $: infoBoxDuration = shouldAnimate ? 100 / $animationSpeed : 0;
  $: heightStart = borderBoxDelay + heightDelayFraction * borderBoxDuration;
  $: titleDelay = shouldAnimate ? 120 / $animationSpeed : 0;
  ...
```

We notice a flicker when rendering the motionless story. With console logging, we realize that when animation duration is 0, the tick function gets called at both 0 and 1, even though it doesn't need to be called at all in this case. So, we fix it and get rid of the flicker:

```ts
    return {
      delay,
      duration,
      tick: (t: number) => {
        if (duration === 0) {
          return;
        }
        ...
      },
    };
```

#### Refactoring timing code

We define new primitives:

```ts
class TransitionTimingMs {
    durationMs: number;
    delayMs: number;

    constructor({durationMs, delayMs}: {durationMs: number, delayMs: number}) {
      this.durationMs = durationMs;
      this.delayMs = delayMs;
    }

    startMs(): number {
      return this.delayMs;
    }

    endMs(): number {
      return this.delayMs + this.durationMs;
    }

    delayByMs(delayMs: number): TransitionTimingMs {
      return new TransitionTimingMs({
        durationMs: this.durationMs,
        delayMs: this.delayMs + delayMs,
      });
    }

    hastenByMs(hastenMs: number): TransitionTimingMs {
      return this.delayByMs(-hastenMs);
    }

    toFraction(totalDurationMs: number): TransitionTimingFraction {
      return new TransitionTimingFraction({
        durationFraction: this.durationMs / totalDurationMs,
        delayFraction: this.delayMs / totalDurationMs,
      });
    }

    nestInside(container: TransitionTimingMs): TransitionTimingFraction {
      return this.hastenByMs(container.delayMs).toFraction(container.durationMs);
    }
  }

  class TransitionTimingFraction {
    durationFraction: number;
    delayFraction: number;

    constructor({durationFraction, delayFraction}: {durationFraction: number, delayFraction: number}) {
      this.durationFraction = durationFraction;
      this.delayFraction = delayFraction;
    }

    toMs(totalDurationMs: number): TransitionTimingMs {
      return new TransitionTimingMs({
        durationMs: this.durationFraction * totalDurationMs,
        delayMs: this.delayFraction * totalDurationMs
      });
    }

    unnestFrom(container: TransitionTimingMs): TransitionTimingMs {
      return this.toMs(container.durationMs).delayByMs(container.delayMs);
    }
  }
```

We generalize the border box timing to use these new primitives:

```ts
  class BorderBoxTimingMs {
    growX: TransitionTimingMs;
    growY: TransitionTimingMs;

    constructor({growX, growY}: {growX: TransitionTimingMs, growY: TransitionTimingMs}) {
      this.growX = growX;
      this.growY = growY;
    }

    startMs(): number {
      return Math.min(this.growX.startMs(), this.growY.startMs());
    }

    endMs(): number {
      return Math.max(this.growX.endMs(), this.growY.endMs());
    }

    overallTiming(): TransitionTimingMs {
      const duration = this.endMs() - this.startMs();
      return new TransitionTimingMs({
        durationMs: duration,
        delayMs: this.startMs(),
      });
    }

    toFraction(): BorderBoxTimingFraction {
      const overall = this.overallTiming();
      return new BorderBoxTimingFraction({
        overall,
        growX: this.growX.nestInside(overall),
        growY: this.growY.nestInside(overall),
      });
    }
  }

  class BorderBoxTimingFraction {
    overall: TransitionTimingMs;
    growX: TransitionTimingFraction;
    growY: TransitionTimingFraction;

    constructor({ overall, growX, growY }: { overall: TransitionTimingMs, growX: TransitionTimingFraction, growY: TransitionTimingFraction }) {
      this.overall = overall;
      this.growX = growX;
      this.growY = growY;
    }

    toMs(): BorderBoxTimingMs {
      return new BorderBoxTimingMs({
        growX: this.growX.unnestFrom(this.overall),
        growY: this.growY.unnestFrom(this.overall),
      });
    }
  }
```

and in doing so discover that we can generalize further to the case of any number of sub-animations, and have `BorderBoxTimingMs` and `BorderBoxTimingFraction` extend from the specific cases:

```ts
  class BorderBoxTimingMs extends TimingGroupMs {
    growX(): TransitionTimingMs {
      return this.children[0];
    }

    growY(): TransitionTimingMs {
      return this.children[1];
    }

    toFraction(): BorderBoxTimingFraction {
      const groupTimingFraction = super.toFraction();
      return new BorderBoxTimingFraction({
        overall: groupTimingFraction.overall,
        children: groupTimingFraction.children,
      });
    }
  }

  class BorderBoxTimingFraction extends TimingGroupFraction {
    growX(): TransitionTimingFraction {
      return this.children[0];
    }

    growY(): TransitionTimingFraction {
      return this.children[1];
    }

    toMs(): BorderBoxTimingMs {
      const groupTimingMs = super.toMs();
      return new BorderBoxTimingMs(groupTimingMs.children);
    }
  }
```

However, we find that `InfoBoxTimingMs` cannot extend `TimingGroupMs` like `BorderBoxTimingMs` does, because a `TimingGroupMs` cannot be nested inside of another `TimingGroupMs` because it only expects `TransitionTimingMs` as children. So we once again refactor out the similarities. In a classic case of over-engineering, we now have to deal with edge cases of how to nest and unnest nested children, that have nothing to do with the specific problem at hand. We ignore these edge cases by electing to implement the interface functions by throwing an exception.

We also wrap the cursor fade into the same animation group as the title typing sequence, instead of kicking off the effect separately via CSS, for consistency and synchronization. Unfortunately, there does not appear to be a way to apply [multiple Svelte transitions on the same element](https://stackoverflow.com/questions/68235936/how-to-use-two-svelte-transitions-on-the-same-togglable-element), so manually building out the infrastructure ourselves appears to be the next best option.

The final code:

```svelte
<script lang="ts" context="module">
  export interface TransitionTimingMs {
    durationMs(): number;
    delayMs(): number;
    startMs(): number;
    endMs(): number;
    delayByMs(delayMs: number): TransitionTimingMs;
    hastenByMs(hastenMs: number): TransitionTimingMs;
    scaleBy(factor: number): TransitionTimingMs;
    nestInside(container: TransitionTimingMs): TransitionTimingFraction;
  }

  export interface TransitionTimingFraction {
    durationFraction(): number;
    delayFraction(): number;
    startFraction(): number;
    endFraction(): number;
    unnestFrom(container: TransitionTimingMs): TransitionTimingMs;
    localize(globalTimeFraction: number): number;
  }

  export class PrimitiveTimingMs implements TransitionTimingMs {
    _durationMs: number;
    _delayMs: number;

    constructor({
      durationMs,
      delayMs,
      startMs,
      endMs,
    }: {
      durationMs?: number;
      delayMs?: number;
      startMs?: number;
      endMs?: number;
    }) {
      if ((delayMs === undefined) === (startMs === undefined)) {
        throw new Error("Exactly one of delayMs or startMs must be provided");
      }
      if ((durationMs === undefined) === (endMs === undefined)) {
        throw new Error("Exactly one of durationMs or endMs must be provided");
      }
      this._delayMs = (delayMs ?? startMs) as number;
      this._durationMs = durationMs ?? (endMs as number) - this._delayMs;
    }

    round(): PrimitiveTimingMs {
      return new PrimitiveTimingMs({
        durationMs: Math.round(this._durationMs),
        delayMs: Math.round(this._delayMs),
      });
    }

    durationMs(): number {
      return this._durationMs;
    }

    delayMs(): number {
      return this._delayMs;
    }

    startMs(): number {
      return this._delayMs;
    }

    endMs(): number {
      return this._delayMs + this._durationMs;
    }

    delayByMs(delayMs: number): PrimitiveTimingMs {
      return new PrimitiveTimingMs({
        durationMs: this._durationMs,
        delayMs: this._delayMs + delayMs,
      });
    }

    hastenByMs(hastenMs: number): PrimitiveTimingMs {
      return this.delayByMs(-hastenMs);
    }

    scaleBy(factor: number): PrimitiveTimingMs {
      return new PrimitiveTimingMs({
        durationMs: this._durationMs * factor,
        delayMs: this._delayMs * factor,
      });
    }

    toFraction(totalDurationMs: number): PrimitiveTimingFraction {
      if (totalDurationMs === 0) {
        return new PrimitiveTimingFraction({
          // if duration is total, then the fraction is meaningless
          // might as well set it to 1 to prevent further division by zero
          durationFraction: 1,
          delayFraction: 1,
        });
      }
      return new PrimitiveTimingFraction({
        durationFraction: this._durationMs / totalDurationMs,
        delayFraction: this._delayMs / totalDurationMs,
      });
    }

    nestInside(container: TransitionTimingMs): PrimitiveTimingFraction {
      return this.hastenByMs(container.delayMs()).toFraction(
        container.durationMs(),
      );
    }
  }

  export class PrimitiveTimingFraction implements TransitionTimingFraction {
    _durationFraction: number;
    _delayFraction: number;

    constructor({
      durationFraction,
      delayFraction,
      startFraction,
      endFraction,
    }: {
      durationFraction?: number;
      delayFraction?: number;
      startFraction?: number;
      endFraction?: number;
    }) {
      if ((delayFraction === undefined) === (startFraction === undefined)) {
        throw new Error(
          "Exactly one of delayFraction or startMs must be provided",
        );
      }
      if ((durationFraction === undefined) === (endFraction === undefined)) {
        throw new Error(
          "Exactly one of durationFraction or endMs must be provided",
        );
      }
      this._delayFraction = (delayFraction ?? startFraction) as number;
      this._durationFraction =
        durationFraction ?? (endFraction as number) - this._delayFraction;
    }

    round(): PrimitiveTimingFraction {
      const precision = 10_000;
      return new PrimitiveTimingFraction({
        durationFraction:
          Math.round(this._durationFraction * precision) / precision,
        delayFraction: Math.round(this._delayFraction * precision) / precision,
      });
    }

    delayFraction(): number {
      return this._delayFraction;
    }

    durationFraction(): number {
      return this._durationFraction;
    }

    startFraction(): number {
      return this._delayFraction;
    }

    endFraction(): number {
      return this._delayFraction + this._durationFraction;
    }

    toMs(totalDurationMs: number): PrimitiveTimingMs {
      return new PrimitiveTimingMs({
        durationMs: this._durationFraction * totalDurationMs,
        delayMs: this._delayFraction * totalDurationMs,
      });
    }

    unnestFrom(container: TransitionTimingMs): PrimitiveTimingMs {
      return this.toMs(container.durationMs()).delayByMs(container.delayMs());
    }

    localize(globalTimeFraction: number): number {
      if (globalTimeFraction < this.startFraction()) {
        return 0;
      } else if (globalTimeFraction > this.endFraction()) {
        return 1;
      }

      const localTimeFraction =
        (globalTimeFraction - this._delayFraction) / this._durationFraction;
      return localTimeFraction;
    }
  }

  export class TimingGroupAsCollection implements TransitionTimingMs {
    children: TransitionTimingMs[];

    constructor(children: TransitionTimingMs[]) {
      this.children = children;
    }

    startMs(): number {
      const startTimes = this.children.map((child) => child.startMs());
      return Math.min(...startTimes);
    }

    endMs(): number {
      const endTimes = this.children.map((child) => child.endMs());
      return Math.max(...endTimes);
    }

    durationMs(): number {
      return this.endMs() - this.startMs();
    }

    delayMs(): number {
      return this.startMs();
    }

    overallTiming(): PrimitiveTimingMs {
      return new PrimitiveTimingMs({
        durationMs: this.durationMs(),
        delayMs: this.delayMs(),
      });
    }

    delayByMs(delayMs: number): TimingGroupAsCollection {
      return new TimingGroupAsCollection(
        this.children.map((child) => child.delayByMs(delayMs)),
      );
    }

    hastenByMs(hastenMs: number): TimingGroupAsCollection {
      return new TimingGroupAsCollection(
        this.children.map((child) => child.hastenByMs(hastenMs)),
      );
    }

    scaleBy(factor: number): TimingGroupAsCollection {
      return new TimingGroupAsCollection(
        this.children.map((child) => child.scaleBy(factor)),
      );
    }

    nestInside(_: TransitionTimingMs): TransitionTimingFraction {
      throw new Error("Recursive nesting not implemented");
    }

    asIndividual(): TimingGroupAsIndividual {
      const overall = this.overallTiming();
      const nestedChildren = this.children.map((child) =>
        child.nestInside(overall),
      );
      return new TimingGroupAsIndividual({
        overall,
        children: nestedChildren,
      });
    }
  }

  export class TimingGroupAsIndividual implements TransitionTimingMs {
    overall: TransitionTimingMs;
    children: TransitionTimingFraction[];

    constructor({
      overall,
      children,
    }: {
      overall: TransitionTimingMs;
      children: TransitionTimingFraction[];
    }) {
      this.overall = overall;
      this.children = children;
    }

    durationMs(): number {
      return this.overall.durationMs();
    }

    delayMs(): number {
      return this.overall.delayMs();
    }

    startMs(): number {
      return this.overall.startMs();
    }

    endMs(): number {
      return this.overall.endMs();
    }

    delayByMs(delayMs: number): TimingGroupAsIndividual {
      return new TimingGroupAsIndividual({
        overall: this.overall.delayByMs(delayMs),
        children: this.children,
      });
    }

    hastenByMs(hastenMs: number): TimingGroupAsIndividual {
      return new TimingGroupAsIndividual({
        overall: this.overall.hastenByMs(hastenMs),
        children: this.children,
      });
    }

    scaleBy(factor: number): TimingGroupAsIndividual {
      return new TimingGroupAsIndividual({
        overall: this.overall.scaleBy(factor),
        children: this.children,
      });
    }

    nestInside(_: TransitionTimingMs): TransitionTimingFraction {
      throw new Error("Recursive nesting not implemented");
    }

    asCollection(): TimingGroupAsCollection {
      const unnestedChildren = this.children.map((child) =>
        child.unnestFrom(this.overall),
      );
      return new TimingGroupAsCollection(unnestedChildren);
    }
  }

  class BorderBoxTimingCollection extends TimingGroupAsCollection {
    growX(): TransitionTimingMs {
      return this.children[0];
    }

    growY(): TransitionTimingMs {
      return this.children[1];
    }

    delayByMs(delayMs: number): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.scaleBy(factor).children);
    }

    asIndividual(): BorderBoxTiming {
      const groupTimingFraction = super.asIndividual();
      return new BorderBoxTiming({
        overall: groupTimingFraction.overall,
        children: groupTimingFraction.children,
      });
    }
  }

  export function newBorderBoxTimingCollection({
    growX,
    growY,
  }: {
    growX: TransitionTimingMs;
    growY: TransitionTimingMs;
  }): BorderBoxTimingCollection {
    return new BorderBoxTimingCollection([growX, growY]);
  }

  export class BorderBoxTiming extends TimingGroupAsIndividual {
    growX(): TransitionTimingFraction {
      return this.children[0];
    }

    growY(): TransitionTimingFraction {
      return this.children[1];
    }

    asCollection(): BorderBoxTimingCollection {
      const groupTimingMs = super.asCollection();
      return new BorderBoxTimingCollection(groupTimingMs.children);
    }
  }

  class TitleTimingCollection extends TimingGroupAsCollection {
    typewriter(): TransitionTimingMs {
      return this.children[0];
    }

    cursorFade(): TransitionTimingMs {
      return this.children[1];
    }

    delayByMs(delayMs: number): TitleTimingCollection {
      return new TitleTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): TitleTimingCollection {
      return new TitleTimingCollection(super.scaleBy(factor).children);
    }

    asIndividual(): TitleTiming {
      const groupTimingFraction = super.asIndividual();
      return new TitleTiming({
        overall: groupTimingFraction.overall,
        children: groupTimingFraction.children,
      });
    }
  }

  export function newTitleTimingCollection({
    typewriter,
    cursorFade,
  }: {
    typewriter: TransitionTimingMs;
    cursorFade: TransitionTimingMs;
  }): TitleTimingCollection {
    return new TitleTimingCollection([typewriter, cursorFade]);
  }

  export class TitleTiming extends TimingGroupAsIndividual {
    typewriter(): TransitionTimingFraction {
      return this.children[0];
    }

    cursorFade(): TransitionTimingFraction {
      return this.children[1];
    }

    asCollection(): TitleTimingCollection {
      const groupTimingMs = super.asCollection();
      return new TitleTimingCollection(groupTimingMs.children);
    }
  }

  export interface InfoBoxTiming {
    borderBox: BorderBoxTiming;
    title: TitleTiming;
    infoBox: TransitionTimingMs;
  }

  class InfoBoxTimingCollection extends TimingGroupAsCollection {
    borderBox(): BorderBoxTimingCollection {
      return this.children[0] as BorderBoxTimingCollection;
    }

    title(): TitleTimingCollection {
      return this.children[1] as TitleTimingCollection;
    }

    infoBox(): TransitionTimingMs {
      return this.children[2];
    }

    delayByMs(delayMs: number): InfoBoxTimingCollection {
      return new InfoBoxTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): InfoBoxTimingCollection {
      return new InfoBoxTimingCollection(super.scaleBy(factor).children);
    }

    finalize(): InfoBoxTiming {
      return {
        borderBox: this.borderBox().asIndividual(),
        title: this.title().asIndividual(),
        infoBox: this.infoBox(),
      };
    }
  }

  function newInfoBoxTimingCollection({
    borderBox,
    title,
    infoBox,
  }: {
    borderBox: BorderBoxTimingCollection;
    title: TitleTimingCollection;
    infoBox: TransitionTimingMs;
  }) {
    return new InfoBoxTimingCollection([borderBox, title, infoBox]);
  }

  export function getAnimationTiming(
    overallDelayMs: number,
    timingScaleFactor: number,
  ): InfoBoxTiming {
    const growX = new PrimitiveTimingMs({
      startMs: 0,
      endMs: 100,
    });
    const growY = new PrimitiveTimingMs({
      startMs: 80,
      endMs: 200,
    });
    const borderBox = newBorderBoxTimingCollection({ growX, growY });
    const typewriter = new PrimitiveTimingMs({
      // give X a head start
      startMs: growX.startMs() + 20,
      // finish by the time Y is ready to start growing
      endMs: growY.startMs(),
    });
    const cursorFade = new PrimitiveTimingMs({
      // starts as soon as title typing ends
      startMs: typewriter.endMs(),
      // finishes simultaneously with Y
      endMs: growY.endMs(),
    });
    const infoBox = new PrimitiveTimingMs({
      // can start fading in before border box finishes growing completely, so long as
      // border box growth is *mostly* done and already contains the entirety of the
      // info box
      delayMs: growY.endMs() - 50,
      durationMs: 100, // go at same speed as Y advance
    });
    const title = newTitleTimingCollection({ typewriter, cursorFade });
    const infoBoxTimingCollection = newInfoBoxTimingCollection({
      borderBox,
      title,
      infoBox,
    });
    return infoBoxTimingCollection
      .delayByMs(overallDelayMs)
      .scaleBy(timingScaleFactor)
      .finalize();
  }

  class SubAnimation<T> {
    timing: TransitionTimingFraction;
    tick: (tLocalFraction: number) => T;

    constructor(anim: {
      timing: TransitionTimingFraction;
      tick: (tLocalFraction: number) => T;
    }) {
      this.timing = anim.timing;
      this.tick = anim.tick;
    }

    tickForGlobalTime(tGlobalFraction: number): T {
      return this.tick(this.timing.localize(tGlobalFraction));
    }
  }
</script>

<script lang="ts">
  import getComponentId from "./label-id";
  import { cubicInOut, cubicOut } from "svelte/easing";
  import { animationSpeed, animationsOn } from "./preferences";
  import { fade, type TransitionConfig } from "svelte/transition";
  import { firstPageLoad } from "./firstPageLoad";

  export let title = "";
  export let preDelay = 100;
  const infoboxId = getComponentId("infobox");

  class ProperyAnimation extends SubAnimation<string> {
    constructor(anim: {
      timing: TransitionTimingFraction;
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
      super({ timing: anim.timing, tick: css });
    }
  }

  function revealOutline(
    node: Element,
    timing: BorderBoxTiming,
  ): TransitionConfig {
    const actualWidth = node.clientWidth;
    const actualHeight = node.clientHeight;
    const minDimensions = 3 * 18; // 3 rem

    const growWidth = new ProperyAnimation({
      timing: timing.growX(),
      property: "width",
      min: minDimensions,
      max: actualWidth,
      unit: "px",
    });

    const growHeight = new ProperyAnimation({
      timing: timing.growY(),
      property: "height",
      min: minDimensions,
      max: actualHeight,
      unit: "px",
    });

    return {
      delay: timing.overall.delayMs(),
      duration: timing.overall.durationMs(),
      css: (tFraction: number) => {
        const width = growWidth.tickForGlobalTime(tFraction);
        const height = growHeight.tickForGlobalTime(tFraction);
        return width + height;
      },
    };
  }

  class TypewriterEffect extends SubAnimation<void> {
    constructor(anim: { node: Element; timing: TransitionTimingFraction }) {
      const text = anim.node.textContent ?? "";
      const length = text.length + 1;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const i = Math.trunc(length * tLocalFraction);
          anim.node.textContent = i === 0 ? "" : text.slice(0, i - 1);
          if (tLocalFraction == 0) {
            anim.node.classList.remove("typewriting");
          } else {
            anim.node.classList.add("typewriting");
          }
        },
      });
    }
  }

  class FadeCursorEffect extends SubAnimation<void> {
    constructor(anim: { node: Element; timing: TransitionTimingFraction }) {
      const easingFunction = cubicOut;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const opacity = 1 - easingFunction(tLocalFraction);
          anim.node.setAttribute("style", `--cursor-opacity: ${opacity};`);
        },
      });
    }
  }

  function revealTitle(node: Element, timing: TitleTiming): TransitionConfig {
    const typewriter = new TypewriterEffect({
      node,
      timing: timing.typewriter(),
    });
    const cursorFade = new FadeCursorEffect({
      node,
      timing: timing.cursorFade(),
    });

    return {
      delay: timing.overall.delayMs(),
      duration: timing.overall.durationMs(),
      tick: (tGlobalFraction: number) => {
        if (timing.durationMs() === 0) {
          return;
        }
        typewriter.tickForGlobalTime(tGlobalFraction);
        cursorFade.tickForGlobalTime(tGlobalFraction);
      },
    };
  }

  $: shouldAnimate = $animationsOn && $firstPageLoad;
  $: timingScaleFactor = shouldAnimate ? 1 / $animationSpeed : 0;
  $: timing = getAnimationTiming(preDelay, timingScaleFactor);
  $: infoBoxArgs = {
    delay: timing.infoBox.delayMs(),
    duration: timing.infoBox.durationMs(),
  };
</script>

...
  <div class="border-container">
    <div class="border-box" in:revealOutline|global={timing.borderBox}></div>
    <div class="info-box">
      <h2 in:revealTitle|global={timing.title} id={infoboxId}>
        {title}
      </h2>
      <div class="info-content" in:fade|global={infoBoxArgs}>
        <slot />
      </div>
    </div>
  </div>

<style>
  ...

  .info-box h2 {
    --cursor-opacity: 0;
    margin: -0.25rem 0 0.5rem var(--cut);
  }

  .info-box :global(h2.typewriting::after) {
    content: "█";
    opacity: var(--cursor-opacity);
  }
</style>
```

We added a `preDelay` prop to the component to allow us to test the info box animation in isolation in Storybook without having to wait for a page transition effect that doesn't exist. We edit `src-svelte/src/lib/InfoBox.stories.ts` accordingly:

```ts
...

export const MountTransition: StoryObj = Template.bind({}) as any;
MountTransition.args = {
  ...
  preDelay: 0,
};
...

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  ...
  preDelay: 0,
};
...
```

Now we write tests for all this new logic at `src-svelte/src/lib/InfoBox.test.ts`. Note that we first check that our primitives are doing what we expect them to, that timing intervals represented in milliseconds are equivalent under coversion to intervals represented in fractions, and vice versa. Then, we use this confidence in our primitives to make further assertions about entire groups of animations and their timing. Also note that we have moved the content fade-in animation up by 10 ms. We know have the confidence to make changes to our animation structure without worrying about whether we've forgotten to edit some other derived value.

```ts
import {
  getAnimationTiming,
  PrimitiveTimingMs,
  PrimitiveTimingFraction,
  TimingGroupAsCollection,
  TimingGroupAsIndividual,
} from "./InfoBox.svelte";

describe("InfoBox animation timing", () => {
  it("should enable ms timings to be defined in different ways", () => {
    const timingMs1 = new PrimitiveTimingMs({ startMs: 100, endMs: 300 });
    const timingMs2 = new PrimitiveTimingMs({ delayMs: 100, durationMs: 200 });
    expect(timingMs1).toEqual(timingMs2);
  });

  it("should enable fractional timings to be defined in different ways", () => {
    const timingMs1 = new PrimitiveTimingFraction({
      startFraction: 0.2,
      endFraction: 0.7,
    });
    const timingMs2 = new PrimitiveTimingFraction({
      delayFraction: 0.2,
      durationFraction: 0.5,
    });
    expect(timingMs1.round()).toEqual(timingMs2.round());
  });

  it("should nest and unnest timings correctly", () => {
    const timingMs = new PrimitiveTimingMs({ startMs: 200, endMs: 400 });
    const timingFraction = new PrimitiveTimingFraction({
      startFraction: 0.2,
      endFraction: 0.6,
    });
    const overall = new PrimitiveTimingMs({ startMs: 100, endMs: 600 });
    expect(timingMs.nestInside(overall).round()).toEqual(
      timingFraction.round(),
    );
    expect(timingFraction.unnestFrom(overall).round()).toEqual(timingMs);
  });

  it("should correctly combine groups of sub-animations into one", () => {
    const collectionMs = new TimingGroupAsCollection([
      new PrimitiveTimingMs({ startMs: 100, endMs: 400 }),
      new PrimitiveTimingMs({ startMs: 200, endMs: 500 }),
    ]);
    const collectionFraction = new TimingGroupAsIndividual({
      overall: new PrimitiveTimingMs({ startMs: 100, endMs: 500 }),
      children: [
        new PrimitiveTimingFraction({ startFraction: 0.0, endFraction: 0.75 }),
        new PrimitiveTimingFraction({ startFraction: 0.25, endFraction: 1.0 }),
      ],
    });
    expect(collectionMs.asIndividual()).toEqual(collectionFraction);
    expect(collectionFraction.asCollection()).toEqual(collectionMs);
  });

  it("should be the default if no additional scaling or delay", () => {
    const preDelay = 0;
    const timingScaleFactor = 1;
    const timing = getAnimationTiming(preDelay, timingScaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX().startFraction()).toEqual(0.0);
    expect(timing.borderBox.growX().endFraction()).toEqual(0.5);
    expect(timing.borderBox.growY().startFraction()).toEqual(0.4);
    expect(timing.borderBox.growY().endFraction()).toEqual(1.0);
    expect(timing.borderBox.overall.startMs()).toEqual(0);
    expect(timing.borderBox.overall.endMs()).toEqual(200);

    // regular title animation values
    const titleCollectionMs = new TimingGroupAsCollection([
      new PrimitiveTimingMs({ startMs: 20, endMs: 80 }),
      new PrimitiveTimingMs({ startMs: 80, endMs: 200 }),
    ]);
    expect(timing.title.asCollection()).toEqual(titleCollectionMs);

    // regular info box animation values
    expect(timing.infoBox.delayMs()).toEqual(150);
    expect(timing.infoBox.durationMs()).toEqual(100);
  });

  it("should not let delays affect fractions or durations", () => {
    const preDelay = 100;
    const timingScaleFactor = 1;
    const timing = getAnimationTiming(preDelay, timingScaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX().startFraction()).toEqual(0.0);
    expect(timing.borderBox.growX().endFraction()).toEqual(0.5);
    expect(timing.borderBox.growY().startFraction()).toEqual(0.4);
    expect(timing.borderBox.growY().endFraction()).toEqual(1.0);
    expect(timing.borderBox.overall.startMs()).toEqual(100);
    expect(timing.borderBox.overall.endMs()).toEqual(300);

    // regular title animation values
    const titleCollectionMs = new TimingGroupAsCollection([
      new PrimitiveTimingMs({ startMs: 120, endMs: 180 }),
      new PrimitiveTimingMs({ startMs: 180, endMs: 300 }),
    ]);
    expect(timing.title.asCollection()).toEqual(titleCollectionMs);

    // regular info box animation values
    expect(timing.infoBox.delayMs()).toEqual(250);
    expect(timing.infoBox.durationMs()).toEqual(100);
  });

  it("should not let scaling affect fractions", () => {
    const preDelay = 100;
    const timingScaleFactor = 10;
    const timing = getAnimationTiming(preDelay, timingScaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX().startFraction()).toEqual(0.0);
    expect(timing.borderBox.growX().endFraction()).toEqual(0.5);
    expect(timing.borderBox.growY().startFraction()).toEqual(0.4);
    expect(timing.borderBox.growY().endFraction()).toEqual(1.0);
    expect(timing.borderBox.overall.startMs()).toEqual(1000);
    expect(timing.borderBox.overall.endMs()).toEqual(3000);

    // regular title animation values
    const titleCollectionMs = new TimingGroupAsCollection([
      new PrimitiveTimingMs({ startMs: 1200, endMs: 1800 }),
      new PrimitiveTimingMs({ startMs: 1800, endMs: 3000 }),
    ]);
    expect(timing.title.asCollection()).toEqual(titleCollectionMs);

    // regular info box animation values
    expect(timing.infoBox.delayMs()).toEqual(2500);
    expect(timing.infoBox.durationMs()).toEqual(1000);
  });
});

```

Now we can truly experiment in confidence with our timings. We edit `src-svelte/src/routes/PageTransitionView.svelte` to experiment with how a longer title looks and feels:

```svelte
      ...
      <InfoBox title="Reality: Subjective or Objective?">
        ...
      </InfoBox>
      ...
```

and then adjust the timings in `src-svelte/src/lib/InfoBox.svelte` as such:

```ts
  export function getAnimationTiming(
    overallDelayMs: number,
    timingScaleFactor: number,
  ): InfoBoxTiming {
    const growX = new PrimitiveTimingMs({
      startMs: 0,
      durationMs: 200,
    });
    const growY = new PrimitiveTimingMs({
      startMs: growX.endMs() - 20,
      durationMs: 150,
    });
    const borderBox = newBorderBoxTimingCollection({ growX, growY });
    const typewriter = new PrimitiveTimingMs({
      // give X a head start
      startMs: growX.startMs() + 20,
      // but finish at the same time
      endMs: growX.endMs(),
    });
    const cursorFade = new PrimitiveTimingMs({
      // stay for a second after typewriter finishes
      startMs: typewriter.endMs() + 40,
      // finishes simultaneously with Y
      endMs: growY.endMs(),
    });
    const infoBox = new PrimitiveTimingMs({
      // can start fading in before border box finishes growing completely, so long as
      // border box growth is *mostly* done and already contains the entirety of the
      // info box
      delayMs: growY.endMs() - (growY.durationMs() / 3),
      durationMs: 100, // regular speed
    });
```

We add rounding support to all interfaces:

```ts
  export interface TransitionTimingMs {
    ...
    round(): TransitionTimingMs;
    ...
  }

  export interface TransitionTimingFraction {
    ...
    round(): TransitionTimingFraction;
    ...
  }

  ...

  export class TimingGroupAsCollection implements TransitionTimingMs {
    ...
    round(): TimingGroupAsCollection {
      return new TimingGroupAsCollection(
        this.children.map((child) => child.round()),
      );
    }
    ...
  }

  export class TimingGroupAsIndividual implements TransitionTimingMs {
    ...
    round(): TimingGroupAsIndividual {
      return new TimingGroupAsIndividual({
        overall: this.overall.round(),
        children: this.children.map((child) => child.round()),
      });
    }
    ...
  }

  class BorderBoxTimingCollection extends TimingGroupAsCollection {
    ...
    round(): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.round().children);
    }
    ...
  }

  export class BorderBoxTiming extends TimingGroupAsIndividual {
    ...
    round(): BorderBoxTiming {
      const rounded = super.round();
      return new BorderBoxTiming({
        overall: rounded.overall,
        children: rounded.children,
      });
    }
    ...
  }

```

While this turns out to not be needed in the end, we keep it as training data for a demonstration of generalization.

We update our tests in `src-svelte/src/lib/InfoBox.test.ts`:

```ts
  it("should be the default if no additional scaling or delay", () => {
    const preDelay = 0;
    const timingScaleFactor = 1;
    const timing = getAnimationTiming(preDelay, timingScaleFactor);

    // regular border box animation values
    const borderBoxMs = new TimingGroupAsCollection([
      // grow x
      new PrimitiveTimingMs({ startMs: 0, endMs: 200 }),
      // grow y
      new PrimitiveTimingMs({ startMs: 180, endMs: 330 }),
    ]);
    expect(timing.borderBox.asCollection()).toEqual(borderBoxMs);

    // regular title animation values
    const titleCollectionMs = new TimingGroupAsCollection([
      // typewriter
      new PrimitiveTimingMs({ startMs: 20, endMs: 200 }),
      // cursor fade
      new PrimitiveTimingMs({ startMs: 240, endMs: 330 }),
    ]);
    expect(timing.title.asCollection()).toEqual(titleCollectionMs);

    // regular info box animation values
    expect(timing.infoBox).toEqual(
      new PrimitiveTimingMs({
        startMs: 280,
        endMs: 380,
      }),
    );
  });

  it("should not let delays affect fractions or durations", () => {
    const delay = 100;
    const scaleFactor = 1;
    const timing = getAnimationTiming(0, scaleFactor);
    const scaledTiming = getAnimationTiming(delay, scaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX()).toEqual(scaledTiming.borderBox.growX());
    expect(timing.borderBox.growY()).toEqual(scaledTiming.borderBox.growY());
    expect(timing.borderBox.overall.delayMs() + delay).toEqual(
      scaledTiming.borderBox.overall.delayMs(),
    );
    expect(timing.borderBox.overall.durationMs()).toEqual(
      scaledTiming.borderBox.overall.durationMs(),
    );

    // regular title animation values
    expect(timing.title.typewriter()).toEqual(scaledTiming.title.typewriter());
    expect(timing.title.cursorFade()).toEqual(scaledTiming.title.cursorFade());
    expect(timing.title.overall.delayMs() + delay).toEqual(
      scaledTiming.title.overall.delayMs(),
    );
    expect(timing.title.overall.durationMs()).toEqual(
      scaledTiming.title.overall.durationMs(),
    );

    // regular info box animation values
    expect(timing.infoBox.delayMs() + delay).toEqual(
      scaledTiming.infoBox.delayMs(),
    );
    expect(timing.infoBox.durationMs()).toEqual(
      scaledTiming.infoBox.durationMs(),
    );
  });

  it("should not let scaling affect fractions", () => {
    const preDelay = 0;
    const scaleFactor = 10;
    const timing = getAnimationTiming(preDelay, 1);
    const scaledTiming = getAnimationTiming(preDelay, scaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX()).toEqual(scaledTiming.borderBox.growX());
    expect(timing.borderBox.growY()).toEqual(scaledTiming.borderBox.growY());
    expect(timing.borderBox.overall.startMs() * scaleFactor).toEqual(
      scaledTiming.borderBox.overall.startMs(),
    );
    expect(timing.borderBox.overall.endMs() * scaleFactor).toEqual(
      scaledTiming.borderBox.overall.endMs(),
    );

    // regular title animation values
    expect(timing.title.typewriter()).toEqual(scaledTiming.title.typewriter());
    expect(timing.title.cursorFade()).toEqual(scaledTiming.title.cursorFade());
    expect(timing.title.overall.startMs() * scaleFactor).toEqual(
      scaledTiming.title.overall.startMs(),
    );
    expect(timing.title.overall.endMs() * scaleFactor).toEqual(
      scaledTiming.title.overall.endMs(),
    );

    // regular info box animation values
    expect(timing.infoBox.startMs() * 10).toEqual(
      scaledTiming.infoBox.startMs(),
    );
    expect(timing.infoBox.endMs() * 10).toEqual(scaledTiming.infoBox.endMs());
  });
```

Now we try to make a smoother vertical fade by first refactoring `src-svelte/src/lib/InfoBox.svelte` to animate for us with a custom function:

```svelte
<script lang="ts>
  ...
  function revealInfoBox(node: Element, timing: TransitionTimingMs) {
    return {
      delay: timing.delayMs(),
      duration: timing.durationMs(),
      tick: (tGlobalFraction: number) => {
        if (timing.durationMs() === 0) {
          return;
        }
        node.setAttribute("style", `opacity: ${tGlobalFraction};`);
      },
    };
  }

  ...
</script>

...
      <div class="info-content" in:revealInfoBox|global={timing.infoBox}>
...
```

Now we implement the vertical fade by first implementing the inverse of Svelte's cubic in-out function in `src-svelte/src/lib/InfoBox.svelte`:

```ts
<script lang="ts" context="module">
  ...
  export function inverseCubicInOut(t: number) {
    if (t < 0.5) {
        // Solve the cubic equation for t < 0.5
        return Math.cbrt(t / 4.0);
    } else {
        // Solve the cubic equation for t >= 0.5
        return (Math.cbrt(2.0 * (t - 1.0)) + 2.0) / 2.0;
    }
  }
</script>
```

and then testing it in `src-svelte/src/lib/InfoBox.test.ts`:

```ts
import {
  ...
  inverseCubicInOut,
} from "./InfoBox.svelte";
import { cubicInOut } from "svelte/easing";

describe("InfoBox animation timing", () => {
  it("should invert cubic in-out correctly", () => {
    expect(inverseCubicInOut(0)).toEqual(0);
    expect(inverseCubicInOut(0.5)).toEqual(0.5);
    expect(inverseCubicInOut(1)).toEqual(1);
    expect(inverseCubicInOut(cubicInOut(0.25))).toEqual(0.25);
    expect(inverseCubicInOut(cubicInOut(0.75))).toEqual(0.75);
  });

  ...
});
```

Now we implement the rest in `src-svelte/src/lib/InfoBox.svelte`:

```ts
<script lang="ts" context="module">
  ...
    const infoBox = new PrimitiveTimingMs({
      ...
      delayMs: growY.startMs(),
      durationMs: 260, // regular speed
    });
  ...
</script>

<script lang="ts">
  ...
    const growHeight = new ProperyAnimation({
      ...
      easingFunction: cubicInOut,
    });
  ...

  class RevealContent extends SubAnimation<void> {
    constructor(anim: { node: Element; timing: TransitionTimingFraction }) {
      const easingFunction = cubicOut;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const opacity = easingFunction(tLocalFraction);
          anim.node.setAttribute("style", `opacity: ${opacity};`);
        },
      });
    }
  }

  function revealInfoBox(node: Element, timing: InfoBoxTiming) {
    // the items near the bottom can be revealed early instead of waiting for the
    // border box to completely finish growing. This is because the cubic in-out growth
    // feels very slow towards the end, and to wait for this to finish before starting
    // the fade-in makes the fade-in of the last item in particular feel
    // disproportionately slow. Therefore, we cap the "effective" bottom of the node
    // at 70% of the parent's actual height.
    const earlyRevealFraction = 0.3;
    const revealCutoffFraction = 1 - earlyRevealFraction;
    // how much time we have in total to kick off animations:
    // 1. This should actually take the same amount of time as Y takes to grow, 
    //    except that it's slightly delayed to give Y growth a headstart
    // 2. This should leave enough time for the last element to transition
    const totalKickoffMs = timing.borderBox.asCollection().growY().durationMs();
    const theoreticalTotalKickoffFraction = totalKickoffMs / timing.infoBox.durationMs();
    if (theoreticalTotalKickoffFraction > 1) {
      throw new Error("Info box animation is too short to reveal all elements");
    }
    const actualTotalKickoffFraction = theoreticalTotalKickoffFraction * revealCutoffFraction;
    const perElementRevealFraction = 1 - actualTotalKickoffFraction;
    const { height: infoBoxHeight, top: infoBoxTop } = node.getBoundingClientRect();
    const revealAnimations: RevealContent[] = [];

    const getChildKickoffFraction = (child: Element) => {
      const childRect = child.getBoundingClientRect();
      const childBottomYRelativeToInfoBox = childRect.top + childRect.height - infoBoxTop;
      const equivalentYProgress = inverseCubicInOut(childBottomYRelativeToInfoBox / infoBoxHeight);
      const adjustedYProgress = Math.min(revealCutoffFraction, equivalentYProgress);
      const delayFraction = adjustedYProgress * theoreticalTotalKickoffFraction;
      return new PrimitiveTimingFraction({ 
        delayFraction,
        durationFraction: perElementRevealFraction,
      });
    };

    const addNodeAnimations = (currentNode: Element) => {
      // if there are text-only elements that are not part of any node, we fade-in the
      // whole parent at once to avoid the text appearing before anything else -- e.g.
      // if there's something like "some text in <em>some tag</em>", the "some text in"
      // will appear immediately while "some tag" takes a moment to fade in
      if (currentNode.children.length === 0 || (currentNode.children.length === currentNode.childNodes.length)) {
        revealAnimations.push(new RevealContent({
          node: currentNode,
          timing: getChildKickoffFraction(currentNode),
        }));
      } else {
        for (const child of currentNode.children) {
          addNodeAnimations(child);
        }
      }
    }

    addNodeAnimations(node);

    return {
      delay: timing.infoBox.delayMs(),
      duration: timing.infoBox.durationMs(),
      tick: (tGlobalFraction: number) => {
        if (timing.infoBox.durationMs() === 0) {
          return;
        }

        revealAnimations.forEach((anim) => {
          anim.tickForGlobalTime(tGlobalFraction);
        });
      },
    };
  }
  ...
</script>

      ...
      <div class="info-content" in:revealInfoBox|global={timing}>
      ...
```

An explanation of `children` versus `childNodes` can be found [here](https://stackoverflow.com/a/7935719).

After some testing, we find that we should actually change the content fade-in to be linear. Due to the nature of this attribute, values near the extremes are nearly indistinguishable, and time spent at the extremes is not noticed and makes the entire animation feel faster than it actually is.

```ts
  class RevealContent extends SubAnimation<void> {
    ... {
      const easingFunction = linear;
      ...
    }
  }
```

Finally, we once again update the test `src-svelte/src/lib/InfoBox.test.ts` to record that our expected test timings have indeed changed:

```ts
  it("should be the default if no additional scaling or delay", () => {
    ...
    // regular info box animation values
    expect(timing.infoBox).toEqual(
      new PrimitiveTimingMs({
        delayMs: 180,
        durationMs: 260,
      }),
    );
  });
```

##### Refactoring generic timing code

We realize that we should refactor the non-trivial portions of the code referring to generic timing logic into a new file at `src-svelte/src/lib/animation-timing.ts`, along with the corresponding generic tests at `src-svelte/src/lib/animation-timing.test.ts`.

We find that we should move `SubAnimation` and `PropertyAnimation` as well (with the typo fixed from `ProperyAnimation`), and import the new definitions in the old file.

#### Cursor layout

The addition of the cursor pseudo-element with its own content causes the layout to shift. This would not be a problem, except that the layout appears different when navigating to a page for the first time versus the second time.

There appears to be [no way](https://stackoverflow.com/questions/41947434/how-stop-pseudo-elements-affecting-layout) to display a pseudo-element with content without letting it affect layout, at least not if we want to preserve word wrapping.

As such, we fix this by always making the pseudo-element present, even if it is completely invisible. We get rid of the `typewriting` class, which is no longer needed, and set the pseudo-element display to always contain the cursor block:

```css
  .info-box h2::after {
    content: "█";
    opacity: var(--cursor-opacity);
  }
```

We could also manually draw the cursor ourselves by creating a pseudo-element with no content but its own non-zero width and height and background color, as Svelte Typewriter does:

```css
  .info-box h2::after {
    content: "";
    width: 1ex;
    height: 2ex;
    background-color: var(--color-header);
    display: inline-block;
    opacity: var(--cursor-opacity);
  }
```

However, because this still affects the layout due to the `inline-block` display, there is no point to doing this except for perhaps browser compatibility issues.

We now rename the `System Information` box to `System Info` to avoid having a multi-line title with the default app sizing.

#### Multi-line title

When word wrap applies and the title spans multiple lines, the cursor block will run ahead of the width expansion. To avoid this, we'll have to check at runtime whether or not the title element is being wrapped. We first bind the title element to a new variable:

```svelte
<script lang="ts">
  ...
  let titleElement: HTMLElement | undefined;
  ...
</script>

...
      <h2 ... bind:this={titleElement}>
        ...
      </h2>
...
```

Then, when computing the transition effects, we will query for the title height. We notice from console logging observation that the title is at 26px on Chrome when there's only a single line. Firefox renders the title height at 27px. So, anything significantly greater than this will mean it's wrapped. When it's wrapped:

1. We'll have the info box height start at a value large enough to capture the entire vertical width of the title
2. We'll grow the info box width quickly using the cubic out easing function, because otherwise the turbocharged linear growth of the title (2x or even 3x depending on how many lines it's wrapped across) will overtake the info box width

```ts
  function revealOutline(
    node: Element,
    timing: BorderBoxTiming,
  ): TransitionConfig {
    ...
    const heightPerTitleLinePx = 26;
    const titleHeight = (titleElement as HTMLElement).clientHeight;
    // multiply by 1.3 to account for small pixel differences between browsers
    const titleIsMultiline = titleHeight > heightPerTitleLinePx * 1.3;
    const minHeight = titleHeight + heightPerTitleLinePx; // add a little for padding
    const minWidth = 3.5 * heightPerTitleLinePx;

    const growWidth = new ProperyAnimation({
      ...
      min: minWidth,
      ...
      easingFunction: titleIsMultiline ? cubicOut : linear,
    });

    const growHeight = new ProperyAnimation({
      ...
      min: minHeight,
      ...
    });
```

### Staggered initialization

Let's try staggering info box reveal animations so that if there's multiple of them on the same page, they don't all start and end at the exact same time. We edit `src-svelte/src/lib/InfoBox.svelte` to take in this new option:

```ts
  ...
  export let childNumber = 0;
  ...
  const perChildStagger = 100;
  const totalDelay = preDelay + (childNumber * perChildStagger);

  ...
  $: timing = getAnimationTiming(totalDelay, timingScaleFactor);
```

And we have to pass this prop through the other components that use an info box, such as `src-svelte/src/routes/ApiKeysDisplay.svelte`:

```svelte
<InfoBox ... {...$$restProps}>
  ...
</Infobox>
```

and `src-svelte/src/routes/Metadata.svelte`:

```svelte
<InfoBox ... {...$$restProps}>
  ...
</InfoBox>
```

Now we can finally set this attribute in `src-svelte/src/routes/Homepage.svelte` (this requires the Storybook refactor mentioned [here](ui/homepage.md)):

```svelte
<section ...>
  ...
    <Metadata childNumber={0} />
  ...
</section>

<section>
  <ApiKeysDisplay childNumber={1} />
</section>
```

Having done this, it now becomes apparent that the info boxes waiting for their turn to display their transition animation should not be visible until shortly before their display is set to begin. We add a new timing to the info box at `src-svelte/src/lib/InfoBox.svelte`:

```ts
  ...

  export interface InfoBoxTiming {
    ...
    overallFadeIn: TransitionTimingMs;
  }

  class InfoBoxTimingCollection extends TimingGroupAsCollection {
    ...

    overallFadeIn(): TransitionTimingMs {
      return this.children[3];
    }

    ...

    finalize(): InfoBoxTiming {
      return {
        ...
        overallFadeIn: this.overallFadeIn(),
      };
    }
  }

  function newInfoBoxTimingCollection({
    ...
    overallFadeIn,
  }: {
    ...
    overallFadeIn: TransitionTimingMs;
  }) {
    return new InfoBoxTimingCollection([..., overallFadeIn]);
  }
```

and we calculate when this timing should happen, which would be 50ms before all the other animations are set to occur. In other words, whenever the main animations are supposed to play, the info box will start fading in 50ms before that, and finish in time for everything else to start playing -- *unless* there is no delay to speak of in the first place.

```ts
  export function getAnimationTiming(
    ...
  ): InfoBoxTiming {
    ...
    const effectsGroup = new TimingGroupAsCollection([
      borderBox,
      title,
      infoBox,
    ]).delayByMs(overallDelayMs);
    const [delayedBorder, delayedTitle, delayedInfo] = effectsGroup.children;

    const overallFadeIn = new PrimitiveTimingMs({
      startMs: Math.max(0, effectsGroup.startMs() - 50),
      endMs: effectsGroup.startMs(),
    });

    const infoBoxTimingCollection = newInfoBoxTimingCollection({
      borderBox: delayedBorder as BorderBoxTimingCollection,
      title: delayedTitle as TitleTimingCollection,
      infoBox: delayedInfo,
      overallFadeIn,
    });
    return infoBoxTimingCollection.scaleBy(timingScaleFactor).finalize();
  }
```

Now, we actually use this newly computed timing:

```svelte
<script lang="ts">
  ...
  import { fade, type TransitionConfig } from "svelte/transition";
  ...

  $: overallFadeInArgs = {
    delay: timing.overallFadeIn.delayMs(),
    duration: timing.overallFadeIn.durationMs(),
  };
</script>

<section
  ...
  in:fade|global={overallFadeInArgs}
>
  ...
</section>
```
