use serde::Deserialize;
use thiserror::Error;
use crate::app::{ResponseResult, SuccessResponse};

#[derive(Error, Clone, Debug, Deserialize)]
pub enum VoicemailError {
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal,
    #[error("No voicemail with this id")]
    #[serde(rename = "invalid_id")]
    InvalidId
}

#[derive(Clone, Debug, Deserialize)]
pub struct VoicemailEntry {
    pub id: String,
    pub country_code: String,
    pub phone_number: String,
    pub date: i32,
    pub read: bool,
    pub duration: i32,
}

pub type ListVoicemailResponse = ResponseResult<Vec<VoicemailEntry>>;
pub type GetVoicemailResponse = ResponseResult<VoicemailEntry>;
pub type DeleteVoicemailResponse = SuccessResponse;
pub type UpdateVoicemailResponse = ResponseResult<VoicemailEntry>;
pub type RetrieveVoicemailResponse = ResponseResult<Vec<u8>>;