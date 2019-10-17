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
pub struct SeasonObject {
    #[serde(rename = "air_date", skip_serializing_if = "Option::is_none")]
    pub air_date: Option<String>,
    #[serde(rename = "episode_count", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "season_number", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
}
