use crate::commands::errors::ZammResult;
use crate::sample_call::{Disk, SampleCall};
use crate::test_helpers::database::setup_zamm_db;
use crate::test_helpers::database_contents::{
    read_database_contents, write_database_contents,
};
use crate::test_helpers::temp_files::get_temp_test_dir;
use crate::ZammDatabase;
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn read_sample(filename: &str) -> SampleCall {
    let file_path = Path::new(filename);
    let abs_file_path = file_path.absolutize().unwrap();
    let sample_str = fs::read_to_string(&abs_file_path)
        .unwrap_or_else(|_| panic!("No file found at {}", abs_file_path.display()));
    serde_yaml::from_str(&sample_str).unwrap()
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn compare_files(
    expected_file_path: impl AsRef<Path>,
    actual_file_path: impl AsRef<Path>,
    output_replacements: &HashMap<String, String>,
) {
    let expected_path_abs = expected_file_path.as_ref().absolutize().unwrap();
    let actual_path_abs = actual_file_path.as_ref().absolutize().unwrap();

    let expected_file = fs::read(&expected_path_abs).unwrap_or_else(|_| {
        panic!(
            "Error reading expected file at {}",
            expected_path_abs.as_ref().display()
        )
    });
    let actual_file = fs::read(&actual_path_abs).unwrap_or_else(|_| {
        panic!(
            "Error reading actual file at {}",
            actual_path_abs.as_ref().display()
        )
    });

    let expected_file_str = String::from_utf8(expected_file).unwrap();
    let actual_file_str = String::from_utf8(actual_file).unwrap();

    let replaced_actual_str = output_replacements
        .iter()
        .fold(actual_file_str, |acc, (k, v)| acc.replace(k, v));
    assert_eq!(expected_file_str, replaced_actual_str);
}

fn compare_dir_all(
    expected_output_dir: impl AsRef<Path>,
    actual_output_dir: impl AsRef<Path>,
    output_replacements: &HashMap<String, String>,
) {
    let mut expected_outputs = vec![];
    for entry in fs::read_dir(expected_output_dir).unwrap() {
        let entry = entry.unwrap();
        expected_outputs.push(entry);
    }

    let mut actual_outputs = vec![];
    for entry in fs::read_dir(actual_output_dir).unwrap() {
        let entry = entry.unwrap();
        actual_outputs.push(entry);
    }

    assert_eq!(
        expected_outputs
            .iter()
            .map(|e| e.file_name())
            .collect::<Vec<OsString>>(),
        actual_outputs
            .iter()
            .map(|e| e.file_name())
            .collect::<Vec<OsString>>()
    );
    for (expected_output, actual_output) in
        expected_outputs.iter().zip(actual_outputs.iter())
    {
        let file_type = expected_output.file_type().unwrap();
        if file_type.is_dir() {
            compare_dir_all(
                expected_output.path(),
                actual_output.path(),
                output_replacements,
            );
        } else {
            compare_files(
                expected_output.path(),
                actual_output.path(),
                output_replacements,
            );
        }
    }
}

pub struct SampleCallResult<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Serialize,
{
    pub sample: SampleCall,
    pub args: Option<T>,
    pub result: U,
}

#[derive(Default)]
pub struct SideEffectsHelpers {
    pub temp_test_dir: Option<PathBuf>,
    pub disk: Option<PathBuf>,
    pub db: Option<ZammDatabase>,
}

pub fn standard_test_subdir(api_call: &str, test_fn_name: &str) -> String {
    let test_logical_path = test_fn_name.split("::").collect::<Vec<&str>>();
    let test_name = test_logical_path[test_logical_path.len() - 2];
    format!("{}/{}", api_call, test_name)
}

impl SampleCall {
    pub fn db_start_dump(&self) -> Option<String> {
        self.side_effects
            .as_ref()
            .and_then(|se| se.database.as_ref())
            .and_then(|db| db.start_state_dump.as_deref())
            .map(|p: &str| format!("api/sample-database-writes/{}", p))
    }

    pub fn db_end_dump(&self) -> Option<String> {
        self.side_effects
            .as_ref()
            .and_then(|se| se.database.as_ref())
            .map(|db| db.end_state_dump.as_ref())
            .map(|p: &str| format!("api/sample-database-writes/{}", p))
    }
}

pub trait SampleCallTestCase<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Serialize,
{
    const EXPECTED_API_CALL: &'static str;
    const CALL_HAS_ARGS: bool;

    fn parse_args(&self, request_str: &str) -> T {
        serde_json::from_str(request_str).unwrap()
    }

    fn temp_test_subdirectory(&self) -> String {
        unimplemented!()
    }

    async fn make_request(
        &mut self,
        args: &Option<T>,
        side_effects: &SideEffectsHelpers,
    ) -> U;

    fn serialize_result(&self, sample: &SampleCall, result: &U) -> String;

    async fn check_result(&self, sample: &SampleCall, args: Option<&T>, result: &U);

    fn get_temp_dir(&self) -> PathBuf {
        get_temp_test_dir(&self.temp_test_subdirectory())
    }

    fn output_replacements(
        &self,
        _sample: &SampleCall,
        _result: &U,
    ) -> HashMap<String, String> {
        HashMap::new()
    }

    fn initialize_temp_dir_inputs(&self, disk_side_effect: &Disk, temp_dir: &PathBuf) {
        fs::create_dir_all(temp_dir).unwrap();
        if let Some(input_dir) = &disk_side_effect.start_state_directory {
            let relative_input_dir = format!("api/sample-disk-writes/{}", input_dir);
            copy_dir_all(relative_input_dir, temp_dir).unwrap();
        }
    }

    fn compare_temp_dir_outputs(
        &self,
        disk_side_effect: &Disk,
        actual_output_dir: &PathBuf,
        output_replacements: &HashMap<String, String>,
    ) {
        let relative_expected_output_dir = format!(
            "api/sample-disk-writes/{}",
            &disk_side_effect.end_state_directory
        );
        let expected_output_dir = Path::new(&relative_expected_output_dir);
        compare_dir_all(expected_output_dir, actual_output_dir, output_replacements);
    }

    async fn check_sample_call(&mut self, sample_file: &str) -> SampleCallResult<T, U> {
        // sanity check that sample inputs are as expected
        let sample = read_sample(sample_file);
        if Self::CALL_HAS_ARGS {
            assert_eq!(sample.request.len(), 2);
        } else {
            assert_eq!(sample.request.len(), 1);
        }
        assert_eq!(sample.request[0], Self::EXPECTED_API_CALL);

        // prepare side-effects
        let current_dir = env::current_dir().unwrap();
        let mut side_effects_helpers = SideEffectsHelpers::default();
        if let Some(side_effects) = &sample.side_effects {
            let temp_test_dir = self.get_temp_dir();
            println!(
                "Test will use temp directory at {}",
                &temp_test_dir.display()
            );

            // prepare db if necessary
            if side_effects.database.is_some() {
                let test_db = setup_zamm_db();
                if let Some(initial_contents) = sample.db_start_dump() {
                    read_database_contents(&test_db, &initial_contents)
                        .await
                        .unwrap();
                }
                side_effects_helpers.db = Some(test_db);
            }

            // prepare disk if necessary
            if let Some(disk_side_effect) = &side_effects.disk {
                let mut test_disk_dir = temp_test_dir.clone();
                test_disk_dir.push("disk");
                self.initialize_temp_dir_inputs(disk_side_effect, &test_disk_dir);

                env::set_current_dir(&test_disk_dir).unwrap();
                side_effects_helpers.disk = Some(test_disk_dir);
            }

            side_effects_helpers.temp_test_dir = Some(temp_test_dir);
        }

        // make the call
        let args = if Self::CALL_HAS_ARGS {
            Some(self.parse_args(&sample.request[1]))
        } else {
            None
        };
        let result = self.make_request(&args, &side_effects_helpers).await;
        env::set_current_dir(current_dir).unwrap();
        let replacements = self.output_replacements(&sample, &result);
        println!("Replacements:");
        for (k, v) in &replacements {
            println!("  {} -> {}", k, v);
        }

        // check the call against sample outputs
        let actual_json = self.serialize_result(&sample, &result);
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);
        self.check_result(&sample, args.as_ref(), &result).await;

        // check the call against disk side-effects
        if let Some(test_disk_dir) = &side_effects_helpers.disk {
            let disk_side_effect =
                &sample.side_effects.as_ref().unwrap().disk.as_ref().unwrap();
            self.compare_temp_dir_outputs(
                disk_side_effect,
                test_disk_dir,
                &replacements,
            );
        }

        // check the call against db side-effects
        if let Some(test_db) = &side_effects_helpers.db {
            let mut test_db_file = side_effects_helpers.temp_test_dir.unwrap().clone();
            test_db_file.push("db.yaml");
            write_database_contents(test_db, &test_db_file)
                .await
                .unwrap();

            compare_files(sample.db_end_dump().unwrap(), &test_db_file, &replacements);
        }

        SampleCallResult {
            sample,
            args,
            result,
        }
    }
}

pub trait DirectReturn<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Serialize,
{
    fn serialize_result(&self, _sample: &SampleCall, result: &U) -> String {
        serde_json::to_string_pretty(result).unwrap()
    }

    async fn check_result(&self, _sample: &SampleCall, _args: Option<&T>, _result: &U) {
    }
}

pub fn serialize_zamm_result<T>(result: &ZammResult<T>) -> String
where
    T: Serialize,
{
    match result {
        Ok(r) => serde_json::to_string_pretty(&r).unwrap(),
        Err(e) => serde_json::to_string_pretty(&e).unwrap(),
    }
}

pub fn check_zamm_result<T>(sample: &SampleCall, result: &ZammResult<T>)
where
    T: std::fmt::Debug,
{
    if sample.response.success == Some(false) {
        assert!(result.is_err(), "API call should have thrown error");
    } else {
        assert!(result.is_ok(), "API call failed: {:?}", result);
    }
}

pub trait ZammResultReturn<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Serialize + std::fmt::Debug,
{
    fn serialize_result(&self, _sample: &SampleCall, result: &ZammResult<U>) -> String {
        serialize_zamm_result(result)
    }

    async fn check_result(
        &self,
        sample: &SampleCall,
        _args: Option<&T>,
        result: &ZammResult<U>,
    ) {
        check_zamm_result(sample, result)
    }
}
