use std::collections::HashMap;
use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::args::{DeviceGetArgs, DeviceUpdateArgs, WakeUpArgs};
use crate::models::exception::ClientError;
use crate::models::freebox::configuration::lan::{LanHost, UpdateLanHostBody, WakeOnLanBody};
use crate::services::api::LanCalls;
use crate::terminal::{handler_ok, HandlerResult};

pub struct Device;

impl Device {
    pub async fn list(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        println!("session: {}", session);
        let lan_interfaces = app.api.list_lan_interfaces(&app.client, &session).await?;
        for x in lan_interfaces.result.unwrap_or(vec![]) {
            println!("Interface: {}", x.name);
            let a = app.api.list_devices(&app.client, &session, x.name.as_str()).await?;
            a.display();
        }
        Err(ClientError::UnknownError("Not implemented"))
    }

    pub async fn get_device(app: &mut App, args: &DeviceGetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        match find_device(app, &session, &args.id).await {
            Some((.., host)) => handler_ok(host).await,
            _ => Err(ClientError::UnknownError("Device not found"))
        }
    }

    pub async fn update(app: &mut App, args: DeviceUpdateArgs) -> HandlerResult {
        let session = required_login(app).await?;
        let (interface, device) = find_device(app, &session, &args.name).await.ok_or(ClientError::UnknownError("Device not found"))?;
        handler_ok(app.api.update_device(&app.client, &session, interface.as_str(), device.id.as_str(), UpdateLanHostBody {primary_name: args.new_name, host_type: args.r#type, persistent: args.persistent}).await?).await
    }

    pub async fn wak_on_lan(app: &mut App, args: WakeUpArgs) -> HandlerResult {
        let session = required_login(app).await?;
        let (interface, host) = find_device(app, &session, &args.name).await.ok_or(ClientError::UnknownError("Device not found"))?;
        let response = app.api.woa_device(&app.client, &session, &interface, WakeOnLanBody::new(host.l2ident.id, args.password)).await?;
        println!("{:?}", response);
        todo!()
    }
}

pub async fn list_all_device(app: &mut App, session: &str) -> Result<HashMap<String, LanHost>, ClientError> {
    let mut devices = HashMap::new();
    let lan_interfaces = app.api.list_lan_interfaces(&app.client, session).await?;
    for x in lan_interfaces.result.unwrap_or_default() {
        let a = app.api.list_devices(&app.client, session, x.name.as_str()).await?;
        for device in a.result.unwrap_or_default() {
            devices.insert(device.id.clone(), device);
        }
    }
    Ok(devices)
}

pub async fn find_device(app: &mut App, session: &str, device_id: &str) -> Option<(String, LanHost)> {
    let lan_interfaces = app.api.list_lan_interfaces(&app.client, session).await.ok()?;
    for interface in lan_interfaces.result? {
        let devices = app.api.list_devices(&app.client, session, interface.name.clone().as_str()).await.ok()?.result?;
        if let Some(device) = devices.into_iter().find(|device| device.primary_name.eq_ignore_ascii_case(device_id) || device.id.eq(device_id)) {
            return Some((interface.name, device));
        }
    }
    None
}