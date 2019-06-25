/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration, urlencode};

pub struct GenresApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl GenresApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> GenresApiClient {
        GenresApiClient {
            configuration: configuration,
        }
    }
}

pub trait GenresApi {
    fn get_all_movie_genres_list(&self, language: &str) -> Result<::models::GenresList, Error>;
    fn get_all_tv_genres_list(&self, language: &str) -> Result<::models::GenresList, Error>;
    fn get_movies_by_genre_paginated(&self, genre_id: i32, language: &str, include_adult: bool, sort_by: &str) -> Result<::models::MoviePaginated, Error>;
}

impl GenresApi for GenresApiClient {
    fn get_all_movie_genres_list(&self, language: &str) -> Result<::models::GenresList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/genre/movie/list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_all_tv_genres_list(&self, language: &str) -> Result<::models::GenresList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/genre/tv/list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_movies_by_genre_paginated(&self, genre_id: i32, language: &str, include_adult: bool, sort_by: &str) -> Result<::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/genre/{genre_id}/movies", configuration.base_path, genre_id=genre_id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
        req_builder = req_builder.query(&[("include_adult", &include_adult.to_string())]);
        req_builder = req_builder.query(&[("sort_by", &sort_by.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}