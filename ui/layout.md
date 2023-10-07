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
