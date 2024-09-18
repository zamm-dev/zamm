use crate::commands::errors::ZammResult;
use crate::models::asciicasts::AsciiCastData;
use anyhow::anyhow;
use async_trait::async_trait;
use rexpect::session::PtySession;
use rexpect::spawn;
use rexpect::ReadUntil;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Terminal: Send + Sync {
    async fn run_command(&mut self, command: &str) -> ZammResult<String>;
    fn get_cast(&self) -> &AsciiCastData;
}

pub struct ActualTerminal {
    pub session: Option<Arc<Mutex<PtySession>>>,
    pub session_data: AsciiCastData,
}

impl ActualTerminal {
    pub fn new() -> Self {
        Self {
            session: None,
            session_data: AsciiCastData::new(),
        }
    }

    fn read_updates(&mut self) -> ZammResult<String> {
        let session = self.session.as_mut().ok_or(anyhow!("No session"))?;
        let reader = &mut session.lock()?.reader;
        let output = match reader.read_until(&ReadUntil::EOF) {
            Ok((_, output)) => output,
            Err(e) => match e {
                rexpect::error::Error::Timeout { got, .. } => got,
                _ => return Err(e.into()),
            },
        };

        let output_time = chrono::Utc::now();
        let duration = output_time
            - self
                .session_data
                .header
                .timestamp
                .ok_or(anyhow!("No timestamp"))?;
        self.session_data.entries.push(asciicast::Entry {
            time: duration.num_milliseconds() as f64 / 1000.0,
            event_type: asciicast::EventType::Output,
            event_data: output.clone(),
        });
        Ok(output)
    }
}

#[async_trait]
impl Terminal for ActualTerminal {
    async fn run_command(&mut self, command: &str) -> ZammResult<String> {
        if self.session.is_some() {
            return Err(anyhow!("Session already started").into());
        }

        self.session_data.header.command = Some(command.to_string());

        let starting_time = chrono::Utc::now();
        self.session_data.header.timestamp = Some(starting_time);

        let session = spawn(command, Some(100))?;
        self.session = Some(Arc::new(Mutex::new(session)));

        let result = self.read_updates()?;
        Ok(result)
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

    #[tokio::test]
    async fn test_capture_output_without_blocking() {
        let mut terminal = ActualTerminal::new();
        let output = terminal.run_command("bash").await.unwrap();

        assert!(output.ends_with("bash-3.2$ "), "Output: {}", output);
    }
}
