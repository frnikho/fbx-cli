use crate::client::HttpClient;
use crate::models::freebox::authorization::{
    AuthLoginResponse, AuthLogoutResponse, AuthSessionStartRequest, AuthSessionStartResponse,
    AuthTokenRequest, AuthTokenResponse, AuthTrackAuthorizationProgressResponse,
};
use crate::models::freebox::version::VersionResponse;
use std::collections::HashMap;
use crate::app::SuccessResponse;
use crate::handlers::auth::{auth_header, HEADER_FBX_TOKEN};
use crate::models::freebox::configuration::dhcp::{DHCPConfigBody, DHCPConfigResponse, DHCPStaticLeaseCreateBody, DHCPStaticLeaseUpdateBody, DHCPv6ConfigBody, DHCPv6ConfigResponse};
use crate::models::freebox::configuration::ftp::{FtpCalls, FtpConfigBody, FtpConfigResponse, UpdateFtpConfigResponse};
use crate::models::freebox::configuration::lan::{GetLanResponse, ListLanCountResponse, ListLanResponse, UpdateLanHostBody, UpdateLanResponse, WakeOnLanBody, WakeOnLanResponse};
use crate::models::freebox::configuration::lcd::{LcdInfoResponse, LcdUpdateBody};
use crate::models::freebox::configuration::port_forwarding::{GetPortForwardingResponse, ListPortForwardingResponse};
use crate::models::freebox::configuration::system::{GetSystemInfoRequest, GetUpdateStatusResponse, RebootRequest, ShutdownRequest};
use crate::models::freebox::language::{GetLanguageSupportResponse, LanguageSupportUpdateBody, UpdateLanguageSupportResponse};
use crate::models::freebox::wifi::access_point::{GetWifiAllowedCombResponse, GetWifiApResponse, GetWifiStationsResponse, ListWifiApChannelSurveyResponse, ListWifiApResponse, ListWifiStationsResponse, RestartWifiApResponse, WifiApBody, WifiApCalls};
use crate::models::freebox::wifi::bss::{GetWifiBss, ListWifiBss, WifiBssCalls};
use crate::models::freebox::wifi::guest::{GetWifiCustomKey, ListWifiCustomKeys, WifiCustomKeyBody, WifiGuestCalls};
use crate::models::freebox::wifi::mac_filter::{DeleteWifiMacFilter, ListWifiMacFilter, WifiMacFilter, WifiMacFilterBody, WifiMacFilterCalls, WifiMacFilterCreate};
use crate::models::freebox::wifi::planning::{WifiPlanning, WifiPlanningBody, WifiPlanningCalls};
use crate::models::freebox::wifi::wps::{ListWpsSessions, WifiWpsCalls, WifiWpsConfigBody, WifiWpsConfigResponse, WifiWpsStartBody, WifiWpsStartSessionResponse, WifiWpsStopBody, WifiWpsStopSessionResponse};

pub trait FreeboxOSCalls<T: HttpClient> {
    async fn api_version(&self, client: &T) -> Result<VersionResponse, T::Error>;
    async fn get_update_status(&self, client: &T, session_token: &str) -> Result<GetUpdateStatusResponse, T::Error>;
}

pub trait LoginCalls<T: HttpClient> {
    async fn authorize_app(&self, client: &T, body: AuthTokenRequest) -> Result<AuthTokenResponse, T::Error>;
    async fn get_authorization_status(&self, client: &T, track_id: i32) -> Result<AuthTrackAuthorizationProgressResponse, T::Error>;
    async fn start_session(&self, client: &T, session: AuthSessionStartRequest) -> Result<AuthSessionStartResponse, T::Error>;
    async fn login( &self, client: &T, session: Option<&str>) -> Result<AuthLoginResponse, T::Error>;
    async fn logout(&self, client: &T, session: &str, ) -> Result<AuthLogoutResponse, T::Error>;
}

pub trait SystemCalls<T: HttpClient> {
    async fn get_system_info(&self, client: &T, session: &str) -> Result<GetSystemInfoRequest, T::Error>;
    async fn reboot(&self, client: &T, session: &str) -> Result<RebootRequest, T::Error>;
    async fn shutdown(&self, client: &T, session: &str) -> Result<ShutdownRequest, T::Error>;
    async fn get_language(&self, client: &T, session: &str) -> Result<GetLanguageSupportResponse, T::Error>;
    async fn set_language(&self, client: &T, session: &str, body: LanguageSupportUpdateBody) -> Result<UpdateLanguageSupportResponse, T::Error>;
}

pub trait LcdCalls<T: HttpClient> {
    async fn get_lcd_info(&self, client: &T, session: &str) -> Result<LcdInfoResponse, T::Error>;
    async fn update_lcd_info(&self, client: &T, session: &str, body: LcdUpdateBody) -> Result<LcdInfoResponse, T::Error>;
}

pub trait LanCalls<T: HttpClient> {
    async fn list_lan_interfaces(&self, client: &T, session: &str) -> Result<ListLanCountResponse, T::Error>;
    async fn list_devices(&self, client: &T, session: &str, interface: &str) -> Result<ListLanResponse, T::Error>;
    async fn get_device(&self, client: &T, session: &str, interface: &str, device_id: &str) -> Result<GetLanResponse, T::Error>;
    async fn update_device(&self, client: &T, session: &str, interface: &str, device_id: &str, body: UpdateLanHostBody) -> Result<UpdateLanResponse, T::Error>;
    async fn woa_device(&self, client: &T, session: &str, interface: &str, body: WakeOnLanBody) -> Result<WakeOnLanResponse, T::Error>;
}

//TODO: implement that
pub trait StandbyCalls<T: HttpClient> {
    async fn get_standby_status(&self, client: &T, session: &str) -> Result<(), T::Error>;
    async fn get_standby_config(&self, client: &T, session: &str) -> Result<(), T::Error>;
    async fn update_standby_config(&self, client: &T, session: &str,) -> Result<(), T::Error>;
}

pub trait DhcpCalls<T: HttpClient> {
    async fn get_dhcp_config(&self, client: &T, session: &str) -> Result<DHCPConfigResponse, T::Error>;
    async fn update_dhcp_config(&self, client: &T, session: &str, body: DHCPConfigBody) -> Result<DHCPConfigResponse, T::Error>;
    async fn get_dhcp_v6_config(&self, client: &T, session: &str) -> Result<DHCPv6ConfigResponse, T::Error>;
    async fn update_dhcp_v6_config(&self, client: &T, session: &str, body: DHCPv6ConfigBody) -> Result<DHCPv6ConfigResponse, T::Error>;
    async fn list_dhcp_static_lease(&self, client: &T, session: &str) -> Result<(), T::Error>;
    async fn list_dhcp_dynamic_lease(&self, client: &T, session: &str) -> Result<(), T::Error>;
    async fn create_dhcp_lease(&self, client: &T, session: &str, body: DHCPStaticLeaseCreateBody) -> Result<(), T::Error>;
    async fn get_dhcp_lease(&self, client: &T, session: &str, lease: &str) -> Result<(), T::Error>;
    async fn update_dhcp_lease(&self, client: &T, session: &str, lease: &str, args: DHCPStaticLeaseUpdateBody) -> Result<(), T::Error>;
    async fn delete_dhcp_lease(&self, client: &T, session: &str, lease: &str) -> Result<(), T::Error>;
}

pub trait PortForwardingCalls<T: HttpClient> {
    async fn list_port_forwarding(&self, client: &T, session: &str) -> Result<ListPortForwardingResponse, T::Error>;
    async fn get_port_forwarding(&self, client: &T, session: &str, id: &str) -> Result<GetPortForwardingResponse, T::Error>;
    async fn create_port_forwarding(&self, client: &T, session: &str) -> Result<GetPortForwardingResponse, T::Error>;
    async fn update_port_forwarding(&self, client: &T, session: &str, id: &str) -> Result<GetPortForwardingResponse, T::Error>;
    async fn delete_port_forwarding(&self, client: &T, session: &str, id: &str) -> Result<SuccessResponse, T::Error>;
}

#[derive(Debug, Clone, Default)]
pub struct FreeboxOSApi;

impl<T> FreeboxOSCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
    async fn api_version(&self, client: &T) -> Result<VersionResponse, T::Error> {
        client.get("/api_version", None).await
    }
    
    async fn get_update_status(&self, client: &T, session_token: &str) -> Result<GetUpdateStatusResponse, T::Error> {
        client.get("/update/", auth_header(session_token)).await
    }

}

impl<T> LoginCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
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
}

impl<T> SystemCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
    async fn get_system_info(&self, client: &T, session_token: &str) -> Result<GetSystemInfoRequest, T::Error> {
        client.get("/system/", auth_header(session_token)).await
    }

    async fn reboot(&self, client: &T, session_token: &str) -> Result<RebootRequest, T::Error> {
        client.post("/system/reboot/", None::<()>, auth_header(session_token)).await
    }

    async fn shutdown(&self, client: &T, session_token: &str) -> Result<ShutdownRequest, T::Error> {
        client.post("/system/shutdown/", Some(()), auth_header(session_token)).await
    }

    async fn get_language(&self, client: &T, session: &str) -> Result<GetLanguageSupportResponse, T::Error> {
        client.get("/lang/", auth_header(session)).await
    }

    async fn set_language(&self, client: &T, session: &str, body: LanguageSupportUpdateBody) -> Result<UpdateLanguageSupportResponse, T::Error> {
        client.put("/lang/", Some(body), auth_header(session)).await
    }

}

impl <T> LcdCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
    async fn get_lcd_info(&self, client: &T, session_token: &str) -> Result<LcdInfoResponse, T::Error> {
        client.get("/lcd/config/", auth_header(session_token)).await
    }

    async fn update_lcd_info(&self, client: &T, session: &str, body: LcdUpdateBody) -> Result<LcdInfoResponse, T::Error> {
        client.put("/lcd/config/", Some(body), auth_header(session)).await
    }
}

impl <T> LanCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
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
        client.post(format!("/lan/wol/{}/", interface).as_str(), Some(body), auth_header(session_token)).await
    }
}

impl<T> FtpCalls<T> for FreeboxOSApi
where
    T: HttpClient,
{
    async fn get_ftp_config(&self, client: &T, session_token: &str) -> Result<FtpConfigResponse, T::Error> {
        client.get("/ftp/config/", auth_header(session_token)).await
    }

    async fn update_ftp_config(&self, client: &T, session_token: &str, body: FtpConfigBody) -> Result<UpdateFtpConfigResponse, T::Error> {
        client.put("/ftp/config/", Some(body), auth_header(session_token)).await
    }
}

impl <T> DhcpCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn get_dhcp_config(&self, client: &T, session_token: &str) -> Result<DHCPConfigResponse, T::Error> {
        client.get("/dhcp/config/", auth_header(session_token)).await
    }

    async fn update_dhcp_config(&self, client: &T, session_token: &str, body: DHCPConfigBody) -> Result<DHCPConfigResponse, T::Error> {
        client.put("/dhcp/config/", Some(body), auth_header(session_token)).await
    }

    async fn get_dhcp_v6_config(&self, client: &T, session_token: &str) -> Result<DHCPv6ConfigResponse, T::Error> {
        client.get("/dhcpv6/config/", auth_header(session_token)).await
    }

    async fn update_dhcp_v6_config(&self, client: &T, session_token: &str, body: DHCPv6ConfigBody) -> Result<DHCPv6ConfigResponse, T::Error> {
        client.put("/dhcpv6/config/", Some(body), auth_header(session_token)).await
    }

    async fn list_dhcp_static_lease(&self, client: &T, session_token: &str) -> Result<(), T::Error> {
        client.get("/dhcp/static_lease/", auth_header(session_token)).await
    }

    async fn list_dhcp_dynamic_lease(&self, client: &T, session_token: &str) -> Result<(), T::Error> {
        client.get("/dhcp/static_lease/", auth_header(session_token)).await
    }

    async fn create_dhcp_lease(&self, client: &T, session_token: &str,  body: DHCPStaticLeaseCreateBody) -> Result<(), T::Error> {
        client.post("/dhcp/static_lease/", Some(body), auth_header(session_token)).await
    }

    async fn get_dhcp_lease(&self, client: &T, session_token: &str, lease: &str) -> Result<(), T::Error> {
        client.get(format!("/dhcp/static_lease/{}", lease).as_str(), auth_header(session_token)).await
    }

    async fn update_dhcp_lease(&self, client: &T, session_token: &str, lease: &str, args: DHCPStaticLeaseUpdateBody) -> Result<(), T::Error> {
        client.put(format!("/dhcp/static_lease/{}", lease).as_str(), Some(args), auth_header(session_token)).await
    }

    async fn delete_dhcp_lease(&self, client: &T, session_token: &str, lease: &str) -> Result<(), T::Error> {
        client.delete(format!("/dhcp/static_lease/{}", lease).as_str(), auth_header(session_token)).await
    }
}

impl<T> WifiGuestCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn list_wifi_guest_access(&self, client: &T, session_token: &str) -> Result<ListWifiCustomKeys, T::Error> {
        client.get("/wifi/custom_key/", auth_header(session_token)).await
    }

    async fn get_wifi_guest_access(&self, client: &T, session_token: &str, key: &str) -> Result<GetWifiCustomKey, T::Error> {
        client.get(format!("/wifi/custom_key/{}", key).as_str(), auth_header(session_token)).await
    }

    async fn delete_wifi_guest_access(&self, client: &T, session_token: &str, key: &str) -> Result<SuccessResponse, T::Error> {
        client.delete(format!("/wifi/custom_key/{}", key).as_str(), auth_header(session_token)).await
    }

    async fn create_wifi_guest_access(&self, client: &T, session_token: &str, body: WifiCustomKeyBody) -> Result<GetWifiCustomKey, T::Error> {
        client.post("/wifi/custom_key/", Some(body), auth_header(session_token)).await
    }
}

impl<T> WifiWpsCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn get_wifi_wps_config(&self, client: &T, session_token: &str) -> Result<WifiWpsConfigResponse, T::Error> {
        client.get("/wifi/wps/config/", auth_header(session_token)).await
    }

    async fn update_wifi_wps_config(&self, client: &T, session_token: &str, body: WifiWpsConfigBody) -> Result<SuccessResponse, T::Error> {
        client.put("/wifi/wps/config/", Some(body), auth_header(session_token)).await
    }

    async fn start_wifi_wps_session(&self, client: &T, session_token: &str, bssid: &str) -> Result<WifiWpsStartSessionResponse, T::Error> {
        client.post("/wifi/wps/start/", Some(WifiWpsStartBody {bssid: bssid.to_string()}), auth_header(session_token)).await
    }

    async fn cancel_wifi_wps_session(&self, client: &T, session_token: &str, wps: &str) -> Result<WifiWpsStopSessionResponse, T::Error> {
        client.post("/wifi/wps/stop/", Some(WifiWpsStopBody {session_id: wps.to_string()}), auth_header(session_token)).await
    }

    async fn list_wifi_wps_sessions(&self, client: &T, session_token: &str) -> Result<ListWpsSessions, T::Error> {
        client.get("/wifi/wps/sessions/", auth_header(session_token)).await
    }

    async fn clear_wifi_wps_sessions(&self, client: &T, session_token: &str) -> Result<SuccessResponse, T::Error> {
        client.delete("/wifi/wps/sessions/", auth_header(session_token)).await
    }
}

impl<T> WifiMacFilterCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn list_wifi_mac_filter(&self, client: &T, session_token: &str) -> Result<ListWifiMacFilter, T::Error> {
        client.get("/wifi/mac_filter/", auth_header(session_token)).await
    }

    async fn get_wifi_mac_filter(&self, client: &T, session_token: &str, mac: &str) -> Result<WifiMacFilter, T::Error> {
        client.get(format!("/wifi/mac_filter/{}", mac).as_str(), auth_header(session_token)).await
    }

    async fn update_wifi_mac_filter(&self, client: &T, session_token: &str, mac: &str, body: WifiMacFilterBody) -> Result<WifiMacFilter, T::Error> {
        client.put(format!("/wifi/mac_filter/{}", mac).as_str(), Some(body), auth_header(session_token)).await
    }

    async fn delete_wifi_mac_filter(&self, client: &T, session_token: &str, mac: &str) -> Result<DeleteWifiMacFilter, T::Error> {
        client.delete(format!("/wifi/mac_filter/{}", mac).as_str(), auth_header(session_token)).await
    }

    async fn create_wifi_mac_filter(&self, client: &T, session_token: &str, body: WifiMacFilterCreate) -> Result<WifiMacFilter, T::Error> {
        client.post("/wifi/mac_filter/", Some(body), auth_header(session_token)).await
    }
}

impl<T> WifiPlanningCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn get_wifi_planning(&self, client: &T, session_token: &str) -> Result<WifiPlanning, T::Error> {
        client.get("/wifi/planning/", auth_header(session_token)).await
    }

    async fn update_wifi_planning(&self, client: &T, session_token: &str, body: WifiPlanningBody) -> Result<WifiPlanning, T::Error> {
        client.put("/wifi/planning/", Some(body), auth_header(session_token)).await
    }
}

impl<T> WifiBssCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn list_wifi_bss(&self, client: &T, session_token: &str) -> Result<ListWifiBss, T::Error> {
        client.get("/wifi/bss/", auth_header(session_token)).await
    }

    async fn get_wifi_bss(&self, client: &T, session_token: &str, bss: &str) -> Result<GetWifiBss, T::Error> {
        client.get(format!("/wifi/bss/{}", bss).as_str(), auth_header(session_token)).await
    }

    async fn update_wifi_bss(&self, client: &T, session_token: &str, bss: &str, body: WifiApBody) -> Result<GetWifiBss, T::Error> {
        client.put(format!("/wifi/bss/{}", bss).as_str(), Some(body), auth_header(session_token)).await
    }
}

impl<T> WifiApCalls<T> for FreeboxOSApi where T: HttpClient {
    async fn list_wifi_access_points(&self, client: &T, session: &str) -> Result<ListWifiApResponse, T::Error> {
        client.get("/wifi/ap/", auth_header(session)).await
    }

    async fn get_wifi_access_point(&self, client: &T, session: &str, ap_id: &str) -> Result<GetWifiApResponse, T::Error> {
        client.get(format!("/wifi/ap/{}", ap_id).as_str(), auth_header(session)).await
    }

    async fn update_wifi_access_point(&self, client: &T, session: &str, ap_id: &str, body: WifiApBody) -> Result<GetWifiApResponse, T::Error> {
        client.put(format!("/wifi/ap/{}", ap_id).as_str(), Some(body), auth_header(session)).await
    }

    async fn get_wifi_access_point_allowed_channels(&self, client: &T, session: &str, ap_id: &str) -> Result<GetWifiAllowedCombResponse, T::Error> {
        client.get(format!("/wifi/ap/{}/allowed_channel_comb", ap_id).as_str(), auth_header(session)).await
    }

    async fn list_wifi_access_point_stations(&self, client: &T, session: &str, ap_id: &str) -> Result<ListWifiStationsResponse, T::Error> {
        client.get(format!("/wifi/ap/{}/stations", ap_id).as_str(), auth_header(session)).await
    }

    async fn get_wifi_access_point_station(&self, client: &T, session: &str, ap_id: &str, station_mac: &str) -> Result<GetWifiStationsResponse, T::Error> {
        client.get(format!("/wifi/ap/{}/stations/{}", ap_id, station_mac).as_str(), auth_header(session)).await
    }

    async fn get_wifi_access_point_channel_survey_history(&self, client: &T, session: &str, ap_id: &str, timestamp: i32) -> Result<ListWifiApChannelSurveyResponse, T::Error> {
        client.get(format!("/wifi/ap/{}/channel_survey_history/{}", ap_id, timestamp).as_str(), auth_header(session)).await
    }

    async fn restart_wifi_access_point(&self, client: &T, session: &str, ap_id: &str) -> Result<RestartWifiApResponse, T::Error> {
        client.post(format!("/wifi/ap/{}/restart", ap_id).as_str(), Some(()), auth_header(session)).await
    }
}