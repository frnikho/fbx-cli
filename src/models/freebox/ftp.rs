use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FTPError {
    #[serde(rename = "internal_error")]
    Internal,
    #[serde(rename = "weak_password")]
    WeakPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpConfig {
    pub enabled: bool,
    pub allow_anonymous: bool,
    pub allow_anonymous_write: bool,
    pub username: String,
    pub allow_remote_access: bool,
    pub weak_password: bool,
    pub port_ctrl: i32,
    pub port_data: i32,
    pub remote_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpConfigBody {
    pub enabled: Option<bool>,
    pub allow_anonymous: Option<bool>,
    pub allow_anonymous_write: Option<bool>,
    pub password: Option<String>,
    pub allow_remote_access: Option<bool>,
    pub port_ctrl: Option<i32>,
    pub port_data: Option<i32>,
    pub remote_domain: Option<String>,
}

pub type GetFtpConfigResponse = ResponseResult<FtpConfig>;
