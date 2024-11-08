use super::parse::clean_output;
use crate::commands::errors::ZammResult;
use crate::commands::terminal::models::TerminalSessionInfo;
use crate::models::asciicasts::AsciiCast;
use crate::models::EntityId;
use crate::schema::asciicasts;
use crate::ZammDatabase;
use crate::ZammTerminalSessions;
use anyhow::anyhow;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use specta::specta;
use tauri::State;
use uuid::Uuid;

async fn get_terminal_session_helper(
    zamm_db: &ZammDatabase,
    zamm_sessions: &ZammTerminalSessions,
    id: Uuid,
) -> ZammResult<TerminalSessionInfo> {
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;
    let sessions = zamm_sessions.0.lock().await;

    let parsed_uuid: EntityId = id.into();
    let result: AsciiCast = asciicasts::table
        .filter(asciicasts::id.eq(&parsed_uuid))
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
        .join("");
    let is_active = sessions.contains_key(&parsed_uuid);
    let recovered_session = TerminalSessionInfo {
        id: result.id,
        timestamp: result.timestamp,
        command: result.command.clone(),
        os: result.os,
        output: concantenated_output,
        is_active,
    };
    Ok(recovered_session)
}

#[tauri::command(async)]
#[specta]
pub async fn get_terminal_session(
    database: State<'_, ZammDatabase>,
    zamm_sessions: State<'_, ZammTerminalSessions>,
    id: Uuid,
) -> ZammResult<TerminalSessionInfo> {
    get_terminal_session_helper(&database, &zamm_sessions, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::api_testing::{standard_test_subdir, TerminalHelper};
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use serde::{Deserialize, Serialize};
    use stdext::function_name;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct GetTerminalSessionRequest {
        id: Uuid,
    }

    struct GetTerminalSessionTestCase {
        test_fn_name: &'static str,
        session_should_exist: bool,
    }

    impl SampleCallTestCase<GetTerminalSessionRequest, ZammResult<TerminalSessionInfo>>
        for GetTerminalSessionTestCase
    {
        const EXPECTED_API_CALL: &'static str = "get_terminal_session";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &GetTerminalSessionRequest,
            side_effects: &mut SideEffectsHelpers,
        ) -> ZammResult<TerminalSessionInfo> {
            let mut terminal_helper = TerminalHelper::new();
            if self.session_should_exist {
                terminal_helper.change_mock_id(args.id).await;
            }

            get_terminal_session_helper(
                side_effects.db.as_ref().unwrap(),
                &terminal_helper.sessions,
                args.id,
            )
            .await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<TerminalSessionInfo>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: &GetTerminalSessionRequest,
            result: &ZammResult<TerminalSessionInfo>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<GetTerminalSessionRequest, TerminalSessionInfo>
        for GetTerminalSessionTestCase
    {
    }

    #[tokio::test]
    async fn test_start_bash() {
        let mut test_case = GetTerminalSessionTestCase {
            test_fn_name: function_name!(),
            session_should_exist: true,
        };
        test_case
            .check_sample_call("./api/sample-calls/get_terminal_session-bash.yaml")
            .await;
    }

    #[tokio::test]
    async fn test_bash_interleaved() {
        let mut test_case = GetTerminalSessionTestCase {
            test_fn_name: function_name!(),
            session_should_exist: false,
        };
        test_case
            .check_sample_call(
                "./api/sample-calls/get_terminal_session-bash-interleaved.yaml",
            )
            .await;
    }
}
