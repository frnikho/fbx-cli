use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupport {
    pub lang: String,
    pub available: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupportUpdateBody {
    pub lang: String,
}

pub type GetLanguageSupportResponse = ResponseResult<LanguageSupport>;
pub type UpdateLanguageSupportResponse = ResponseResult<Option<()>>;
