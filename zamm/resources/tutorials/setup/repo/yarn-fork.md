# Forking a repo locally

If some repo does *almost* what you want them to do, but not quite, you can clone it with

```bash
$ mkdir forks
$ cd forks
$ git submodule add https://github.com/PuruVJ/neodrag.git
```

Then you can edit the `package.json` in your main project to point to this new location on disk. If the project does not have the package in their top-level directory, as this one does not, you will have to point it to the specific package you want instead:

```json
{
  ...
  "dependencies": {
    ...
    "@neodrag/svelte": "file:./forks/neodrag/packages/svelte"
  }
}
```

Run `yarn` again to update the install location for the dependency.

Follow the instructions [here](/zamm/resources/tutorials/libraries/svelte/neodrag.md) to install the dependencies and compile the project.

If you get an error such as this:

```
11:08:15 PM [vite] Internal server error: Failed to resolve entry for package "@neodrag/svelte". The package may have incorrect main/module/exports specified in its package.json.
  Plugin: vite:import-analysis
  File: /root/zamm/src-svelte/src/lib/Switch.svelte
      at packageEntryFailure (file:///root/zamm/node_modules/vite/dist/node/chunks/dep-df561101.js:28691:11)
      ...
```

We head to the `src-svelte/forks/neodrag/packages/svelte` directory and see that the corresponding entries in the `package.json` are:

```json
{
  ...
  "main": "./dist/index.js",
	"module": "./dist/index.js",
	"exports": {
		".": {
			"types": "./dist/index.d.ts",
			"import": {
				"production": "./dist/min/index.js",
				"development": "./dist/index.js"
			},
			"default": "./dist/min/index.js"
		},
		"./package.json": "./package.json"
	},
  ...
}
```

We had followed the instructions to build it, so it does exist:

```bash
$ ls ./dist/index.js
./dist/index.js
```

We can instead go back to our own project, and reinstall our dependencies:

```bash
$ rm -rf node_modules
$ yarn
```

Now everything works. Note that if you're using workspaces, you may have to remove `node_modules` from both the workspace and the project root. In particular, Storybook keeps a cache in the local workspace `node_modules`, whereas non-executable project dependencies go in the root project `node_modules`.

Note that you'll want to make sure that you can make changes that are observably reflected in downstream usage, for example with a Storybook component, before moving on to make real changes. This might include removing exports to see if the build process breaks (if it doesn't, that means the change was not registered in the build process). If things keep not working, you may want to resort to more and more drastic measures, such as removing the build output entirely and seeing if the build process now errors out on the missing output. If it starts working, you may want to start testing smaller and smaller changes, such as changing the functionality of the component in obvious ways to see if those particular changes are being registered as well. For JavaScript, this might invovlve setting off an alert, which is very obvious when called.

To change this to use your own fork (if you didn't already create a fork), do this inside the fork repo:

```bash
$ git remote remove origin
$ git remote add origin https://github.com/amosjyng/neodrag.git
$ git push --set-upstream origin z-customizations
...
remote: Resolving deltas: 100% (8/8), completed with 4 local objects.
remote: 
remote: Create a pull request for 'z-customizations' on GitHub by visiting:
remote:      https://github.com/amosjyng/neodrag/pull/new/z-customizations
remote: 
To https://github.com/amosjyng/neodrag.git
 * [new branch]      z-customizations -> z-customizations
Branch 'z-customizations' set up to track remote branch 'z-customizations' from 'origin'.
```

Then, in your own project root, edit `.gitmodules` to point to your own fork.

When you're done with your changes, you can open a pull request to see if the original maintainers are interested in incorporating your changes upstream.

## Makefile

If you add this to your Makefile:

```Makefile
build: forks/neodrag/packages/svelte/dist $(shell find src -type f \( -name "*.svelte" -o -name "*.js" -o -name "*.ts" -o -name "*.html" \) -not -path "*/node_modules/*")
	yarn && yarn svelte-kit sync && yarn build

forks/neodrag/packages/core/dist: forks/neodrag/packages/core/src/*
	cd forks/neodrag/packages/core && pnpm install && pnpm compile

forks/neodrag/packages/svelte/dist: forks/neodrag/packages/core/dist forks/neodrag/packages/svelte/src/*
	cd forks/neodrag/packages/svelte && pnpm install && pnpm compile
```

then you should also add the corresponding clean command:

```Makefile
 clean:
       rm -rf build node_modules ../node_modules forks/neodrag/packages/svelte/dist/dist
```
