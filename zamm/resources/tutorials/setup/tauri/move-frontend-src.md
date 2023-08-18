# Moving the frontend into its own src/ folder

The default Tauri app creator puts Svelte files into the main build directory. To change this:

```bash
$ mkdir src-svelte
```

then split off `package.json`:

```json
{
  "name": "zamm",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "tauri": "tauri",
    "test": "vitest",
    "e2e-test": "cd webdriver && yarn test"
  },
  "workspaces": [
    "webdriver"
  ],
  "dependencies": {
    "@tauri-apps/api": "^1.4.0"
  },
  "devDependencies": {
    "@fontsource/fira-mono": "^4.5.10",
    "@neoconfetti/svelte": "^1.0.0",
    "@types/cookie": "^0.5.1",
    "@sveltejs/adapter-static": "^1.0.0-next.50",
    "@sveltejs/kit": "^1.22.6",
    "@sveltejs/vite-plugin-svelte": "^2.4.2",
    "@tauri-apps/cli": "^1.4.0",
    "@testing-library/dom": "^9.3.1",
    "@testing-library/jest-dom": "^6.0.0",
    "@testing-library/svelte": "^4.0.3",
    "@testing-library/user-event": "^14.4.3",
    "@tsconfig/svelte": "^5.0.0",
    "@typescript-eslint/eslint-plugin": "^5.52.0",
    "@typescript-eslint/parser": "^6.3.0",
    "eslint": "^8.0.1",
    "eslint-config-prettier": "^9.0.0",
    "eslint-config-standard-with-typescript": "^37.0.0",
    "eslint-plugin-import": "^2.25.2",
    "eslint-plugin-n": "^15.0.0 || ^16.0.0 ",
    "eslint-plugin-promise": "^6.0.0",
    "eslint-plugin-svelte": "^2.32.4",
    "jsdom": "^22.1.0",
    "prettier": "^3.0.1",
    "prettier-plugin-svelte": "^3.0.3",
    "svelte": "^4.0.5",
    "svelte-check": "^3.4.6",
    "svelte-eslint-parser": "^0.32.2",
    "svelte-preprocess": "^5.0.0",
    "tslib": "^2.6.0",
    "typescript": "*",
    "vite": "^4.4.4",
    "vitest": "^0.34.1"
  }
}
```

such that Svelte-specific parts go into `src-svelte/package.json`:

```json
{
  "name": "gui",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "test": "vitest run",
    "test-watch": "vitest"
  },
  "devDependencies": {
    "@fontsource/fira-mono": "^4.5.10",
    "@neoconfetti/svelte": "^1.0.0",
    "@types/cookie": "^0.5.1",
    "@sveltejs/adapter-static": "^1.0.0-next.50",
    "@sveltejs/kit": "^1.22.6",
    "@sveltejs/vite-plugin-svelte": "^2.4.2",
    "@testing-library/dom": "^9.3.1",
    "@testing-library/jest-dom": "^6.0.0",
    "@testing-library/svelte": "^4.0.3",
    "@testing-library/user-event": "^14.4.3",
    "@tsconfig/svelte": "^5.0.0",
    "eslint-plugin-svelte": "^2.32.4",
    "jsdom": "^22.1.0",
    "svelte": "^4.0.5",
    "svelte-check": "^3.4.6",
    "svelte-eslint-parser": "^0.32.2",
    "svelte-preprocess": "^5.0.0",
    "vite": "^4.4.4",
    "vitest": "^0.34.1"
  }
}
```

and the original `package.json` is now:

```json
{
  "name": "zamm",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "tauri": "tauri",
    "e2e-test": "cd webdriver && yarn test"
  },
  "workspaces": [
    "src-svelte",
    "webdriver"
  ],
  "dependencies": {
    "@tauri-apps/api": "^1.4.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.4.0",
    "@typescript-eslint/eslint-plugin": "^5.52.0",
    "@typescript-eslint/parser": "^6.3.0",
    "eslint": "^8.0.1",
    "eslint-config-prettier": "^9.0.0",
    "eslint-config-standard-with-typescript": "^37.0.0",
    "eslint-plugin-import": "^2.25.2",
    "eslint-plugin-n": "^15.0.0 || ^16.0.0 ",
    "eslint-plugin-promise": "^6.0.0",
    "prettier": "^3.0.1",
    "prettier-plugin-svelte": "^3.0.3",
    "tslib": "^2.6.0",
    "typescript": "*"
  }
}
```

Split off the original `.gitignore` into `src-svelte/.gitignore`:

```
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

Split off the original `.eslintrc.yaml`:

```yaml
# https://github.com/janosh/awesome-sveltekit/blob/main/site/.eslintrc.yml
env:
  browser: true
  node: true
extends:
  - prettier
  - plugin:svelte/recommended
  - eslint:recommended
  - plugin:@typescript-eslint/recommended
overrides:
  - files: ["*.svelte"]
    parser: svelte-eslint-parser
    parserOptions:
      parser: "@typescript-eslint/parser"
rules:
  no-inner-declarations: off
  "@typescript-eslint/ban-ts-comment": off
  "@typescript-eslint/no-explicit-any": off
ignorePatterns: [build/, dist/]
```

such that Svelte-specific parts go into `src-svelte/.eslintrc.yaml`:

```
# https://github.com/janosh/awesome-sveltekit/blob/main/site/.eslintrc.yml
extends:
  - plugin:svelte/recommended
overrides:
  - files: ["*.svelte"]
    parser: svelte-eslint-parser
    parserOptions:
      parser: "@typescript-eslint/parser"
rules:
  no-inner-declarations: off
ignorePatterns: [build/, dist/]
```

and the original `.eslintrc.yaml` is now:

```
# https://github.com/janosh/awesome-sveltekit/blob/main/site/.eslintrc.yml
env:
  browser: true
  node: true
extends:
  - prettier
  - eslint:recommended
  - plugin:@typescript-eslint/recommended
rules:
  max-len:
    - error
    - code: 88
  "@typescript-eslint/ban-ts-comment": off
  "@typescript-eslint/no-explicit-any": off
```

Split off the original `.prettierrc` into `src-svelte/.prettierrc`:

```
{
  "plugins": ["prettier-plugin-svelte"]
}
```

Move various files and folders into `src-svelte/`:

```bash
$ mv src src-svelte/
$ mv static src-svelte/
$ mv tsconfig.json src-svelte/
$ mv tsconfig.node.json src-svelte/
$ mv svelte.config.js src-svelte
$ mv vite.config.ts src-svelte
$ mv vitest.config.ts src-svelte
```

Edit `src/tauri.conf.json` to change the `distDir` to `src-svelte`:

```json
{
  "build": {
    ...
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "distDir": "../build"
  },
  ...
}
```

to

```json
{
  "build": {
    ...
    "beforeDevCommand": "yarn workspace gui dev",
    "beforeBuildCommand": "yarn workspace gui build",
    "distDir": "../src-svelte/build"
  },
  ...
}
```
