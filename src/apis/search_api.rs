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

pub struct SearchApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl SearchApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> SearchApiClient {
        SearchApiClient {
            configuration,
        }
    }
}

pub trait SearchApi {
    fn get_search_collection_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::CollectionPaginated, Error>;
    fn get_search_company_paginated(
        &self,
        query: &str,
        page: Option<i32>,
    ) -> Result<crate::models::CompanyPaginated, Error>;
    fn get_search_keyword_paginated(
        &self,
        query: &str,
        page: Option<i32>,
    ) -> Result<crate::models::KeywordPaginated, Error>;
    fn get_search_movie_paginated(
        &self,
        query: &str,
        year: Option<i32>,
        primary_release_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_search_multi_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::SearchMultiResultsPaginated, Error>;
    fn get_search_person_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::PersonPaginated, Error>;
    fn get_search_tv_paginated(
        &self,
        query: &str,
        first_air_date_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::TvPaginated, Error>;
}

impl SearchApi for SearchApiClient {
    fn get_search_collection_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::CollectionPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/collection", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_search_company_paginated(
        &self,
        query: &str,
        page: Option<i32>,
    ) -> Result<crate::models::CompanyPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/company", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_search_keyword_paginated(
        &self,
        query: &str,
        page: Option<i32>,
    ) -> Result<crate::models::KeywordPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/keyword", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_search_movie_paginated(
        &self,
        query: &str,
        year: Option<i32>,
        primary_release_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/movie", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = year {
            req_builder = req_builder.query(&[("year", &s.to_string())]);
        }
        if let Some(ref s) = primary_release_year {
            req_builder = req_builder.query(&[("primary_release_year", &s.to_string())]);
        }
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_search_multi_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::SearchMultiResultsPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/multi", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_search_person_paginated(
        &self,
        query: &str,
        language: Option<&str>,
        page: Option<i32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::models::PersonPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/person", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_search_tv_paginated(
        &self,
        query: &str,
        first_air_date_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/tv", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = first_air_date_year {
            req_builder = req_builder.query(&[("first_air_date_year", &s.to_string())]);
        }
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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
