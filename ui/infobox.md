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

## Transitions

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
