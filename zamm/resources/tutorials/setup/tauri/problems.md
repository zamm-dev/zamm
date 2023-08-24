# Common problems

## Svelte not watching files

You can try editing `src-svelte/svelte.config.js` to enable polling:

```js
/** @type {import('@sveltejs/kit').Config} */
const config = {
  ...

  server: {
    watch: {
      usePolling: true,
    },
  },
};

```

As mentioned [here](https://vitejs.dev/config/server-options.html), this will result in high CPU usage, but will at least tell us if the problem lies there or not.

If it doesn't change anything, that's probably not the problem. Try editing `src-svelte/src/app.html` to see if Vite detects that. If it does:

```
5:24:50 AM [vite] page reload src/app.html
```

then we know the problem is something else. Try to edit other files. If they don't work either, try running the Vite server separately by editing `src-tauri/tauri.conf.json`, from

```json
{
  "build": {
    "beforeDevCommand": "yarn workspace gui dev",
    "beforeBuildCommand": "yarn workspace gui build",
    ...
  }
  ...
}  
```

to

```json
{
  "build": {
    "beforeBuildCommand": "yarn workspace gui build",
    ...
  }
  ...
}  
```

 opening the page in the browser instead. If you now see

```
5:59:47 AM [vite] hmr update /src/routes/+page.svelte
```

You know the problem is with the Tauri connection to Vite. Try deleting everything inside the `src-svelte/build` folder, but keep it as a folder so that you don't get the error

```
   Compiling zamm v0.0.0 (/root/zamm/src-tauri)
error: proc macro panicked
  --> src/main.rs:35:14
   |
35 |         .run(tauri::generate_context!())
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: message: The `distDir` configuration is set to `"../dist"` but this path doesn't exist
```

when building the project.

```bash
$ rm -rf src-svelte/build
$ mkdir src-svelte/build
```

If you start the project up again and see

```bash
$ yarn tauri dev
yarn run v1.22.19
$ tauri dev
        Info Watching /root/zamm/src-tauri for changes...
   Compiling zamm v0.0.0 (/root/zamm/src-tauri)
    Finished dev [unoptimized + debuginfo] target(s) in 10.16s
Connected to DB at /root/.local/share/zamm/zamm.sqlite3
Xlib:  extension "XInputExtension" missing on display ":1.0".
Asset `index.html` not found; fallback to index.html.html
Asset `index.html` not found; fallback to index.html/index.html
Asset `index.html` not found; fallback to index.html
AssetNotFound("index.html")
```

that means it is truly not connecting to the Vite server. [This issue](https://github.com/tauri-apps/tauri/issues/3082) explains why: [our fix](./e2e-testing.md) aroudn enabling the "custom-protocol" feature for end-to-end tests is interfering with the dev environment. Now that we have a better understanding of how this works, let's fix it properly by moving the custom protocol back:

```toml
[dependencies]
tauri = { version = "1.4", features = [ "shell-sidecar", "shell-open", "process-command-api" ] }
...

[features]
# this feature is used for production builds or when `devPath` points to the filesystem
# DO NOT REMOVE!!
custom-protocol = ["tauri/custom-protocol"]
```

This time we edit `src-tauri/Makefile` to add the `custom-protocol` in there:

```Makefile
target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cargo build --release --features custom-protocol
	touch target/release/zamm
```

If you see an error such as

```
Error: Not found: /node_modules/@sveltejs/kit/src/runtime/client/start.js
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:483:13)
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:285:5)
    at #options.hooks.handle (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/index.js:56:56)
    at Module.respond (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:282:40)
    at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
Error: Not found: /node_modules/vite/dist/client/env.mjs
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:483:13)
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:285:5)
    at #options.hooks.handle (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/index.js:56:56)
    at Module.respond (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:282:40)
    at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
Error: Not found: /@fs/root/zamm/.svelte-kit/generated/client/app.js
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:483:13)
    at resolve (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:285:5)
    at #options.hooks.handle (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/index.js:56:56)
    at Module.respond (/root/zamm/node_modules/@sveltejs/kit/src/runtime/server/respond.js:282:40)
    at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
```

You may need to run `yarn svelte-kit sync` in `src-svelte`.

## Could not compile

If you see an error such as

```
...
          /usr/bin/ld: /root/zamm/src-tauri/target/debug/deps/zamm-6fbba8229eceab27.321vbrwfobcrnnws.rcgu.o: in function `zamm::main::{{closure}}':
          /root/zamm/src-tauri/src/main.rs:34: undefined reference to `zamm::commands::greet'
          /usr/bin/ld: /root/zamm/src-tauri/target/debug/deps/zamm-6fbba8229eceab27.321vbrwfobcrnnws.rcgu.o: in function `tauri::command::private::ResultTag::block':
          /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/tauri-1.4.1/src/command.rs:227: undefined reference to `core::result::Result<T,E>::map_err'
          /usr/bin/ld: /root/zamm/src-tauri/target/debug/deps/zamm-6fbba8229eceab27.321vbrwfobcrnnws.rcgu.o: in function `zamm::main::{{closure}}':
          /root/zamm/src-tauri/src/main.rs:34: undefined reference to `core::ptr::drop_in_place<core::result::Result<alloc::string::String,zamm::commands::Error>>'
          /usr/bin/ld: /root/zamm/src-tauri/target/debug/deps/zamm-6fbba8229eceab27.50siejulecamy1hm.rcgu.o: in function `zamm::main::export':
          /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/specta-1.0.5/src/functions/mod.rs:206: undefined reference to `zamm::commands::greet'
          /usr/bin/ld: /root/zamm/src-tauri/target/debug/deps/zamm-6fbba8229eceab27: hidden symbol `_ZN4zamm8commands5greet17h347d7408ef0408e1E' isn't defined
          /usr/bin/ld: final link failed: bad value
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `zamm` (bin "zamm") due to previous error
```

try cleaning your Rust directory:

```bash
$ rm -rf src-tauri/target
```

Afterwards, try again.
