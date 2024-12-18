use crate::commands::errors::ZammResult;
use crate::commands::terminal::parse::clean_output;
use crate::models::llm_calls::EntityId;
use crate::schema::asciicasts;

use crate::{ZammDatabase, ZammTerminalSessions};
use anyhow::anyhow;
use diesel::prelude::*;
use specta::specta;
use tauri::State;
use uuid::Uuid;

async fn send_command_input_helper(
    zamm_db: &ZammDatabase,
    zamm_sessions: &ZammTerminalSessions,
    session_id: &Uuid,
    input: &str,
) -> ZammResult<String> {
    let db = &mut zamm_db.0.lock().await;
    let mut sessions = zamm_sessions.0.lock().await;
    let session_entity_id = EntityId { uuid: *session_id };
    let terminal = sessions
        .get_mut(&session_entity_id)
        .ok_or_else(|| anyhow!("No session found"))?;
    #[cfg(target_os = "windows")]
    let input_with_newline = format!("{}\r\n", input);
    #[cfg(not(target_os = "windows"))]
    let input_with_newline = format!("{}\n", input);
    let raw_output = terminal.send_input(&input_with_newline)?;
    let output = clean_output(&raw_output);

    if let Some(conn) = db.as_mut() {
        let db_update_result = diesel::update(asciicasts::table)
            .filter(asciicasts::id.eq(session_entity_id))
            .set(asciicasts::cast.eq(terminal.get_cast()?))
            .execute(conn)?;
        if db_update_result == 0 {
            return Err(anyhow!("Couldn't update session in database").into());
        }
    }

    Ok(output)
}

#[tauri::command(async)]
#[specta]
pub async fn send_command_input(
    database: State<'_, ZammDatabase>,
    sessions: State<'_, ZammTerminalSessions>,
    session_id: Uuid,
    input: String,
) -> ZammResult<String> {
    send_command_input_helper(&database, &sessions, &session_id, &input).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SendInputRequest {
        session_id: Uuid,
        input: String,
    }

    async fn make_request_helper(
        args: &SendInputRequest,
        side_effects: &mut SideEffectsHelpers,
    ) -> ZammResult<String> {
        let terminal_helper = side_effects.terminal.as_mut().unwrap();
        terminal_helper.change_mock_id(args.session_id).await;
        send_command_input_helper(
            side_effects.db.as_mut().unwrap(),
            &terminal_helper.sessions,
            &args.session_id,
            &args.input,
        )
        .await
    }

    impl_result_test_case!(
        SendInputTestCase,
        send_command_input,
        true,
        SendInputRequest,
        String
    );

    #[cfg(not(target_os = "windows"))]
    check_sample!(
        SendInputTestCase,
        test_bash_interleaved,
        "./api/sample-calls/send_command_input-bash-interleaved.yaml"
    );

    #[cfg(target_os = "windows")]
    check_sample!(
        SendInputTestCase,
        test_cmd_dir,
        "./api/sample-calls/send_command_input-cmd-dir.yaml"
    );
}
