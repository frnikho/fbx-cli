use std::error::Error;
use url::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientError {
    Timeout,
    RequestError(&'static str),
    UnknownError(&'static str),
    Unauthorized(&'static str),
    NotFound(&'static str),
    BuilderError,
    InternalError,
    InvalidUrl(&'static str),
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
        match _value.source() {
            Some(source) => {
                if source.to_string().contains("404") {
                    ApiError::NotFound
                } else if source.to_string().contains("401") {
                    ApiError::Unauthorized("Unauthorized !".to_string())
                } else if source.to_string().contains("400") {
                    ApiError::BadRequest("Bad request !".to_string())
                } else if source.to_string().contains("403") {
                    ApiError::Forbidden("Forbidden !".to_string())
                } else {
                    ApiError::Internal("Internal error !".to_string())
                }
            }
            None => ApiError::Internal("Internal error !".to_string()),
        }
    }
}

impl From<ApiError> for ClientError {
    fn from(value: ApiError) -> Self {
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

impl Into<ClientError> for ParseError {
    fn into(self) -> ClientError {
        match self {
            _ => ClientError::InvalidUrl("Url non valide !"),
        }
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
