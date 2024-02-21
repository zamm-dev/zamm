use rodio::{source::Source, Decoder, OutputStream};
use serde::{Deserialize, Serialize};
use specta::specta;
use specta::Type;

use std::include_bytes;
use std::thread;

use std::io::Cursor;

use crate::commands::errors::ZammResult;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
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
    use crate::test_helpers::SampleCallTestCase;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        sound: Sound,
        volume: f32,
        speed: f32,
    }

    struct PlaySoundTestCase {
        // pass
    }

    impl SampleCallTestCase<PlaySoundRequest> for PlaySoundTestCase {
        const EXPECTED_API_CALL: &'static str = "play_sound";
        const CALL_HAS_ARGS: bool = true;
    }

    fn check_play_sound_sample(file_prefix: &str) {
        let test_case = PlaySoundTestCase {};
        let result = test_case.check_sample_call(file_prefix);

        let request = result.args.unwrap();
        #[allow(clippy::let_unit_value)]
        let actual_result = play_sound(request.sound, request.volume, request.speed);
        let actual_json = serde_json::to_string(&actual_result).unwrap();
        let expected_json = result.sample.response.message;
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_play_switch() {
        check_play_sound_sample("./api/sample-calls/play_sound-switch.yaml");
    }

    #[test]
    fn test_play_whoosh() {
        check_play_sound_sample("./api/sample-calls/play_sound-whoosh.yaml");
    }
}
