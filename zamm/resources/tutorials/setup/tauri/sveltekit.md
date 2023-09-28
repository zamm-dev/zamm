# Using SvelteKit with Tauri

Setup a new SvelteKit project to take a look at how it works:

```bash
yarn create svelte
yarn create v1.22.19
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
success Installed "create-svelte@5.0.5" with binaries:
      - create-svelte
[########] 8/8
create-svelte version 5.0.5

┌  Welcome to SvelteKit!
│
◇  Where should we create your project?
│  zamm
│
◇  Which Svelte app template?
│  SvelteKit demo app
│
◇  Add type checking with TypeScript?
│  Yes, using TypeScript syntax
│
◇  Select additional options (use arrow keys/space bar)
│  Add ESLint for code linting, Add Prettier for code formatting, Add Vitest for
unit testing
│
└  Your project is ready!

✔ Typescript
  Inside Svelte components, use <script lang="ts">

✔ ESLint
  https://github.com/sveltejs/eslint-plugin-svelte

✔ Prettier
  https://prettier.io/docs/en/options.html
  https://github.com/sveltejs/prettier-plugin-svelte#options

✔ Vitest
  https://vitest.dev

Install community-maintained integrations:
  https://github.com/svelte-add/svelte-add

Next steps:
  1: cd zamm
  2: npm install (or pnpm install, etc)
  3: git init && git add -A && git commit -m "Initial commit" (optional)
  4: npm run dev -- --open

To close the dev server, hit Ctrl-C

Stuck? Visit us at https://svelte.dev/chat
Done in 35.05s.
```

If you have an existing project, then start by copying over `svelte.config.js`.

First install

```bash
$ yarn add -D @sveltejs/adapter-static@next @sveltejs/kit
```

Copy over SvelteKit ignores to .gitignore:

```
# SvelteKit ignores
/build
/.svelte-kit
/package
.env
.env.*
!.env.example
.vercel
.output
vite.config.js.timestamp-*
vite.config.ts.timestamp-*
```

Then edit `svelte.config.js` from

```js
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
```

to

```js
import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
```

Then edit `vite.config.ts` from

```ts
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: [`VITE_`, `TAURI_`],
}));
```

to

```ts
import { defineConfig } from "vite";
import { sveltekit } from '@sveltejs/kit/vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: [`VITE_`, `TAURI_`],
}));
```

Edit the `check` command in `package.json`, from

```json
    "check": "svelte-check --tsconfig ./tsconfig.json",
```

to

```json
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
```

and edit `tsconfig.json` from

```json
{
  "extends": "@tsconfig/svelte/tsconfig.json",
  "compilerOptions": {
    "target": "ESNext",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "resolveJsonModule": true,
    /**
     * Typecheck JS in `.svelte` and `.js` files by default.
     * Disable checkJs if you'd like to use dynamic types in JS.
     * Note that setting allowJs false does not prevent the use
     * of JS in `.svelte` files.
     */
    "allowJs": true,
    "checkJs": true,
    "isolatedModules": true
  },
  "include": ["src/**/*.d.ts", "src/**/*.ts", "src/**/*.js", "src/**/*.svelte"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

to

```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "allowJs": true,
    "checkJs": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "sourceMap": true,
    "strict": true
  }
}
```

and edit `src-tauri/tauri.conf.json` to change from

```
    "distDir": "../dist",
```

to

```
    "distDir": "../build",
```

Then copy over some folders into the current directory:

```bash
$ cp -R /tmp/zamm/.svelte-kit .
$ cp -R /tmp/zamm/src/* src/
$ cp -R /tmp/zamm/static .
```

Merge `src/types/tauri-env.d.ts` into `src/app.d.ts`

Delete `./index.html` because `src/app.html` now serves its purpose. Delete `public/` because `static` now serves its purpose. Merge instead of deleting these if appropriate.

Create a `src/routes/+layout.ts` file with the contents

```ts
export const prerender = true
export const ssr = false
```

Now you can edit `src-svelte/src/routes/+page.ts` to remove the `prerender` value because it will be set to the default anyways. If that file is now empty, you can even remove it entirely. If you get an error such as

```
Error: Failed to load url /src/routes/+page.ts (resolved id: /root/zamm/src-svelte/src/routes/+page.ts) in /root/zamm/src-svelte/.svelte-kit/generated/client/nodes/2.js. Does the file exist?
    at loadAndTransform (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:54939:21)
    at async instantiateModule (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:55875:10) {
  code: 'ERR_LOAD_URL'
}
```

you can simply restart the Vite server and the error will disappear.

`yarn tauri dev` now works to serve the new UI! Edit `src/routes/+page.svelte` as recommended, from

```svelte
<script>
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcome_fallback} alt="Welcome" />
			</picture>
		</span>

		to your new<br />SvelteKit app
	</h1>

	<h2>
		try editing <strong>src/routes/+page.svelte</strong>
	</h2>

	<Counter />
</section>
...
```

to

```svelte
<script>
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png';
	import Greet from '$lib/Greet.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcome_fallback} alt="Welcome" />
			</picture>
		</span>

		to your new<br />SvelteKit app
	</h1>

	<Greet />

	<Counter />
</section>
...
```

The app should automatically refresh to show the new greeting component. Enter in a name and hit the button to confirm that a message such as "Hello, ZAMM! You have been greeted..." appears.

Note that because of the SvelteKit server logic here, the app won't be able to be bundled as-is with the demo SvelteKit app due to this error:

```
Error: Cannot prerender pages with actions
    at render_page (file:///root/zamm/.svelte-kit/output/server/index.js:1899:15)
    at async resolve (file:///root/zamm/.svelte-kit/output/server/index.js:2647:24)
    at async respond (file:///root/zamm/.svelte-kit/output/server/index.js:2533:22)
    at async visit (file:///root/zamm/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:203:20)

node:internal/event_target:1054
  process.nextTick(() => { throw err; });
                           ^
Error: 500 /sverdle
To suppress or handle this error, implement `handleHttpError` in https://kit.svelte.dev/docs/configuration#prerender
    at file:///root/zamm/node_modules/@sveltejs/kit/src/core/config/options.js:212:13
    at file:///root/zamm/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:64:25
    at save (file:///root/zamm/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:403:4)
    at visit (file:///root/zamm/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:236:3)
Emitted 'error' event on Worker instance at:
    at [kOnErrorMessage] (node:internal/worker:326:10)
    at [kOnMessage] (node:internal/worker:337:37)
    at MessagePort.<anonymous> (node:internal/worker:232:57)
    at [nodejs.internal.kHybridDispatch] (node:internal/event_target:778:20)
    at exports.emitMessage (node:internal/per_context/messageport:23:28)

Node.js v20.5.1
error Command failed with exit code 1.
```

To fix this, remove the `src/routes/sverdle` folder. Then move `src/lib/Greet*` to `src/routes/greet/`, and rename `Greet.svelte` to `+page.svelte` because Svelte routes based on file directory location. Then edit `src/routes/Header.svelte` from

```
      <li aria-current={$page.url.pathname.startsWith("/sverdle") ? "page" : undefined}>
        <a href="/sverdle">Sverdle</a>
```

to

```
      <li aria-current={$page.url.pathname.startsWith("/greet") ? "page" : undefined}>
        <a href="/greet">Greet</a>
```

Finally, remove references to `Greet` from `src/routes/+page.svelte`.

## Testing

For the end-to-end tests at `e2e.test.js`, let's define a convenience click function:

```js
  click = async(selector) => {
    // workaround for https://github.com/tauri-apps/tauri/issues/6541
    const element = await $(selector);
    await element.waitForClickable();
    await browser.execute('arguments[0].click();', element);
  };
```

Then edit the tests to handle the new pages:

```js
  it("should show greet button", async function () {
    const text = await $("a=Greet").getText();
    expect(text).toMatch(/^GREET$/);
  });

  it("should greet user when button pressed", async function () {
    await click("a=Greet");

    const original = await $("p#greet-message").getText();
    expect(original).toMatch(/^$/);

    const greetInput = await $("#greet-input");
    // workaround for https://github.com/tauri-apps/tauri/issues/6541
    await browser.execute(`arguments[0].value="me"`, greetInput);
    await browser.execute(
      'arguments[0].dispatchEvent(new Event("input", { bubbles: true }))',
      greetInput,
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
    const inputText = await $("#greet-input").getValue();
    expect(inputText).toMatch(/^me$/);

    await click("button=Greet")

    await new Promise((resolve) => setTimeout(resolve, 1000));
    const text = await $("p#greet-message").getText();
    expect(text).toMatch(/^Hello, me! You have been greeted/);
  });
```

## Committing

Edit `src/app.d.ts` to include:

```ts
  interface Window {
    __TAURI_IPC__?: () => void;
    // @ts-ignore
    [key: string]: any;
  }
```

If you do that, then you also need to add this to `.eslintrc.yaml`:

```yaml
rules:
  ...
  "@typescript-eslint/no-explicit-any": off
```
