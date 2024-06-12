use crate::app::{App};
use crate::handlers::auth::required_login;
use crate::services::api::FreeboxOSApiCalls;
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
}
