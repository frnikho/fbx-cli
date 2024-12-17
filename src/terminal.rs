use std::fmt::Display;
use std::future::Future;
use serde_json::Value;
use crate::models::exception::{ApiError, ClientError};

#[derive(Clone, Debug, Default)]
pub struct CliDisplayArg {
    pub no_color: Option<bool>,
    pub json: Option<bool>,
    pub raw: Option<bool>,
}

pub type HandlerResult = Result<Box<dyn CliDisplay>, ClientError>;

pub async fn handler_result<T>(t: impl Future<Output = Result<T, ApiError>>) -> HandlerResult where T: CliDisplay + 'static {
    Ok(Box::new(t.await?))
}

pub async fn handler_ok<T>(t: T) -> HandlerResult where T: CliDisplay + 'static {
    Ok(Box::new(t))
}

pub async fn handler_error(t: ClientError) -> HandlerResult {
    Err(t)
}

pub trait CliDisplay {
    fn json(&self) -> Value;
    fn stdout(&self, arg: CliDisplayArg) -> CliResult;
    fn raw(&self, arg: CliDisplayArg) -> CliResult;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum CliResultCode {
    #[default]
    Success,
    Error
}

pub struct CliResult {
    pub result: Box<dyn Display>,
    pub code: CliResultCode
}

impl Display for CliResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.result)
    }
}

impl CliResult {
    pub fn new(result: Box<dyn Display>, code: CliResultCode) -> Self {
        Self {
            result,
            code
        }
    }
    
    pub fn success(result: Box<dyn Display>) -> Self {
        Self::new(result, CliResultCode::Success)
    }
    
    pub fn error(result: Box<dyn Display>) -> Self {
        Self::new(result, CliResultCode::Error)
    }
}