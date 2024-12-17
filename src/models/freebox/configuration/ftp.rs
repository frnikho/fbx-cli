use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::client::HttpClient;
use crate::models::args::{FtpAnonymousMode, FtpSetArgs};
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

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

impl CliDisplay for FtpConfig {
    fn json(&self) -> Value {
        json!(self)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        let mut s = String::from("FTP configuration:\n");
        s.push_str(&format!("Enabled: {}\n", self.enabled));
        s.push_str(&format!("Allow anonymous: {}\n", self.allow_anonymous));
        s.push_str(&format!("Allow anonymous write: {}\n", self.allow_anonymous_write));
        s.push_str(&format!("Username: {}\n", self.username));
        s.push_str(&format!("Allow remote access: {}\n", self.allow_remote_access));
        s.push_str(&format!("Weak password: {}\n", self.weak_password));
        s.push_str(&format!("Control port: {}\n", self.port_ctrl));
        s.push_str(&format!("Data port: {}\n", self.port_data));
        s.push_str(&format!("Remote domain: {}\n", self.remote_domain));
        CliResult::success(Box::new(s))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
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

impl From<FtpSetArgs> for FtpConfigBody {
    fn from(value: FtpSetArgs) -> Self {
        Self {
            enabled: value.enable,
            allow_anonymous: value.anonymous.clone().map(|x| x == FtpAnonymousMode::Read || x == FtpAnonymousMode::ReadWrite),
            allow_anonymous_write: value.anonymous.map(|x| x == FtpAnonymousMode::ReadWrite),
            password: value.password,
            allow_remote_access: value.allow_remote,
            port_ctrl: value.remote_port,
            port_data: value.remote_port_data,
            remote_domain: value.remote_domain,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpConfigResponse(pub ResponseResult<FtpConfig>);

impl CliDisplay for FtpConfigResponse {
    fn json(&self) -> Value {
        match &self.0.result {
            Some(x) => x.json(),
            None => json!({}),
        }
    }

    fn stdout(&self, arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(x) => x.stdout(arg),
            None => CliResult::success(Box::new("No FTP configuration found")),
        }
    }

    fn raw(&self, arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(x) => x.raw(arg),
            None => CliResult::success(Box::new("No FTP configuration found")),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFtpConfigResponse(pub ResponseResult<FtpConfig>);

impl CliDisplay for UpdateFtpConfigResponse {
    fn json(&self) -> Value {
        match &self.0.result {
            Some(x) => x.json(),
            None => json!({}),
        }
    }

    fn stdout(&self, arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(x) => x.stdout(arg),
            None => CliResult::success(Box::new("No FTP configuration found")),
        }
    }

    fn raw(&self, arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(x) => x.raw(arg),
            None => CliResult::success(Box::new("No FTP configuration found")),
        }
    }
}

pub trait FtpCalls<T: HttpClient> {
    async fn get_ftp_config(&self, client: &T, session: &str) -> Result<FtpConfigResponse, T::Error>;
    async fn update_ftp_config(&self, client: &T, session: &str, body: FtpConfigBody) -> Result<UpdateFtpConfigResponse, T::Error>;
}