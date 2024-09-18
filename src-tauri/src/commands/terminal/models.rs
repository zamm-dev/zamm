use crate::commands::errors::ZammResult;
use crate::models::asciicasts::AsciiCastData;
use async_trait::async_trait;
use rexpect::spawn;

#[async_trait]
pub trait Terminal: Send + Sync {
    async fn run_command(&mut self, command: &str) -> ZammResult<String>;
    fn get_cast(&self) -> &AsciiCastData;
}

pub struct ActualTerminal {
    pub session_data: AsciiCastData,
}

impl ActualTerminal {
    pub fn new() -> Self {
        Self {
            session_data: AsciiCastData::new(),
        }
    }
}

#[async_trait]
impl Terminal for ActualTerminal {
    async fn run_command(&mut self, command: &str) -> ZammResult<String> {
        self.session_data.header.command = Some(command.to_string());

        let starting_time = chrono::Utc::now();
        self.session_data.header.timestamp = Some(starting_time);

        let mut session = spawn(command, None)?;
        let output = session.exp_eof()?;
        let output_time = chrono::Utc::now();
        let duration = output_time - starting_time;
        self.session_data.entries.push(asciicast::Entry {
            time: duration.num_milliseconds() as f64 / 1000.0,
            event_type: asciicast::EventType::Output,
            event_data: output.clone(),
        });
        Ok(output)
    }

    fn get_cast(&self) -> &AsciiCastData {
        &self.session_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capture_command_output() {
        let mut terminal = ActualTerminal::new();
        #[cfg(target_os = "windows")]
        let output = terminal
            .run_command("cmd /C \"echo hello world\"")
            .await
            .unwrap();
        #[cfg(not(target_os = "windows"))]
        let output = terminal.run_command("echo hello world").await.unwrap();
        assert_eq!(output, "hello world\r\n");
    }

    #[tokio::test]
    async fn test_capture_interleaved_output() {
        let mut terminal = ActualTerminal::new();
        let output = terminal
            .run_command("python api/sample-terminal-sessions/interleaved.py")
            .await
            .unwrap();

        assert_eq!(output, "stdout\r\nstderr\r\nstdout\r\n");
    }
}
