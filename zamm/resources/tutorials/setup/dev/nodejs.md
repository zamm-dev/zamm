# Setting up NodeJS with `asdf`

Assuming that you already have `asdf` set up, next install the NodeJS plugin:

```bash
$ asdf plugin add nodejs
updating plugin repository...remote: Enumerating objects: 50, done.
remote: Counting objects: 100% (50/50), done.
remote: Compressing objects: 100% (28/28), done.
remote: Total 50 (delta 26), reused 43 (delta 22), pack-reused 0
Unpacking objects: 100% (50/50), 69.95 KiB | 1023.00 KiB/s, done.
From https://github.com/asdf-vm/asdf-plugins
   097d4aa..b03baaa  master     -> origin/master
HEAD is now at b03baaa feat: add asdf-oapi-codegen plugin (#864)
```

Then install the latest version of NodeJS:

```bash
$ asdf install nodejs latest
Trying to update node-build... ok
Downloading node-v20.5.0-linux-x64.tar.gz...
-> https://nodejs.org/dist/v20.5.0/node-v20.5.0-linux-x64.tar.gz
Installing node-v20.5.0-linux-x64...
Installed node-v20.5.0-linux-x64 to /home/amos/.asdf/installs/nodejs/20.5.0
```

Mark the version we just installed as the global version of NodeJS:

```bash
$ asdf global nodejs 20.5.0
```

To add support for yarn as well, install it via npm:

```bash
$ npm install --global yarn

added 1 package in 4s
npm notice 
npm notice New patch version of npm available! 9.8.0 -> 9.8.1
npm notice Changelog: https://github.com/npm/cli/releases/tag/v9.8.1
npm notice Run npm install -g npm@9.8.1 to update!
npm notice 
Reshimming asdf nodejs...
```
