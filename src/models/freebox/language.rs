use crate::app::{ResponseResult, SuccessResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupport {
    pub lang: String,
    pub avalaible: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupportUpdateBody {
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLanguageSupportResponse(ResponseResult<LanguageSupport>);

impl CliDisplay for GetLanguageSupportResponse {
    fn json(&self) -> serde_json::Value {
        json!(self.0.result)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(res) => {
                let mut s = String::new();
                s.push_str(&format!("Current language: {}\n", res.lang));
                s.push_str("Available languages:\n");
                for lang in &res.avalaible {
                    s.push_str(&format!("- {}\n", lang));
                }
                CliResult::success(Box::new(s))
            }
            None => CliResult::error(Box::new("Pas de données disponibles pour le moment")),
        }
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLanguageSupportResponse(SuccessResponse);

impl CliDisplay for UpdateLanguageSupportResponse {
    fn json(&self) -> serde_json::Value {
        json!(self.0)
    }

    fn stdout(&self, _: crate::terminal::CliDisplayArg) -> CliResult {
        CliResult::success(Box::new("Langage mis à jour avec succès"))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}
