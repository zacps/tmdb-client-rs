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

use super::{configuration, urlencode, Error};

pub struct AccountApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl AccountApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> AccountApiClient {
        AccountApiClient {
            configuration,
        }
    }
}

pub trait AccountApi {
    fn get_account_favorite_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_account_favorite_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error>;
    fn get_account_rated_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_account_rated_tv_episodes_paginated(
        &self,
        account_id: &str,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvEpisodesPaginated, Error>;
    fn get_account_rated_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error>;
    fn get_account_watchlist_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_account_watchlist_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error>;
    fn get_current_account_details(
        &self,
        session_id: &str,
    ) -> Result<crate::models::AccountDetails, Error>;
    fn get_current_account_lists_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        api_key: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::ListsPaginated, Error>;
    fn post_account_favorite(
        &self,
        account_id: i32,
        session_id: &str,
        content_type: &str,
        body: Option<crate::models::MediaFavoriteBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error>;
    fn post_account_watchlist(
        &self,
        account_id: i32,
        content_type: &str,
        session_id: &str,
        body: Option<crate::models::MediaWatchlistBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error>;
}

impl AccountApi for AccountApiClient {
    fn get_account_favorite_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/favorite/movies",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_favorite_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/favorite/tv",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_rated_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/rated/movies",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_rated_tv_episodes_paginated(
        &self,
        account_id: &str,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvEpisodesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/rated/tv/episodes",
            configuration.base_path,
            account_id = urlencode(account_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_rated_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/rated/tv",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_watchlist_movies_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/watchlist/movies",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_account_watchlist_tv_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/watchlist/tv",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_current_account_details(
        &self,
        session_id: &str,
    ) -> Result<crate::models::AccountDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/account", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
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

    fn get_current_account_lists_paginated(
        &self,
        account_id: i32,
        session_id: &str,
        api_key: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::ListsPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/lists",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
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

    fn post_account_favorite(
        &self,
        account_id: i32,
        session_id: &str,
        content_type: &str,
        body: Option<crate::models::MediaFavoriteBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/favorite",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
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

    fn post_account_watchlist(
        &self,
        account_id: i32,
        content_type: &str,
        session_id: &str,
        body: Option<crate::models::MediaWatchlistBody>,
    ) -> Result<crate::models::StatusCodeMessage, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/account/{account_id}/watchlist",
            configuration.base_path,
            account_id = account_id
        );
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
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
