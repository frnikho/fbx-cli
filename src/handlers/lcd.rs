use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::freebox::lcd::LcdUpdateBody;
use crate::services::api::FreeboxOSApiCalls;
use crate::terminal::{handler_result, HandlerResult};

pub struct Lcd;

impl Lcd {

    pub async fn get(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.get_lcd_info(&app.client, &session)).await
    }

    pub async fn set_brightness(app: &mut App, brightness: u8) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_lcd_info(&app.client, &session, LcdUpdateBody::brightness(brightness))).await
    }

    pub async fn set_orientation(app: &mut App, _orientation: String) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_lcd_info(&app.client, &session, LcdUpdateBody::orientation(0))).await //TODO: remove forced value
    }

    pub async fn hide_password(app: &mut App, _hide: String) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_lcd_info(&app.client, &session, LcdUpdateBody::hide_wifi_key(false))).await //TODO: remove forced value
    }

}