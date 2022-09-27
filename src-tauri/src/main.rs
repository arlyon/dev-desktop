#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::ListContainerResponse;
use podman_api::{
    opts::{ContainerListFilter, ContainerListOpts},
    Podman,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn containers_list() -> ListContainerResponse {
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    podman
        .containers()
        .list(
            &ContainerListOpts::builder()
                .all(true)
                .filter([ContainerListFilter::Pod("services".into())])
                .build(),
        )
        .await
        .map(|v| {
            ListContainerResponse::Ok(
                v.iter()
                    .map(|c| {
                        format!(
                            "{}",
                            c.names
                                .as_ref()
                                .and_then(|n| n.iter().next())
                                .unwrap_or(&"unknown".to_string())
                        )
                    })
                    .collect(),
            )
        })
        .unwrap_or_else(|e| ListContainerResponse::Err(format!("{:?}", e)))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![containers_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
