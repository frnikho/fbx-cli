use crate::client::HttpClient;
use crate::models::freebox::authorization::{
    AuthLoginResponse, AuthLogoutResponse, AuthSessionStartRequest, AuthSessionStartResponse,
    AuthTokenRequest, AuthTokenResponse, AuthTrackAuthorizationProgressResponse,
};
use crate::models::freebox::version::VersionResponse;
use std::collections::HashMap;
use crate::handlers::auth::{auth_header, HEADER_FBX_TOKEN};
use crate::models::freebox::lan::{GetLanResponse, ListLanCountResponse, ListLanResponse, UpdateLanHostBody, UpdateLanResponse, WakeOnLanBody, WakeOnLanResponse};
use crate::models::freebox::lcd::{LcdInfoResponse, LcdUpdateBody};
use crate::models::freebox::system::{GetSystemInfoRequest, GetUpdateStatusResponse, RebootRequest, ShutdownRequest};

pub trait FreeboxOSApiCalls<T: HttpClient> {
    async fn api_version(&self, client: &T) -> Result<VersionResponse, T::Error>;
    async fn authorize_app(&self, client: &T, body: AuthTokenRequest) -> Result<AuthTokenResponse, T::Error>;
    async fn get_authorization_status(&self, client: &T, track_id: i32) -> Result<AuthTrackAuthorizationProgressResponse, T::Error>;
    async fn start_session(&self, client: &T, session: AuthSessionStartRequest) -> Result<AuthSessionStartResponse, T::Error>;
    async fn login( &self, client: &T, session_token: Option<&str>) -> Result<AuthLoginResponse, T::Error>;
    async fn logout(&self, client: &T, session_token: &str, ) -> Result<AuthLogoutResponse, T::Error>;

    async fn get_system_info(&self, client: &T, session_token: &str) -> Result<GetSystemInfoRequest, T::Error>;
    async fn reboot(&self, client: &T, session_token: &str) -> Result<RebootRequest, T::Error>;
    async fn shutdown(&self, client: &T, session_token: &str) -> Result<ShutdownRequest, T::Error>;

    async fn list_lan_interfaces(&self, client: &T, session_token: &str) -> Result<ListLanCountResponse, T::Error>;
    async fn list_devices(&self, client: &T, session_token: &str, interface: &str) -> Result<ListLanResponse, T::Error>;
    async fn get_device(&self, client: &T, session_token: &str, interface: &str, device_id: &str) -> Result<GetLanResponse, T::Error>;
    async fn update_device(&self, client: &T, session_token: &str, interface: &str, device_id: &str, body: UpdateLanHostBody) -> Result<UpdateLanResponse, T::Error>;
    async fn woa_device(&self, client: &T, session_token: &str, interface: &str, body: WakeOnLanBody) -> Result<WakeOnLanResponse, T::Error>;

    async fn get_update_status(&self, client: &T, session_token: &str) -> Result<GetUpdateStatusResponse, T::Error>;
    
    async fn get_lcd_info(&self, client: &T, session_token: &str) -> Result<LcdInfoResponse, T::Error>;
    async fn update_lcd_info(&self, client: &T, session_token: &str, body: LcdUpdateBody) -> Result<LcdInfoResponse, T::Error>;
    
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

    async fn authorize_app(&self, client: &T, body: AuthTokenRequest) -> Result<AuthTokenResponse, T::Error> {
        client.post("/login/authorize", Some(body), None).await
    }

    async fn get_authorization_status(&self, client: &T, track_id: i32) -> Result<AuthTrackAuthorizationProgressResponse, T::Error> {
        client.get(format!("/login/authorize/{}", track_id).as_str(), None).await
    }

    async fn start_session(&self, client: &T, session: AuthSessionStartRequest) -> Result<AuthSessionStartResponse, T::Error> {
        client.post("/login/session", Some(session), None).await
    }

    async fn login(&self, client: &T, session_token: Option<&str>) -> Result<AuthLoginResponse, T::Error> {
        let headers = session_token.map_or_else(HashMap::new, |s| {
            HashMap::from([(String::from(HEADER_FBX_TOKEN), s)])
        });
        client.get("/login", Some(headers)).await
    }

    async fn logout(&self, client: &T, session_token: &str) -> Result<AuthLogoutResponse, T::Error> {
        client.post("/login/logout/", Some(()), auth_header(session_token)).await
    }

    async fn get_system_info(&self, client: &T, session_token: &str) -> Result<GetSystemInfoRequest, T::Error> {
        client.get("/system/", auth_header(session_token)).await
    }

    async fn reboot(&self, client: &T, session_token: &str) -> Result<RebootRequest, T::Error> {
        client.post("/system/reboot/", None::<()>, auth_header(session_token)).await
    }

    async fn shutdown(&self, client: &T, session_token: &str) -> Result<ShutdownRequest, T::Error> {
        client.post("/system/shutdown/", Some(()), auth_header(session_token)).await
    }

    async fn list_lan_interfaces(&self, client: &T, session_token: &str) -> Result<ListLanCountResponse, T::Error> {
        client.get("/lan/browser/interfaces/", auth_header(session_token)).await
    }

    async fn list_devices(&self, client: &T, session_token: &str, interface: &str) -> Result<ListLanResponse, T::Error> {
        client.get(format!("/lan/browser/{}/", interface).as_str(), auth_header(session_token)).await
    }

    async fn get_device(&self, client: &T, session_token: &str, interface: &str, device_id: &str) -> Result<GetLanResponse, T::Error> {
        client.get(format!("/lan/browser/{}/{}", interface, device_id).as_str(), auth_header(session_token)).await
    }

    async fn update_device(&self, client: &T, session_token: &str, interface: &str, device_id: &str, body: UpdateLanHostBody) -> Result<UpdateLanResponse, T::Error> {
        client.put(format!("/lan/browser/{}/{}", interface, device_id).as_str(), Some(body), auth_header(session_token)).await
    }

    async fn woa_device(&self, client: &T, session_token: &str, interface: &str, body: WakeOnLanBody) -> Result<WakeOnLanResponse, T::Error> {
        client.post(format!("/wol/{}/", interface).as_str(), Some(body), auth_header(session_token)).await
    }

    async fn get_update_status(&self, client: &T, session_token: &str) -> Result<GetUpdateStatusResponse, T::Error> {
        client.get("/update/", auth_header(session_token)).await
    }

    async fn get_lcd_info(&self, client: &T, session_token: &str) -> Result<LcdInfoResponse, T::Error> {
        client.get("/lcd/config/", auth_header(session_token)).await
    }

    async fn update_lcd_info(&self, client: &T, session_token: &str, body: LcdUpdateBody) -> Result<LcdInfoResponse, T::Error> {
        client.put("/lcd/config/", Some(body), auth_header(session_token)).await
    }

}
