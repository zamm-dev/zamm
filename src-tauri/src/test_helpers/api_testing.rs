use crate::commands::ZammResult;
use crate::sample_call::SampleCall;
use serde::{Deserialize, Serialize};
use std::fs;

fn read_sample(filename: &str) -> SampleCall {
    let sample_str = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file found at {filename}"));
    serde_yaml::from_str(&sample_str).unwrap()
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

    async fn make_request(&mut self, args: &Option<T>) -> U;

    fn serialize_result(&self, sample: &SampleCall, result: &U) -> String;

    async fn check_result(&self, sample: &SampleCall, args: Option<&T>, result: &U);

    async fn check_sample_call(&mut self, sample_file: &str) -> SampleCallResult<T, U> {
        // sanity check that sample inputs are as expected
        let sample = read_sample(sample_file);
        if Self::CALL_HAS_ARGS {
            assert_eq!(sample.request.len(), 2);
        } else {
            assert_eq!(sample.request.len(), 1);
        }
        assert_eq!(sample.request[0], Self::EXPECTED_API_CALL);

        // make the call
        let args = if Self::CALL_HAS_ARGS {
            Some(self.parse_args(&sample.request[1]))
        } else {
            None
        };
        let result = self.make_request(&args).await;

        // check the call against sample outputs
        let actual_json = self.serialize_result(&sample, &result);
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);
        self.check_result(&sample, args.as_ref(), &result).await;

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
