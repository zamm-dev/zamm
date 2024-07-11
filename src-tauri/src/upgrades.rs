use std::env;
use std::path::PathBuf;

use crate::commands::{
    errors::ZammResult,
    preferences::{get_preferences_file_contents, set_preferences_helper},
};
use anyhow::anyhow;
use chrono::NaiveDateTime;
use diesel::dsl::not;
use diesel::prelude::*;

use crate::models::llm_calls::ChatPrompt;
use crate::models::llm_calls::EntityId;
use crate::models::llm_calls::Prompt;
use crate::schema::{llm_call_follow_ups, llm_calls};
use crate::ZammDatabase;

async fn upgrade_to_v_0_1_4(zamm_db: &ZammDatabase) -> ZammResult<()> {
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;

    let llm_calls_without_followups: Vec<(EntityId, NaiveDateTime, Prompt)> =
        llm_calls::table
            .select((llm_calls::id, llm_calls::timestamp, llm_calls::prompt))
            .filter(not(llm_calls::id.eq_any(
                llm_call_follow_ups::table.select(llm_call_follow_ups::next_call_id),
            )))
            .load::<(EntityId, NaiveDateTime, Prompt)>(conn)?;
    let non_initial_calls: Vec<&(EntityId, NaiveDateTime, Prompt)> =
        llm_calls_without_followups
            .iter()
            .filter(|(_, _, prompt)| {
                match prompt {
                    // if it's just system message + initial human message, then this
                    // must be an initial prompt rather than any previous one.
                    // Otherwise, it should have at least 4 messages: the initial system
                    // message, the initial human message, the initial AI reply, and
                    // the human follow-up
                    Prompt::Chat(chat_prompt) => chat_prompt.messages.len() >= 4,
                    _ => false,
                }
            })
            .collect();

    let mut num_links_added = 0;
    for (id, timestamp, prompt) in non_initial_calls {
        let (search_prompt, search_completion) = match prompt {
            Prompt::Chat(chat_prompt) => {
                let length = chat_prompt.messages.len();
                let previous_messages = chat_prompt.messages[..length - 2].to_vec();
                let previous_prompt = Prompt::Chat(ChatPrompt {
                    messages: previous_messages,
                });
                let previous_completion = &chat_prompt.messages[length - 2];
                (previous_prompt, previous_completion)
            }
            _ => continue,
        };

        let prior_api_call: Option<EntityId> = llm_calls::table
            .select(llm_calls::id)
            .filter(llm_calls::timestamp.lt(timestamp))
            .filter(llm_calls::prompt.eq(search_prompt))
            .filter(llm_calls::completion.eq(search_completion))
            .first::<EntityId>(conn)
            .optional()?;

        if let Some(prior_call_id) = prior_api_call {
            diesel::insert_into(llm_call_follow_ups::table)
                .values((
                    llm_call_follow_ups::previous_call_id.eq(prior_call_id),
                    llm_call_follow_ups::next_call_id.eq(id),
                ))
                .execute(conn)?;
            num_links_added += 1;
        }
    }

    if num_links_added > 0 {
        println!(
            "v0.1.4 data migration: Linked {} LLM API calls with their follow-ups",
            num_links_added
        );
    }

    Ok(())
}

fn version_before(a: &Option<String>, b: &str) -> bool {
    match a {
        None => true,
        Some(version) => {
            version_compare::compare_to(version, b, version_compare::Cmp::Lt)
                .unwrap_or(false)
        }
    }
}

pub async fn handle_app_upgrades(
    preferences_dir: &Option<PathBuf>,
    zamm_db: &ZammDatabase,
) -> ZammResult<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let mut preferences = get_preferences_file_contents(preferences_dir)?;

    if version_before(&preferences.version, "0.1.4") {
        upgrade_to_v_0_1_4(zamm_db).await?;
    }

    if version_before(&preferences.version, current_version) {
        preferences.version = Some(current_version.to_string());
        set_preferences_helper(preferences_dir, &preferences)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use stdext::function_name;

    struct HandleAppUpgradesTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<(), ZammResult<()>> for HandleAppUpgradesTestCase {
        const EXPECTED_API_CALL: &'static str = "upgrade";
        const CALL_HAS_ARGS: bool = false;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            _: &(),
            side_effects: &mut SideEffectsHelpers,
        ) -> ZammResult<()> {
            handle_app_upgrades(&side_effects.disk, side_effects.db.as_ref().unwrap())
                .await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<()>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: &(),
            result: &ZammResult<()>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<(), ()> for HandleAppUpgradesTestCase {}

    async fn check_app_upgrades(test_fn_name: &'static str, file_prefix: &str) {
        let mut test_case = HandleAppUpgradesTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_upgrade_first_init() {
        check_app_upgrades(
            function_name!(),
            "./api/sample-calls/upgrade-first-init.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_upgrade_to_v_0_1_4_onwards() {
        check_app_upgrades(
            function_name!(),
            "./api/sample-calls/upgrade-to-v0.1.4.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_upgrade_from_future_version() {
        check_app_upgrades(
            function_name!(),
            "./api/sample-calls/upgrade-from-future-version.yaml",
        )
        .await;
    }
}
