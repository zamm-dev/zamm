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
