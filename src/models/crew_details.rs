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
pub struct CrewDetails {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "credit_id", skip_serializing_if = "Option::is_none")]
    pub credit_id: Option<String>,
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(rename = "original_title", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "media_type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "episode_count", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    #[serde(rename = "first_air_date", skip_serializing_if = "Option::is_none")]
    pub first_air_date: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "original_name", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
}
