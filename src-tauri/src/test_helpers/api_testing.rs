use crate::sample_call::SampleCall;
use serde::{Deserialize, Serialize};
use std::fs;

fn read_sample(filename: &str) -> SampleCall {
    let sample_str = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file found at {filename}"));
    serde_yaml::from_str(&sample_str).unwrap()
}

pub struct SampleCallResult<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub sample: SampleCall,
    pub args: Option<T>,
}

pub trait SampleCallTestCase<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const EXPECTED_API_CALL: &'static str;
    const CALL_HAS_ARGS: bool;

    fn parse_request(&self, request_str: &str) -> T {
        serde_json::from_str(request_str).unwrap()
    }

    fn check_sample_call(&self, sample_file: &str) -> SampleCallResult<T> {
        let sample = read_sample(sample_file);
        if Self::CALL_HAS_ARGS {
            assert_eq!(sample.request.len(), 2);
        } else {
            assert_eq!(sample.request.len(), 1);
        }
        assert_eq!(sample.request[0], Self::EXPECTED_API_CALL);

        let request = if Self::CALL_HAS_ARGS {
            Some(self.parse_request(&sample.request[1]))
        } else {
            None
        };
        SampleCallResult {
            sample,
            args: request,
        }
    }
}
