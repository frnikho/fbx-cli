use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ClientError {
    #[error("Timeout")]
    Timeout,
    #[error("Request error")]
    RequestError(&'static str),
    #[error("Unknown error")]
    UnknownError(&'static str),
    #[error("Unauthorized")]
    Unauthorized(&'static str),
    #[error("Not found")]
    NotFound(&'static str),
    #[error("Builder error")]
    BuilderError,
    #[error("Internal error")]
    InternalError,
    #[error("Invalid url")]
    InvalidUrl(&'static str),
    #[error("Authorization required")]
    CliNeedAuth(Option<&'static str>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    NotFound,
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    Forbidden(String),
    ParseBody,
}

impl From<reqwest::Error> for ApiError {
    fn from(_value: reqwest::Error) -> Self {
        println!("Error: {:?}", _value);
        ApiError::Internal("".to_string())
    }
}

impl From<ApiError> for ClientError {
    fn from(value: ApiError) -> Self {
        println!("Error: {:?}", value);
        match value {
            ApiError::NotFound => ClientError::NotFound("Not found !"),
            ApiError::Unauthorized(_) => ClientError::Unauthorized("Unauthorized !"),
            ApiError::BadRequest(_) => ClientError::RequestError("Bad request !"),
            ApiError::Internal(_) => ClientError::InternalError,
            ApiError::Forbidden(_) => ClientError::RequestError("Forbidden !"),
            ApiError::ParseBody => ClientError::RequestError("Parse body error !"),
        }
    }
}

impl From<ParseError> for ClientError {
    fn from(_value: ParseError) -> Self {
        ClientError::InvalidUrl("Url non valide !")
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        println!("Error: {:?}", error);
        if error.is_builder() {
            ClientError::BuilderError
        } else if error.is_timeout() {
            ClientError::Timeout
        } else if error.is_request() {
            ClientError::RequestError("Request error !")
        } else {
            ClientError::UnknownError("Unknown error !")
        }
    }
}
