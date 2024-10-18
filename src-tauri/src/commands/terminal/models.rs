use crate::commands::errors::ZammResult;
use crate::models::asciicasts::AsciiCastData;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::DateTime;
use portable_pty::{
    native_pty_system, Child, CommandBuilder, MasterPty, PtySize, SlavePty,
};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[async_trait]
pub trait Terminal: Send + Sync {
    async fn run_command(&mut self, command: &str) -> ZammResult<String>;
    fn read_updates(&mut self) -> ZammResult<String>;
    async fn send_input(&mut self, input: &str) -> ZammResult<String>;
    fn get_cast(&self) -> AsciiCastData;
}

struct PtySession {
    #[allow(dead_code)]
    master: Box<dyn MasterPty + Send>,
    #[allow(dead_code)]
    slave: Box<dyn SlavePty + Send>,
    child: Box<dyn Child + Send + Sync>,
    writer: Box<dyn Write + Send>,
    input_receiver: Receiver<char>,
    exit_code: Option<u32>,
}

impl PtySession {
    fn new(command: &str) -> ZammResult<Self> {
        let cmd_and_args = shlex::split(command)
            .ok_or_else(|| anyhow!("Failed to split command '{}'", command))?;
        let parsed_cmd = cmd_and_args
            .first()
            .ok_or_else(|| anyhow!("Failed to get command"))?;
        let parsed_args = &cmd_and_args[1..];
        let mut cmd_builder = CommandBuilder::new(parsed_cmd);
        cmd_builder.args(parsed_args);
        let current_dir = std::env::current_dir()?;
        cmd_builder.cwd(current_dir);

        let session = native_pty_system().openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();
        let mut reader = session.master.try_clone_reader()?;
        spawn(move || {
            let buf = &mut [0; 1];
            loop {
                let bytes_read = reader.read(buf).unwrap();
                if bytes_read == 0 {
                    break;
                }
                tx.send(buf[0] as char).unwrap();
            }
        });

        let child = session.slave.spawn_command(cmd_builder)?;
        let writer = session.master.take_writer()?;
        Ok(Self {
            master: session.master,
            slave: session.slave,
            child,
            writer,
            input_receiver: rx,
            exit_code: None,
        })
    }
}

pub struct ActualTerminal {
    session: Option<Arc<Mutex<PtySession>>>,
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

    fn read_once(&mut self) -> ZammResult<String> {
        let session_mutex = self.session.as_mut().ok_or(anyhow!("No session"))?;
        let session = session_mutex.lock()?;
        let mut partial_output = String::new();
        while let Ok(c) = session.input_receiver.try_recv() {
            partial_output.push(c);
        }
        Ok(partial_output)
    }

    #[allow(dead_code)]
    fn exit_code(&self) -> Option<u32> {
        let session_mutex = self.session.as_ref().unwrap();
        let mut session = session_mutex.lock().unwrap();
        match session.exit_code {
            Some(code) => Some(code),
            None => {
                let status = session
                    .child
                    .try_wait()
                    .unwrap_or(None)
                    .map(|status| status.exit_code());
                if let Some(code) = status {
                    session.exit_code = Some(code);
                    // todo: record exit code and total runtime
                }
                status
            }
        }
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

        let session = PtySession::new(command)?;
        self.session = Some(Arc::new(Mutex::new(session)));

        let result = self.read_updates()?;
        Ok(result)
    }

    fn read_updates(&mut self) -> ZammResult<String> {
        let output = {
            let mut partial_output = String::new();
            loop {
                sleep(Duration::from_millis(100));
                let partial = self.read_once()?;
                if partial.is_empty() {
                    break;
                }
                partial_output.push_str(&partial);
            }
            partial_output
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

    async fn send_input(&mut self, input: &str) -> ZammResult<String> {
        let session_mutex = self.session.as_mut().ok_or(anyhow!("No session"))?;
        {
            let mut session = session_mutex.lock()?;
            session.writer.write_all(input.as_bytes())?;
            session.writer.flush()?;
        }

        let relative_time = chrono::Utc::now() - self.start_time()?;
        self.session_data.entries.push(asciicast::Entry {
            time: relative_time.num_milliseconds() as f64 / 1000.0,
            event_type: asciicast::EventType::Input,
            event_data: input.to_string(),
        });

        self.read_updates()
    }

    fn get_cast(&self) -> AsciiCastData {
        self.session_data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    const SHELL_COMMAND: &str = "cmd";
    #[cfg(not(target_os = "windows"))]
    const SHELL_COMMAND: &str = "bash";

    #[tokio::test]
    async fn test_capture_command_output() {
        let (command, expected_output) = if cfg!(target_os = "windows") {
            ("cmd /C \"echo hello world\"", "\u{1b}[?25l\u{1b}[2J\u{1b}[m\u{1b}[Hhello world\r\n\u{1b}]0;C:\\WINDOWS\\system32\\cmd.EXE\u{7}\u{1b}[?25h")
        } else {
            ("echo hello world", "hello world\r\n")
        };

        let mut terminal = ActualTerminal::new();
        let output = terminal.run_command(command).await.unwrap();
        assert_eq!(output, expected_output);
        assert_eq!(terminal.get_cast().entries.len(), 1);
        assert_eq!(terminal.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_capture_interleaved_output() {
        let mut terminal = ActualTerminal::new();
        let output = terminal
            .run_command("python api/sample-terminal-sessions/interleaved.py")
            .await
            .unwrap();

        // No trailing newline on Windows
        #[cfg(target_os = "windows")]
        assert!(
            output.contains("stdout\r\nstderr\r\nstdout"),
            "Output: {:?}",
            output
        );
        #[cfg(not(target_os = "windows"))]
        assert_eq!(output, "stdout\r\nstderr\r\nstdout\r\n");

        assert_eq!(terminal.get_cast().entries.len(), 1);
        assert_eq!(terminal.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_capture_output_without_blocking() {
        let mut terminal = ActualTerminal::new();
        let output = terminal.run_command(SHELL_COMMAND).await.unwrap();

        // Windows output contains a whole lot of control characters, so we don't test
        // directly with `starts_with` or `ends_with` here
        #[cfg(target_os = "windows")]
        assert!(
            output.contains("(c) Microsoft Corporation. All rights reserved.")
                && output.contains("src-tauri>"),
            "Output: {:?}",
            output
        );
        #[cfg(not(target_os = "windows"))]
        assert!(
            output.ends_with("$ ") || output.ends_with("# "),
            "Output: {}",
            output
        );

        assert_eq!(terminal.get_cast().entries.len(), 1);
        assert_eq!(terminal.exit_code(), None);
    }

    #[tokio::test]
    async fn test_no_entry_on_empty_capture() {
        let mut terminal = ActualTerminal::new();
        terminal.run_command(SHELL_COMMAND).await.unwrap();
        terminal.read_updates().unwrap();
        assert_eq!(terminal.get_cast().entries.len(), 1);
    }

    #[tokio::test]
    async fn test_capture_interaction() {
        let input = if cfg!(target_os = "windows") {
            "python api/sample-terminal-sessions/interleaved.py\r\n"
        } else {
            "python api/sample-terminal-sessions/interleaved.py\n"
        };

        let mut terminal = ActualTerminal::new();
        terminal.run_command(SHELL_COMMAND).await.unwrap();

        let output = terminal.send_input(input).await.unwrap();
        #[cfg(target_os = "windows")]
        assert!(
            output.contains("stdout\r\nstderr\r\nstdout"),
            "Output: {:?}",
            output
        );
        #[cfg(not(target_os = "windows"))]
        assert!(
            output.contains("stdout\r\nstderr\r\nstdout\r\n")
                && (output.ends_with("$ ") || output.ends_with("# ")),
            "Output: {:?}",
            output
        );

        assert_eq!(terminal.get_cast().entries.len(), 3);
        assert_eq!(terminal.exit_code(), None);
    }
}
