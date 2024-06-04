use crate::app::App;
use crate::models::exception::ClientError;

pub struct Device;

impl Device {
    pub async fn list(_app: &mut App) -> Result<(), ClientError> {
        println!("List devices");
        Ok(())
    }
}
