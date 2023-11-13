# Homepage

## Storybook

To test out the homepage display in Storybook, we move all the code from `src-svelte/src/routes/+page.svelte` to `src-svelte/src/routes/Homepage.svelte`, and then import the new homepage component in `src-svelte/src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import Homepage from "./Homepage.svelte";
</script>

<Homepage />

```

In the process of setting up a Storybook story, we find that we need yet another mock page transition element if we want `src-svelte/src/lib/__mocks__/MockPageTransitions.svelte`:

```ts
<script lang="ts">
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed} from "$lib/preferences";

  firstAppLoad.set(true);
  firstPageLoad.set(true);
  animationSpeed.set(0.1);
</script>

<div class="storybook-wrapper">
  <PageTransition currentRoute="/storybook-demo">
    <slot />
  </PageTransition>
</div>

<style>
  .storybook-wrapper {
    --base-animation-speed: 0.1;
  }
</style>

```

Now the page will finally animate as if we're navigating to it. We can now create a Storybook story for the homepage at `src-svelte/src/routes/Homepage.stories.ts`:

```ts
import HompageComponent from "./Homepage.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";

export default {
  component: HompageComponent,
  title: "Screens/Homepage",
  argTypes: {},
  decorators: [
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockPageTransitions,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: HompageComponent,
  props: args,
});

export const Homepage: StoryObj = Template.bind({}) as any;
Homepage.parameters = {
  resolution: {
    openai: null,
  },
};
```

## Renaming to Dashboard

Upon realizing that this was already called the "Dashboard" for Storybook purposes, we rename both files and their imports.
