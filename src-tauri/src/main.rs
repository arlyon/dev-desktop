#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(all(feature = "cocoa", target_os = "macos"))]
mod macos;
mod tunnel;

use commands::{
    ListContainerItem, ListContainerResponse, ListTunnelResponse, PodmanState, TunnelState,
    TunnelStatus,
};
use podman_api::{
    opts::{ContainerListFilter, ContainerListOpts},
    Podman,
};
use tauri::{
    async_runtime::{JoinHandle, Mutex},
    Manager,
};
use tokio_util::sync::CancellationToken;
use tunnel::Tunnel;

struct SSHTunnelConnection {
    task: Option<(CancellationToken, JoinHandle<()>)>,
    tunnel: tunnel::Tunnel,
}

impl SSHTunnelConnection {
    fn connected(&mut self) -> bool {
        if self
            .task
            .as_ref()
            .map(|(t, _)| t.is_cancelled())
            .unwrap_or(false)
        {
            self.task.take();
        }
        self.task.is_some()
    }

    fn connect(&mut self) {
        let token = CancellationToken::new();
        if !self.connected() {
            // this is ok0
            let tunnel = self.tunnel.clone();
            let inner_token = token.clone();
            let handle =
                tauri::async_runtime::spawn(async move { tunnel.open(Some(inner_token)).await });
            self.task.replace((token, handle));
        }
    }

    fn disconnect(&mut self) {
        if let Some((token, handle)) = self.task.take() {
            token.cancel();
        }
    }

    fn cancel(&mut self) -> bool {
        if let Some((token, _)) = &self.task {
            token.cancel();
        }

        self.connected()
    }
}

struct SSHTunnels {
    production: SSHTunnelConnection,
    staging: SSHTunnelConnection,
}

struct SSHTunnelState(Mutex<SSHTunnels>);

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
            ListContainerResponse::Ok({
                let mut data = v
                    .into_iter()
                    .filter(|c| !c.is_infra.unwrap_or(false))
                    .map(|c| ListContainerItem {
                        id: {
                            // println!("{:?}", c);
                            c.id
                        },
                        name: c
                            .names
                            .as_ref()
                            .and_then(|n| n.iter().next())
                            .unwrap_or(&"unknown".to_string())
                            .to_string(),
                        state: c
                            .state
                            .and_then(|s| match s.as_str() {
                                "running" => Some(PodmanState::Running),
                                "exited" => Some(PodmanState::Exited),
                                "stopping" => Some(PodmanState::Stopping),
                                x => {
                                    println!("{:?}", x);
                                    None
                                }
                            })
                            .unwrap_or(PodmanState::Stopped),
                        started_at: c.started_at,
                        exited_at: c.exited_at,
                    })
                    .collect::<Vec<_>>();
                data.sort_by_key(|f| f.state);
                data
            })
        })
        .unwrap_or_else(|e| ListContainerResponse::Err(format!("{:?}", e)))
}

#[tauri::command]
async fn set_container(id: String, state: PodmanState) -> () {
    println!("setting container {} to {:?}", id, state);
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    let container = podman.containers().get(id);
    match state {
        PodmanState::Running => container.start(None).await,
        _ => container.stop(&Default::default()).await,
    };
}

/// note: we need to return a result here, Err corresponds to an exception so do not use it
#[tauri::command]
async fn tunnels_list(state: tauri::State<'_, SSHTunnelState>) -> Result<ListTunnelResponse, ()> {
    let mut tunnels = state.0.lock().await;
    Ok(ListTunnelResponse(
        (
            "Production".to_string(),
            if tunnels.production.connected() {
                TunnelStatus::Connected(tunnels.production.tunnel.local_port)
            } else {
                TunnelStatus::Disconnected
            },
        ),
        (
            "Staging".to_string(),
            if tunnels.staging.connected() {
                TunnelStatus::Connected(tunnels.staging.tunnel.local_port)
            } else {
                TunnelStatus::Disconnected
            },
        ),
    ))
}

/// note: we need to return a result here, Err corresponds to an exception so do not use it
#[tauri::command]
async fn tunnels_toggle(
    id: String,
    state: TunnelState,
    states: tauri::State<'_, SSHTunnelState>,
) -> Result<(), ()> {
    println!("setting tunnel {} to {:?}", id, state);
    let mut tunnels = states.0.lock().await;

    let tunnel = match id.as_str() {
        "Staging" => &mut tunnels.staging,
        "Production" => &mut tunnels.production,
        _ => return Ok(()),
    };

    match state {
        TunnelState::On => tunnel.connect(),
        TunnelState::Off => tunnel.disconnect(),
    };

    Ok(())
}

#[tauri::command]
fn show(window: tauri::Window) {
    println!("show!");
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();

            #[cfg(all(feature = "cocoa", target_os = "macos"))]
            {
                use macos::WindowExt;
                win.set_transparent_titlebar(true);
            }

            Ok(())
        })
        .manage(SSHTunnelState(Mutex::new(SSHTunnels {
            production: SSHTunnelConnection {
                task: None,
                tunnel: Tunnel {
                    local_port: 33006,
                    away_port: 3306,
                    away_host:
                        "mdisrupt-production.cluster-ro-cv1p1ugoa6na.us-east-2.rds.amazonaws.com"
                            .to_string(),
                    target: "ssm-user@i-0462fc9f5f57202e9".to_string(),
                    aws_profile: Some("mdisrupt".to_string()),
                    aws_region: Some("us-east-2".to_string()),
                },
            },
            staging: SSHTunnelConnection {
                task: None,
                tunnel: Tunnel {
                    local_port: 33007,
                    away_port: 3306,
                    away_host: "mdisrupt-dev.cluster-c0s6m6qaswhy.us-west-2.rds.amazonaws.com"
                        .to_string(),
                    target: "ssm-user@i-0dc81149070588e87".to_string(),
                    aws_profile: Some("mdisrupt".to_string()),
                    aws_region: Some("us-west-2".to_string()),
                },
            },
        })))
        .invoke_handler(tauri::generate_handler![
            containers_list,
            set_container,
            tunnels_list,
            tunnels_toggle,
            show
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
