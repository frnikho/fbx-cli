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

#[derive(Error, Clone, Debug, Deserialize)]
pub enum VoicemailError {
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal,
    #[error("No voicemail with this id")]
    #[serde(rename = "invalid_id")]
    InvalidId
}

#[derive(Error, Clone, Debug, Deserialize)]
pub enum ContactError {
    #[error("no entry with this id")]
    #[serde(rename = "noent")]
    NoEntry,
    #[error("an entry already exists")]
    #[serde(rename = "exists")]
    AlreadyExist,
    #[error("no entry matched your request")]
    #[serde(rename = "no_match")]
    NoEntryMatch,
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
pub struct Account {
    pub phone_number: String
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

#[derive(Clone, Debug, Deserialize)]
pub struct ContactEntry {
    pub id: i32,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub photo_url: String,
    pub last_update: i32,
    pub notes: String,
    pub addresses: Vec<ContactAddress>,
    pub emails: Vec<ContactEmail>,
    pub numbers: Vec<ContactNumber>,
    pub urls: Vec<ContactUrl>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactNumber {
    pub id: i32,
    pub contact_id: i32,
    pub r#type: ContactNumberType,
    pub number: String,
    pub is_default: bool,
    pub is_own: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ContactNumberType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "fax")]
    Fax,
    #[serde(rename = "other")]
    Other,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactAddress {
    pub id: i32,
    pub contact_id: i32,
    pub r#type: ContactAddressType,
    pub number: String,
    pub street: String,
    pub street2: String,
    pub city: String,
    pub zipcode: String,
    pub country: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ContactAddressType {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "other")]
    Other
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactEmail {
    pub id: i32,
    pub contact_id: i32,
    pub r#type: ContactEmailType,
    pub email: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ContactEmailType {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "other")]
    Other
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactUrl {
    pub id: i32,
    pub contact_id: i32,
    pub r#type: ContactUrlType,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ContactUrlType {
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "blog")]
    Blog,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "other")]
    Other,
}

pub type ListCallResponse = ResponseResult<Vec<CallEntry>>;
pub type DeleteAllCallResponse = SuccessResponse;
pub type DeleteCallResponse = SuccessResponse;
pub type GetCallResponse = ResponseResult<CallEntry>;
pub type UpdateCallResponse = ResponseResult<CallEntry>;

pub type ListVoicemailResponse = ResponseResult<Vec<VoicemailEntry>>;
pub type GetVoicemailResponse = ResponseResult<VoicemailEntry>;
pub type DeleteVoicemailResponse = SuccessResponse;
pub type UpdateVoicemailResponse = ResponseResult<VoicemailEntry>;
pub type RetrieveVoicemailResponse = ResponseResult<Vec<u8>>;

pub type GetContactResponse = ResponseResult<ContactEntry>;
pub type ListContactResponse = ResponseResult<Vec<ContactEntry>>;
pub type CreateContactResponse = ResponseResult<ContactEntry>;
pub type DeleteContactResponse = SuccessResponse;
pub type UpdateContactResponse = ResponseResult<ContactEntry>;

pub type GetContactNumberResponse = ResponseResult<ContactNumber>;
pub type ListContactNumberResponse = ResponseResult<Vec<ContactNumber>>;
pub type CreateContactNumberResponse = ResponseResult<ContactNumber>;
pub type DeleteContactNumberResponse = SuccessResponse;
pub type UpdateContactNumberResponse = ResponseResult<ContactNumber>;