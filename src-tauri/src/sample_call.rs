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
#[serde(rename_all = "camelCase")]
pub struct SampleCall {
    pub request: Vec<String>,

    pub response: Response,

    pub side_effects: Option<SideEffects>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,

    pub success: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SideEffects {
    pub database: Option<Database>,

    pub disk: Option<Disk>,

    pub network: Option<Network>,

    pub terminal: Option<Terminal>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub end_state_dump: String,

    pub start_state_dump: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub end_state_directory: String,

    pub start_state_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub recording_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
    pub recording_file: String,

    pub starting_index: Option<i64>,
}
