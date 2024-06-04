use crate::models::freebox::version::FreeboxMajorVersion;
use chrono::Utc;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "fbx";
const CONFIG_NAME: &str = "main";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum FbxAppStatus {
    #[default]
    Pending,
    Granted(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FbxConfig {
    pub app: Option<FbxApp>,
    pub session: Option<FbxSession>,
    pub pref: FbxPreferences,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FbxApp {
    pub app_id: String,
    pub version: String,
    pub track_id: i32,
    pub status: FbxAppStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub authorized_at: Option<chrono::DateTime<Utc>>,
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
