use crate::client::ReqwestClient;
use crate::config::FbxConfig;
use crate::models::freebox::authorization::AuthTokenRequest;
use crate::services::api::FreeboxOSApi;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

#[derive(Clone, Debug)]
pub struct App {
    pub api: FreeboxOSApi,
    pub config: FbxConfig,
    pub client: ReqwestClient,
}

impl Default for App {
    fn default() -> Self {
        App {
            api: FreeboxOSApi,
            config: FbxConfig::load(),
            client: ReqwestClient::default(),
        }
    }
}

impl App {
    pub fn save(&self) {
        self.config.save().expect("TODO: panic message")
    }

    pub async fn initialize(&mut self) {
        let pref = &self.config.pref;
        if let Ok(url) = Url::parse(pref.base_url.as_str()) {
            self.client.set_url(url.to_string());
        } else {
            self.client.set_url(format!("{}/api/{}", pref.base_url, pref.version));
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthAppConfig {
    app_id: String,
    app_name: String,
}

impl From<AuthAppConfig> for AuthTokenRequest {
    fn from(value: AuthAppConfig) -> Self {
        AuthTokenRequest {
            app_id: value.app_id,
            app_name: value.app_name,
            app_version: "1.0.0".to_string(),
            device_name: format!("{} - {}", whoami::username(), whoami::distro()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseResult<T> {
    pub success: bool,
    pub message: Option<String>,
    pub error_code: Option<String>,
    pub result: Option<T>,
}

impl<T> ResponseResult<T> {
    pub fn new(success: bool, message: Option<String>, result: Option<T>, error_code: Option<String>) -> Self {
        ResponseResult {
            success,
            message,
            result,
            error_code
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

impl CliDisplay for SuccessResponse {
    fn json(&self) -> Value {
        json!(self)
    }

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new(format!("L'opération a {}.", match self.success {
            true => "réussi",
            false => "échoué"
        })))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new(format!("L'opération a {}.", match self.success {
            true => "réussi",
            false => "échoué"
        })))
    }
}

pub type EmptyResponse = ();

impl CliDisplay for EmptyResponse {
    fn json(&self) -> Value {
        json!(r#"{"success": true}"#)
    }

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new("L'opération a réussi."))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new("L'opération a réussi."))
    }
}