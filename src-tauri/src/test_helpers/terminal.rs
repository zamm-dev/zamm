use crate::commands::errors::ZammResult;
use crate::commands::terminal::{ActualTerminal, Terminal};
use crate::models::asciicasts::AsciiCastData;
use asciicast::EventType;

use either::Either::{self, Left, Right};
use std::thread::sleep;
use std::time::Duration;

pub struct TestTerminal {
    recording_file: String,
    terminal: Either<AsciiCastData, ActualTerminal>,
    entry_index: usize,
}

impl TestTerminal {
    pub fn new(recording_file: &str) -> Self {
        let terminal = match std::fs::metadata(recording_file) {
            Ok(_) => Either::Left(AsciiCastData::load(recording_file).unwrap()),
            Err(_) => Either::Right(ActualTerminal::new()),
        };
        Self {
            recording_file: recording_file.to_string(),
            terminal,
            entry_index: 0,
        }
    }

    pub fn set_entry_index(&mut self, index: usize) {
        self.entry_index = index;
    }

    fn next_entry(&mut self) -> &asciicast::Entry {
        match &self.terminal {
            Left(cast) => {
                let entry = &cast.entries[self.entry_index];
                self.entry_index += 1;
                entry
            }
            Right(_) => panic!("Expected recording"),
        }
    }
}

impl Drop for TestTerminal {
    fn drop(&mut self) {
        if let Right(terminal) = &self.terminal {
            terminal
                .get_cast()
                .unwrap()
                .save(&self.recording_file)
                .unwrap();
        }
    }
}

impl Terminal for TestTerminal {
    fn run_command(&mut self, command: &str) -> ZammResult<String> {
        match &mut self.terminal {
            Left(cast) => {
                let expected_command = cast.header.command.as_ref().unwrap();
                assert_eq!(command, expected_command);

                let entry = self.next_entry();
                assert_eq!(entry.event_type, EventType::Output);
                Ok(entry.event_data.clone())
            }
            Right(actual_terminal) => actual_terminal.run_command(command),
        }
    }

    fn read_updates(&mut self) -> ZammResult<String> {
        match &mut self.terminal {
            Left(_) => {
                let entry = self.next_entry();
                assert_eq!(entry.event_type, EventType::Output);
                Ok(entry.event_data.clone())
            }
            Right(actual_terminal) => actual_terminal.read_updates(),
        }
    }

    fn send_input(&mut self, input: &str) -> ZammResult<String> {
        match &mut self.terminal {
            Left(_) => {
                let input_entry = self.next_entry();
                assert_eq!(input_entry.event_type, EventType::Input);
                assert_eq!(input_entry.event_data, input);

                let output_entry = self.next_entry();
                assert_eq!(output_entry.event_type, EventType::Output);
                Ok(output_entry.event_data.clone())
            }
            Right(actual_terminal) => actual_terminal.send_input(input),
        }
    }

    fn get_cast(&self) -> ZammResult<AsciiCastData> {
        match &self.terminal {
            Left(cast) => Ok(AsciiCastData {
                header: cast.header.clone(),
                entries: cast.entries[..self.entry_index].to_vec(),
            }),
            Right(actual_terminal) => actual_terminal.get_cast(),
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
            .run_command("date \"+%A %B %e, %Y %R %z\"")
            .unwrap();
        assert_eq!(output, "Friday September 20, 2024 18:23 +0700\r\n");
    }

    #[tokio::test]
    async fn test_terminal_pause() {
        let mut terminal = TestTerminal::new("api/sample-terminal-sessions/pause.cast");
        terminal
            .run_command("python api/sample-terminal-sessions/pause.py")
            .unwrap();

        sleep(Duration::from_millis(1_000));
        let output = terminal.read_updates().unwrap();
        assert_eq!(output, "Second\r\n");
    }

    #[tokio::test]
    async fn test_interactivity() {
        let mut terminal = TestTerminal::new("api/sample-terminal-sessions/bash.cast");
        terminal.run_command("bash").unwrap();
        let output = terminal
            .send_input("python api/sample-terminal-sessions/interleaved.py\n")
            .unwrap();
        assert_eq!(
            output,
            "python api/sample-terminal-sessions/interleaved.py\r\nstdout\r\nstderr\r\nstdout\r\nbash-3.2$ "
        );
    }

    #[tokio::test]
    async fn test_windows_interactivity() {
        let mut terminal =
            TestTerminal::new("api/sample-terminal-sessions/windows.cast");
        terminal.run_command("cmd").unwrap();
        terminal.send_input("dir\r\n").unwrap();
        let output = terminal.send_input("echo %cd%\r\n").unwrap();
        assert!(output.contains("src-tauri"));
    }
}
