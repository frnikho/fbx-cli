use crate::models::freebox::version::FreeboxMajorVersion;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::models::freebox::authorization::AuthTokenResponse;

const APP_NAME: &str = "fbx";
const CONFIG_NAME: &str = "main";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum FbxAppStatus {
    #[default]
    Pending,
    Granted,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FbxConfig {
    pub app: Option<FbxApp>,
    pub session: Option<FbxSession>,
    pub pref: FbxPreferences,
}

impl FbxConfig {

    pub fn register_app_pending(&mut self, app_id: &str, app_version: &str, body: &AuthTokenResponse) {
        let mut fbx_app = FbxApp::default();
        fbx_app.register_pending_app(app_id, app_version, body);
        self.app = Some(fbx_app);
    }

    pub fn register_app_granted(&mut self) {
        if let Some(fbx_app) = &mut self.app {
            fbx_app.register_granted_app();
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FbxApp {
    pub app_id: String,
    pub app_token: Option<String>,
    pub version: String,
    pub track_id: i32,
    pub status: FbxAppStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub authorized_at: Option<chrono::DateTime<Utc>>,
}

impl FbxApp {

    pub fn register_pending_app(&mut self, app_id: &str, app_version: &str, body: &AuthTokenResponse) {
        self.status = FbxAppStatus::Pending;
        self.authorized_at = None;
        self.app_id = app_id.to_string();
        self.version = app_version.to_string();
        self.track_id = body.result.track_id;
        self.created_at = Utc::now();
        self.app_token = Some(body.result.app_token.clone());
    }

    pub fn register_granted_app(&mut self) {
        self.status = FbxAppStatus::Granted;
        self.authorized_at = Some(Utc::now());
    }

}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FbxSession {
    pub token_session: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FbxPreferences {
    pub base_url: String,
    pub version: FreeboxMajorVersion,
}

impl Default for FbxPreferences {
    fn default() -> Self {
        FbxPreferences {
            base_url: "http://mafreebox.freebox.fr".to_string(),
            version: FreeboxMajorVersion::default(),
        }
    }
}

impl FbxConfig {
    pub fn load() -> Self {
        confy::load(APP_NAME, CONFIG_NAME).unwrap_or(FbxConfig::default())
    }

    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, self)
    }

    pub fn reset(&self) -> Result<(), confy::ConfyError> {
        println!("Resetting configuration: {:?}", FbxConfig::default());
        confy::store(APP_NAME, CONFIG_NAME, FbxConfig::default())
    }
}
