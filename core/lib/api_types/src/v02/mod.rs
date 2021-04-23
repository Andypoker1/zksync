use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use zksync_types::network::Network;

pub mod block;
pub mod fee;
pub mod pagination;
pub mod status;
pub mod token;
pub mod transaction;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    V02,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultStatus {
    Success,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub network: Network,
    pub api_version: ApiVersion,
    pub resource: String,
    pub args: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub request: Request,
    pub status: ResultStatus,
    pub error: Option<Value>,
    pub result: Option<Value>,
}
