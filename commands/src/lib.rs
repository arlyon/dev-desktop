use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[async_trait::async_trait(?Send)]
pub trait Command: Serialize {
    type OutputType: for<'a> Deserialize<'a>;

    fn name() -> &'static str;

    async fn invoke(&self) -> Self::OutputType {
        from_value(invoke(Self::name(), to_value(self).unwrap()).await).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListContainers {}

#[derive(Deserialize, Serialize, Debug)]
pub enum ListContainerResponse {
    Ok(Vec<ListContainerItem>),
    Err(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListContainerItem {
    pub id: Option<String>,
    pub name: String,
    pub state: PodmanState,
    pub started_at: Option<i64>,
    pub exited_at: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Copy)]
pub enum PodmanState {
    Running,
    Stopping,
    Stopped,
    Exited,
}

#[async_trait::async_trait(?Send)]
impl Command for ListContainers {
    type OutputType = ListContainerResponse;

    fn name() -> &'static str {
        "containers_list"
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetContainerStatus {
    pub id: String,
    pub state: PodmanState,
}

#[async_trait::async_trait(?Send)]
impl Command for SetContainerStatus {
    type OutputType = ();

    fn name() -> &'static str {
        "set_container"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListTunnels {}

#[derive(Serialize, PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum TunnelStatus {
    Connected(u32),
    Disconnected,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TunnelState {
    On,
    Off,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListTunnelResponse(pub Vec<(String, TunnelStatus)>);

#[async_trait::async_trait(?Send)]
impl Command for ListTunnels {
    type OutputType = ListTunnelResponse;

    fn name() -> &'static str {
        "tunnels_list"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ToggleTunnels {
    pub id: String,
    pub state: TunnelState,
}

#[async_trait::async_trait(?Send)]
impl Command for ToggleTunnels {
    type OutputType = ();

    fn name() -> &'static str {
        "tunnels_toggle"
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetHealthCheck {}

#[async_trait::async_trait(?Send)]
impl Command for GetHealthCheck {
    type OutputType = Vec<HealthcheckSection>;

    fn name() -> &'static str {
        "get_healthcheck"
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HealthcheckSection {
    pub name: String,
    pub services: Vec<ServiceHealthCheck>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceHealthCheck {
    pub name: String,
    pub url: String,
    pub up: bool,
    pub db: Option<bool>,
    pub elasticsearch: Option<bool>,
}
