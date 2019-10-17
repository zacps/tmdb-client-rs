/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Cast {
    #[serde(rename = "cast_id", skip_serializing_if = "Option::is_none")]
    pub cast_id: Option<i32>,
    #[serde(rename = "character", skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    #[serde(rename = "credit_id", skip_serializing_if = "Option::is_none")]
    pub credit_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "profile_path", skip_serializing_if = "Option::is_none")]
    pub profile_path: Option<String>,
}
