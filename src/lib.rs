#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate thiserror;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
pub mod files;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
