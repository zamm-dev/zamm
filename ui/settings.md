# Settings page

We create a new top-level page. In this case, the link to it is already in `src-svelte/src/routes/Sidebar.svelte`, so we don't need to do anything there. Instead, we create `src-svelte/src/routes/settings/+page.svelte`:

```svelte
<script lang="ts">
  import Settings from "./Settings.svelte";
</script>

<Settings />

```

It is relatively barebones because we want to be able to display most of the page contents in Storybook. We then create the imported file `src-svelte/src/routes/settings/Settings.svelte`:

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SettingsSwitch from "./SettingsSwitch.svelte";
</script>

<InfoBox title="Settings">
  <div class="container">
    <SettingsSwitch label="Unceasing animations" />
    <SettingsSwitch label="Sounds" toggledOn={true} />
  </div>
</InfoBox>

<style>
  .container {
    --side-padding: 0.8rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.1rem;
    margin: 0 calc(-1 * var(--side-padding)) 0.5rem;
  }

  /* this takes sidebar width into account */
  @media (min-width: 52rem) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>

```

As noted [here](https://css-tricks.com/equal-columns-with-flexbox-its-more-complicated-than-you-might-think/), it's probably better to use CSS grids for equally spaced grid layouts, rathern than flexbox because that's better suited for flexible layouts.

The media query is so that when the page gets wide enough, we can display two columns of switches rather than one. The `--side-padding`, as we will see and as the name implies, controls the spacing on the sides between the switches. However, we don't want much spacing between the switches and the edge of the parent container, so we set `margin` to a negative value.

We add the same media query to `src-svelte/src/lib/Switch.svelte`:

```css
  @media (min-width: 52rem) {
    label {
      white-space: nowrap;
    }
  }
```

This is so that we avoid the situation where labels wrap instead of forcing the entire switch element to elongate. This is mostly really only useful if we do an alternative layout option like flexbox.

We also create `src-svelte/src/routes/settings/SettingsSwitch.svelte`:

```svelte
<script lang="ts">
  import Switch from "$lib/Switch.svelte";

  export let label: string;
  export let toggledOn = false;
</script>

<div class="container">
  <Switch label={label} toggledOn={toggledOn} />
</div>

<style>
  .container {
    padding: calc(0.5 * var(--side-padding)) var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
  }

  .container:hover {
    background: hsla(60, 100%, 50%, 0.20);
  }
</style>

```

This is just a wrapper around our existing switch element that adds some padding and a highlight when the user mouses over. We use a wrapper because we are as of yet unsure how likely this particular pattern is to spread, and it will be easier to merge two disparate files together than to refactor one out into two.

Finally, we add this page to Storybook at `src-svelte/src/routes/settings/Settings.stories.ts`. We set a different default viewport for each story so that we can see how it renders across different screen sizes:

```ts
import SettingsComponent from "./Settings.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: SettingsComponent,
  title: "Screens/Settings",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: SettingsComponent,
  props: args,
});

export const TinyPhoneScreen: StoryObj = Template.bind({}) as any;
TinyPhoneScreen.parameters = {
  viewport: {
      defaultViewport: "mobile1"
  }
}

export const LargePhoneScreen: StoryObj = Template.bind({}) as any;
LargePhoneScreen.parameters = {
  viewport: {
      defaultViewport: "mobile2"
  }
}

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
      defaultViewport: "tablet"
  }
}

```

Finally, we add a new entry to `src-svelte/src/routes/storybook.test.ts` to ensure that we keep track of how this page renders from now on:

```ts
...

const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "settings"],
    variants: ["tiny-phone-screen", "large-phone-screen", "tablet"],
    screenshotEntireBody: true,
  },
];

...
```

Check that the tests now pass.

## WebdriverIO E2E testing

Edit `webdriver/test/specs/e2e.test.js` to add this:

```js
...

async function findAndClick(selector, timeout) {
  const button = await $(selector);
  await button.waitForClickable({
    timeout,
  });
  await browser.execute("arguments[0].click();", button);
}

describe("Welcome screen", function () {
  ...

  it("should allow navigation to the settings page", async function () {
    findAndClick('a[title="Settings"]');
    findAndClick("aria/Sounds");
    await browser.pause(500); // for CSS transitions to finish
    expect(
      await browser.checkFullPageScreen("settings-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });
});
```
