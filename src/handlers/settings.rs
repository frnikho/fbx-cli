use crate::app::App;
use crate::models::exception::ClientError;

pub struct Settings;

impl Settings {
    pub async fn get_url(_app: &mut App) -> Result<(), ClientError> {
        println!("Get settings");
        Ok(())
    }

    pub async fn set_url(_app: &mut App) -> Result<(), ClientError> {
        println!("Set settings");
        Ok(())
    }

    pub async fn reset(_app: &mut App) -> Result<(), ClientError> {
        println!("Reset settings");
        Ok(())
    }

    pub async fn discover(_app: &mut App) -> Result<(), ClientError> {
        println!("Discover settings");
        Ok(())
    }
}
