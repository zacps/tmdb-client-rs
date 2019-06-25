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

use super::{Error, configuration};

pub struct TimezonesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TimezonesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TimezonesApiClient {
        TimezonesApiClient {
            configuration: configuration,
        }
    }
}

pub trait TimezonesApi {
    fn get_timezones_list(&self) -> Result<Vec<serde_json::Value>, Error>;
}

impl TimezonesApi for TimezonesApiClient {
    fn get_timezones_list(&self) -> Result<Vec<serde_json::Value>, Error> {

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/timezones/list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

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
