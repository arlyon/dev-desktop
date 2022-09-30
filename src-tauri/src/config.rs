use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tunnel::Tunnel;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub tunnels: Vec<Tunnel>,
    pub services: Vec<ServiceSection>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct ServiceSection {
    pub name: String,
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Service {
    pub name: String,
    pub url: Url,
    /// Allows you to set a healthcheck
    pub spring_healthcheck: Option<Url>,
}
