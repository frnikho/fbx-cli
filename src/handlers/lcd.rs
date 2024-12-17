use crate::app::App;
use crate::handlers::auth::required_login;
use crate::models::args::{LcdOrientation, YesOrNot};
use crate::models::freebox::configuration::lcd::LcdUpdateBody;
use crate::services::api::LcdCalls;
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

    pub async fn set_orientation(app: &mut App, orientation: LcdOrientation) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_lcd_info(&app.client, &session, LcdUpdateBody::orientation(orientation.value()))).await //TODO: remove forced value
    }

    pub async fn hide_password(app: &mut App, hide: YesOrNot) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.update_lcd_info(&app.client, &session, LcdUpdateBody::hide_wifi_key(hide.value()))).await //TODO: remove forced value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_lcd() {
        let mut app: App = App::default();
        let response = Lcd::get(&mut app).await;
        assert!(response.is_err());
    }
}