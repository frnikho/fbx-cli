use serde::{Deserialize, Serialize};
use crate::client::HttpClient;

#[derive(Debug, Clone, Deserialize)]
pub struct WifiPlanning {
    pub use_planning: bool,
    pub resolution: i32,
    pub mapping: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiPlanningBody {
    pub use_planning: Option<bool>,
    pub mapping: Option<Vec<String>>,
}

pub trait WifiPlanningCalls<T: HttpClient> {
    async fn get_wifi_planning(&self, client: &T, session: &str) -> Result<WifiPlanning, T::Error>;
    async fn update_wifi_planning(&self, client: &T, session: &str, body: WifiPlanningBody) -> Result<WifiPlanning, T::Error>;
}