# Starting a new Tauri project

## Prerequisites

Set up NodeJS, Rust, and Tauri as described in:

- [`nodejs.md`](/zamm/resources/tutorials/setup/dev/nodejs.md)
- [`rust.md`](/zamm/resources/tutorials/setup/dev/rust.md)
- [`tauri.md`](/zamm/resources/tutorials/setup/dev/tauri.md)


Then set up Tauri app creation:

```bash
$ cargo install create-tauri-app --locked
    Updating crates.io index
  Downloaded create-tauri-app v3.7.2
  Downloaded 1 crate (262.6 KB) in 2.07s
...
   Compiling include-flate v0.2.0
   Compiling rust-embed v6.8.1
   Compiling create-tauri-app v3.7.2
    Finished release [optimized] target(s) in 27.98s
  Installing /home/amos/.asdf/installs/rust/1.71.0/bin/cargo-create-tauri-app
   Installed package `create-tauri-app v3.7.2` (executable `cargo-create-tauri-app`)
```

Finally, run `create-tauri-app`

```bash
$ cargo create-tauri-app
✔ Project name · zamm
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm)
✔ Choose your package manager · yarn
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd zamm
  yarn
  yarn tauri dev
```

Follow the instructions:

```bash
$ cd zamm
$ yarn
yarn install v1.22.15
info No lockfile found.
[1/4] Resolving packages...

...

   Compiling webkit2gtk v0.18.2
    Finished dev [unoptimized + debuginfo] target(s) in 1m 41s
Could not determine the accessibility bus address
```

Note that the command does not exit.

Now edit `src-tauri/tauri.conf.json` to change the identifier from the default:

```json
{
      ...
      "active": true,
      "targets": "all",
      "identifier": "com.tauri.dev",
      "icon": [
        "icons/32x32.png",
        ...
      ]
}
```

to something else:

```json
{
      ...
      "active": true,
      "targets": "all",
      "identifier": "dev.zamm",
      "icon": [
        "icons/32x32.png",
        ...
      ]
}
```

Now run this in a new window:

```bash
$ cargo tauri build
...
    Finished 2 bundles at:
        /home/amos/projects/zamm/ui/zamm/src-tauri/target/release/bundle/deb/zamm_0.0.0_amd64.deb
        /home/amos/projects/zamm/ui/zamm/src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage
```

Finally, you can run the generated AppImage.

### Fuse Error

If it errors out with this message:

```bash
$ src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage
dlopen(): error loading libfuse.so.2

AppImages require FUSE to run. 
You might still be able to extract the contents of this AppImage 
if you run it with the --appimage-extract option. 
See https://github.com/AppImage/AppImageKit/wiki/FUSE 
for more information
```

then install `fuse` and rerun the AppImage.

```bash
$ sudo apt install fuse
...
Processing triggers for man-db (2.10.2-1) ...
Processing triggers for libc-bin (2.35-0ubuntu3.1) ...
/sbin/ldconfig.real: /usr/lib/wsl/lib/libcuda.so.1 is not a symbolic link
```

### Makefile edit

You may want to enter this into the Tauri Makefile:

```Makefile
target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cargo build --release
  touch target/release/zamm  # cargo build might not do anything
```

Alternatively, you can just build the target without specifying dependencies, because Cargo is pretty performant anyways when everything is up to date.

### GLIBC error

If on the other hand you see the problem

```bash
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by zamm)
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by zamm)
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by zamm)
...
```

then that means you should follow the instructions at [`cross.md`](zamm/resources/tutorials/setup/dev/cross.md) for compiling in a way that's compatible across Linux versions. We do this because static linking with musl is [not currently supported](https://github.com/tauri-apps/tauri/issues/5466) by Tauri, and compiling with an older version of glibc is necessary due to [the way GLIBC handles backwards-compatibility](https://developers.redhat.com/blog/2019/08/01/how-the-gnu-c-library-handles-backward-compatibility).

Afterwards, edit `src-tauri/tauri.conf.json` according to [its JSON schema](https://github.com/tauri-apps/tauri/blob/4cb51a2/tooling/cli/schema.json), because this feature appears to be undocumented anywhere except for [this issue](https://github.com/tauri-apps/tauri/issues/4255):

```json
{
  "build": {
    ...
    "runner": "cross"
  },
  ...
}
```

If you've edited `src-tauri/Makefile` as mentioned above, edit it again to use `cross` instead of `cargo`:

```Makefile
...

target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cross build --release --features custom-protocol
	touch target/release/zamm
```

Note that the build fails now because `src-svelte` is in another folder and therefore missing from the Docker container:

```
$ make
...
   Compiling tauri-specta v1.0.2
   Compiling zamm v0.0.0 (/project)
error: proc macro panicked
  --> src/main.rs:42:14
   |
42 |         .run(tauri::generate_context!())
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: message: The `distDir` configuration is set to `"../src-svelte/build"` but this path doesn't exist

warning: unused import: `std::env`
  --> src/main.rs:13:5
   |
13 | use std::env;
   |     ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `zamm` (bin "zamm") generated 1 warning
error: could not compile `zamm` (bin "zamm") due to previous error; 1 warning emitted
make: *** [Makefile:4: target/release/zamm] Error 101
```

To fix this, we can mount additional volumes for cross as described [here](https://github.com/cross-rs/cross#mounting-volumes-into-the-build-environment). However, as described by [this issue](https://github.com/cross-rs/cross/issues/388), that doesn't actually work when we try it out and debug with `cross -vv ...`. Therefore, we use mkhattab's workaround in [this comment](https://github.com/cross-rs/cross/issues/388#issuecomment-1076862505) and edit our Makefile again:

```Makefile
...

target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	DOCKER_OPTS="-v $(realpath ../src-svelte):/src-svelte" cross -vv build --release --features custom-protocol
	touch target/release/zamm

...
```

Now compilation works:

```bash
$ make
...
thiserror=/target/x86_64-unknown-linux-gnu/release/deps/libthiserror-3f9911903402c34c.rlib --extern uuid=/target/x86_64-unknown-linux-gnu/release/deps/libuuid-f91a98274275e90b.rlib --cfg desktop`
    Finished release [optimized] target(s) in 16.23s
touch target/release/zamm
```

Now we can remove the `-vv` option from cross debugging in the `src-tauri/Makefile`. However, we try running `make` in the overall directory, and note that it still fails on our local machine:

```bash
$ rsync -P -e ssh hetzner:/root/zamm/src-tauri/target/release/zamm ~/Downloads
zamm
     13,131,960 100%    2.30MB/s    0:00:05 (xfr#1, to-chk=0/1)
$ ./zamm
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by ./zamm)
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by ./zamm)
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by ./zamm)

```

Try running cross again inside `src-tauri` to confirm:

```bash
$ cross clean
$ make
```

Now it does work on the local development machine. This means that `cargo tauri build` is rebuilding the executable without cross.

### GStreamer error

If you see the screen go blank as soon as audio is played, and this error pop up:

```
GStreamer element appsink not found. Please install it
GStreamer element autoaudiosink not found. Please install it

(WebKitWebProcess:44034): GLib-GObject-WARNING **: 16:33:39.921: invalid (NULL) pointer instance

(WebKitWebProcess:44034): GLib-GObject-CRITICAL **: 16:33:39.921: g_signal_connect_data: assertion 'G_TYPE_CHECK_INSTANCE (instance)' failed

```

this is because you need to enable the media bundling as mentioned [here](https://tauri.app/v1/guides/building/linux/#appimage). This is also mentioned in [this issue](https://github.com/tauri-apps/tauri/issues/4642) and [this issue](https://github.com/tauri-apps/tauri/issues/4092). Edit `src-tauri/tauri.conf.json`:

```json
{
  ...
  "tauri": {
    ...
    "bundle": {
      ...
      "appimage": {
        "bundleMediaFramework": true
      },
      ...
    },
    ...
  }
}

```

If you try to build the app now, you might run into this error:

```
cargo tauri build
   Compiling zamm v0.0.0 (/__w/zamm-ui/zamm-ui/src-tauri)
    Finished release [optimized] target(s) in 25.34s
    Bundling zamm_0.0.0_amd64.deb (/__w/zamm-ui/zamm-ui/src-tauri/target/release/bundle/deb/zamm_0.0.0_amd64.deb)
    Bundling zamm_0.0.0_amd64.AppImage (/__w/zamm-ui/zamm-ui/src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage)
       Error failed to bundle project: error running appimage.sh
```

We find [this issue](https://github.com/tauri-apps/tauri/issues/5781), and make the build verbose as requested. We now see these errors:

```
-- Running input plugin: gstreamer -- 
[gstreamer/stdout] Error: patchelf not found
[gstreamer/stdout] 
[gstreamer/stdout] Usage: /zamm/src-tauri/target/release/bundle/appimage/linuxdeploy-plugin-gstreamer.sh --appdir <path to AppDir>
[gstreamer/stdout] 
[gstreamer/stdout] Bundles GStreamer plugins into an AppDir
[gstreamer/stdout] 
[gstreamer/stdout] Required variables:
[gstreamer/stdout]   LINUXDEPLOY=".../linuxdeploy" path to linuxdeploy (e.g., AppImage); set automatically when plugin is run directly by linuxdeploy
[gstreamer/stdout] 
[gstreamer/stdout] Optional variables:
[gstreamer/stdout]   GSTREAMER_INCLUDE_BAD_PLUGINS="1" (default: disabled; set to empty string or unset to disable)
[gstreamer/stdout]   GSTREAMER_PLUGINS_DIR="..." (directory containing GStreamer plugins; default: guessed based on main distro architecture)
[gstreamer/stdout]   GSTREAMER_HELPERS_DIR="..." (directory containing GStreamer helper tools like gst-plugin-scanner; default: guessed based on main distro architecture)
[gstreamer/stdout]   GSTREAMER_VERSION="1.0" (default: 1.0)
ERROR: Failed to run plugin: gstreamer (exit code: 2) 
       Error [tauri_cli] failed to bundle project: error running appimage.sh

```

We install `patchelf` as well in the base Docker image. It compiles, but we now see additional errors:

```
(WebKitWebProcess:68629): GStreamer-CRITICAL **: 12:29:58.529: gst_object_unref: assertion 'object != NULL' failed
GStreamer element audioconvert not found. Please install it
GStreamer element audioconvert not found. Please install it
GStreamer element audioresample not found. Please install it
GStreamer element audioresample not found. Please install it
GStreamer element volume not found. Please install it

(WebKitWebProcess:68629): GStreamer-CRITICAL **: 12:29:58.529: gst_bin_add_many: assertion 'GST_IS_ELEMENT (element_1)' failed

(WebKitWebProcess:68629): GStreamer-CRITICAL **: 12:29:58.529: gst_element_get_static_pad: assertion 'GST_IS_ELEMENT (element)' failed
```

If you want more logs, you can set the `GST_DEBUG` environment variable to 2 to see:

```
0:00:00.104933837 69728 0x5595712c4950 WARN     GST_ELEMENT_FACTORY gstelementfactory.c:456:gst_element_factory_make: no such element factory "autoaudiosink"!
0:00:00.104957714 69728 0x5595712c4950 WARN            webkitcommon GStreamerCommon.cpp:458:createPlatformAudioSink: GStreamer's autoaudiosink not found. Please check your gst-plugins-good installation

```

We actually install GStreamer as mentioned in [their docs](https://gstreamer.freedesktop.org/documentation/installing/on-linux.html?gi-language=c#install-gstreamer-on-ubuntu-or-debian). In the Dockerfile, this looks like:

```Dockerfile
RUN apt install -y libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

Now when we try to play a sound on the app, it takes a few seconds to load, but the logs finally come through:

```
date_time_format_iso8601
Failed to load module: /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so
0:00:00.005019519 72221 0x55be8fd9baf0 WARN                  ladspa gstladspa.c:507:plugin_init:<plugin1> no LADSPA plugins found, check LADSPA_PATH
...
0:00:00.006583997 72292 0x561e63a5cb80 WARN                 default gstsf.c:98:gst_sf_create_audio_template_caps: format 0x190000: 'WVE (Psion Series 3)' is not mapped
0:00:00.006195039 72342 0x55592d515a90 ERROR                x264enc gstx264enc.c:163:load_x264: Failed to load '/usr/lib/x86_64-linux-gnu/x264-10bit/libx264.so.152'
0:00:10.559651313 72057 0x55e74388b400 WARN    webkitregistryscanner GStreamerRegistryScanner.cpp:160:hasElementForMediaType: All video decoder elements matching caps video/x-av1 are disallowed
0:00:10.561124458 72057 0x55e74388b400 WARN    webkitregistryscanner GStreamerRegistryScanner.cpp:160:hasElementForMediaType: All video encoder elements matching caps video/x-av1 are disallowed
0:00:10.595886072 72057 0x55e74388b400 WARN            uridecodebin gsturidecodebin.c:1409:gen_source_element:<uridecodebin0> error: No URI handler implemented for "tauri".

```

Only the last line repeats when we try to press a switch, so the others are likely red herrings.

We find that this is likely due to an existing issue around loading video or audio as assets. We leave [a comment](https://github.com/tauri-apps/tauri/issues/3725#issuecomment-1747970925) on the issue to describe the data we have on this. It is clear now that we must work around this issue instead.

We do so by first setting up Rodio as described in [`rodio.md`](/zamm/resources/tutorials/libraries/rust/rodio.md). We move the sound file to `src-tauri/sounds/switch.ogg`. Since this is our first time using the rodio crate, we define new errors in `src-tauri/src/commands/errors.rs`. We keep to one top-level error per external crate, so we define a new error type `RodioError`:

```rust
#[derive(thiserror::Error, Debug)]
pub enum RodioError {
    #[error(transparent)]
    Stream {
        #[from]
        source: rodio::StreamError,
    },
    #[error(transparent)]
    Decode {
        #[from]
        source: rodio::decoder::DecoderError,
    },
    #[error(transparent)]
    Play {
        #[from]
        source: rodio::PlayError,
    },
}
```

Then we nest this inside the existing `Error`:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error(transparent)]
    Rodio {
        #[from]
        source: RodioError,
    },
    ...
}
```

Unfortunately, `From` trait implementations are not transitive, so we manually implement the nested relation:

```rust
impl From<rodio::StreamError> for Error {
    fn from(err: rodio::StreamError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}

impl From<rodio::decoder::DecoderError> for Error {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}

impl From<rodio::PlayError> for Error {
    fn from(err: rodio::PlayError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}
```

Now in `src-tauri/src/commands/mod.rs`, export our new function:

```rust
...
mod sounds;

...
pub use sounds::play_sound;
```

And in `src-tauri/src/main.rs`, add this new function:

```rust
...
use commands::{..., play_sound};

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![..., play_sound],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();

    ...

    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![..., play_sound])
        ...;
```

`src-svelte/src/lib/bindings.ts` will automatically be edited on the next run of the development app, with the new function

```ts
export function playSound(sound: Sound) {
    return invoke()<null>("play_sound", { sound })
}
```

We therefore edit `src-svelte/src/lib/Switch.svelte` to stop doing

```ts
  import clickSound from "$lib/sounds/switch.ogg";

  function playClick() {
    ...
    const audio = new Audio(clickSound);
    audio.volume = 0.05;
    audio.play();
    ...
  }
```

and instead do:

```ts
  import { playSound } from "./bindings";

  function playClick() {
    ...
    playSound("Switch");
    ...
  }
```

while editing the sound file to be `0.05` of its original volume. We could also use Rodio's `Sink` instead, but that introduces extra complexity that is unnecessary for now.

Next, as usual whenever we add an API call, we test it by creating a sample call file at `src-tauri/api/sample-calls/play_sound-switch.yaml`:

```yaml
request: ["play_sound", "\"Switch\""]
response: null

```

and then we add a test in `src-tauri/src/commands/sounds.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::setup::api_keys::{ApiKey, Source};
    use std::sync::Mutex;

    use std::fs;

    fn parse_sound(request_str: &str) -> Sound {
        serde_json::from_str(request_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_play_sound_sample(file_prefix: &str) {
        let greet_sample = read_sample(file_prefix);
        assert_eq!(greet_sample.request.len(), 2);
        assert_eq!(greet_sample.request[0], "play_sound");

        let requested_sound = parse_sound(&greet_sample.request[1]);
        let actual_result = play_sound(requested_sound);
        let actual_json = serde_json::to_string(&actual_result).unwrap();
        let expected_json = greet_sample.response;
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_get_empty_keys() {
        check_play_sound_sample(
            "./api/sample-calls/play_sound-switch.yaml",
        );
    }
}

```

and edit `src-svelte/src/lib/Switch.test.ts` to confirm that the frontend is calling the backend in the same way. In doing so, we realize that the API call from the frontend's perspective actually looks like this:

```yaml
request:
  - play_sound
  - >
    {
      "sound": "Switch"
    }
response: "null"

```

The switch test file now looks like this:

```ts
import { expect, test, vi, type SpyInstance } from "vitest";
...
import fs from "fs";
import yaml from "js-yaml";
import { Convert, type SampleCall } from "$lib/sample-call";

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);

describe("Switch", () => {
  let switchCall: SampleCall;
  let switchRequest: (string | Record<string, string>)[];
  let spy: SpyInstance;

  beforeAll(() => {
    const sample_call_yaml = fs.readFileSync("../src-tauri/api/sample-calls/play_sound-switch.yaml", "utf-8");
    const sample_call_json = JSON.stringify(yaml.load(sample_call_yaml));
    switchCall = Convert.toSampleCall(sample_call_json);
    switchRequest = switchCall.request;
    switchRequest[1] = JSON.parse(switchCall.request[1]);
  });

  beforeEach(() => {
    spy = vi.spyOn(window, "__TAURI_INVOKE__");
    const response = JSON.parse(switchCall.response);
    tauriInvokeMock.mockResolvedValueOnce(response);
  });

  ...

    test("plays clicking sound during toggle", async () => {
    render(Switch, {});
    expect(spy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(spy).toHaveBeenLastCalledWith(...switchRequest);
  });

  test("does not play clicking sound when sound off", async () => {
    render(Switch, {});
    soundOn.update(() => false);
    expect(spy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(spy).not.toHaveBeenCalled();
  });
});

```

We now also edit `src-svelte/src/routes/settings/Settings.test.ts` to have the same test setup and the test:

```ts
  test("can toggle sound on and off", async () => {
    render(Settings, {});
    expect(get(soundOn)).toBe(true);
    expect(spy).not.toHaveBeenCalled();

    const soundSwitch = screen.getByLabelText("Sounds");
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(false);
    expect(spy).not.toHaveBeenCalled();

    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(true);
    expect(spy).toBeCalledTimes(1);
    expect(spy).toHaveBeenLastCalledWith(...switchRequest);
  });
```

Finally, we should also edit `src-tauri/src/commands/sounds.rs`:

```rust
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        sound: Sound,
    }

    fn parse_request(request_str: &str) -> PlaySoundRequest {
        serde_json::from_str(request_str).unwrap()
    }

    ...

    fn check_play_sound_sample(file_prefix: &str) {
        ...

        let request = parse_request(&greet_sample.request[1]);
        let actual_result = play_sound(request.sound);
        ...
    }
```

`clippy` warns us about assigning a unit type, but we do want the type check here. There is an [outstanding issue](https://github.com/rust-lang/rust-clippy/issues/9048) about this, so we simply ignore the rule for now:

```rust
        #[allow(clippy::let_unit_value)]
        let actual_result = play_sound(request.sound);
```

We now get rid of our previous changes by removnig GStreamer from the Dockerfile and re-disabling `bundleMediaFramework`.

#### Bundle error

If you see

```
    Finished release [optimized] target(s) in 2.40s
    Bundling zamm_0.0.0_amd64.deb (/home/amos/Documents/ui/zamm/src-tauri/target/release/bundle/deb/zamm_0.0.0_amd64.deb)
    Bundling zamm_0.0.0_amd64.AppImage (/home/amos/Documents/ui/zamm/src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage)
       Error failed to bundle project: error running appimage.sh
```

then you may want to turn verbose build mode on with `cargo tauri build --verbose` as noted [here](https://github.com/tauri-apps/tauri/issues/5781).

##### Makefile

To allow passing verbose mode into the Makefile, you can follow [this answer](https://stackoverflow.com/a/2214593) and do:

```Makefile
build: python svelte rust
	cargo tauri build $(ARGS)

...

build-docker:
	docker run --rm -v $(CURRENT_DIR):/zamm -w /zamm $(BUILD_IMAGE) make copy-docker-deps build ARGS=$(ARGS)
```

#### ssl/private permissions error

If you turned verbose build mode on above, and you see

```
++ dirname '{}'
+ find -L /usr/lib /usr/lib32 /usr/lib64 /usr/libexec /usr/libx32 -name WebKitNetworkProcess -exec mkdir -p . ';' -exec cp --parents '{}' . ';'
find: ‘/usr/lib/ssl/private’: Permission denied
+ true

```

note that this is fine. This is just [a symptom](https://github.com/triton-inference-server/server/issues/4030) of the find command skipping `/usr/lib/ssl/private` because it doesn't have permission to enter it, and as the link notes, is a red herring for any actual issues that exist.

Instead, if the last line of the log looks like this:

```
wget -q -4 -N https://raw.githubusercontent.com/tauri-apps/linuxdeploy-plugin-gstreamer/master/linuxdeploy-plugin-gstreamer.sh
```

the error may be if you lost network connectivity.

### Rebuilding target when frontend changes

You may find that making a purely frontend change does not trigger a target rebuild right now, meaning that the executable will be misleadingly stuck on a previous version of the frontend for the end-to-end tests. To fix this, add the Svelte built to the Makefile dependency, like so:

```Makefile
target/release/zamm: ./Cargo.toml ../src-svelte/build $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	...
```

### Continuing on...

Then in the main project Makefile:

```Makefile
build: svelte rust
	cargo tauri build

rust:
	cd src-tauri && make

svelte:
	cd src-svelte && make
```

where `src-tauri` and `src-svelte` are your respective directories for Svelte and Tauri code. If you want a global test command as well, add this:

```Makefile
test:
	cd src-python && make test
	cd src-svelte && make test
	cd src-tauri && make test
	yarn e2e-test
```

If you have followed the instructions for a [Svelte Makefile](zamm/resources/tutorials/setup/makefile/svelte.md), which would enable the above, you should get rid of the redundant `beforeBuildCommand` in `src-tauri/tauri.conf.json`.

## Project dev tooling setup

Svelte setup:

Follow the instructions at:

- [`eslint.md`](/zamm/resources/tutorials/setup/tools/svelte/eslint.md)
- [`prettier.md`](/zamm/resources/tutorials/setup/tools/svelte/prettier.md)

Then for pre-commit, follow the instructions at

- [`pre-commit.md`](/zamm/resources/tutorials/setup/repo/pre-commit/pre-commit.md)
- [`cargo.md`](/zamm/resources/tutorials/setup/repo/pre-commit/cargo.md)
- [`svelte.md`](/zamm/resources/tutorials/setup/repo/pre-commit/svelte.md)

### Debugging

To allow the web inspector in a final build, add the `"devtools"` feature to `Cargo.toml` as mentioned [here](https://github.com/tauri-apps/tauri/discussions/3059).

## Testing setup

Follow the instructions at

- [`e2e-testing.md`](./e2e-testing.md)
