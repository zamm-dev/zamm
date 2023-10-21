# Using Rodio to play audio

Install

```bash
$ apt install libasound2-dev
$ cargo add rodio
```

If you don't install `libasound2-dev` first, you'll get a build error such as:

```
  --- stderr
  thread 'main' panicked at '`PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" PKG_CONFIG_ALLOW_SYSTEM_LIBS="1" "pkg-config" "--libs" "--cflags" "alsa"` did not exit successfully: exit status: 1
  error: could not find system library 'alsa' required by the 'alsa-sys' crate

  --- stderr
  Package alsa was not found in the pkg-config search path.
  Perhaps you should add the directory containing `alsa.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'alsa' found
  ', /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alsa-sys-0.3.1/build.rs:13:18
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Then do something such as

```rust
use crate::python_api::{GreetArgs, GreetResponse};

use specta::specta;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


use crate::commands::api::{process, SidecarExecutor, SidecarExecutorImpl};
use crate::commands::errors::ZammResult;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Sound {
    Switch,
}

#[tauri::command]
#[specta]
pub fn play_sound(sound: Sound) -> ZammResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let embedded_sound = match sound {
        Sound::Switch => include_bytes!("../../sounds/switch.ogg"),
    };
    let cursor = Cursor::new(embedded_sound);
    let source = Decoder::new(cursor)?;
    stream_handle.play_raw(source.convert_samples())?;
}

```

This doesn't actually play because as noted in [this Reddit thread](https://www.reddit.com/r/rust/comments/14xjgos/having_issues_with_rodio_crate_getting_no_audio/), [the documentation](https://docs.rs/rodio/0.17.1/rodio/struct.OutputStream.html) for `rodio::OutputStream` says:

> If this is dropped playback will end & attached OutputStreamHandles will no longer work.

Instead, we follow the example in the index and add a `thread::sleep(std::time::Duration::from_secs(1));`. This now works, but blocks the entire main thread. If you're using Tauri, this means the UI is blocked as well, as the Webkit renderer runs on the main thread. To fix this, we can use `std::thread::spawn` to spawn a new thread to play the audio.

```rust
#[tauri::command]
#[specta]
pub fn play_sound(sound: Sound) -> ZammResult<()> {
    thread::spawn(move || {
        if let Err(e) = play_sound_async(sound) {
            eprintln!("Error playing sound: {}", e);
        }
    });

    Ok(())
}

fn play_sound_async(sound: Sound) -> ZammResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let embedded_sound = match sound {
        Sound::Switch => include_bytes!("../../sounds/switch.ogg"),
    };
    let cursor = Cursor::new(embedded_sound);
    let source = Decoder::new(cursor)?;
    stream_handle.play_raw(source.convert_samples())?;
    thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
```

In fact, here we don't even need the `ZammResult` anymore in the public function.

## Controlling volume

Call `amplify` on the sound source, like so:

```rust
let source = Decoder::new(cursor)?.amplify(volume);
```

## Docker

If you're using this library with Docker, remember to update your Docker image to include the installation of `libasound2-dev`.

If you're using GitHub CI, remember to update the CI build environment as well.
