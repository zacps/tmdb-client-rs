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
pub struct MovieDetails {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "backdrop_path", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    #[serde(rename = "belongs_to_collection", skip_serializing_if = "Option::is_none")]
    pub belongs_to_collection: Option<serde_json::Value>,
    #[serde(rename = "budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<i64>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<crate::models::Genre>>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "imdb_id", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_title", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(
        rename = "production_companies",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_companies: Option<Vec<crate::models::CompanyObject>>,
    #[serde(
        rename = "production_countries",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_countries: Option<Vec<crate::models::Translation>>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "revenue", skip_serializing_if = "Option::is_none")]
    pub revenue: Option<i64>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "spoken_languages", skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<crate::models::Translation>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tagline", skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f32>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    // "append-to" options
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none")]
    pub credits: Option<crate::models::Credits>,
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<crate::models::VideosList>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<crate::models::Images>,
    #[serde(rename = "release_dates", skip_serializing_if = "Option::is_none")]
    pub release_dates: Option<crate::models::ReleaseDatesList>,
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<crate::models::TranslationsList>,
    #[serde(rename = "keywords", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<crate::models::KeywordsList>,
    #[serde(rename = "reviews", skip_serializing_if = "Option::is_none")]
    pub reviews: Option<crate::models::ReviewsPaginated>,
    #[serde(rename = "external_ids", skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<crate::models::ExternalIds>,
}

impl MovieDetails {
    pub fn new() -> MovieDetails {
        MovieDetails {
            adult: None,
            backdrop_path: None,
            belongs_to_collection: None,
            budget: None,
            genres: None,
            homepage: None,
            id: None,
            imdb_id: None,
            original_language: None,
            original_title: None,
            overview: None,
            popularity: None,
            poster_path: None,
            production_companies: None,
            production_countries: None,
            release_date: None,
            revenue: None,
            runtime: None,
            spoken_languages: None,
            status: None,
            tagline: None,
            title: None,
            video: None,
            vote_average: None,
            vote_count: None,
            credits: None,
            videos: None,
            images: None,
            release_dates: None,
            translations: None,
            keywords: None,
            reviews: None,
            external_ids: None,
        }
    }
}
