// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::sample_call;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: sample_call = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleCall {
    pub request: Vec<String>,

    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,

    pub success: Option<bool>,
}
