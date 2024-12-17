use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct VersionResponse {
    pub uid: String,
    pub https_available: bool,
    pub api_domain: String,
    pub api_version: String,
    pub api_base_url: String,
    pub https_port: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum FreeboxMajorVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    #[default]
    LATEST,
}

impl FreeboxMajorVersion {
    pub fn from_api_version(version: String) -> Self {
        let b = version.split('.').collect::<Vec<&str>>();
        if b.len() > 1 {
            return FreeboxMajorVersion::from(b[0]);
        }
        FreeboxMajorVersion::V8
    }
}

impl From<&str> for FreeboxMajorVersion {
    fn from(value: &str) -> Self {
        FreeboxMajorVersion::from(String::from(value))
    }
}

impl From<String> for FreeboxMajorVersion {
    fn from(value: String) -> Self {
        match value {
            v if v == "v1" => FreeboxMajorVersion::V1,
            v if v == "v2" => FreeboxMajorVersion::V2,
            v if v == "v3" => FreeboxMajorVersion::V3,
            v if v == "v4" => FreeboxMajorVersion::V4,
            v if v == "v5" => FreeboxMajorVersion::V5,
            v if v == "v6" => FreeboxMajorVersion::V6,
            v if v == "v7" => FreeboxMajorVersion::V7,
            v if v == "v8" => FreeboxMajorVersion::V8,
            v if v == "v9" => FreeboxMajorVersion::V9,
            v if v == "v10" => FreeboxMajorVersion::V10,
            v if v == "v11" => FreeboxMajorVersion::V11,
            v if v == "latest" => FreeboxMajorVersion::LATEST,
            _ => FreeboxMajorVersion::V11,
        }
    }
}

impl Display for FreeboxMajorVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FreeboxMajorVersion::V1 => "v1".to_string(),
            FreeboxMajorVersion::V2 => "v2".to_string(),
            FreeboxMajorVersion::V3 => "v3".to_string(),
            FreeboxMajorVersion::V4 => "v4".to_string(),
            FreeboxMajorVersion::V5 => "v5".to_string(),
            FreeboxMajorVersion::V6 => "v6".to_string(),
            FreeboxMajorVersion::V7 => "v7".to_string(),
            FreeboxMajorVersion::V8 => "v8".to_string(),
            FreeboxMajorVersion::V9 => "v9".to_string(),
            FreeboxMajorVersion::V10 => "v10".to_string(),
            FreeboxMajorVersion::V11 => "v11".to_string(),
            FreeboxMajorVersion::LATEST => "latest".to_string(),
        };
        write!(f, "{}", str)
    }
}
