use crate::commands::errors::ZammResult;
use crate::sample_call::{Disk, SampleCall};
use crate::test_helpers::temp_files::get_temp_test_dir;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn read_sample(filename: &str) -> SampleCall {
    let sample_str = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file found at {filename}"));
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

fn compare_dir_all(
    expected_output_dir: impl AsRef<Path>,
    actual_output_dir: impl AsRef<Path>,
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
            compare_dir_all(expected_output.path(), actual_output.path());
        } else {
            let expected_file = fs::read(expected_output.path()).unwrap();
            let actual_file = fs::read(actual_output.path()).unwrap();

            let expected_file_str = String::from_utf8(expected_file).unwrap();
            let actual_file_str = String::from_utf8(actual_file).unwrap();
            assert_eq!(expected_file_str, actual_file_str);
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

pub struct SideEffectsHelpers {
    pub disk: Option<PathBuf>,
}

pub fn standard_test_subdir(api_call: &str, test_fn_name: &str) -> String {
    let test_logical_path = test_fn_name.split("::").collect::<Vec<&str>>();
    let test_name = test_logical_path[test_logical_path.len() - 2];
    format!("{}/{}", api_call, test_name)
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

    fn initialize_temp_dir_inputs(&self, disk_side_effect: &Disk, temp_dir: &PathBuf) {
        if let Some(input_dir) = &disk_side_effect.start_state_directory {
            let relative_input_dir = format!("api/sample-disk-writes/{}", input_dir);
            copy_dir_all(relative_input_dir, temp_dir).unwrap();
        }
    }

    fn compare_temp_dir_outputs(
        &self,
        disk_side_effect: &Disk,
        actual_output_dir: &PathBuf,
    ) {
        let relative_expected_output_dir = format!(
            "api/sample-disk-writes/{}",
            &disk_side_effect.end_state_directory
        );
        let expected_output_dir = Path::new(&relative_expected_output_dir);
        compare_dir_all(&expected_output_dir, actual_output_dir);
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
        let mut temp_dir: Option<PathBuf> = None;
        if let Some(side_effects) = &sample.side_effects {
            // prepare disk if necessary
            if let Some(disk_side_effect) = &side_effects.disk {
                temp_dir = Some(self.get_temp_dir());
                self.initialize_temp_dir_inputs(
                    disk_side_effect,
                    &temp_dir.as_ref().unwrap(),
                );
                println!(
                    "Test will use temp directory at {}",
                    temp_dir.as_ref().unwrap().display()
                );
            }
        }
        let side_effects_helpers = SideEffectsHelpers { disk: temp_dir };

        // make the call
        let args = if Self::CALL_HAS_ARGS {
            Some(self.parse_args(&sample.request[1]))
        } else {
            None
        };
        let result = self.make_request(&args, &side_effects_helpers).await;

        // check the call against sample outputs
        let actual_json = self.serialize_result(&sample, &result);
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);
        self.check_result(&sample, args.as_ref(), &result).await;

        // check the call against disk side-effects
        if let Some(temp_test_dir) = &side_effects_helpers.disk {
            let disk_side_effect =
                &sample.side_effects.as_ref().unwrap().disk.as_ref().unwrap();
            self.compare_temp_dir_outputs(disk_side_effect, &temp_test_dir);
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
