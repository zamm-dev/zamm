# Transitions

## Between pages

We edit `src-svelte/src/lib/InfoBox.svelte` to add in a basic in-out transition that depends on the overall animation speed of the app:

```svelte
<script lang="ts">
  ...
  import { fly } from 'svelte/transition';
  import { animationSpeed } from "$lib/preferences";

  ...
  let transitionSpeedMs: number;
  $: transitionSpeedMs = 100 / $animationSpeed;
</script>

<section class="container" aria-labelledby={infoboxId} transition:fly={{ x: "-20%", duration: transitionSpeedMs }}>
  ...
</section>

```

We can change the base animation speed for this component specifically to taste.

To then test these *transitions* specifically in Storybook, create a new decorator `src-svelte/src/lib/__mocks__/MockTransitions.svelte`:

```svelte
<script lang="ts">
  let visible = true;

  function toggleVisibility() {
    visible = !visible;
  }
</script>

<div class="storybook-wrapper">
  <button class="visibility-toggle" on:click={toggleVisibility}>Toggle visibility</button>
  {#if visible}
    <slot />
  {/if}
</div>

<style>
  .storybook-wrapper {
    --base-animation-speed: 0.1;
  }

  .visibility-toggle {
    margin-bottom: 1rem;
  }
</style>

```

Note that because we'll always have to hide and then show the component again to trigger a transition, we might as well do both with a single click of the button:

```svelte
<script lang="ts">
  ...

  function toggleVisibility() {
    visible = !visible;

    setTimeout(() => {
      visible = !visible;
    }, 100);
  }
</script>

...
  <button ...>Remount</button>
...
```

Unfortunately, when we test this on the app, we find that navigating between pages causes the contents of both pages to be laid out simulatenously without overlap. We see that in the future, we'll perhaps be able to use the [View Transitions API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API) to handle this case, but for now we'll have to use other means. We follow the instructions [here](https://joshcollinsworth.com/blog/sveltekit-page-transitions) to create `src-svelte/src/routes/PageTransition.svelte`:

```svelte
<script lang="ts">
  import { page } from "$app/stores";
  import { fly } from 'svelte/transition';
  import { cubicIn, cubicOut } from 'svelte/easing';
  import { animationSpeed } from "$lib/preferences";

  let transitionDuration: number;
  let transitionDelay: number;
  let currentRoute: string;
  $: transitionDuration = 50 / $animationSpeed;
  $: transitionDelay = 60 / $animationSpeed;
  $: currentRoute = $page.url?.pathname || "/";
</script>

{#key currentRoute}
  <div
    out:fly={{ x: "-20%", duration: transitionDuration, easing: cubicIn }}
    in:fly={{ x: "-20%", duration: transitionDuration, easing: cubicOut, delay: transitionDelay }}
  >
    <slot />
  </div>
{/key}

```

and then wrap everything in it in `src-svelte/src/routes/+layout.svelte`:

```svelte
<script>
  ...
  import PageTransition from "./PageTransition.svelte";
  ...
</script>

<AppLayout>
  <PageTransition>
    <slot />
  </PageTransition>
</AppLayout>

```

We finally remove the previous changes to `src-svelte/src/lib/InfoBox.svelte`.

We find that this creates the desired animation effect... but instantly replaces the old content with the new one, thus making it look as if the same content is fading in and out. We find from [this page](https://joyofcode.xyz/sveltekit-page-transitions) that using the `page` store doesn't work, so we create `src-svelte/src/routes/+layout.ts` as originally directed to:

```ts
export const prerender = true;
export const ssr = false;

export function load({ url }) {
  return {
    url: url.pathname,
  }
}

```

and now edit `src-svelte/src/routes/PageTransition.svelte` to take in the currentRoute instead of retrieving it ourselves:

```ts
  export let currentRoute: string;
```

and edit `src-svelte/src/routes/+layout.svelte` to pass in the new attribute:

```svelte
<script>
  import AppLayout from "./AppLayout.svelte";
  import PageTransition from "./PageTransition.svelte";
  import "./styles.css";

  export let data;
</script>

<AppLayout>
  <PageTransition currentRoute={data.url}>
    <slot />
  </PageTransition>
</AppLayout>

```

Now it works as expected except for the vertical scrollbars that appear temporarily when both of them are displayed simultaneously. We lay them on top of each other instead by editing `src-svelte/src/routes/PageTransition.svelte` to add a `transition-container` class to the div:

```css
  .transition-container {
    position: absolute;
  }

```

We notice that the settings page is now squished because the settings container only takes up half the space. We move the `1 rem` padding for `main` from `src-svelte/src/routes/AppLayout.svelte` to `PageTransition.svelte`:

```css
  .transition-container {
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
  }

```

Now everything is looking as it was before, with no superfluous scroll bars appearing temporarily just to disappear again.

The timing looks a bit off to us. We would like to control this whole animation sequence based on two variables of our own choosing, the total duration taken and the delay between the two animations. We edit `src-svelte/src/routes/PageTransition.svelte` to calculate animation attributes based on these variables:

```svelte
<script lang="ts" context="module">
  interface TransitionTiming {
    duration: number;
    delay?: number;
  }

  interface Transition extends TransitionTiming {
    x: string;
    easing: (t: number) => number;
  }

  interface Transitions {
    out: Transition;
    in: Transition;
  }

  export function getTransitionTiming(totalDurationMs: number, spacingFraction: number): TransitionTiming {
    // let
    //   d = duration of a single transition
    //   s = spacing between transitions as fraction of d
    //   and T = total duration of entire transition,
    // then
    //   d + (d * (1 + spacing)) = T
    //   d * (2 + spacing) = T
    //   d = T / (2 + spacing)
    const transitionDurationMs = totalDurationMs / (2 + spacingFraction);
    const transitionDelayMs = transitionDurationMs * (1 + spacingFraction);
    return {
      duration: Math.round(transitionDurationMs),
      delay: Math.round(transitionDelayMs),
    };
  }

  export function getTransitions(totalDurationMs: number, spacingFraction: number): Transitions {
    const {duration, delay} = getTransitionTiming(totalDurationMs, spacingFraction);
    const commonalities = { x: "-20%", duration };
    return {
      out: {
        ...commonalities,
        easing: cubicIn,
      },
      in: {
        ...commonalities,
        easing: cubicOut,
        delay,
      },
    };
  }
</script>

<script lang="ts">
  import { fly } from "svelte/transition";
  import { cubicIn, cubicOut } from "svelte/easing";
  import { animationSpeed } from "$lib/preferences";

  export let currentRoute: string;

  // same speed as sidebar UI
  $: totalDurationMs = 100 / $animationSpeed;
  $: transitions = getTransitions(totalDurationMs, 0.2);
</script>

{#key currentRoute}
  <div class="transition-container" in:fly={transitions.in} out:fly={transitions.out}>
    <slot />
  </div>
{/key}

...
```

We add `src-svelte/src/routes/PageTransition.test.ts` to test:

```ts
import { getTransitionTiming } from "./PageTransition.svelte";

describe("PageTransition", () => {
  it("should halve the duration if no overlap", () => {
    const totalTime = 100;
    const spacingFraction = 0;
    const expectedDuration = 50;
    const expectedDelay = 50;
    // check that our test is doing the math right
    // both durations will have the same length, so the total time is the time delay
    // before the second one starts plus the length of the second one
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    // check that our function is doing the math right
    expect(getTransitionTiming(totalTime, spacingFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });

  it("should increase delay if positive overlap", () => {
    const totalTime = 220;
    const spacingFraction = 0.2;
    const expectedDuration = 100;
    const expectedDelay = 120;
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    expect(getTransitionTiming(totalTime, spacingFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });

  it("should increase duration if negative overlap", () => {
    const totalTime = 180;
    const spacingFraction = -0.2;
    const expectedDuration = 100;
    const expectedDelay = 80;
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    expect(getTransitionTiming(totalTime, spacingFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });
});

```

Further experimentation reveals that it's more natural to control the "overlap" rather than the "spacing" between the two transitions, so we change the function argument:

```ts
  export function getTransitionTiming(
    totalDurationMs: number,
    overlapFraction: number,
  ): TransitionTiming {
    const spacingFraction = -overlapFraction;
    ...
  }

  export function getTransitions(
    totalDurationMs: number,
    overlapFraction: number,
  ): Transitions {
    const { duration, delay } = getTransitionTiming(
      totalDurationMs,
      overlapFraction,
    );
    ...
  }
```

and change the tests, for example

```ts
  it("should increase delay if negative overlap", () => {
    const totalTime = 220;
    const overlapFraction = -0.2;
    ...
    expect(getTransitionTiming(totalTime, overlapFraction)).toEqual({
      ...
    });
  });
```

We find that the page transitions *feel* a lot faster than the snappy switch toggle, even though both technically take the same amount of time. We need to iterate quickly by making it easy to try out different timing and easing functions, so we create `src-svelte/src/routes/PageTransitionView.svelte`:

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import PageTransition from "./PageTransition.svelte";

  let routeA = true;

  function toggleRoute() {
    routeA = !routeA;
  }
  
  $: currentRoute = routeA ? "/a.html" : "/b.html";
</script>

<div class="storybook-wrapper">
  <button class="route-toggle" on:click={toggleRoute}
    >Toggle route</button
  >
  <PageTransition {currentRoute} {...$$restProps}>
    {#if routeA}
      <InfoBox title="Simulation">
        <p>
          How do we know that even the realest of realities wouldn't be subjective, in
          the final analysis? Nobody can prove his existence, can he? &mdash; <em
            >Simulacron 3</em
          >
        </p>
      </InfoBox>
    {:else}
      <InfoBox title="Reality">
        <SubInfoBox subheading="Stuart Candy">
          <p>It is better to be surprised by a simulation, rather than blindsided by reality.</p>
        </SubInfoBox>

        <SubInfoBox subheading="Jean Baudrillard">
          <p>It is no longer a question of a false representation of reality (ideology) but of concealing the fact that the real is no longer real, and thus of saving the reality principle.</p>

          <p>And once freed from reality, we can produce the 'realer than real' - hyperrealism.</p>
        </SubInfoBox>
      </InfoBox>
    {/if}
  </PageTransition>
</div>

<style>
  .storybook-wrapper {
    width: 100%;
    box-sizing: border-box;
    position: relative;
  }

  .route-toggle {
    margin-bottom: 1rem;
  }
</style>

```

Then we create the stories at `src-svelte/src/routes/PageTransition.stories.ts`:

```ts
import PageTransitionView from "./PageTransitionView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: PageTransitionView,
  title: "Layout/Page Transitions",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: PageTransitionView,
  props: args,
});

export const Default: StoryObj = Template.bind({}) as any;

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};

```

After some experimentation, we land on the following adjustments:

 in `src-svelte/src/routes/PageTransition.svelte`:

```ts
  export function getTransitions(
    ...
  ): Transitions {
    ...
    const out = { x: "-20%", duration, easing: cubicIn };
    return {
      out,
      in: { ...out, delay, easing: backOut },
    };
  }

  ...

  // twice the speed of sidebar UI slider
  $: totalDurationMs = 200 / $animationSpeed;
  $: transitions = getTransitions(totalDurationMs, 0);
```

and in `src-svelte/src/routes/SidebarUI.svelte`, we delay the color transition for the selected item to match the incoming part of the page transition:

```css
  .icon > :global(:only-child) {
    ...
    transition: color var(--animation-duration) ease-out;
  }

  .icon[aria-current="page"] > :global(:only-child) {
    ...
    transition-delay: var(--animation-duration);
  }
```

In `src-svelte/src/routes/AppLayout.svelte`, we prevent the page from creating a temporary horizontal scrollbar during the transition by changing `overflow: scroll;` to:

```css
  .main-container {
    overflow-x: hidden;
    overflow-y: scroll;
  }
```

Because we have a setting to turn animations on or off, we should also respect this setting. We first build the test scaffolding. In `src-svelte/src/lib/__mocks__/stores.ts`:

```ts
import {
  animationsOn,
  ...
} from "$lib/preferences";

interface Preferences {
  animationsOn?: boolean;
  ...
}

...

const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  ...

  if (preferences?.animationsOn === undefined) {
    animationsOn.set(true);
  } else {
    animationsOn.set(preferences.animationsOn);
  }

  ...
};
```

Then in `src-svelte/src/routes/PageTransition.stories.ts`:

```ts
export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.parameters = {
  preferences: {
    animationsOn: false,
    animationSpeed: 0.1,
  },
};
```

and in `src-svelte/src/routes/PageTransitionView.svelte`, we mock the app layout functionality:

```svelte
<div class="storybook-wrapper" class:animations-disabled={!$animationsOn}>
  ...
</div>

<style>
  ...

  .animations-disabled :global(*) {
    animation-play-state: paused !important;
    transition: none !important;
  }

  ...
</style>
```

Finally, because there [doesn't](https://github.com/carbon-design-system/carbon-components-svelte/issues/686#issuecomment-876031440) appear to be a way to dynamically modify Svelte transitions, we simply set the animation duration to 0 in `src-svelte/src/routes/PageTransition.svelte`:

```ts
  import { animationsOn, animationSpeed } from "$lib/preferences";

  ...

  $: totalDurationMs = $animationsOn ? 200 / $animationSpeed : 0;
  $: transitions = getTransitions(totalDurationMs, 0);
```

If `totalDurationMs` is recording 2000 at every turn, it is likely because you forgot the `$` in front of `animationsOn`.

### Animating on first mount

Once we add info box animations, we notice that the animations don't run when the page is first loaded, but they do run when we navigate to another page and back to the first page, as if we've never visited the first page. We could decide to stomp this out entirely, or else we could animate the app on the first load as well.

It turns out that in order to get Svelte to perform transitions and animations on [initial page load](https://stackoverflow.com/a/64444463), we'll have to avoid rendering the content until the page is loaded. We edit `src-svelte/src/routes/PageTransition.svelte` accordingly:

```svelte
<script lang="ts">
  import { onMount, tick } from "svelte";
  ...
  let ready = false;

  onMount(async () => {
    ready = true;
    await tick();
    checkFirstPageLoad(currentRoute);
  });

  function checkFirstPageLoad(route: string) {
    if (!ready) {
      return;
    }

    if (visitedKeys.has(route)) {
      firstPageLoad.set(false);
    } else {
      visitedKeys.add(route);
      firstPageLoad.set(true);
    }
  }

  ...
</script>

{#key currentRoute}
  {#if ready}
    <div
      class="transition-container"
      in:fly={transitions.in}
      out:fly={transitions.out}
    >
      <slot />
    </div>
  {/if}
{/key}

```

The transitions don't seem to play well with both `key` and `if`. After some debugging, we realize that this is because we now need to add `|global` to both `fly` effects:

```svelte
    <div
      ...
      in:fly|global={transitions.in}
      out:fly|global={transitions.out}
    >
      ...
    </div>
```

Next, we notice that the animation on first load is delayed, as if it's waiting for the transition out to finish. Since it's the first load, there's no transition out to speak of, and we can being the transition in immediately. We edit the transition duration accordingly:

```ts
  onMount(async () => {
    const regularDelay = transitions.in.delay;
    transitions.in.delay = 0;
    ready = true;
    await tick();
    transitions.in.delay = regularDelay;
    ...
  });
```

We await the tick here to ensure that the non-delayed animation plays first before we reset the delay to its longer value, or else the animation will play using the most up-to-date value of `transitions.in.delay`.

Now we notice that the info box is still delayed. As such, we add a new variable to determine whether it's the first page load or not. We add this to `src-svelte/src/lib/firstPageLoad.ts`:

```ts
export const firstAppLoad = writable(true);
```

Then in `src-svelte/src/lib/InfoBox.svelte`:

```ts
  import { firstAppLoad, firstPageLoad } from "./firstPageLoad";

  ...
  export let preDelay = $firstAppLoad ? 0 : 100;
```

We now set this in `src-svelte/src/routes/PageTransition.svelte` after the initial mount:

```ts
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";

  ...

  onMount(async () => {
    ...
    transitions.in.delay = regularDelay;
    firstAppLoad.set(false);
    ...
  });
```

Finally, we edit `src-svelte/src/lib/__mocks__/stores.ts` to make it easier to test components by remounting in Storybook rather than having to refresh the entire page:

```ts
import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";

...

const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  ...

  // set to their defaults on first load
  firstAppLoad.set(true);
  firstPageLoad.set(true);

  ...
};
```
