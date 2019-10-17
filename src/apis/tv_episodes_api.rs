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

use super::{configuration, Error};

pub struct TVEpisodesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TVEpisodesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TVEpisodesApiClient {
        TVEpisodesApiClient {
            configuration,
        }
    }
}

pub trait TVEpisodesApi {
    fn delete_tv_season_episode_rating(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::StatusCodeMessage, Error>;
    fn get_tv_episode_changes(
        &self,
        episode_id: i32,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error>;
    fn get_tv_season_episode_account_states(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::EpisodeRatingList, Error>;
    fn get_tv_season_episode_credits(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::Credits, Error>;
    fn get_tv_season_episode_details(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::EpisodeDetails, Error>;
    fn get_tv_season_episode_external_ids(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::ExternalIds, Error>;
    fn get_tv_season_episode_images(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error>;
    fn get_tv_season_episode_translations_list(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::TranslationsList, Error>;
    fn get_tv_season_episode_videos_list(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error>;
    fn post_tv_season_episode_rating(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
        body: Option<crate::models::ValueBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error>;
}

impl TVEpisodesApi for TVEpisodesApiClient {
    fn delete_tv_season_episode_rating(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::StatusCodeMessage, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_tv_episode_changes(
        &self,
        episode_id: i32,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/episode/{episode_id}/changes",
            configuration.base_path,
            episode_id = episode_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = start_date {
            req_builder = req_builder.query(&[("start_date", &s.to_string())]);
        }
        if let Some(ref s) = end_date {
            req_builder = req_builder.query(&[("end_date", &s.to_string())]);
        }
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

    fn get_tv_season_episode_account_states(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::EpisodeRatingList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/account_states",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
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

    fn get_tv_season_episode_credits(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::Credits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/credits",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
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

    fn get_tv_season_episode_details(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::EpisodeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
        if let Some(ref s) = append_to_response {
            req_builder = req_builder.query(&[("append_to_response", &s.to_string())]);
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

    fn get_tv_season_episode_external_ids(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::ExternalIds, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/external_ids",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
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

    fn get_tv_season_episode_images(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/images",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_tv_season_episode_translations_list(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
    ) -> Result<crate::models::TranslationsList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/translations",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
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

    fn get_tv_season_episode_videos_list(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/videos",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
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

    fn post_tv_season_episode_rating(
        &self,
        tv_id: i32,
        season_number: i32,
        episode_number: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
        body: Option<crate::models::ValueBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number,
            episode_number = episode_number
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
