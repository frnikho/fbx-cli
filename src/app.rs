use crate::client::ReqwestClient;
use crate::config::FbxConfig;
use crate::models::args::Cli;
use crate::models::freebox::authorization::AuthTokenRequest;
use crate::services::api::FreeboxOSApi;
use clap::Parser;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug)]
pub struct App {
    pub api: FreeboxOSApi,
    pub config: FbxConfig,
    pub client: ReqwestClient,
    pub cli: Cli,
}

impl Default for App {
    fn default() -> Self {
        App {
            api: FreeboxOSApi,
            config: FbxConfig::load(),
            client: ReqwestClient::default(),
            cli: Cli::parse(),
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

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseResult<T> {
    pub success: bool,
    pub message: Option<String>,
    pub result: Option<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
}

pub type EmptyResponse = ();