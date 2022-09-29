use commands::TunnelState;
use serde::{Deserialize, Serialize};

use crate::tunnel::Tunnel;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tunnels: Vec<Tunnel>,
}
