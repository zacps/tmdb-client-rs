/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct GuestSessionResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "guest_session_id", skip_serializing_if = "Option::is_none")]
    pub guest_session_id: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl GuestSessionResponse {
    pub fn new() -> GuestSessionResponse {
        GuestSessionResponse {
            success: None,
            guest_session_id: None,
            expires_at: None,
        }
    }
}

