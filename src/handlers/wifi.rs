use crate::app::App;
use crate::handlers::auth::required_login;
use crate::terminal::HandlerResult;

pub struct Wifi;

impl Wifi {
    
    pub async fn get(app: &mut App) -> HandlerResult {
        let _session = required_login(app).await?;
        //let wifi = app.api.get_wifi_config(&app.client, &session).await?;
        todo!("get wifi handler");
    }

}