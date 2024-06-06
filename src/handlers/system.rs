use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::exception::ClientError;
use crate::services::api::FreeboxOSApiCalls;

pub struct System;

impl System {
    pub async fn shutdown(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        let r = app.api.shutdown(&app.client, &session).await?;
        println!("{:?}", r);
        Ok(())
    }

    pub async fn reboot(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        let r = app.api.reboot(&app.client, &session).await?;
        println!("{:?}", r);
        Ok(())
    }

    pub async fn get(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        let r = app.api.get_system_info(&app.client, &session).await?;
        println!("{:?}", r);
        Ok(())
    }
    
    pub async fn get_update_status(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        let update = app.api.get_update_status(&app.client, &session).await?;
        println!("{:?}", update);
        Ok(())
    }
}
