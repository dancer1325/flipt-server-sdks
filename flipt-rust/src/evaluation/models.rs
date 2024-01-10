use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct BatchEvaluationRequest {
    pub requests: Vec<EvaluationRequest>,
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationRequest {
    pub namespace_key: String,
    pub flag_key: String,
    pub entity_id: String,
    pub context: HashMap<String, String>,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanEvaluationResponse {
    pub enabled: bool,
    pub reason: String,
    pub request_id: String,
    pub request_duration_millis: f64,
    pub timestamp: DateTime<Utc>,
    pub flag_key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantEvaluationResponse {
    #[serde(rename = "match")]
    pub r#match: bool,
    pub segment_keys: Vec<String>,
    pub reason: String,
    pub variant_key: String,
    pub variant_attachment: String,
    pub request_id: String,
    pub request_duration_millis: f64,
    pub timestamp: DateTime<Utc>,
    pub flag_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvaluationResponse {
    pub flag_key: String,
    pub namespace_key: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEvaluationResponse {
    pub request_id: String,
    pub responses: Vec<Response>,
    pub request_duration_millis: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub r#type: String,
    pub boolean_response: Option<BooleanEvaluationResponse>,
    pub variant_response: Option<VariantEvaluationResponse>,
    pub error_response: Option<ErrorEvaluationResponse>,
}
