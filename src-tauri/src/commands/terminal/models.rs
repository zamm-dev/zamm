use crate::commands::errors::ZammResult;
use crate::models::asciicasts::AsciiCastData;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::DateTime;
use rexpect::session::PtySession;
use rexpect::spawn;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

#[async_trait]
pub trait Terminal: Send + Sync {
    async fn run_command(&mut self, command: &str) -> ZammResult<String>;
    fn read_updates(&mut self) -> ZammResult<String>;
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

    fn start_time(&self) -> ZammResult<DateTime<chrono::Utc>> {
        let result = self
            .session_data
            .header
            .timestamp
            .ok_or(anyhow!("No timestamp"))?;
        Ok(result)
    }

    fn drain_read_buffer(&mut self) -> ZammResult<String> {
        let mut output = String::new();
        if let Some(session) = self.session.as_mut() {
            let reader = &mut session.lock()?.reader;
            while let Some(chunk) = reader.try_read() {
                output.push(chunk);
            }
        }
        Ok(output)
    }

    #[cfg(test)]
    async fn send_input(&mut self, input: &str) -> ZammResult<String> {
        let session_mutex = self.session.as_mut().ok_or(anyhow!("No session"))?;
        {
            let mut session = session_mutex.lock()?;
            session.send(input)?;
            session.flush()?;
        }

        let relative_time = chrono::Utc::now() - self.start_time()?;
        self.session_data.entries.push(asciicast::Entry {
            time: relative_time.num_milliseconds() as f64 / 1000.0,
            event_type: asciicast::EventType::Input,
            event_data: input.to_string(),
        });

        self.read_updates()
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

    fn read_updates(&mut self) -> ZammResult<String> {
        let output = {
            let session_mutex = self.session.as_mut().ok_or(anyhow!("No session"))?;
            let output_until_eof = {
                let mut session = session_mutex.lock()?;
                session.exp_eof().ok()
            };
            match output_until_eof {
                Some(full_output) => full_output,
                None => {
                    let mut interim_output = String::new();
                    loop {
                        sleep(Duration::from_millis(100));
                        let new_output = self.drain_read_buffer()?;
                        if new_output.is_empty() {
                            break;
                        }
                        interim_output.push_str(&new_output);
                    }
                    interim_output
                }
            }
        };

        if !output.is_empty() {
            let output_time = chrono::Utc::now();
            let relative_time = output_time - self.start_time()?;
            self.session_data.entries.push(asciicast::Entry {
                time: relative_time.num_milliseconds() as f64 / 1000.0,
                event_type: asciicast::EventType::Output,
                event_data: output.clone(),
            });
        }
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
        assert_eq!(terminal.get_cast().entries.len(), 1);
    }

    #[tokio::test]
    async fn test_capture_interleaved_output() {
        let mut terminal = ActualTerminal::new();
        let output = terminal
            .run_command("python api/sample-terminal-sessions/interleaved.py")
            .await
            .unwrap();

        assert_eq!(output, "stdout\r\nstderr\r\nstdout\r\n");
        assert_eq!(terminal.get_cast().entries.len(), 1);
    }

    #[tokio::test]
    async fn test_capture_output_without_blocking() {
        let mut terminal = ActualTerminal::new();
        let output = terminal.run_command("bash").await.unwrap();

        assert!(output.ends_with("bash-3.2$ "), "Output: {}", output);
        assert_eq!(terminal.get_cast().entries.len(), 1);
    }

    #[tokio::test]
    async fn test_no_entry_on_empty_capture() {
        let mut terminal = ActualTerminal::new();
        terminal.run_command("bash").await.unwrap();
        terminal.read_updates().unwrap();
        assert_eq!(terminal.get_cast().entries.len(), 1);
    }

    #[tokio::test]
    async fn test_capture_interaction() {
        let mut terminal = ActualTerminal::new();
        terminal.run_command("bash").await.unwrap();

        let output = terminal
            .send_input("python api/sample-terminal-sessions/interleaved.py\n")
            .await
            .unwrap();
        assert_eq!(output, "stdout\r\nstderr\r\nstdout\r\nbash-3.2$ ");
        assert_eq!(terminal.get_cast().entries.len(), 3);
    }
}
