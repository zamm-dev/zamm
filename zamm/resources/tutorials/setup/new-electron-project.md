# Starting a new Electron project

Suppose our current path is `/tmp`. We create a new Vite project with Electron and React (pick React when it comes up):

```bash
$ yarn create electron-vite my-app
yarn create v1.22.15
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
success Installed "create-electron-vite@0.3.0" with binaries:
      - create-electron-vite
      - cev
✔ Project name: … my-app
✔ Target directory "my-app" is not empty. Remove existing files and continue? … yes
✔ Project template: › React

Scaffolding project in /tmp/my-app...

Done. Now run:

  cd my-app
  yarn
  yarn dev

Done in 14.91s.
```

Run `yarn` next to install dependencies:

```bash
$ cd my-app
$ yarn
...
info "@esbuild/win32-x64@0.18.17" is an optional dependency and failed compatibility check. Excluding it from installation.
[3/4] Linking dependencies...
[4/4] Building fresh packages...
success Saved lockfile.
Done in 105.69s.
```

Now try

```bash
$ yarn dev
```

If you get error output like this:

```
yarn run v1.22.15
$ vite

  VITE v4.4.7  ready in 255 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h to show help
vite v4.4.7 building for development...

watching for file changes...
vite v4.4.7 building for development...

watching for file changes...

build started...

build started... (x2)
✓ 1 modules transformed.
✓ 1 modules transformed. (x2)
dist-electron/main.js  0.65 kB │ gzip: 0.39 kB
built in 82ms.
/tmp/my-app/node_modules/electron/dist/electron: error while loading shared libraries: libnss3.so: cannot open shared object file: No such file or directory
dist-electron/preload.js  1.55 kB │ gzip: 0.76 kB
built in 87ms.
error Command failed with exit code 127.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

then set up `libnss3` like so:

```bash
$ sudo apt install libnss3 libnss3-dev libatk1.0-0 libasound2
```

If successful, the command `yarn dev` should show:

```bash
$ yarn dev
✓ 1 modules transformed. (x2)
dist-electron/main.js  0.65 kB │ gzip: 0.39 kB
built in 83ms.
dist-electron/preload.js  1.55 kB │ gzip: 0.76 kB
built in 87ms.
```

Note that it runs persistently.

`eslint` should be set up automatically:

```bash
$ yarn run eslint
yarn run v1.22.15
$ /tmp/my-app/node_modules/.bin/eslint
Done in 0.15s.
```

## Setting up Electron Forge

Follow [setup instructions](https://www.electronforge.io/import-existing-project) to add Electron Forge to dev dependencies:

```bash
$ yarn add --dev @electron-forge/cli @electron-forge/plugin-vite
$ yarn electron-forge import
```

Try running `yarn make`. If it fails with the following error:

```bash
$ yarn make                                 
yarn run v1.22.15
$ electron-forge make
✔ Checking your system
✔ Loading configuration
✖ Resolving make targets
  › Cannot make for rpm, the following external binaries need to be installed: …
◼ Running package command
◼ Running preMake hook
◼ Making distributables
◼ Running postMake hook

An unhandled rejection has occurred inside Forge:
Error: Cannot make for rpm, the following external binaries need to be installed: rpmbuild
at MakerRpm.ensureExternalBinariesExist (/home/amos/projects/zamm/ui/node_modules/@electron-forge/maker-base/dist/Maker.js:103:19)
    at Task.task (/home/amos/projects/zamm/ui/node_modules/@electron-forge/core/dist/api/make.js:118:27)
    at Task.run (/home/amos/projects/zamm/ui/node_modules/listr2/dist/index.cjs:978:35)
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

fix that with

```bash
$ sudo apt install rpm
```

Make sure your `package.json` has `"description"` and `"license"` fields. For example, if it currently lacks them:

```json
{
  "name": "ui",
  "private": true,
  "version": "0.0.0",
  "scripts": {
    ...
  },
  ...
}
```

then add those fields in:

```json
{
  "name": "ui",
  "private": true,
  "version": "0.0.0",
  "description": "Generalized Automation Driver",
  "license": "GPL-3.0-or-later",
  "scripts": {
    ...
  },
  ...
}
```

## electron-builder

Alternatively

```bash
$ yarn add electron-builder --dev
```
