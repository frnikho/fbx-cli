use crate::app::{App};
use crate::handlers::auth::required_login;
use crate::models::args::{FtpGetArgs, FtpSetArgs};
use crate::models::freebox::configuration::ftp::{FtpCalls, FtpConfigBody};
use crate::models::freebox::language::LanguageSupportUpdateBody;
use crate::services::api::{FreeboxOSCalls, SystemCalls};
use crate::terminal::{handler_result, HandlerResult};

pub struct System;

impl System {
    pub async fn shutdown(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.shutdown(&app.client, &session)).await
    }

    pub async fn reboot(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.reboot(&app.client, &session)).await
    }

    pub async fn get(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_system_info(&app.client, &session)).await
    }
    
    pub async fn get_update_status(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_update_status(&app.client, &session)).await
    }
    
    pub async fn get_sfp(app: &mut App) -> HandlerResult {
        let _session = required_login(app).await?;
        //handler_result(app.api.get_sfp_info(&app.client, &session)).await
        todo!("Not implemented GET_SFP")
    }
    
    pub async fn update_sfp(app: &mut App) -> HandlerResult {
        let _session = required_login(app).await?;
        //handler_result(app.api.update_sfp_info(&app.client, &session)).await
        todo!("Not implemented UPDATE_SFP")
    }
    
    pub async fn get_ftp_config(app: &mut App, _args: FtpGetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_ftp_config(&app.client, &session)).await
    }
    
    pub async fn update_ftp_config(app: &mut App, args: FtpSetArgs) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_ftp_config(&app.client, &session, FtpConfigBody::from(args))).await
    }
    
    pub async fn get_language(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_language(&app.client, &session)).await
    }
    
    pub async fn update_language(app: &mut App, lang: &String) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.set_language(&app.client, &session, LanguageSupportUpdateBody {lang: lang.clone()})).await
    }
    
}
