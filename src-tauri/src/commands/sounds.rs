use rodio::{source::Source, Decoder, OutputStream};
use serde::{Deserialize, Serialize};
use specta::specta;
use specta::Type;

use std::include_bytes;
use std::io::Cursor;
use std::thread;

use crate::commands::errors::ZammResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Sound {
    Switch,
    Whoosh,
}

#[tauri::command(async)]
#[specta]
pub fn play_sound(sound: Sound, volume: f32, speed: f32) {
    thread::spawn(move || {
        if let Err(e) = play_sound_async(sound, volume, speed) {
            eprintln!("Error playing sound: {}", e);
        }
    });
}

fn play_sound_async(sound: Sound, volume: f32, speed: f32) -> ZammResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let embedded_sound: &[u8] = match sound {
        Sound::Switch => include_bytes!("../../sounds/switch.ogg"),
        Sound::Whoosh => include_bytes!("../../sounds/whoosh.ogg"),
    };
    let cursor = Cursor::new(embedded_sound);
    let source = Decoder::new(cursor)?.amplify(volume).speed(speed);
    stream_handle.play_raw(source.convert_samples())?;
    thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_direct_test_case};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        sound: Sound,
        volume: f32,
        speed: f32,
    }

    async fn make_request_helper(args: &PlaySoundRequest, _: &SideEffectsHelpers) {
        play_sound(args.sound, args.volume, args.speed);
    }

    impl_direct_test_case!(PlaySoundTestCase, play_sound, true, PlaySoundRequest, ());

    check_sample!(
        PlaySoundTestCase,
        test_switch,
        "./api/sample-calls/play_sound-switch.yaml"
    );

    check_sample!(
        PlaySoundTestCase,
        test_whoosh,
        "./api/sample-calls/play_sound-whoosh.yaml"
    );
}
