/*use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct HomeAdapter {
    pub id: i32,
    pub icon_url: String,
    pub label: String,
    pub status: HomeAdapterStatus,
    //pub r#type: AdapterType,
    pub props: Value,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeAdapterStatus {
    #[serde(rename = "unplugged")]
    Unplugged,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "active")]
    Active
}

#[derive(Clone, Debug, Serialize)]
pub struct HomePairingStep {
    pub fields: Vec<HomePairingStepField>,
    pub icon_url: String,
    pub pageid: i32,
    pub refresh: i32,
    pub session: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomePairingStepField {
    pub width: HomePairingStepFieldText,
    pub text: HomePairingStepFieldText
}

#[derive(Clone, Debug, Serialize)]
pub enum HomePairingStepFieldText {
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "display_qrcode")]
    DisplayQrcode,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "bar_button_left")]
    BarButtonLeft,
    #[serde(rename = "bar_button_right")]
    BarButtonRight,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNode {
    pub adapter: i32,
    pub category: String,
    pub id: i32,
    pub label: String,
    pub name: String,
    pub show_endpoints: Vec<HomeNodeEndpoint>,
    //pub signal_links: Vec<HomeNodeLink>,
    //pub slot_links: Vec<HomeNodeLink>,
    pub status: HomeNodeStatus,
    pub r#type: HomeNodeType,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeNodeStatus {
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "unpaired")]
    Unpaired
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNodeEndpoint {
    pub category: String,
    pub ep_type: HomeNodeEndpointType,
    pub id: i32,
    pub visibility: HomeNodeEndpointVisibility,
    pub access: HomeNodeEndpointAccess,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeNodeEndpointType {
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "slot")]
    Slot,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeNodeEndpointVisibility {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "dashboard")]
    Dashboard
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeNodeEndpointAccess {
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "w")]
    Write,
    #[serde(rename = "rw")]
    ReadWrite,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNodeType {
    pub icon: String,
    pub label: String,
    pub physical: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNodeEndpointUi {
    pub display: HomeNodeEndpointUiDisplay,
    pub icon_url: String,
    pub unit: String,
    pub icon_color: String,
    pub text_color: String,
    pub value_color: String,
    pub range: Vec<f32>,
    pub icon_color_range: Vec<String>,
    pub text_color_range: Vec<String>,
    pub value_color_range: Vec<String>,
    pub status_text_range: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNodeEndpointValue {
    pub value: String,
    pub unit: String,
    pub refresh: i32,
    pub value_type: String,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeNodeEndpointUiDisplay {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "slider")]
    Slider,
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "warning")]
    Warning,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeTile {
    pub node_id: i32,
    pub label: String,
    pub action: HomeTileAction,
    pub r#type: HomeTileType,
    pub group: HomeNodeGroup,
    pub data: Vec<HomeTileData>
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeTileAction {
    #[serde(rename = "tileset")]
    Tileset,
    #[serde(rename = "graph")]
    Graph,
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "store_slider")]
    StoreSlider,
    #[serde(rename = "color_picker")]
    ColorPicker,
    #[serde(rename = "heat_picker")]
    HeatPicker,
    #[serde(rename = "intensity_picker")]
    IntensityPicker,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Serialize)]
pub enum HomeTileType {
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "alarm_sensor")]
    AlarmSensor,
    #[serde(rename = "alarm_control")]
    AlarmControl,
    #[serde(rename = "camera")]
    Camera,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeNodeGroup {
    pub label: String,
    pub icon_url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct HomeTileData {
    pub refresh: i32,
    pub label: String,
    pub ep_id: i32,
    pub value_type: String,
    pub value: String,
    pub ui: HomeNodeEndpointUi
}

#[derive(Clone, Debug, Serialize)]
pub struct Camera {
    pub id: i32,
    pub node_id: i32,
    pub name: String,
    pub stream_url: String,
    pub lan_gid: String
}*/