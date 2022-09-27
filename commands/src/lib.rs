use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListContainers {}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum ListContainerResponse {
    Ok(Vec<String>),
    Err(String),
}

#[async_trait::async_trait(?Send)]
pub trait Command {
    type OutputType;

    fn name() -> &'static str;
    async fn invoke(self) -> Self::OutputType;
}

#[async_trait::async_trait(?Send)]
impl Command for ListContainers {
    type OutputType = ListContainerResponse;

    fn name() -> &'static str {
        "containers_list"
    }

    async fn invoke(self) -> Self::OutputType {
        // let value = .unwrap();
        from_value(invoke(Self::name(), to_value(&ListContainers {}).unwrap()).await).unwrap()
    }
}
