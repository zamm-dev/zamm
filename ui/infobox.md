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

Note the overlap, where the height starts growing before the width stops completely.

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

### AnimeJS

We find out that it is [possible](https://dev.to/manyeya/custom-transitions-and-staggered-transitions-in-svelte-with-animejs-plm) to use AnimeJS with Svelte. We leave this for the future.
