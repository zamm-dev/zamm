use crate::commands::errors::ZammResult;
use crate::commands::terminal::ActualTerminal;
use crate::models::asciicasts::NewAsciiCast;
use crate::models::llm_calls::EntityId;
use crate::models::os::get_os;
use crate::schema::asciicasts::{self};
use crate::{ZammDatabase, ZammTerminalSessions};
use anyhow::anyhow;
use chrono::naive::NaiveDateTime;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use specta::specta;
use tauri::State;

#[derive(PartialEq)]
enum EscapeSequence {
    None,
    Start,
    InEscape,
    InOperatingSystemEscape,
    LineStart,
}

static ESCAPE_COMMANDS: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'b', 'm'];

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RunCommandResponse {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub output: String,
}

fn clean_output(output: &str) -> String {
    let mut cleaned_lines = Vec::<String>::new();
    output.split('\n').for_each(|line| {
        let mut escape: EscapeSequence = EscapeSequence::None;
        let mut cleaned_line = String::new();
        let mut current_escape_arg = String::new();
        let mut escape_args = Vec::<String>::new();
        let mut escape_command = ' ';
        line.chars().for_each(|c| {
            if c == '\u{001B}' {
                escape = EscapeSequence::Start;
            } else if escape == EscapeSequence::Start {
                if c == '[' || c == '(' {
                    escape = EscapeSequence::InEscape;
                } else if c == ']' {
                    escape = EscapeSequence::InOperatingSystemEscape;
                } else {
                    escape = EscapeSequence::None;
                    cleaned_line.push(c);
                }
            } else if escape == EscapeSequence::InEscape
                || escape == EscapeSequence::InOperatingSystemEscape
            {
                if c == '?' {
                    // it's just a private sequence marker, do nothing
                } else if escape == EscapeSequence::InEscape
                    && ESCAPE_COMMANDS.contains(&c)
                {
                    escape_command = c;
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                    escape = EscapeSequence::None;
                } else if c == ';' {
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                } else if c == '\u{0007}' {
                    if let Some(last_char) = current_escape_arg.pop() {
                        escape_command = last_char;
                    }
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                    escape = EscapeSequence::None;
                } else {
                    current_escape_arg.push(c);
                }

                if escape == EscapeSequence::None {
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();

                    // now we actually handle the escape sequence
                    if escape_command == 'H' {
                        if let Some(first_arg) = escape_args.first() {
                            if let Ok(row) = first_arg.parse::<usize>() {
                                if row > cleaned_lines.len() {
                                    cleaned_lines.push(cleaned_line.clone());
                                    cleaned_line.clear();
                                    // -1 because the new cleaned_line will be added at
                                    // the end as the next line
                                    cleaned_lines.resize(row - 1, "".to_string());
                                }
                            }
                        }
                    }
                }
            } else if c == '\r' {
                escape = EscapeSequence::LineStart;
            } else if escape == EscapeSequence::LineStart {
                escape = EscapeSequence::None;
                cleaned_line.clear();
                cleaned_line.push(c);
            } else {
                cleaned_line.push(c);
            }
        });
        cleaned_lines.push(cleaned_line);
    });
    cleaned_lines.join("\n").trim_start().to_string()
}

async fn run_command_helper(
    zamm_db: &ZammDatabase,
    zamm_sessions: &ZammTerminalSessions,
    session_id: &EntityId,
    command: &str,
) -> ZammResult<RunCommandResponse> {
    let db = &mut zamm_db.0.lock().await;
    let mut sessions = zamm_sessions.0.lock().await;
    let terminal = sessions
        .get_mut(session_id)
        .ok_or_else(|| anyhow!("No session found"))?;

    let raw_output = terminal.run_command(command)?;
    let cast = terminal.get_cast()?;
    let timestamp = cast
        .header
        .timestamp
        .map(|t| t.naive_utc())
        .ok_or_else(|| anyhow!("No timestamp in cast"))?;

    if let Some(conn) = db.as_mut() {
        let command = cast
            .header
            .command
            .clone()
            .ok_or_else(|| anyhow!("No command in cast"))?;
        diesel::insert_into(asciicasts::table)
            .values(NewAsciiCast {
                id: session_id,
                timestamp: &timestamp,
                command: &command,
                os: get_os(),
                cast: &cast,
            })
            .execute(conn)?;
    }

    let output = clean_output(&raw_output);

    Ok(RunCommandResponse {
        id: session_id.clone(),
        timestamp,
        output,
    })
}

#[tauri::command(async)]
#[specta]
pub async fn run_command(
    database: State<'_, ZammDatabase>,
    sessions: State<'_, ZammTerminalSessions>,
    command: String,
) -> ZammResult<RunCommandResponse> {
    let terminal = ActualTerminal::new();
    let new_session_id = EntityId::new();
    sessions
        .0
        .lock()
        .await
        .insert(new_session_id.clone(), Box::new(terminal));

    run_command_helper(&database, &sessions, &new_session_id, &command).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_sample;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct RunCommandRequest {
        command: String,
    }

    struct RunCommandTestCase {
        test_fn_name: &'static str,
    }

    fn to_yaml_string<T: Serialize>(obj: &T) -> String {
        serde_yaml::to_string(obj).unwrap().trim().to_string()
    }

    fn parse_response(response_str: &str) -> RunCommandResponse {
        serde_json::from_str(response_str).unwrap()
    }

    impl SampleCallTestCase<RunCommandRequest, ZammResult<RunCommandResponse>>
        for RunCommandTestCase
    {
        const EXPECTED_API_CALL: &'static str = "run_command";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &RunCommandRequest,
            side_effects: &mut SideEffectsHelpers,
        ) -> ZammResult<RunCommandResponse> {
            let terminal_helper = side_effects.terminal.as_ref().unwrap();
            run_command_helper(
                side_effects.db.as_ref().unwrap(),
                &terminal_helper.sessions,
                &terminal_helper.mock_session_id,
                &args.command,
            )
            .await
        }

        fn output_replacements(
            &self,
            sample: &SampleCall,
            result: &ZammResult<RunCommandResponse>,
        ) -> HashMap<String, String> {
            let expected_output = parse_response(&sample.response.message);
            let actual_output = result.as_ref().unwrap();
            let expected_output_timestamp = to_yaml_string(&expected_output.timestamp);
            let actual_output_timestamp = to_yaml_string(&actual_output.timestamp);
            let asciicast_filename = &sample
                .side_effects
                .as_ref()
                .unwrap()
                .terminal
                .as_ref()
                .unwrap()
                .recording_file;
            let expected_os = if asciicast_filename.ends_with("bash.cast") {
                "Mac"
            } else if asciicast_filename.ends_with("windows.cast") {
                "Windows"
            } else {
                "Linux"
            };
            HashMap::from([
                (
                    to_yaml_string(&actual_output.id),
                    to_yaml_string(&expected_output.id),
                ),
                (
                    // sqlite dump produces timestamps with space instead of T
                    actual_output_timestamp.replace('T', " "),
                    expected_output_timestamp.replace('T', " "),
                ),
                (actual_output_timestamp, expected_output_timestamp),
                #[cfg(target_os = "windows")]
                ("Windows".to_string(), expected_os.to_string()),
                #[cfg(target_os = "macos")]
                ("Mac".to_string(), expected_os.to_string()),
                #[cfg(target_os = "linux")]
                ("Linux".to_string(), expected_os.to_string()),
            ])
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<RunCommandResponse>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: &RunCommandRequest,
            result: &ZammResult<RunCommandResponse>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<RunCommandRequest, RunCommandResponse> for RunCommandTestCase {}

    check_sample!(
        RunCommandTestCase,
        test_date,
        "./api/sample-calls/run_command-date.yaml"
    );

    check_sample!(
        RunCommandTestCase,
        test_start_bash,
        "./api/sample-calls/run_command-bash.yaml"
    );

    check_sample!(
        RunCommandTestCase,
        test_start_cmd,
        "./api/sample-calls/run_command-cmd.yaml"
    );
}
