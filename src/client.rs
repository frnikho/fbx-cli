use crate::models::exception::ApiError;
use crate::models::freebox::version::FreeboxMajorVersion;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

/// HttpClient trait is used to represent the http client
pub trait HttpClient {
    type Error;
    async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<impl Serialize>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error>;
    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error>;
    async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<impl Serialize>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error>;
    async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
    base_url: String,
    timeout: u64,
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: "https://mafreebox.freebo.fr/v1".to_string(),
            timeout: 5,
        }
    }
}

impl ReqwestClient {
    pub fn new(base_url: String) -> Self {
        ReqwestClient {
            client: reqwest::Client::new(),
            base_url,
            timeout: 5,
        }
    }

    pub fn set_url(&mut self, base_url: String) {
        self.base_url = format!("{}api/{}", base_url, FreeboxMajorVersion::default());
    }

    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    pub fn set_full_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    pub fn fmt_url(&self, url: &str) -> String {
        format!("{}{}", self.base_url, url)
    }

    pub async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ApiError> {
        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await?),
            201 => Ok(response.json::<T>().await?),
            204 => Ok(response.json::<T>().await?),
            403 => Err(ApiError::Unauthorized("".to_string())),
            404 => Err(ApiError::NotFound),
            500 => Err(ApiError::Internal("".to_string())),
            _ => Err(ApiError::Internal("".to_string())),
        }
    }
}

impl HttpClient for ReqwestClient {
    type Error = ApiError;

    async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<impl Serialize>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error> {
        let mut builder = self.client.post(self.fmt_url(url)).json(&body);
        if let Some(headers) = headers {
            builder = headers.iter().fold(builder, |acc, (k, v)| acc.header(k, v));
        }
        Ok(builder.send().await?.json::<T>().await?)
    }

    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error> {
        let mut builder = self
            .client
            .get(self.fmt_url(url))
            .timeout(std::time::Duration::from_secs(self.timeout));
        if let Some(headers) = headers {
            builder = headers.iter().fold(builder, |acc, (k, v)| acc.header(k, v));
        }
        Ok(builder.send().await?.json::<T>().await?)
    }

    async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<impl Serialize>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error> {
        let mut builder = self
            .client
            .put(self.fmt_url(url))
            .json(&body)
            .timeout(std::time::Duration::from_secs(self.timeout));
        if let Some(headers) = headers {
            builder = headers.iter().fold(builder, |acc, (k, v)| acc.header(k, v));
        }
        Ok(builder.send().await?.json::<T>().await?)
    }

    async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, Self::Error> {
        let mut builder = self
            .client
            .delete(self.fmt_url(url))
            .timeout(std::time::Duration::from_secs(self.timeout));
        if let Some(headers) = headers {
            builder = headers.iter().fold(builder, |acc, (k, v)| acc.header(k, v));
        }
        Ok(builder.send().await?.json::<T>().await?)
    }
}
