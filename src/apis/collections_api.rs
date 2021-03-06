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
use std::rc::Rc;

use reqwest;

use super::configuration;
use crate::Error;

pub struct CollectionsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CollectionsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CollectionsApiClient {
        CollectionsApiClient {
            configuration,
        }
    }
}

pub trait CollectionsApi {
    fn get_collection_details(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::CollectionObject, Error>;
    fn get_collection_images_list(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::Images, Error>;
    fn get_collection_translations_list(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error>;
}

impl CollectionsApi for CollectionsApiClient {
    fn get_collection_details(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::CollectionObject, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/collection/{collection_id}",
            configuration.base_path,
            collection_id = collection_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
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

    fn get_collection_images_list(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/collection/{collection_id}/images",
            configuration.base_path,
            collection_id = collection_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
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

    fn get_collection_translations_list(
        &self,
        collection_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/collection/{collection_id}/translations",
            configuration.base_path,
            collection_id = collection_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
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
