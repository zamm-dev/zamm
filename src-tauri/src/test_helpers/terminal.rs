use crate::commands::errors::ZammResult;
use crate::commands::terminal::{command_args_to_string, ActualTerminal, Terminal};
use crate::models::asciicasts::AsciiCastData;
use asciicast::EventType;
use rvcr::VCRMode;

pub struct TestTerminal {
    mode: VCRMode,
    recording_file: String,
    cast: AsciiCastData,
}

impl TestTerminal {
    fn new(recording_file: &str) -> Self {
        let mode = match std::fs::metadata(recording_file) {
            Ok(_) => VCRMode::Replay,
            Err(_) => VCRMode::Record,
        };
        let cast = match mode {
            VCRMode::Record => AsciiCastData::new(),
            VCRMode::Replay => AsciiCastData::load(recording_file).unwrap(),
        };
        Self {
            mode,
            recording_file: recording_file.to_string(),
            cast,
        }
    }
}

impl Drop for TestTerminal {
    fn drop(&mut self) {
        if self.mode == VCRMode::Record {
            self.cast.save(&self.recording_file).unwrap();
        }
    }
}

impl Terminal for TestTerminal {
    async fn run_command(
        &mut self,
        command: &str,
        args: &[&str],
    ) -> ZammResult<String> {
        let actual_command = command_args_to_string(command, args);
        match self.mode {
            VCRMode::Record => {
                let mut actual_terminal = ActualTerminal::new();
                let actual_output = actual_terminal.run_command(command, args).await?;
                self.cast = actual_terminal.session_data.clone();
                Ok(actual_output)
            }
            VCRMode::Replay => {
                let expected_command = self.cast.header.command.as_ref().unwrap();
                assert_eq!(&actual_command, expected_command);

                assert_eq!(self.cast.entries.len(), 1);
                let entry = &self.cast.entries[0];
                assert_eq!(entry.event_type, EventType::Output);
                Ok(entry.event_data.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminal_replay() {
        let mut terminal = TestTerminal::new("api/sample-terminal-sessions/date.cast");
        let output = terminal
            .run_command("date", &["+%A %B %e, %Y %R %z"])
            .await
            .unwrap();
        assert_eq!(output, "Tuesday July  9, 2024 21:03 +0700");
    }
}
