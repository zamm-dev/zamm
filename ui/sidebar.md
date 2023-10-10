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

## Sound

We compare the whooshing sound from [here](https://pixabay.com/sound-effects/whoosh-6316/) and [here](https://pixabay.com/sound-effects/quick-swhooshing-noise-80898/), and choose the latter for its snappiness. We convert it to ogg following the instructions [here](https://superuser.com/a/584177):

```bash
$ ffmpeg -i quick-swhooshing-noise-80898.mp3 -c:a libvorbis -q:a 4 whoosh.ogg
```

It sounds too high-pitched for us, so we take it down from F to D in Audacity as described [here](https://www.businessinsider.com/guides/tech/how-to-change-pitch-in-audacity), and lower the volume by 8dB as described [here](https://www.techsoup.ca/community/blog/getting-good-audio-on-a-budget-how-to-edit-sound).

We update `src-tauri/src/commands/sounds.rs` to include this new sound:

```rust
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Sound {
    Switch,
    Whoosh,
}

...

fn play_sound_async(sound: Sound) -> ZammResult<()> {
    ...
    let embedded_sound: &[u8] = match sound {
        Sound::Switch => include_bytes!("../../sounds/switch.ogg"),
        Sound::Whoosh => include_bytes!("../../sounds/whoosh.ogg"),
    };
    ...
}
```

Note that we had to change `embedded_sound` to be of type `&[u8]`, or else the error

```
error[E0308]: `match` arms have incompatible types
  --> src/commands/sounds.rs:33:26
   |
31 |       let embedded_sound = match sound {
   |  __________________________-
32 | |         Sound::Switch => include_bytes!("../../sounds/switch.ogg"),
   | |                          ----------------------------------------- this is found to be of type `&[u8; 6409]`
33 | |         Sound::Whoosh => include_bytes!("../../sounds/whoosh.ogg"),
   | |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 6409 elements, found one with 10075 elements
34 | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)
```

appears because the different sound files have different lengths.

Following the rest of the file, we add a test for this new API call in `src-tauri/src/commands/sounds.rs`:

```rust

```rust
    #[test]
    fn test_play_whoosh() {
        check_play_sound_sample("./api/sample-calls/play_sound-whoosh.yaml");
    }
```

where `src-tauri/api/sample-calls/play_sound-whoosh.yaml` looks like:

```yaml
request:
  - play_sound
  - >
    {
      "sound": "Whoosh"
    }
response: "null"

```

We see that Specta has automatically updated `src-svelte/src/lib/bindings.ts` to have

```ts
export type Sound = "Switch" | "Whoosh"
```

We edit `src-svelte/src/routes/SidebarUI.svelte` accordingly:

```ts
<script lang="ts">
  ...
  import { playSound } from "$lib/bindings";
  import { soundOn } from "../preferences";

  ...

  function playWhooshSound() {
    if ($soundOn) {
      playSound("Whoosh");
    }
  }

  function routeChangeSimulator(newRoute: App.Route) {
    return (e: MouseEvent) => {
      e.preventDefault();
      if (newRoute.path !== currentRoute) {
        playWhooshSound();
      }
      if (dummyLinks) {
        currentRoute = newRoute.path;
      }
    };
  }

  ...
</script>

<header>
  ...
  <nav>
    ...
    {#each routes as route}
      <a
        ...
        on:click={routeChangeSimulator(route)}
      >
        ...
      </a>
    {/each}
  </nav>
</header>
```

Now we test it like we did with the switch by creating `src-svelte/src/routes/SidebarUI.test.ts`:

```ts
import { expect, test, vi, type SpyInstance } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import SidebarUI from "./SidebarUI.svelte";
import { soundOn } from "../preferences";
import fs from "fs";
import yaml from "js-yaml";
import { Convert, type SampleCall } from "$lib/sample-call";

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);


describe("Sidebar", () => {
  let whooshCall: SampleCall;
  let whooshRequest: (string | Record<string, string>)[];
  let spy: SpyInstance;
  let homeLink: HTMLElement;
  let settingsLink: HTMLElement;

  beforeAll(() => {
    const sample_call_yaml = fs.readFileSync(
      "../src-tauri/api/sample-calls/play_sound-whoosh.yaml",
      "utf-8",
    );
    const sample_call_json = JSON.stringify(yaml.load(sample_call_yaml));
    whooshCall = Convert.toSampleCall(sample_call_json);
    whooshRequest = whooshCall.request;
    whooshRequest[1] = JSON.parse(whooshCall.request[1]);
  });

  beforeEach(() => {
    spy = vi.spyOn(window, "__TAURI_INVOKE__");
    const response = JSON.parse(whooshCall.response);
    tauriInvokeMock.mockResolvedValueOnce(response);

    render(SidebarUI, {
      currentRoute: "/",
      dummyLinks: true,
    });
    homeLink = screen.getByTitle("Home");
    settingsLink = screen.getByTitle("Settings");
    expect(homeLink).toHaveAttribute("aria-current", "page");
    expect(settingsLink).not.toHaveAttribute("aria-current", "page");
    expect(spy).not.toHaveBeenCalled();
  });

  afterEach(() => {
    vi.clearAllMocks();
  })

  test("can change page path", async () => {
    await act(() => userEvent.click(settingsLink));
    expect(homeLink).not.toHaveAttribute("aria-current", "page");
    expect(settingsLink).toHaveAttribute("aria-current", "page");
  });

  test("plays whoosh sound during page path change", async () => {
    await act(() => userEvent.click(settingsLink));
    expect(spy).toHaveBeenLastCalledWith(...whooshRequest);
  });

  test("does not play whoosh sound when sound off", async () => {
    soundOn.update(() => false);

    await act(() => userEvent.click(settingsLink));
    expect(homeLink).not.toHaveAttribute("aria-current", "page");
    expect(settingsLink).toHaveAttribute("aria-current", "page");
    expect(spy).not.toHaveBeenCalled();
  });

  test("does not play whoosh sound when path unchanged", async () => {
    await act(() => userEvent.click(homeLink));
    expect(homeLink).toHaveAttribute("aria-current", "page");
    expect(settingsLink).not.toHaveAttribute("aria-current", "page");
    expect(spy).not.toHaveBeenCalled();
  });
});

```

At first, when we try importing from `Sidebar.svelte` instead to also test the page change functionality, we see the error

```
 FAIL  src/routes/Sidebar.test.ts [ src/routes/Sidebar.test.ts ]
Error: Failed to resolve import "$app/stores" from "src/routes/Sidebar.svelte". Does the file exist?
```

We're running into the same old problem encountered above. If we try to edit `src-svelte/vitest.config.ts` to make it consistent with `vite.config.ts`:

```ts
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
  plugins: [
    sveltekit(),
    ...
  ],
  ...
});
```

then we get

```
 FAIL  src/routes/Sidebar.test.ts [ src/routes/Sidebar.test.ts ]
Error: Cannot find package '__sveltekit' imported from /root/zamm/node_modules/@sveltejs/kit/src/runtime/app/environment.js

Serialized Error: { code: 'ERR_MODULE_NOT_FOUND' }
```

This appears to be a problem that has been mentioned [here](https://github.com/sveltejs/kit/issues/9162) and [here](https://github.com/vitest-dev/vitest/issues/3483) and fixed already. However, in our attempt to create a working minimal repro, we run into the store subscription issue mentioned [here](/zamm/resources/tutorials/setup/tauri/vitest.md). We try to use a modified version of the [workaround](https://github.com/sveltejs/kit/issues/5525#issuecomment-1186390654) mentioned there by creating `src-svelte/src/vitest-mocks/stores.ts`:

```ts
import { readable, writable } from 'svelte/store';
import type { Subscriber } from 'svelte/store';

interface Page {
  url: URL;
  params: Record<string, string>;
}

const getStores = () => ({
  navigating: readable(null),
  page: readable({ url: new URL('http://localhost'), params: {} }),
  session: writable(null),
  updated: readable(false)
});

export const page = {
  subscribe(fn: Subscriber<Page>) {
    return getStores().page.subscribe(fn);
  }
};

```

and editing `src-svelte/vitest.config.ts` to point to this resolution:

```ts
export default defineConfig({
  ...
  resolve: {
    alias: {
      ...
      $app: path.resolve("src/vitest-mocks"),
    },
  },
});

```

However, this still doesn't actually simulate the store being updated with the correct path. As such, we revert to importing `SidebarUI` instead to simply mock the values.
