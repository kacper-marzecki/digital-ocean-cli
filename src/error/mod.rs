
use reqwest::{Error as ReqError};

pub enum AppError {
    CommandError(String),
    NetworkingError(String)
}

impl std::convert::From<ReqError> for AppError {
    fn from(err: ReqError)-> Self {
        AppError::NetworkingError(format!("{}", err))
    }
}