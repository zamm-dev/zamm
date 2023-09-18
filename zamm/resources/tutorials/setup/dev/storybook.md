# Setting up Storybook with SvelteKit

Follow the instructions [here](https://storybook.js.org/docs/svelte/get-started/install):

```bash
$ npx storybook@latest init
Need to install the following packages:
  storybook@7.4.0
Ok to proceed? (y) y

 storybook init - the simplest way to add a Storybook to your project. 

 • Detecting project type. ✓
 • Preparing to install dependencies. ✓


yarn install v1.22.19
[1/4] Resolving packages...
success Already up-to-date.
Done in 0.36s.
. ✓
 • Adding Storybook support to your "SvelteKit" app
  ✔ Getting the correct version of 10 packages
  ✔ Installing Storybook dependencies
. ✓
 • Preparing to install dependencies. ✓


yarn install v1.22.19
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
warning " > @typescript-eslint/eslint-plugin@5.62.0" has incorrect peer dependency "@typescript-eslint/parser@^5.0.0".
warning "workspace-aggregator-6111915a-9d82-4040-9d33-56d36c9385f4 > webdriver > ts-node@10.9.1" has unmet peer dependency "@types/node@*".
warning Workspaces can only be enabled in private projects.
warning Workspaces can only be enabled in private projects.
[4/4] Building fresh packages...
Done in 3.82s.
. ✓
╭─────────────────────────────────────────────────────────────────────────╮│                                                                         ││   Storybook was successfully installed in your project! 🎉              ││   To run Storybook manually, run yarn storybook. CTRL+C to stop.        ││                                                                         ││   Wanna know more about Storybook? Check out                            ││   https://storybook.js.org/                                             ││   Having trouble or want to chat? Join us at                            ││   https://discord.gg/storybook/                                         ││                                                                         │╰─────────────────────────────────────────────────────────────────────────╯

Running Storybook
yarn run v1.22.19
$ storybook dev -p 6006 --quiet
@storybook/cli v7.4.0

info => Starting manager..
The following Vite config options will be overridden by SvelteKit:
  - base

Error: We've detected a SvelteKit project using the @storybook/svelte-vite framework, which is not supported in Storybook 7.0
Please use the @storybook/sveltekit framework instead.
You can migrate automatically by running

npx storybook@latest automigrate

See https://github.com/storybookjs/storybook/blob/next/MIGRATION.md#sveltekit-needs-the-storybooksveltekit-framework
    at handleSvelteKit (/root/zamm/node_modules/@storybook/svelte-vite/dist/preset.js:1:1635)
    at async viteFinal (/root/zamm/node_modules/@storybook/svelte-vite/dist/preset.js:9:2478)
    at async Object.viteFinal (/root/zamm/node_modules/@storybook/sveltekit/dist/preset.js:1:1319)
    at async createViteServer (/root/zamm/node_modules/@storybook/builder-vite/dist/index.js:159:10530)
    at async Module.start (/root/zamm/node_modules/@storybook/builder-vite/dist/index.js:159:12527)
    at async storybookDevServer (/root/zamm/node_modules/@storybook/core-server/dist/index.js:101:8374)
    at async buildDevStandalone (/root/zamm/node_modules/@storybook/core-server/dist/index.js:116:3013)
    at async withTelemetry (/root/zamm/node_modules/@storybook/core-server/dist/index.js:101:4155)
    at async dev (/root/zamm/node_modules/@storybook/cli/dist/generate.js:502:401)
    at async Command.<anonymous> (/root/zamm/node_modules/@storybook/cli/dist/generate.js:504:225)

WARN Broken build, fix the error above.
WARN You may need to refresh the browser.

error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

Let's listen to the error and do this:

```bash
$ npx storybook@latest automigrate
🔎 checking possible migrations..

🔎 found a 'github-flavored-markdown-mdx' migration:
╭ Automigration detected ─────────────────────────────────────────────────╮│                                                                         ││   In MDX1 you had the option of using GitHub flavored markdown.         ││                                                                         ││   Storybook 7.0 uses MDX2 for compiling MDX, and thus no longer         ││   supports GFM out of the box.                                          ││   Because of this you need to explicitly add the GFM plugin in the      ││   addon-docs options:                                                   ││   https://storybook.js.org/docs/react/writing-docs/mdx#lack-of-github   ││   -flavored-markdown-gfm                                                ││                                                                         ││   We recommend you follow the guide on the link above, however we can   ││   add a temporary storybook addon that helps make this migration        ││   easier.                                                               ││   We'll install the addon and add it to your storybook config.          ││                                                                         │╰─────────────────────────────────────────────────────────────────────────╯
✔ Do you want to run the 'github-flavored-markdown-mdx' migration on your project? … yes
✅ Adding "@storybook/addon-mdx-gfm" addon
✅ ran github-flavored-markdown-mdx migration

🔎 found a 'wrap-require' migration:
╭ Automigration detected ─────────────────────────────────────────────────╮│                                                                         ││   We have detected that you're using Storybook 7.4.0 in a monorepo      ││   project.                                                              ││   For Storybook to work correctly, some fields in your main config      ││   must be updated. We can do this for you automatically.                ││                                                                         ││   More info: https://storybook.js.org/docs/react/faq#how-do-i-fix-mod   ││   ule-resolution-in-special-environments                                ││                                                                         │╰─────────────────────────────────────────────────────────────────────────╯
✔ Do you want to run the 'wrap-require' migration on your project? … yes
✅ ran wrap-require migration

╭ Migration check ran successfully ───────────────────────────────────────╮│                                                                         ││   Successful migrations:                                                ││                                                                         ││   github-flavored-markdown-mdx, wrap-require                            ││                                                                         ││   ─────────────────────────────────────────────────                     ││                                                                         ││   If you'd like to run the migrations again, you can do so by running   ││   'npx storybook@next automigrate'                                      ││                                                                         ││   The automigrations try to migrate common patterns in your project,    ││   but might not contain everything needed to migrate to the latest      ││   version of Storybook.                                                 ││                                                                         ││   Please check the changelog and migration guide for manual             ││   migrations and more information:                                      ││   https://storybook.js.org/migration-guides/7.0                         ││   And reach out on Discord if you need help:                            ││   https://discord.gg/storybook                                          ││                                                                         │╰─────────────────────────────────────────────────────────────────────────╯

```

It still doesn't work. We see that there is [an issue](https://github.com/storybookjs/storybook/issues/23777). Follow the instructions there and edit `src-svelte/.storybook/main.ts` to refer to:

```ts
const config = {
  ...
  framework: {
    name: '@storybook/sveltekit',
    options: {},
  },
  ...
};
export default config;
```

Next, follow the instructions [here](https://www.thisdot.co/blog/integrating-storybook-with-sveltekit-typescript-and-scss/). Create a file `src-svelte/src/routes/api_keys_display.stories.ts`:

```ts
import ApiKeysDisplay from './api_keys_display.svelte';

export default {
    component: ApiKeysDisplay,
    title: 'Example/API Keys Display',
    argTypes: {},
};

const Template = ({ ...args }) => ({
    Component: ApiKeysDisplay,
    props: args,
});

export const Default = Template.bind({});

```

Then we go to our storybook and find `API Keys Display > Default`. We see the error

```
invoke() is not a function

getApiKeys@http://localhost:6006/src/lib/bindings.ts:6:18
instance@http://localhost:6006/src/routes/api_keys_display.svelte:359:17
init@http://localhost:6006/node_modules/.cache/sb-vite/deps/chunk-SW64H2IU.js?v=507a655b:2166:23
...
```

Mock the function in the story:

```ts
import type { ApiKeys } from '$lib/bindings';

...

const mock_invoke: (arg: string) => Promise<ApiKeys> = () => Promise.resolve({
  openai: null,
});
window.__TAURI_INVOKE__ = mock_invoke as any;

```

We see that the text now shows up but is completely unstyled. We edit `src-svelte/.storybook/preview.ts`:

```ts
import '../src/routes/styles.css';

...
```

Now the color is there, but the fonts are not. Edit the config again:

```ts
const config = {
  ...
  staticDirs: ['../static'],
  ...
};
```

**Restart** the storybook server. Now it should look just as we expected.

Delete the default components folder because we aren't using any of those.

To avoid the error

```
/root/zamm/src-svelte/.storybook/main.ts
  0:0  warning  File ignored by default.  Use a negated ignore pattern (like "--ignore-pattern '!<relative/path/to/filename>'") to override
```

which is due to [eslint ignoring hidden directories by default](https://stackoverflow.com/a/71829427), edit `src-svelte/.eslintrc.yaml`:

```yaml
...
ignorePatterns:
  ...
  - "!.storybook"
  ...
```

We want to visualize this component in multiple states, but we can only mock one return value for the `mock_invoke` function at a time. We could either:

- try to explicitly pass in the function to the component so that it can be mocked, or
- try to create a separate `*.stories.ts` file for each different version of the function, using [single-story hoisting](https://storybook.js.org/docs/react/writing-stories/naming-components-and-hierarchy#single-story-hoisting) to make sure the tree stays clean

We try the first option, but there are issues with the function getting passed in, and it also annoyingly changes the existing code without testing it as thoroughly as the Vitest.

We try the second option. Rename `src-svelte/src/routes/api_keys_display.stories.ts` to `src-svelte/src/routes/api_keys_display.unknown.stories.ts` and make the export the same name to take advantage of the hoisting:

```ts
...

export default {
  component: ApiKeysDisplay,
  title: "Settings/API Keys Display/Unknown",
  argTypes: {},
};

...

export const Unknown = Template.bind({});
```

Then create `src-svelte/src/routes/api_keys_display.loading.stories.ts` with a long wait:

```ts
import ApiKeysDisplay from "./api_keys_display.svelte";
import type { ApiKeys } from "$lib/bindings";

export default {
  component: ApiKeysDisplay,
  title: "Settings/API Keys Display/Loading",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ApiKeysDisplay,
  props: args,
});

const mock_invoke: (arg: string) => Promise<ApiKeys> = () =>
  new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        openai: null,
      });
    }, 1_000_000);
  });
window.__TAURI_INVOKE__ = mock_invoke as any;

export const Loading = Template.bind({});
```

It works... until you switch to the other page and `window.__TAURI_INVOKE__` gets overwritten once again.

We look at [this answer](https://stackoverflow.com/a/70115093) instead and find that Storybook does offer a built-in way after all to mock functions. We look at the documentation for [decorators](https://storybook.js.org/docs/react/writing-stories/decorators) and parameters, and create a mock invoke function at `src-svelte/src/lib/__mocks__/invoke.ts`: 

```ts
import type {StoryFn, Decorator, StoryContext} from "@storybook/svelte";

let nextResolution: any;
let nextShouldWait: boolean = false;

window.__TAURI_INVOKE__ = () => {
  return new Promise((resolve) => {
    if (nextShouldWait) {
      setTimeout(() => {
        resolve(nextResolution);
      }, 0); // the re-render never happens, so any timeout is fine
    } else {
      resolve(nextResolution);
    }
  });
}

interface TauriInvokeArgs {
  resolution: any;
  shouldWait?: boolean | undefined;
  [key: string]: any;
}

const tauri_invoke_decorator: Decorator = (story: StoryFn, context: StoryContext) => {
  const { args, parameters } = context;
  const { resolution, shouldWait } = parameters as TauriInvokeArgs;
  nextResolution = resolution;
  nextShouldWait = shouldWait || false;
  return story(args, context);
}

export default tauri_invoke_decorator;
```

We make sure to add that decorator in `src-svelte/.storybook/preview.ts`:

```ts
import "../src/routes/styles.css";
import tauri_invoke_decorator from "../src/lib/__mocks__/invoke";

/** @type { import('@storybook/svelte').Preview } */
const preview = {
  ...
  decorators: [tauri_invoke_decorator],
};

export default preview;

```

Now in the original `src-svelte/src/routes/api_keys_display.stories.ts`, we set the new expected parameters:

```ts
import ApiKeysDisplay from "./api_keys_display.svelte";
import type { ApiKeys } from "$lib/bindings";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ApiKeysDisplay,
  title: "Settings/API Keys Display",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ApiKeysDisplay,
  props: args,
});

const unknownKeys: ApiKeys = {
  openai: null,
};

const knownKeys: ApiKeys = {
  openai: {
    value: "sk-1234567890",
    source: "Environment",
  },
};

export const Loading: StoryObj = Template.bind({}) as any;
Loading.parameters = {
  resolution: unknownKeys,
  shouldWait: true,
};

export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  resolution: unknownKeys,
};

export const Known: StoryObj = Template.bind({}) as any;
Known.parameters = {
  resolution: knownKeys,
};

```

Now we can render all three different component states based on mocked return values.

## Custom viewport for component

To make the component render at a specific size, we can use the `@storybook/addon-viewport` addon. Install it:

```bash
$ yarn add -D @storybook/addon-viewport
```

Then edit `src-svelte/.storybook/main.ts`:

```ts
...

const config: StorybookConfig = {
  ...
  addons: [
    ...
    getAbsolutePath("@storybook/addon-viewport"),
  ],
  ...
};
```

Then, as shown [here](https://stackoverflow.com/a/73028857) (note the typo in the answer), edit your story at, say, `src-svelte/src/routes/Metadata.stories.ts`,  to add the parameters:

```ts
...

export const Metadata: StoryObj = Template.bind({}) as any;
Metadata.parameters = {
  viewport: {
      defaultViewport: "mobile2"
  }
}
```

### Custom props for component

To create different stories that have different prop values by default, you can do this:

```ts
const Template = ({ ...args }) => ({
  Component: BackgroundComponent,
  props: args,
});

export const Static: StoryObj = Template.bind({}) as any;
Static.args = {
  animated: false,
};
export const Dynamic: StoryObj = Template.bind({}) as any;
Dynamic.args = {
  animated: true,
};
```

## Errors and warnings

If you see this warning when starting Storybook up:

```
WARN No story files found for the specified pattern: src/**/*.mdx
```

remove `"../src/**/*.mdx"` from your `StorybookConfig` in `src-svelte/.storybook/main.ts`, from

```ts
...

const config: StorybookConfig = {
  stories: ["../src/**/*.mdx", "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
  ...
};
export default config;
```

to

```ts
...

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
  ...
};
export default config;
```
