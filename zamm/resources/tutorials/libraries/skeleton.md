# Setting up Skeleton

Skeleton is a UI framework for SvelteKit. We follow the docs [here](https://www.skeleton.dev/docs/get-started):

```bash
$ yarn add -D @skeletonlabs/skeleton @skeletonlabs/tw-plugin
$ npx svelte-add@latest tailwindcss
$ yarn
```

You may want to add `csstools.postcss` to `.vscode/extensions.json`.

Convert `src-svelte/tailwind.config.cjs` to TypeScript.

Note that the file `src-svelte/postcss.config.cjs` cannot be converted to TypeScript, or else the error

```
yarn tauri dev
yarn run v1.22.19
$ tauri dev
     Running BeforeDevCommand (`yarn workspace gui dev`)
$ vite

[Failed to load PostCSS config: Failed to load PostCSS config (searchPath: /root/zamm/src-svelte): [Error] Must use import to load ES Module: /root/zamm/src-svelte/postcss.config.ts
require() of ES modules is not supported.
require() of /root/zamm/src-svelte/postcss.config.ts from /root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js is an ES module file as it is a .ts file whose nearest parent package.json contains "type": "module" which defines all .ts files in that package scope as ES modules.
Instead change the requiring code to use import(), or remove "type": "module" from /root/zamm/src-svelte/package.json.
...
```

appears.


Now edit `src-svelte/tailwind.config.ts` from:

```ts
import {Config} from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {},
  },

  plugins: [],
};

module.exports = config;

```

to:

```ts
import { join } from "path";
import { Config } from "tailwindcss";
import { skeleton } from "@skeletonlabs/tw-plugin";

const config: Config = {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(
      require.resolve("@skeletonlabs/skeleton"),
      "../**/*.{html,js,svelte,ts}",
    ),
  ],

  theme: {
    extend: {},
  },

  plugins: [
    skeleton({
      themes: { preset: ["gold-nouveau"] },
    }),
  ],
};

module.exports = config;
```

Edit `src-svelte/src/app.html` as instructed:

```html
<!doctype html>
<html lang="en">
  ...
  <body>
    <div
      style="display: contents"
      class="h-full overflow-hidden"
      data-theme="gold-nouveau"
    >
      %sveltekit.body%
    </div>
  </body>
</html>
```

You can of course choose a different theme if you'd like.

Rename `src-svelte/src/routes/Header.svelte` to `src-svelte/src/routes/Sidebar.svelte` and edit it as instructed:

```svelte
<script>
  import { page } from "$app/stores";
  import { AppRail, AppRailAnchor } from "@skeletonlabs/skeleton";
</script>

<AppRail>
  <AppRailAnchor href="/" selected={$page.url.pathname === "/"}>
    (icon)
  </AppRailAnchor>
  <AppRailAnchor href="/about" selected={$page.url.pathname === "/about"}>
    (icon)
  </AppRailAnchor>
</AppRail>

```

Edit `src-svelte/src/routes/+layout.svelte`:

```svelte
<script>
  import "../app.postcss";
  import Sidebar from "./Sidebar.svelte";
  import "./styles.css";
  import { AppShell } from "@skeletonlabs/skeleton";
</script>

<AppShell>
  <svelte:fragment slot="sidebarLeft">
    <Sidebar />
  </svelte:fragment>

  <main>
    <slot />
  </main>
</AppShell>
```

Copy everything from `src-svelte/src/routes/styles.css` to `src-svelte/src/app.postcss`:

```css
/* Write your global styles here, in PostCSS syntax */
@import "@fontsource/jetbrains-mono";

@tailwind base;
@tailwind components;
@tailwind utilities;

@font-face {
  font-family: 'Nasalization';
  font-style: normal;
  font-weight: 400;
  src: url('/fonts/nasalization-rg.otf') format("opentype");
}

@font-face {
  font-family: 'Good Timing';
  font-style: normal;
  font-weight: 400;
  src: url('/fonts/good-timing-rg.ttf') format("truetype");
}

:root {
  --font-body: "Good Timing", sans-serif;
  --font-header: "Nasalization", sans-serif;
  --font-mono: "Jetbrains Mono", monospace;
  --color-header: rgba(255, 0, 0, 0.7);
  --color-faded: rgba(0, 0, 0, 0.4);
  --color-text: rgba(0, 0, 0, 1);

  --theme-font-family-base: var(--font-body);
  --theme-font-family-heading: var(--font-header);
  font-size: 18px;
}

html, body {
  @apply h-full overflow-hidden;
}

```

## Styling box shadow

It turns out that it is not really feasible to style the sidebar box shadow using Skeleton. Editing `src-svelte/src/routes/Sidebar.svelte`:

```svelte
<AppRail background="shadow-inset">
  ...
</AppRail>
```

doesn't actually work to create a shadow.

[Others](https://old.reddit.com/r/SvelteKit/comments/13yumh1/removing_skeleton_ui_from_sveltekit/) have noted similar difficulties with customization at this point in Skeleton's development.

## Errors

If, upon restarting the dev server, you encounter this error:

```
4:39:40 AM [vite] Internal server error: [postcss] /root/zamm/src-svelte/src/routes/api_keys_display.svelte?svelte&type=style&lang.css:2:12: Unknown word
  Plugin: vite:css
  File: /root/zamm/src-svelte/src/routes/api_keys_display.svelte?svelte&type=style&lang.css:2:12
  1  |  <script lang="ts">
  2  |    import { getApiKeys } from "$lib/bindings";
     |              ^
  3  |  
  4  |    let api_keys = getApiKeys();
      at Input.error (/root/zamm/node_modules/vite/node_modules/postcss/lib/input.js:106:16)
```

this is [an unresolved issue](https://stackoverflow.com/questions/75893882/sveltekit-vite-postcss-error-unknown-word) around using Tauri, SvelteKit, postcss, and Tailwind. This error is non-deterministic, so it can be worked around if desired by simply restarting the dev build.
