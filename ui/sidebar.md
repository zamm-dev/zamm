# Sidebar

## Functionality

To get the sidebar to actually work, we have to:

- Mark an icon as selected based on the current route
- Make each icon a link to the corresponding route

To do this, we edit `src-svelte/src/routes/Sidebar.svelte`. We define all the route data so that we don't have to copy-paste, and use [this trick](https://stackoverflow.com/a/65656641) to programmatically render the icons:

```svelte
<script>
  import { page } from "$app/stores";
  ...

  const routes = [
    {
      name: "Home",
      path: "/",
      icon: IconDashboard,
    },
    {
      name: "Chat",
      path: "/chat",
      icon: IconChat,
    },
    {
      name: "Settings",
      path: "/settings",
      icon: IconSettings,
    },
  ];
</script>
```

We then take a look at commit `353ff88`, back when we ran the SvelteKit template generator, to see how they defined the header, and take inspiration from that:

```svelte
<header>
  ...

  <nav>
    {#each routes as route}
      <a
        aria-current={route.path === $page.url.pathname ? 'page' : undefined}
        class="icon"
        title="{route.name}"
        href="{route.path}">
        <svelte:component this="{route.icon}" />
      </a>
    {/each}
  </nav>
</header>
```

We then modify all the `.selected` modifiers to use `.icon[aria-current="page"]` instead.

When we look at Storybook for the sidebar now, we see:

```
ctx[0].url is undefined

hydrate@http://localhost:6006/src/routes/Sidebar.svelte:90:4
create@http://localhost:6006/src/routes/Sidebar.svelte:73:9
create@http://localhost:6006/src/routes/Sidebar.svelte:218:20
create_component@http://localhost:6006/node_modules/.cache/sb-vite/deps/chunk-SDAT3RDT.js?v=ff18f0c2:2103:18
...
```

We see that there is [an issue](https://github.com/storybookjs/builder-vite/issues/321) about the inability to use Svelte `$app` stores in Storybook. The [linked solution](https://github.com/nickbreaton/vitest-svelte-kit/blob/91f48a3/packages/vitest-svelte-kit/src/plugins/kit-module-emulator.ts) involves emulating the SvelteKit module, and is based on a package that is no longer maintained. This appears rather complicated, so we'll try to see if there are any other solutions.

We see [in this semi-official tutorial](https://storybook.js.org/tutorials/intro-to-storybook/svelte/en/data/) that they sidestep the problem of mocking a store altogether by separating the data and presentational elements completely. Given that this is a good idea for code clarity in general, we'll take this approach.

Note also that in the Storybook [announcement](https://storybook.js.org/blog/storybook-for-sveltekit/) for Svelte, it is mentioned that:

> Stores in `$app/stores` are supported out of the box

However, we are using SvelteKit, so this does not apply to us.

We edit `src-svelte/src/routes/Sidebar.svelte` to look like this:

```svelte
<script lang="ts">
  import { page } from "$app/stores";
  import SidebarUi from "./SidebarUI.svelte";

  const currentRoute = $page.url.pathname;
</script>

<SidebarUi {currentRoute} />

```

Then we refactor out `src-svelte/src/routes/SidebarUI.svelte` to look like this:

```svelte
<script lang="ts">
  import IconSettings from "~icons/ion/settings";
  import IconChat from "~icons/ph/chat-dots-fill";
  import IconDashboard from "~icons/material-symbols/monitor-heart";

  const routes: App.Route[] = [
    {
      name: "Home",
      path: "/",
      icon: IconDashboard,
    },
    {
      name: "Chat",
      path: "/chat",
      icon: IconChat,
    },
    {
      name: "Settings",
      path: "/settings",
      icon: IconSettings,
    },
  ];

  export let currentRoute: string;
</script>

<header>
  ...

  <nav>
    {#each routes as route}
      <a
        aria-current={route.path === currentRoute ? 'page' : undefined}
        class="icon"
        title="{route.name}"
        href="{route.path}">
        <svelte:component this="{route.icon}" />
      </a>
    {/each}
  </nav>
</header>

...

```

We will have to update `src-svelte/src/app.d.ts` to define the new type:

```ts
...
import IconSettings from "~icons/ion/settings";

declare global {
  namespace App {
    interface Route {
      name: string;
      path: string;
      icon: typeof IconSettings;
    }
  }

  ...
}
```

By editing this file, you may run into the problem noted with [Eslint](/zamm/resources/tutorials/setup/tools/svelte/eslint.md).

We also rename the accompanying stories file to `src-svelte/src/routes/SidebarUI.stories.ts`. Now in this file, we can test out the effect of highlighting different routes:

```ts
import SidebarUI from "./SidebarUI.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: SidebarUI,
  title: "Navigation/Sidebar",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: SidebarUI,
  props: args,
});

export const DashboardSelected: StoryObj = Template.bind({}) as any;
DashboardSelected.args = {
  currentRoute: "/",
};

export const SettingsSelected: StoryObj = Template.bind({}) as any;
SettingsSelected.args = {
  currentRoute: "/settings",
};

```

The screenshot `src-svelte/screenshots/baseline/navigation/sidebar/settings-selected.png` should have been renamed to `src-svelte/screenshots/baseline/navigation/sidebar/dashboard-selected.png` back when that had been modified.

## Sidebar interactivity

Note that when you click a sidebar icon in Storybook, the browser tries to leave Storybook and actually navigate to that page. In order to manually test sidebar interactivity in the browser, we'll add dummy link functionality to `src-svelte/src/routes/SidebarUI.svelte`:

```svelte
<script lang="ts">
  ...
  export let dummyLinks: boolean = false;

  function routeChangeSimulator(newRoute: App.Route) {
    return (e: MouseEvent) => {
      e.preventDefault();
      currentRoute = newRoute.path;
    };
  }
</script>

<header>
  ...
  <nav>
    {#each routes as route}
      <a
        ...
        href={dummyLinks ? "#" : route.path}
        on:click={dummyLinks ? routeChangeSimulator(route) : undefined}
      >
        ...
      </a>
    {/each}
  </nav>
</header>
```

The `e.preventDefault();` prevents the browser from navigating away from Storybook.

Then in `src-svelte/src/routes/SidebarUI.stories.ts`:

```ts
...

export const DashboardSelected: StoryObj = Template.bind({}) as any;
DashboardSelected.args = {
  ...
  dummyLinks: true,
};

export const SettingsSelected: StoryObj = Template.bind({}) as any;
SettingsSelected.args = {
  ...
  dummyLinks: true,
};
```

Now we can click around and see each icon get highlighted in turn.

## CSS animations

The abrupt nature of the sidebar icon highlighting produces a jarring inconsistency with the rest of the UI. As such, we take inspiration from [this](https://codepen.io/tomhodgins/pen/zGYmor), except that we slide the indicator vertically instead of horiontally. We see that we'll need to stop changing the background of the item itself, but instead have a separate indicator element that slides under the icons, meaning we'll have to also take the z-index into account.

Unlike the example Codepen, we will use JavaScript instead of CSS to set the desired location of the indicator, so that we don't need to edit the CSS as well every time we change how the icons are laid out.

```svelte
<script lang="ts>
  let indicatorPosition: string;

  function setIndicatorPosition(newRoute: string) {
    indicatorPosition = `calc(var(--icons-top-offset) + ${routes.findIndex(r => r.path === newRoute)} * var(--sidebar-icon-size))`;
    return indicatorPosition;
  }

  ...

  $: indicatorPosition = setIndicatorPosition(currentRoute);
</script>

<header>
  ...

  <nav>
    <div class="indicator" style="top: {indicatorPosition};"></div>
    {#each routes as route}
      <a
        ...
        id="nav-{route.name.toLowerCase()}"
        ...
        on:click={routeChangeSimulator(route)}
      >
        ...
      </a>
    {/each}
  </nav>
</header>

<style>
  header {
    --animation-duration: 0.1s;
    --icons-top-offset: 0.75rem;
    --sidebar-left-padding: 0.5rem;
    --sidebar-icon-size: calc(var(--sidebar-width) - var(--sidebar-left-padding));
    padding-top: var(--icons-top-offset);
    ...
  }

  .icon, .indicator {
    width: var(--sidebar-icon-size);
    height: var(--sidebar-icon-size);
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon > :global(:only-child) {
    font-size: calc(0.5 * var(--sidebar-icon-size));
    color: #aaa;
    filter: url(#inset-shadow);
    z-index: 2;
    transition: color var(--animation-duration) ease-in;
  }

  .icon[aria-current="page"] > :global(:only-child) {
    color: #1a75ff;
    filter: url(#inset-shadow-selected);
  }

  .indicator {
    border-top-left-radius: var(--corner-roundness);
    border-bottom-left-radius: var(--corner-roundness);
    position: absolute;
    background-color: var(--color-background);
    box-shadow: 0 var(--shadow-offset) var(--shadow-blur) 0 #ccc;
    z-index: 1;
    transition: top var(--animation-duration) ease-out;
  }

  ...
</style>
```

Note that we replace all `.icon[aria-current="page"]` with `.indicator`. We also have the indicator move with an `ease-out` function so that it moves quickly towards its final position, while the icon color changes as `ease-in` so that it gets off to a slow start with the color change while waiting for the indicator to move into position.

We also move sidebar-specific CSS variables from `src-svelte/src/routes/styles.css` into the only place where they are used, `src-svelte/src/routes/SidebarUI.svelte`.

We see that eslint complains about line length, and prettier doesn't let us split it apart from string interpolation alone, so we rewrite it as:

```ts
  function setIndicatorPosition(newRoute: string) {
    const routeIndex = routes.findIndex((r) => r.path === newRoute);
    indicatorPosition =
      `calc(var(--icons-top-offset) + ` +
      routeIndex +
      `* var(--sidebar-icon-size))`;
    return indicatorPosition;
  }
```

## Sidebar icon update

We see on the actual app, outside of Storybook, that the page contents update but not the sidebar when we navigate to a new page. This is because we need to actually set the prop in a reactive statement, as shown [here](https://stackoverflow.com/a/57895205):

```svelte
<script lang="ts">
  ...

  let currentRoute: string;
  $: currentRoute = $page.url.pathname;
</script>
```

## WebdriverIO E2E tests

We need some way of inspecting what WebdriverIO is showing. We edit `src-svelte/src/routes/SidebarUI.svelte` to add this:

```svelte
    <p id="debug">current={currentRoute}</p>
```

We now log it in `webdriver/test/specs/e2e.test.js`:

```js
  ...

  it("should render the welcome screen correctly", async function () {
    await $("table"); // ensure page loads before taking screenshot
    console.log(await $("#debug").getText());
    ...
  });

  ...
```

We see from the execution logs that the path is indeed empty:

```
...
[0-0] 2023-10-02T03:23:39.023Z INFO webdriver: COMMAND findElement("css selector", "#debug")
[0-0] 2023-10-02T03:23:39.024Z INFO webdriver: [POST] http://localhost:4444/session/ca14bbf1-6260-470e-ba93-77a7eea8a36b/element
[0-0] 2023-10-02T03:23:39.024Z INFO webdriver: DATA { using: 'css selector', value: '#debug' }
[0-0] 2023-10-02T03:23:39.075Z INFO webdriver: RESULT {
[0-0]   'element-6066-11e4-a52e-4f735466cecf': 'node-8661461B-EFC9-49E9-B4DE-72580626B1BF'
[0-0] }
[0-0] 2023-10-02T03:23:39.077Z INFO webdriver: COMMAND getElementText("node-8661461B-EFC9-49E9-B4DE-72580626B1BF")
[0-0] 2023-10-02T03:23:39.078Z INFO webdriver: [GET] http://localhost:4444/session/ca14bbf1-6260-470e-ba93-77a7eea8a36b/element/node-8661461B-EFC9-49E9-B4DE-72580626B1BF/text
[0-0] 2023-10-02T03:23:39.123Z INFO webdriver: RESULT current=
[0-0] current=
...
```

We fix this by editing `src-svelte/src/routes/Sidebar.svelte` to ensure that the current path is never undefined:

```svelte
<script lang="ts">
  ...
  $: currentRoute = $page.url.pathname || "/";
</script>
```

## Lag

You might notice that the sidebar animations lag so much as to be nigh imperceptible on Linux. This appears to be because of performance issues with webkit2gtk on Linux, as noted [here](https://github.com/tauri-apps/tauri/issues/7021) and [here](https://github.com/tauri-apps/tauri/issues/3988).
