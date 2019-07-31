/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonEpisodeIds {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "season_id", skip_serializing_if = "Option::is_none")]
    pub season_id: Option<i32>,
    #[serde(rename = "episode_id", skip_serializing_if = "Option::is_none")]
    pub episode_id: Option<i32>,
}

impl SeasonEpisodeIds {
    pub fn new() -> SeasonEpisodeIds {
        SeasonEpisodeIds {
            id: None,
            season_id: None,
            episode_id: None,
        }
    }
}


