use super::parse::clean_output;
use crate::commands::errors::ZammResult;
use crate::models::asciicasts::AsciiCast;
use crate::models::os::OS;
use crate::models::EntityId;
use crate::schema::asciicasts;
use crate::ZammDatabase;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use specta::specta;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RecoveredTerminalSession {
    id: EntityId,
    timestamp: NaiveDateTime,
    command: String,
    os: Option<OS>,
    output: String,
}

async fn get_terminal_session_helper(
    zamm_db: &ZammDatabase,
    id: &str,
) -> ZammResult<RecoveredTerminalSession> {
    let parsed_uuid = EntityId {
        uuid: Uuid::parse_str(id)?,
    };
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;

    let result: AsciiCast = asciicasts::table
        .filter(asciicasts::id.eq(parsed_uuid))
        .first::<AsciiCast>(conn)?;
    let concantenated_output = result
        .cast
        .entries
        .iter()
        .flat_map(|e| {
            if e.event_type == asciicast::EventType::Output {
                Some(clean_output(&e.event_data))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    let recovered_session = RecoveredTerminalSession {
        id: result.id,
        timestamp: result.timestamp,
        command: result.command.clone(),
        os: result.os,
        output: concantenated_output,
    };
    Ok(recovered_session)
}

#[tauri::command(async)]
#[specta]
pub async fn get_terminal_session(
    database: State<'_, ZammDatabase>,
    id: &str,
) -> ZammResult<RecoveredTerminalSession> {
    get_terminal_session_helper(&database, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct GetTerminalSessionRequest {
        id: String,
    }

    async fn make_request_helper(
        args: &GetTerminalSessionRequest,
        side_effects: &mut SideEffectsHelpers,
    ) -> ZammResult<RecoveredTerminalSession> {
        get_terminal_session_helper(side_effects.db.as_ref().unwrap(), &args.id).await
    }

    impl_result_test_case!(
        GetTerminalSessionTestCase,
        get_terminal_session,
        true,
        GetTerminalSessionRequest,
        RecoveredTerminalSession
    );

    check_sample!(
        GetTerminalSessionTestCase,
        test_start_bash,
        "./api/sample-calls/get_terminal_session-bash.yaml"
    );

    check_sample!(
        GetTerminalSessionTestCase,
        test_bash_interleaved,
        "./api/sample-calls/get_terminal_session-bash-interleaved.yaml"
    );
}
