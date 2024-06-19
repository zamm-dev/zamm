use crate::commands::errors::ZammResult;
use crate::sample_call::{Disk, SampleCall};
use crate::test_helpers::database::{setup_database, setup_zamm_db};
use crate::test_helpers::database_contents::{
    dump_sqlite_database, load_sqlite_database, read_database_contents,
    write_database_contents,
};
use crate::test_helpers::temp_files::get_temp_test_dir;
use crate::ZammDatabase;
use path_absolutize::Absolutize;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use rvcr::{VCRMiddleware, VCRMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use tokio::sync::Mutex;
use vcr_cassette::Headers;

fn read_sample(filename: &str) -> SampleCall {
    let file_path = Path::new(filename);
    let abs_file_path = file_path.absolutize().unwrap();
    let sample_str = fs::read_to_string(&abs_file_path)
        .unwrap_or_else(|_| panic!("No file found at {}", abs_file_path.display()));
    serde_yaml::from_str(&sample_str).unwrap()
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)
        .unwrap_or_else(|_| panic!("Error creating directory at {:?}", dst.as_ref()));
    for entry in fs::read_dir(&src).unwrap_or_else(|_| {
        panic!("Error reading from directory at {:?}", src.as_ref())
    }) {
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

fn apply_replacements(input: &str, replacements: &HashMap<String, String>) -> String {
    replacements
        .iter()
        .fold(input.to_string(), |acc, (k, v)| acc.replace(k, v))
}

fn copy_missing_gold_file(expected_path_abs: &Path, actual_path_abs: &Path) {
    fs::create_dir_all(expected_path_abs.parent().unwrap()).unwrap();
    fs::copy(actual_path_abs, expected_path_abs).unwrap();
    eprintln!(
        "Gold file not found at {}, copied actual file from {}",
        expected_path_abs.display(),
        actual_path_abs.display(),
    );
}

async fn dump_sql_to_yaml(
    expected_sql_dump_abs: &PathBuf,
    expected_yaml_dump_abs: &PathBuf,
) {
    let mut db = setup_database(None);
    load_sqlite_database(&mut db, expected_sql_dump_abs);
    let zamm_db = ZammDatabase(Mutex::new(Some(db)));
    write_database_contents(&zamm_db, expected_yaml_dump_abs)
        .await
        .unwrap();
}

async fn setup_gold_db_files(
    expected_yaml_dump: impl AsRef<Path>,
    actual_yaml_dump: impl AsRef<Path>,
    expected_sql_dump: impl AsRef<Path>,
    actual_sql_dump: impl AsRef<Path>,
) {
    let expected_yaml_dump_abs = expected_yaml_dump.as_ref().absolutize().unwrap();
    let actual_yaml_dump_abs = actual_yaml_dump.as_ref().absolutize().unwrap();
    let expected_sql_dump_abs = expected_sql_dump.as_ref().absolutize().unwrap();
    let actual_sql_dump_abs = actual_sql_dump.as_ref().absolutize().unwrap();

    if !expected_yaml_dump_abs.exists() && !expected_sql_dump_abs.exists() {
        copy_missing_gold_file(&expected_yaml_dump_abs, &actual_yaml_dump_abs);
        copy_missing_gold_file(&expected_sql_dump_abs, &actual_sql_dump_abs);
        panic!(
            "Copied gold files to {}",
            expected_yaml_dump_abs.parent().unwrap().display()
        );
    } else if !expected_yaml_dump_abs.exists() && expected_sql_dump_abs.exists() {
        dump_sql_to_yaml(
            &expected_sql_dump_abs.to_path_buf(),
            &expected_yaml_dump_abs.to_path_buf(),
        )
        .await;
        panic!(
            "Dumped YAML from SQL to {}",
            expected_yaml_dump_abs.display()
        );
    } else {
        if !expected_yaml_dump_abs.exists() {
            panic!("No YAML dump found at {}", expected_yaml_dump_abs.display());
        }
        if !expected_sql_dump_abs.exists() {
            panic!("No SQL dump found at {}", expected_sql_dump_abs.display());
        }
    }
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

    let replaced_actual_str = apply_replacements(&actual_file_str, output_replacements);
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

pub struct NetworkHelper {
    pub network_client: ClientWithMiddleware,
    pub mode: VCRMode,
}

#[derive(Default)]
pub struct SideEffectsHelpers {
    pub temp_test_dir: Option<PathBuf>,
    pub disk: Option<PathBuf>,
    pub db: Option<ZammDatabase>,
    pub network: Option<NetworkHelper>,
}

pub fn standard_test_subdir(api_call: &str, test_fn_name: &str) -> String {
    let test_logical_path = test_fn_name.split("::").collect::<Vec<&str>>();
    let test_name = test_logical_path[test_logical_path.len() - 2];
    format!("{}/{}", api_call, test_name)
}

impl SampleCall {
    pub fn network_recording(&self) -> String {
        let recording_file = &self
            .side_effects
            .as_ref()
            .unwrap()
            .network
            .as_ref()
            .unwrap()
            .recording_file;

        format!("api/sample-network-requests/{}", recording_file)
    }

    pub fn db_start_dump(&self) -> Option<String> {
        self.side_effects
            .as_ref()
            .and_then(|se| se.database.as_ref())
            .and_then(|db| db.start_state_dump.as_deref())
            .map(|p: &str| format!("api/sample-database-writes/{}/dump.yaml", p))
    }

    pub fn db_end_dump(&self, extension: &str) -> String {
        let end_state_dump_dir = &self
            .side_effects
            .as_ref()
            .unwrap()
            .database
            .as_ref()
            .unwrap()
            .end_state_dump;

        format!(
            "api/sample-database-writes/{}/dump.{}",
            end_state_dump_dir, extension
        )
    }
}

struct TestDatabaseInfo {
    pub temp_db_dir: PathBuf,
    pub temp_db_file: PathBuf,
}

const CENSORED: &str = "<CENSORED>";

fn censor_headers(headers: &Headers, blacklisted_keys: &[&str]) -> Headers {
    return headers
        .clone()
        .iter()
        .map(|(k, v)| {
            if blacklisted_keys.contains(&k.as_str()) {
                (k.clone(), vec![CENSORED.to_string()])
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect();
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
        let mut test_db_info: Option<TestDatabaseInfo> = None;
        if let Some(side_effects) = &sample.side_effects {
            let temp_test_dir = self.get_temp_dir();
            println!(
                "Test will use temp directory at {}",
                &temp_test_dir.display()
            );

            // prepare network if necessary
            if side_effects.network.is_some() {
                let recording_path = PathBuf::from(sample.network_recording());
                let vcr_mode = if !recording_path.exists() {
                    VCRMode::Record
                } else {
                    VCRMode::Replay
                };
                let middleware = VCRMiddleware::try_from(recording_path)
                    .unwrap()
                    .with_mode(vcr_mode.clone())
                    .with_modify_request(|req| {
                        req.headers = censor_headers(&req.headers, &["authorization"]);
                    })
                    .with_modify_response(|resp| {
                        resp.headers =
                            censor_headers(&resp.headers, &["openai-organization"]);
                    });

                let network_client: ClientWithMiddleware =
                    ClientBuilder::new(reqwest::Client::new())
                        .with(middleware)
                        .build();

                side_effects_helpers.network = Some(NetworkHelper {
                    network_client,
                    mode: vcr_mode,
                });
            }

            // prepare db if necessary
            if side_effects.database.is_some() {
                let temp_db_dir = temp_test_dir.join("database");
                fs::create_dir_all(&temp_db_dir).unwrap();
                let temp_db_file = temp_db_dir.join("db.sqlite3");

                let test_db = setup_zamm_db(Some(&temp_db_file));
                if let Some(initial_yaml_dump) = sample.db_start_dump() {
                    let initial_yaml_dump_abs = Path::new(&initial_yaml_dump)
                        .absolutize()
                        .unwrap()
                        .to_path_buf();
                    if !initial_yaml_dump_abs.exists() {
                        let initial_sql_dump_abs =
                            initial_yaml_dump_abs.with_extension("sql");
                        dump_sql_to_yaml(&initial_sql_dump_abs, &initial_yaml_dump_abs)
                            .await;
                        panic!(
                            "Dumped YAML from SQL to {}",
                            initial_yaml_dump_abs.display()
                        );
                    }

                    read_database_contents(&test_db, &initial_yaml_dump)
                        .await
                        .unwrap();
                }

                side_effects_helpers.db = Some(test_db);
                test_db_info = Some(TestDatabaseInfo {
                    temp_db_dir,
                    temp_db_file,
                });
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
        let replaced_actual_json = apply_replacements(&actual_json, &replacements);
        let expected_json = sample.response.message.trim();
        assert_eq!(replaced_actual_json, expected_json);
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
            let db_info = test_db_info.unwrap();
            let actual_db_yaml_dump = db_info.temp_db_dir.join("dump.yaml");
            let actual_db_sql_dump = db_info.temp_db_dir.join("dump.sql");
            write_database_contents(test_db, &actual_db_yaml_dump)
                .await
                .unwrap();
            dump_sqlite_database(&db_info.temp_db_file, &actual_db_sql_dump);

            setup_gold_db_files(
                sample.db_end_dump("yaml"),
                &actual_db_yaml_dump,
                sample.db_end_dump("sql"),
                &actual_db_sql_dump,
            )
            .await;

            compare_files(
                sample.db_end_dump("yaml"),
                &actual_db_yaml_dump,
                &replacements,
            );
            compare_files(
                sample.db_end_dump("sql"),
                &actual_db_sql_dump,
                &replacements,
            );
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
