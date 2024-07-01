use super::errors::ZammResult;
use duct::cmd;

#[allow(dead_code)]
async fn capture_command_output(command: &str, args: &[&str]) -> ZammResult<String> {
    let output = cmd(command, args).stderr_to_stdout().read()?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capture_command_output() {
        let output = capture_command_output("echo", &["hello", "world"])
            .await
            .unwrap();
        assert_eq!(output, "hello world");
    }

    #[tokio::test]
    async fn test_capture_interleaved_output() {
        let output = capture_command_output(
            "python",
            &["api/sample-terminal-sessions/interleaved.py"],
        )
        .await
        .unwrap();
        assert_eq!(output, "stdout\nstderr\nstdout");
    }
}
