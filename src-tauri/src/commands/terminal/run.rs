use specta::specta;

use crate::commands::errors::ZammResult;
use crate::commands::terminal::models::{ActualTerminal, Terminal};

async fn run_command_helper(
    terminal: &mut dyn Terminal,
    command: &str,
) -> ZammResult<String> {
    let output = terminal.run_command(command).await?;
    Ok(output)
}

#[allow(dead_code)]
#[tauri::command(async)]
#[specta]
pub async fn run_command(command: String) -> ZammResult<String> {
    let mut terminal = ActualTerminal::new();
    run_command_helper(&mut terminal, &command).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct RunCommandRequest {
        command: String,
    }

    async fn make_request_helper(
        args: &RunCommandRequest,
        side_effects: &mut SideEffectsHelpers,
    ) -> ZammResult<String> {
        let terminal_mut = side_effects.terminal.as_mut().unwrap();
        run_command_helper(&mut **terminal_mut, &args.command).await
    }

    impl_result_test_case!(
        RunCommandTestCase,
        run_command,
        true,
        RunCommandRequest,
        String
    );

    check_sample!(
        RunCommandTestCase,
        test_date,
        "./api/sample-calls/run_command-date.yaml"
    );
}
