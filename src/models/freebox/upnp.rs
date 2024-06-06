use serde::Deserialize;
use crate::app::ResponseResult;

#[derive(Clone, Debug, Deserialize)]
pub enum UpnpError {
    #[serde(rename = "internal_error")]
    Internal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpnpAvConfig {
    pub enabled: bool,
}

pub type GetUpnpAvConfigResponse = ResponseResult<UpnpAvConfig>;
pub type UpdateUpnpAvConfigResponse = ResponseResult<UpnpAvConfig>;