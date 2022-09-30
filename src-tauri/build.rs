use std::fs::File;

// One **exactly one** of this...
#[path = "src/tunnel.rs"]
mod tunnel;

// One **exactly one** of this...
#[path = "src/config.rs"]
mod config;

use schemars::schema_for;

fn main() {
    let file = File::create("./schema.json").unwrap();

    let schema = schema_for!(config::Config);
    serde_json::to_writer_pretty(file, &schema).unwrap();

    tauri_build::build()
}
