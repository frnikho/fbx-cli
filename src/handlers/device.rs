use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::exception::ClientError;
use crate::models::freebox::lan::WakeOnLanBody;
use crate::services::api::FreeboxOSApiCalls;

pub struct Device;

impl Device {
    pub async fn list(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        let lan_interfaces = app.api.list_lan_interfaces(&app.client, &session).await?;
        println!("{:?}", lan_interfaces);

        for x in lan_interfaces.result.unwrap_or(vec![]) {
            let a = app.api.list_devices(&app.client, &session, x.name).await?;
            println!("{:?}", a);
        }

        println!("List devices");
        Ok(())
    }
    
    pub async fn update(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        todo!()
    }
    
    pub async fn wak_on_lan(app: &mut App) -> Result<(), ClientError> {
        let session = required_login(app).await?;
        //app.api.woa_device(&app.client, session, "", WakeOnLanBody {}):
        todo!() 
    }
    
}
