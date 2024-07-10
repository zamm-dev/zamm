use super::errors::ZammResult;
use crate::models::asciicasts::AsciiCastData;
use duct::cmd;

pub fn command_args_to_string(command: &str, args: &[&str]) -> String {
    let escaped_args = args
        .iter()
        .map(|arg| {
            if arg.contains(' ') {
                format!("\"{}\"", arg)
            } else {
                arg.to_string()
            }
        })
        .collect::<Vec<String>>();
    format!("{} {}", command, escaped_args.join(" "))
}

pub trait Terminal {
    async fn run_command(&mut self, command: &str, args: &[&str])
        -> ZammResult<String>;
}

pub struct ActualTerminal {
    pub session_data: AsciiCastData,
}

impl ActualTerminal {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            session_data: AsciiCastData::new(),
        }
    }
}

impl Terminal for ActualTerminal {
    async fn run_command(
        &mut self,
        command: &str,
        args: &[&str],
    ) -> ZammResult<String> {
        self.session_data.header.command = Some(command_args_to_string(command, args));

        let starting_time = chrono::Utc::now();
        self.session_data.header.timestamp = Some(starting_time);

        let output = cmd(command, args).stderr_to_stdout().read()?;
        let output_time = chrono::Utc::now();
        let duration = output_time - starting_time;
        self.session_data.entries.push(asciicast::Entry {
            time: duration.num_milliseconds() as f64 / 1000.0,
            event_type: asciicast::EventType::Output,
            event_data: output.clone(),
        });
        Ok(output)
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
            .run_command("cmd", &["/C", "echo hello world"])
            .await
            .unwrap();
        #[cfg(not(target_os = "windows"))]
        let output = terminal
            .run_command("echo", &["hello", "world"])
            .await
            .unwrap();
        assert_eq!(output, "hello world");
    }

    #[tokio::test]
    async fn test_capture_interleaved_output() {
        let mut terminal = ActualTerminal::new();
        let output = terminal
            .run_command("python", &["api/sample-terminal-sessions/interleaved.py"])
            .await
            .unwrap();

        #[cfg(target_os = "windows")]
        assert_eq!(output, "stdout\r\nstderr\r\nstdout");
        #[cfg(not(target_os = "windows"))]
        assert_eq!(output, "stdout\nstderr\nstdout");
    }
}
