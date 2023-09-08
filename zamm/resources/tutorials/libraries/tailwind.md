# Setting up Tailwind with SvelteKit

Follow the instructions [here](https://tailwindcss.com/docs/guides/sveltekit):

```bash
$ yarn add -D tailwindcss postcss autoprefixer
$ yarn tailwindcss init -p
```

If you've followed previous setup steps, `src-svelte/svelte.config.js` should already have a `preprocess: vitePreprocess()` line.

Edit `src-svelte/tailwind.config.ts` to contain `content`:

```ts
import { Config } from "tailwindcss";

const config: Config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
  },
  plugins: [],
};

export default config;

```

Edit your app's main CSS file `src-svelte/src/routes/styles.css` to add these lines to the beginning after any imports:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Now you can use Tailwind CSS. For example, edit `src-svelte/src/routes/+layout.svelte` from:

```svelte
<script>
  import Sidebar from "./Sidebar.svelte";
  import "./styles.css";
</script>

<div class="app">
  <Sidebar />

  <main>
    <slot />
  </main>
</div>

<style>
  main {
    margin-left: var(--sidebar-width);
    padding: 20px;
  }
</style>

```

to:

```svelte
<script>
  import Sidebar from "./Sidebar.svelte";
  import "./styles.css";
</script>

<div class="app">
  <Sidebar />

  <main class="p-4">
    <slot />
  </main>
</div>

<style>
  main {
    margin-left: var(--sidebar-width);
  }
</style>

```

## Errors

If you get an error such as this:

```
$ yarn tauri dev
...
[postcss] /root/zamm/src-svelte/src/routes/api_keys_display.svelte?svelte&type=style&lang.css:2:12: Unknown word
3:27:06 AM [vite] Internal server error: [postcss] /root/zamm/src-svelte/src/routes/api_keys_display.svelte?svelte&type=style&lang.css:2:12: Unknown word
  Plugin: vite:css
  File: /root/zamm/src-svelte/src/routes/api_keys_display.svelte?svelte&type=style&lang.css:2:12
  1  |  <script lang="ts">
  2  |    import { getApiKeys } from "$lib/bindings";
     |              ^
  3  |  
  4  |    let api_keys = getApiKeys();
      at Input.error (/root/zamm/node_modules/vite/node_modules/postcss/lib/input.js:106:16)
```

then restarting the app appears to fix this issue, as it appears the issue is non-deterministic. If restarting the app does not help, then you can disable postcss (and thus Tailwind) by deleting the `src-svelte/postcss.config.js` file.
