use crate::app::App;
use crate::models::exception::ClientError;

pub struct System;

impl System {
    pub async fn shutdown(_app: &mut App) -> Result<(), ClientError> {
        println!("Shutdown system");
        Ok(())
    }

    pub async fn reboot(_app: &mut App) -> Result<(), ClientError> {
        println!("Reboot system");
        Ok(())
    }

    pub async fn get(_app: &mut App) -> Result<(), ClientError> {
        println!("get system");
        Ok(())
    }
}
