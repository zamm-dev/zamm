use rodio::{source::Source, Decoder, OutputStream};
use serde::{Deserialize, Serialize};
use specta::specta;
use specta::Type;

use std::include_bytes;
use std::io::Cursor;
use std::thread;

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
    use crate::sample_call::SampleCall;
    use crate::test_helpers::{DirectReturn, SampleCallTestCase, SideEffectsHelpers};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PlaySoundRequest {
        sound: Sound,
        volume: f32,
        speed: f32,
    }

    struct PlaySoundTestCase {
        // pass
    }

    impl SampleCallTestCase<PlaySoundRequest, ()> for PlaySoundTestCase {
        const EXPECTED_API_CALL: &'static str = "play_sound";
        const CALL_HAS_ARGS: bool = true;

        async fn make_request(
            &mut self,
            args: &Option<PlaySoundRequest>,
            _: &SideEffectsHelpers,
        ) {
            let actual_args = args.as_ref().unwrap().clone();
            play_sound(actual_args.sound, actual_args.volume, actual_args.speed);
        }

        fn serialize_result(&self, sample: &SampleCall, result: &()) -> String {
            DirectReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&PlaySoundRequest>,
            result: &(),
        ) {
            DirectReturn::check_result(self, sample, args, result).await
        }
    }

    impl DirectReturn<PlaySoundRequest, ()> for PlaySoundTestCase {}

    async fn check_play_sound_sample(file_prefix: &str) {
        let mut test_case = PlaySoundTestCase {};
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_play_switch() {
        check_play_sound_sample("./api/sample-calls/play_sound-switch.yaml").await;
    }

    #[tokio::test]
    async fn test_play_whoosh() {
        check_play_sound_sample("./api/sample-calls/play_sound-whoosh.yaml").await;
    }
}
