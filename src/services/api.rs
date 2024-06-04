use crate::client::HttpClient;
use crate::models::freebox::authorization::{
    AuthLoginResponse, AuthLogoutResponse, AuthSessionStartRequest, AuthSessionStartResponse,
    AuthTokenRequest, AuthTokenResponse, AuthTrackAuthorizationProgressResponse,
};
use crate::models::freebox::version::VersionResponse;
use std::collections::HashMap;

const HEADER_FBX_TOKEN: &str = "X-Fbx-App-Auth";

pub trait FreeboxOSApiCalls<T: HttpClient> {
    async fn api_version(&self, client: &T) -> Result<VersionResponse, T::Error>;
    async fn authorize_app(
        &self,
        client: &T,
        body: AuthTokenRequest,
    ) -> Result<AuthTokenResponse, T::Error>;
    async fn get_authorization_status(
        &self,
        client: &T,
        track_id: i32,
    ) -> Result<AuthTrackAuthorizationProgressResponse, T::Error>;
    async fn start_session(
        &self,
        client: &T,
        session: AuthSessionStartRequest,
    ) -> Result<AuthSessionStartResponse, T::Error>;
    async fn login(
        &self,
        client: &T,
        session_token: Option<String>,
    ) -> Result<AuthLoginResponse, T::Error>;
    async fn logout(
        &self,
        client: &T,
        session_token: String,
    ) -> Result<AuthLogoutResponse, T::Error>;
}

#[derive(Debug, Clone, Default)]
pub struct FreeboxOSApi;

impl<T> FreeboxOSApiCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
    async fn api_version(&self, client: &T) -> Result<VersionResponse, T::Error> {
        client.get("/api_version", None).await
    }

    async fn authorize_app(
        &self,
        client: &T,
        body: AuthTokenRequest,
    ) -> Result<AuthTokenResponse, T::Error> {
        client.post("/login/authorize", Some(body), None).await
    }

    async fn get_authorization_status(
        &self,
        client: &T,
        track_id: i32,
    ) -> Result<AuthTrackAuthorizationProgressResponse, T::Error> {
        client
            .get(format!("/login/authorize/{}", track_id).as_str(), None)
            .await
    }

    async fn start_session(
        &self,
        client: &T,
        session: AuthSessionStartRequest,
    ) -> Result<AuthSessionStartResponse, T::Error> {
        client.post("/login/session", Some(session), None).await
    }

    async fn login(
        &self,
        client: &T,
        session_token: Option<String>,
    ) -> Result<AuthLoginResponse, T::Error> {
        let headers = session_token.map_or_else(HashMap::new, |s| {
            HashMap::from([(String::from(HEADER_FBX_TOKEN), s)])
        });
        client.get("/login", Some(headers)).await
    }

    async fn logout(
        &self,
        client: &T,
        session_token: String,
    ) -> Result<AuthLogoutResponse, T::Error> {
        client
            .post(
                "/login/logout/",
                Some(()),
                Some(HashMap::from([(
                    String::from(HEADER_FBX_TOKEN),
                    session_token,
                )])),
            )
            .await
    }
}
