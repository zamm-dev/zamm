# Setting up unplugin with SvelteKit

For icons, we could either use [Iconify's Svelte support](https://iconify.design/docs/icon-components/svelte/), where syntax looks like this:

```html
<Icon icon="mdi-light:home" />
```

Or we can use [unplugin](https://github.com/unplugin/unplugin-icons), which also uses Iconify data under the hood but whose syntax looks like this:

```svelte
<script>
  import IconAccessibility from '~icons/carbon/accessibility';
</script>

<IconAccessibility />
```

If you like the look of the second one more, then do:

```bash
$ yarn add -D unplugin-icons @iconify/json
```

Then, edit `src-svelte/vite.config.ts` to add the Icons plugin:

```ts
...
import Icons from 'unplugin-icons/vite'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    sveltekit(),
    Icons({
      compiler: 'svelte',
    }),
  ],

  ...
}));

```

If you are using Vitest, you'll want to also edit `src-svelte/vitest.config.ts` to do the same thing:

```ts
import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import Icons from "unplugin-icons/vite";
...

export default defineConfig({
  plugins: [
    svelte({ hot: !process.env.VITEST }),
    Icons({
      compiler: "svelte",
    }),
  ],
  ...
});

```

Now use it. For example, in `src-svelte/src/routes/Sidebar.svelte`:

```svelte
<script>
  import IconSettings from "~icons/lucide/settings";
</script>

<header>
  <nav>
    <div class="selected icon">
      <IconSettings />
    </div>
  </nav>
</header>

...
```



If you get a `svelte-check` error:

```
/root/zamm/src-svelte/src/routes/Sidebar.svelte:2:28
Error: Cannot find module '~icons/lucide/settings' or its corresponding type declarations. (js)
<script>
  import IconSettings from "~icons/lucide/settings";
```

then add this to `src-svelte/src/app.d.ts`:

```ts
import "unplugin-icons/types/svelte";
```

If you are using Storybook, you may get an error such as

```
ENOENT: no such file or directory, open '~icons/lucide/settings.svelte'
12:02:02 AM [vite] Internal server error: ENOENT: no such file or directory, open '~icons/lucide/settings.svelte'
  Plugin: storybook:svelte-docgen-plugin
  File: ~icons/lucide/settings.svelte
      at Object.openSync (node:fs:602:3)
      at Object.readFileSync (node:fs:470:35)
      at TransformContext.transform (/root/zamm/node_modules/@storybook/svelte-vite/dist/preset.js:9:1251)
      at Object.transform (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:44283:62)
      at async loadAndTransform (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:54950:29)
      at async viteTransformMiddleware (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:64345:32)
```

Note that this is a [known issue](https://github.com/storybookjs/storybook/issues/20562). We'll follow the workaround mentioned [here](https://github.com/storybookjs/storybook/issues/20562#issuecomment-1467329472). Their example is:

```ts
// .storybook/main.ts
import type { Plugin, InlineConfig } from 'vite'
import type { StorybookConfig } from '@storybook/sveltekit'

// https://github.com/storybookjs/storybook/issues/20562
const workaroundSvelteDocgenPluginConflictWithUnpluginIcons = (config: InlineConfig) => {
  if (!config.plugins) return config

  const [_internalPlugins, ...userPlugins] = config.plugins as Plugin[]
  const docgenPlugin = userPlugins.find(plugin => plugin.name === 'storybook:svelte-docgen-plugin')
  if (docgenPlugin) {
    const origTransform = docgenPlugin.transform
    const newTransform: typeof origTransform = (code, id, options) => {
      if (id.startsWith('~icons/')) {
        return
      }
      return (origTransform as Function)?.call(docgenPlugin, code, id, options)
    }
    docgenPlugin.transform = newTransform
    docgenPlugin.enforce = 'post'
  }
  return config
}

const config: StorybookConfig = {
  stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|ts|tsx)'],
  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-interactions',
  ],
  framework: {
    name: '@storybook/sveltekit',
    options: {},
  },
  docs: {
    autodocs: 'tag',
  },
  viteFinal(config) {
    return workaroundSvelteDocgenPluginConflictWithUnpluginIcons(config)
  }
}
export default config
```

Our own `src-svelte/.storybook/main.ts` is slightly different, so we do this (and rename the workaround function to be something shorter):

```ts
...
import { mergeConfig, Plugin, InlineConfig } from "vite";

...

// https://github.com/storybookjs/storybook/issues/20562
const unpluginIconsWorkaround = (config: InlineConfig) => {
  ... // same as example
}

const config: StorybookConfig = {
  ...
  async viteFinal(config: InlineConfig) {
    const workaroundConfig = unpluginIconsWorkaround(config);
    return mergeConfig(workaroundConfig, {
      resolve: {
        alias: { $lib: path.resolve(__dirname, "../src/lib") },
      },
    });
  },
};
export default config;
```
