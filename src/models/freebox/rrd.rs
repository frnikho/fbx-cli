use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RRDFetch {
    pub db: RDDDatabase,
    pub date_start: i32,
    pub date_end: i32,
    pub precision: i32,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RDDDatabase {
    Network,
    Temperature,
    Xdsl,
    Switch,
}
