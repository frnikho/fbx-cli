use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SFPError {
    #[error("Invalid parameters")]
    #[serde(rename = "inval")]
    InvalidParameters,
    #[error("Invalid id")]
    #[serde(rename = "noent")]
    InvalidId,
    #[error("System internal error")]
    #[serde(rename = "internal")]
    InternalError,
}

#[derive(Debug, Clone, Serialize)]
pub struct SfpConfig {
    pub sfp_type_forced: bool,
    pub sfp_type_forced_value: SFPTypes,
    pub available_sfp_types: Vec<SFPTypes>
}

pub struct SfpConfigUpdate {
    pub sfp_type_forced: Option<bool>,
    pub sfp_type_forced_value: Option<SFPTypes>,
}

#[derive(Debug, Clone, Serialize)]
pub enum SFPTypes {
    #[serde(rename = "p2p_1g")]
    P2p1g,
    #[serde(rename = "p2p_2d5g_no_aneg")]
    P2p2d5gNoAneg,
    #[serde(rename = "p2p_10g	")]
    P2p10g,
    #[serde(rename = "copper_1g")]
    Copper1g,
    #[serde(rename = "copper_sgmii_1g")]
    CopperSgmii1g,
    #[serde(rename = "copper_sgmii_10g")]
    CopperSgmii10g,
}

#[derive(Debug, Clone, Serialize)]
pub struct SfpStatus {
    pub present: bool,
    pub eeprom_valid: bool,
    pub supported: bool,
    pub r#type: SFPTypes,  //TODO: check if type is good
    pub power_good: bool,
    pub link: bool,
    pub vendor_name: String,
    pub part_number: String,
    pub hardware_rev: String,
    pub serial_number: String,
}

pub type GetSfpStatusResponse = ResponseResult<SfpStatus>;
pub type GetSfpConfigResponse = ResponseResult<SfpConfig>;
pub type UpdateSfpConfigResponse = ResponseResult<SfpConfig>;

pub trait SfpCalls<T: HttpClient> {
    async fn get_sfp_status(&self, client: &T, session: &str) -> Result<GetSfpStatusResponse, T::Error>;
    async fn get_sfp_config(&self, client: &T, session: &str) -> Result<GetSfpConfigResponse, T::Error>;
    async fn update_sfp_config(&self, client: &T, session: &str, body: SfpConfigUpdate) -> Result<UpdateSfpConfigResponse, T::Error>;
}