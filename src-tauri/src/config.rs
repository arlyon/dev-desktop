use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tunnel::Tunnel;

/// The configuration for the developer desktop tool
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    /// A set of toggleable ssh tunnels
    pub tunnels: Vec<Tunnel>,
    /// A set of services to healthcheck
    pub services: Vec<ServiceSection>,
}

/// A logical group of services to run healthchecks on
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct ServiceSection {
    /// The name of the section in the UI
    pub name: String,
    /// The list of services in that section
    pub services: Vec<Service>,
}

/// A service to run a healthcheck on
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Service {
    /// The name of the service
    pub name: String,
    /// The url to the service
    pub url: Url,
    /// Optional spring healthcheck URL for more detailed information
    pub spring_healthcheck: Option<Url>,
}
