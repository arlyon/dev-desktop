use podman_api::models::Health;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpringHealthCheck {
    pub status: HealthStatus,
    pub components: SpringComponents,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    UP,
    DOWN,
    OUT_OF_SERVICE,
}

impl From<HealthStatus> for bool {
    fn from(s: HealthStatus) -> Self {
        s == HealthStatus::UP
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpringComponents {
    pub db: ComponentStatus<DB>,
    pub elasticsearch: ComponentStatus<ElasticSearch>,
    #[serde(rename = "buildInfo")]
    pub build_info: ComponentStatus<BuildInfo>,
    pub ping: Ping,
}

#[derive(Serialize, Deserialize)]
pub struct Ping {
    pub status: HealthStatus,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentStatus<T> {
    pub status: HealthStatus,
    pub details: T,
}

#[derive(Serialize, Deserialize)]
pub struct DB {
    pub database: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildInfo {
    pub build: BuildInfoInner,
}

#[derive(Serialize, Deserialize)]
pub struct BuildInfoInner {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ElasticSearch {
    pub cluster_name: Option<String>,
}
