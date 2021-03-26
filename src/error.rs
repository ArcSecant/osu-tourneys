use std::backtrace::Backtrace;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("rocket error: {source}, {backtrace}")]
    RocketError {
        #[from]
        source: rocket::error::Error,
        backtrace: Backtrace,
    },
    #[error("request error: {source}, {backtrace}")]
    RequestError {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },
    #[error("oppai error")]
    OppaiError(#[from] oppai_rs::Error),
    #[error("json error: {source}, {backtrace}")]
    JsonError {
        #[from]
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[error("postgres error: {source}, {backtrace}")]
    DbError {
        #[from]
        source: tokio_postgres::Error,
        backtrace: Backtrace,
    },
}
