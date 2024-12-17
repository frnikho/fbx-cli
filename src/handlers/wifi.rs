use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::args::{WifiGetArgs, WifiQrCodeArgs};
use crate::models::freebox::wifi::access_point::WifiApCalls;
use crate::models::freebox::wifi::bss::WifiBssCalls;
use crate::terminal::{CliDisplay, CliDisplayArg, handler_result, HandlerResult};

pub struct Wifi;

impl Wifi {
    
    pub async fn get(app: &mut App) -> HandlerResult {
        let _session = required_login(app).await?;
        //let wifi = app.api.get_wifi_config(&app.client, &session).await?;
        todo!("get wifi handler");
    }
    
    pub async fn get_qr_code(app: &mut App, _arg: WifiQrCodeArgs) -> HandlerResult {
        let session = required_login(app).await?;
        let wifi_interfaces = app.api.list_wifi_bss(&app.client, &session).await?;
        let a = wifi_interfaces.0.result.unwrap().get(0).unwrap().config.key.clone();
        println!("{}", qr2term::generate_qr_string(a).unwrap());
        todo!("get qr code handler");
    }
    
    pub async fn list_wifi(app: &mut App, _: WifiGetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        let access_points = app.api.list_wifi_access_points(&app.client, &session).await?;
        println!("{}", access_points.stdout(CliDisplayArg::default()));
        handler_result(app.api.list_wifi_bss(&app.client, &session)).await
    }
    
    pub async fn list_wifi_ap(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.list_wifi_access_points(&app.client, &session)).await
    }

}