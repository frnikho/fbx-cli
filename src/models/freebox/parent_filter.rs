/*use crate::app::ResponseResult;
use crate::models::freebox::lan::LanHost;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileCreated {
    id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkControl {
    pub profile_id: i32,
    pub next_change: i32,
    pub override_mode: NetworkControlMode,
    pub current_mode: NetworkControlMode,
    //pub rule_mode: current_mode,
    pub override_until: i32,
    pub r#override: bool,
    pub macs: Vec<String>,
    pub hosts: Vec<LanHost>,
    pub resolution: i32,
    pub cdayranges: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkControlMode {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "webonly")]
    WebOnly,
}

pub type ListProfileResponse = ResponseResult<Vec<Profile>>;
pub type GetProfileResponse = ResponseResult<Profile>;
pub type UpdateProfileResponse = ResponseResult<Profile>;
pub type DeleteProfileResponse = ResponseResult<()>;
pub type CreateProfileResponse = ResponseResult<ProfileCreated>;
*/