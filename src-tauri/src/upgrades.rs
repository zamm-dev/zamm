use std::env;
use std::path::PathBuf;

use crate::commands::{
    errors::ZammResult,
    preferences::{get_preferences_file_contents, set_preferences_helper},
};

pub fn handle_app_upgrades(preferences_dir: &Option<PathBuf>) -> ZammResult<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let mut preferences = get_preferences_file_contents(preferences_dir)?;
    let last_touched_by_previous_zamm = match preferences.version {
        None => true,
        Some(pref_version) => version_compare::compare_to(
            pref_version,
            current_version,
            version_compare::Cmp::Lt,
        )
        .unwrap_or(false),
    };

    if last_touched_by_previous_zamm {
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
            _: &Option<()>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<()> {
            handle_app_upgrades(&side_effects.disk)
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
            args: Option<&()>,
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
    async fn test_upgrade_from_v_0_1_3() {
        check_app_upgrades(
            function_name!(),
            "./api/sample-calls/upgrade-from-v0.1.3.yaml",
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
