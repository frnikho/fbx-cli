use crate::app::App;
use crate::models::args::{VmGetArgs, VmListArgs};
use crate::models::exception::ClientError;

pub struct Vm;

impl Vm {
    pub async fn list(_app: &mut App, _arg: &VmListArgs) -> Result<(), ClientError> {
        Ok(())
    }

    pub async fn get(_app: &mut App, _arg: &VmGetArgs) -> Result<(), ClientError> {
        Ok(())
    }
}
