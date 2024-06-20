use std::fmt::Display;
use std::future::Future;
use serde_json::Value;
use crate::models::exception::{ApiError, ClientError};

#[derive(Clone, Debug,)]
pub struct CliDisplayArg {
    pub no_color: bool,
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
    fn stdout(&self, arg: CliDisplayArg) -> Box<dyn Display>;
}