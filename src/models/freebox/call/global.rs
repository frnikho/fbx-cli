use serde::Deserialize;
use thiserror::Error;
use crate::app::{ResponseResult, SuccessResponse};

#[derive(Error, Clone, Debug, Deserialize)]
pub enum CallError {
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal,
    #[error("No call with this id")]
    #[serde(rename = "invalid_id")]
    InvalidId,
    #[error("Invalid call category")]
    #[serde(rename = "invalid_category")]
    InvalidCategory,
}

#[derive(Clone, Debug, Deserialize)]
pub enum CallEntryType {
    #[serde(rename = "missed")]
    Missed,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "outgoing")]
    Outgoing
}

#[derive(Clone, Debug, Deserialize)]
pub struct CallEntry {
    pub id: i32,
    pub r#type: CallEntryType,
    pub datetime: i32,
    pub number: String,
    pub name: String,
    pub duration: i32,
    pub new: bool,
    pub contact_id: Option<i32>,
}


pub type ListCallResponse = ResponseResult<Vec<CallEntry>>;
pub type DeleteAllCallResponse = SuccessResponse;
pub type DeleteCallResponse = SuccessResponse;
pub type GetCallResponse = ResponseResult<CallEntry>;
pub type UpdateCallResponse = ResponseResult<CallEntry>;