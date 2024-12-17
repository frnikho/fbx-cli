use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::args::{CreateLeaseArgs, DeleteLeaseArgs, DhcpDnsArgs, DhcpGetArgs, DhcpIpv6Args, DhcpSetArgs, GetLeaseArgs, ListLeaseArgs, UpdateLeaseArgs};
use crate::models::exception::ClientError;
use crate::models::freebox::configuration::dhcp::{DHCPConfigResponse, DHCPDns, DHCPv6ConfigResponse, DHCPv6Dns};
use crate::services::api::DhcpCalls;
use crate::terminal::{handler_ok, handler_result, HandlerResult};

pub struct Dhcp;

impl Dhcp {

    pub async fn get_config(app: &mut App, args: DhcpGetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        match &args.ipv6.unwrap_or_default() {
            true => handler_result(app.api.get_dhcp_v6_config(&app.client, &session)).await,
            false => handler_result(app.api.get_dhcp_config(&app.client, &session)).await,
        }
    }

    pub async fn update_config(app: &mut App, args: DhcpSetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        match args.ipv6.unwrap_or_default() {
            true => handler_result(app.api.update_dhcp_v6_config(&app.client, &session, args.into())).await,
            false => handler_result(app.api.update_dhcp_config(&app.client, &session, args.into())).await
        }
    }

    pub async fn get_lease(app: &mut App, args: GetLeaseArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_dhcp_lease(&app.client, &session, &args.lease)).await
    }

    pub async fn update_lease(app: &mut App, args: UpdateLeaseArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_dhcp_lease(&app.client, &session, &args.clone().lease, args.into())).await
    }

    pub async fn delete_lease(app: &mut App, args: DeleteLeaseArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.delete_dhcp_lease(&app.client, &session, &args.lease)).await
    }

    pub async fn add_lease(app: &mut App, args: CreateLeaseArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.create_dhcp_lease(&app.client, &session, args.into())).await
    }

    pub async fn list_lease(app: &mut App, args: ListLeaseArgs) -> HandlerResult {
        let session = required_login(app).await?;
        match args.dynamic.unwrap_or_default() {
            true => handler_result(app.api.list_dhcp_dynamic_lease(&app.client, &session)).await,
            false => handler_result(app.api.list_dhcp_static_lease(&app.client, &session)).await,
        }
    }

    pub async fn list_dns(app: &mut App, args: DhcpIpv6Args) -> HandlerResult {
        let session = required_login(app).await?;
        match args.ipv6.unwrap_or_default() {
            true => Self::list_dns_v6(app.api.get_dhcp_v6_config(&app.client, &session).await?).await,
            false => Self::list_dns_v4(app.api.get_dhcp_config(&app.client, &session).await?).await,
        }
    }
    
    async fn list_dns_v4(config: DHCPConfigResponse) -> HandlerResult {
        let c = config.result.ok_or(ClientError::RequestError("No DHCP config found"))?;
        handler_ok(DHCPDns(c)).await
    }
    
    async fn list_dns_v6(config: DHCPv6ConfigResponse) -> HandlerResult {
        let c = config.result.ok_or(ClientError::RequestError("No DHCP config found"))?;
        handler_ok(DHCPv6Dns(c)).await
    }

    pub async fn set_dns(_app: &mut App, _arg: DhcpDnsArgs) -> HandlerResult {
        todo!()
    }

    pub async fn add_dns(app: &mut App, args: DhcpDnsArgs) -> HandlerResult {
        let session = required_login(app).await?;
        match args.ipv6.unwrap_or_default() {
            true => Self::add_dns_v6(app, &session, args).await,
            false => Self::add_dns_v4(app, &session, args).await,
        }
    }

    async fn add_dns_v4(app: &mut App, session: &str, _arg: DhcpDnsArgs) -> HandlerResult {
        let config = app.api.get_dhcp_config(&app.client, session).await?.result.ok_or(ClientError::RequestError("No DHCP config found"))?;
        let mut dns = config.dns.clone();
        dns.append(&mut config.dns.clone());
        
        println!("{:?}", config);
        todo!()
    }

    async fn add_dns_v6(app: &mut App, session: &str, _arg: DhcpDnsArgs) -> HandlerResult {
        let config = app.api.get_dhcp_v6_config(&app.client, session).await?.result.ok_or(ClientError::RequestError("No DHCP config found"))?;
        println!("{:?}", config);

        todo!()
    }

    pub async fn remove_dns(_app: &mut App, _arg: DhcpDnsArgs) -> HandlerResult {
        todo!()
    }

    pub async fn clean_dns(_app: &mut App, _args: DhcpIpv6Args) -> HandlerResult {
        todo!()
    }

}