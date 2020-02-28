use reqwest::Error as ReqError;

pub enum AppError {
    CommandError(String),
    LogicError(String),
    NetworkingError(String),
    InteruptionError,
    InputError,
}

impl std::convert::From<ssh2::Error> for AppError {
    fn from(err: ssh2::Error) -> Self {
        AppError::LogicError(format!("Ssh error: {}", err))
    }
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InputError
    }
}

impl std::convert::From<ReqError> for AppError {
    fn from(err: ReqError) -> Self {
        AppError::NetworkingError(format!("{}", err))
    }
}

impl std::convert::From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InputError
    }
}
