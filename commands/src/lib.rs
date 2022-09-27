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
    pub status: PodmanStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Copy)]
pub enum PodmanStatus {
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
    pub status: PodmanStatus,
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
pub struct ListTunnelResponse(pub (String, TunnelStatus), pub (String, TunnelStatus));

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
