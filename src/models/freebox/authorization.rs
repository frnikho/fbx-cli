use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::str::FromStr;
use crate::app::{ResponseResult, SuccessResponse};

const APP_ID: &str = "dev.nikho.fbxcli";
const APP_NAME: &str = "fbx-cli";
const APP_VERSION: &str = "1.0.0";

#[derive(Serialize, Debug, Clone)]
pub struct AuthTokenRequest {
    pub app_id: String,
    pub app_name: String,
    pub app_version: String,
    pub device_name: String,
}

impl Default for AuthTokenRequest {
    fn default() -> Self {
        AuthTokenRequest {
            app_id: APP_ID.to_string(),
            app_name: APP_NAME.to_string(),
            app_version: APP_VERSION.to_string(),
            device_name: format!("{} - {}", whoami::username(), whoami::distro()),
        }
    }
}

impl AuthTokenRequest {
    pub fn new(app_id: &Option<String>, app_version: &Option<String>) -> Self {
        AuthTokenRequest {
            app_id: app_id.clone().unwrap_or(APP_ID.to_string()),
            app_name: APP_NAME.to_string(),
            app_version: app_version.clone().unwrap_or(APP_VERSION.to_string()),
            device_name: format!("{} - {}", whoami::username(), whoami::distro()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthTokenResult {
    pub app_token: String,
    pub track_id: i32,
}

impl AuthTokenResult {
    pub fn new(app_token: String, track_id: i32) -> Self {
        AuthTokenResult { app_token, track_id }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthTokenResponse {
    pub success: bool,
    pub result: AuthTokenResult,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AuthTrackAuthorizationProgressStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthTrackAuthorizationProgressResult {
    pub status: AuthTrackAuthorizationProgressStatus,
    pub challenge: String,
}

pub type AuthTrackAuthorizationProgressResponse = ResponseResult<AuthTrackAuthorizationProgressResult>;

// Session

#[derive(Serialize, Debug, Clone)]
pub struct AuthSessionStartRequest {
    pub app_id: String,
    pub app_version: String,
    pub password: String,
}

impl AuthSessionStartRequest {
    pub fn new(token: AuthTokenRequest, app_token: String, challenge: String) -> Option<Self> {
        Some(AuthSessionStartRequest {
            app_id: token.app_id,
            app_version: token.app_version,
            password: Self::generate_password(app_token, challenge)?,
        })
    }

    pub fn new_basic(password: String) -> Self {
        let auth_token = AuthTokenRequest::default();
        AuthSessionStartRequest {
            app_id: auth_token.app_id,
            app_version: auth_token.app_version,
            password,
        }
    }

    pub fn generate_password(app_token: String, challenge: String) -> Option<String> {
        let mut hasher: Hmac<Sha1> = Mac::new_from_slice(app_token.as_bytes())
            .map_err(|_| ())
            .ok()?;
        hasher.update(challenge.as_bytes());
        Some(
            hasher
                .finalize()
                .into_bytes()
                .iter()
                .fold(String::new(), |mut acc, b| {
                    acc.push_str(&format!("{:02x}", b));
                    acc
                }),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthSessionPermissions {
    pub settings: Option<bool>,
    pub contacts: Option<bool>,
    pub calls: Option<bool>,
    pub explorer: Option<bool>,
    pub downloader: Option<bool>,
    pub parental: Option<bool>,
    pub pvr: Option<bool>,
    pub profile: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthSessionStartResult {
    pub session_token: String,
    pub challenge: String,
    pub permissions: AuthSessionPermissions,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthSessionStartResponse {
    pub success: bool,
    pub result: AuthSessionStartResult,
}

// Login

#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResult {
    pub logged_in: bool,
    pub challenge: Option<String>,
}

pub type AuthLoginResponse = ResponseResult<AuthLoginResult>;
pub type AuthLogoutResponse = SuccessResponse;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct AuthorizationError {
    pub success: bool,
    pub error_code: AuthorizationErrorKind,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum AuthorizationErrorKind {
    #[serde(rename = "auth_required")]
    AuthRequired,
    #[serde(rename = "invalid_token")]
    InvalidToken,
    #[serde(rename = "pending_token")]
    PendingToken,
    #[serde(rename = "insufficient_rights")]
    InsufficientRights,
    #[serde(rename = "denied_from_external_ip")]
    DeniedFromExternalIp,
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[serde(rename = "rate_limited")]
    RateLimited,
    #[serde(rename = "new_apps_denied")]
    NewAppsDenied,
    #[serde(rename = "apps_denied")]
    AppsDenied,
    #[serde(rename = "internal_error")]
    InternalError,
}

impl FromStr for AuthorizationErrorKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auth_required" => Ok(AuthorizationErrorKind::AuthRequired),
            "invalid_token" => Ok(AuthorizationErrorKind::InvalidToken),
            "pending_token" => Ok(AuthorizationErrorKind::PendingToken),
            "insufficient_rights" => Ok(AuthorizationErrorKind::InsufficientRights),
            "denied_from_external_ip" => Ok(AuthorizationErrorKind::DeniedFromExternalIp),
            "invalid_request" => Ok(AuthorizationErrorKind::InvalidRequest),
            "rate_limited" => Ok(AuthorizationErrorKind::RateLimited),
            "new_apps_denied" => Ok(AuthorizationErrorKind::NewAppsDenied),
            "apps_denied" => Ok(AuthorizationErrorKind::AppsDenied),
            "internal_error" => Ok(AuthorizationErrorKind::InternalError),
            _ => Err(()),
        }
    }
}
