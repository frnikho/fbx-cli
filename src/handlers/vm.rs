use crate::app::App;
use crate::models::args::{VmGetArgs, VmListArgs};
use crate::models::exception::ClientError;
use crate::terminal::HandlerResult;

pub struct Vm;

impl Vm {
    pub async fn list(_app: &mut App, _arg: &VmListArgs) -> HandlerResult {
        Err(ClientError::UnknownError("Not implemented"))
    }

    pub async fn get(_app: &mut App, _arg: &VmGetArgs) -> HandlerResult {
        Err(ClientError::UnknownError("Not implemented"))
    }
}
