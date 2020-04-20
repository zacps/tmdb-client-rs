/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
use std::option::Option;
use std::sync::Arc;

use reqwest;

use super::configuration;
use crate::Error;

pub struct KeywordsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl KeywordsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> KeywordsApiClient {
        KeywordsApiClient {
            configuration,
        }
    }
}

pub trait KeywordsApi {
    fn get_keyword_details(&self, keyword_id: i32) -> Result<crate::models::Keyword, Error>;
    fn get_movies_by_keyword_paginated(
        &self,
        keyword_id: i32,
        language: Option<&str>,
        include_adult: Option<bool>,
    ) -> Result<crate::models::MoviePaginated, Error>;
}

impl KeywordsApi for KeywordsApiClient {
    fn get_keyword_details(&self, keyword_id: i32) -> Result<crate::models::Keyword, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keyword/{keyword_id}",
            configuration.base_path,
            keyword_id = keyword_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_movies_by_keyword_paginated(
        &self,
        keyword_id: i32,
        language: Option<&str>,
        include_adult: Option<bool>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keyword/{keyword_id}/movies",
            configuration.base_path,
            keyword_id = keyword_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
