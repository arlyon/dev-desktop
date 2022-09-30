#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(box_patterns)]

mod config;
#[cfg(all(feature = "cocoa", target_os = "macos"))]
mod macos;
mod tunnel;

use std::{fs::File, path::PathBuf};

use commands::{
    HealthcheckSection, ListContainerItem, ListContainerResponse, ListTunnelResponse, PodmanState,
    ServiceHealthCheck, TunnelState, TunnelStatus,
};
use config::ServiceSection;
use directories::ProjectDirs;
use podman_api::{
    opts::{ContainerListFilter, ContainerListOpts},
    Podman,
};
use serde::{Deserialize, Serialize};
use tauri::{
    api::cli::{Matches, SubcommandMatches},
    async_runtime::{block_on, JoinHandle, Mutex},
    Manager,
};
use tokio_util::sync::CancellationToken;
use tunnel::Tunnel;

use crate::config::Config;

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

impl From<Tunnel> for SSHTunnelConnection {
    fn from(tunnel: Tunnel) -> Self {
        Self { task: None, tunnel }
    }
}

struct SSHTunnels(Vec<SSHTunnelConnection>);
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
    let vals = tunnels
        .0
        .iter_mut()
        .map(|t| {
            (
                t.tunnel.name.clone(),
                if t.connected() {
                    TunnelStatus::Connected(t.tunnel.local_port)
                } else {
                    TunnelStatus::Disconnected
                },
            )
        })
        .collect();
    Ok(ListTunnelResponse(vals))
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

    let tunnel = match tunnels.0.iter_mut().find(|t| t.tunnel.name == id.as_str()) {
        Some(t) => t,
        None => return Ok(()),
    };

    match state {
        TunnelState::On => tunnel.connect(),
        TunnelState::Off => tunnel.disconnect(),
    };

    Ok(())
}

#[tauri::command]
fn show(window: tauri::Window) {
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn get_healthcheck(
    state: tauri::State<'_, ServiceHealthCheckState>,
) -> Result<Vec<HealthcheckSection>, ()> {
    Ok(state
        .0
        .lock()
        .await
        .iter()
        .cloned()
        .map(|c| HealthcheckSection {
            name: c.name,
            services: c
                .services
                .into_iter()
                .map(|s| ServiceHealthCheck {
                    up: if s.name.contains("Storybook") {
                        false
                    } else {
                        true
                    },
                    url: s.url.to_string(),
                    db: if s.name == "MDisrupt API" {
                        Some(true)
                    } else {
                        None
                    },
                    elasticsearch: if s.name == "MDisrupt API" {
                        Some(false)
                    } else {
                        None
                    },
                    name: s.name,
                })
                .collect(),
        })
        .collect())
}

struct ServiceHealthCheckState(Mutex<Vec<ServiceSection>>);

fn main() {
    let config_dir = ProjectDirs::from("dev", "arlyon", "developer-dashboard")
        .map(|d| d.config_dir().to_owned());
    let config_file = config_dir.as_ref().map(|p| p.join("config.json"));

    let config: Option<Config> = config_file
        .as_ref()
        .and_then(|f| File::open(f).ok())
        .and_then(|f| match serde_json::from_reader(f) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("unable to load config, ignoring: {}", e);
                None
            }
        });

    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(Matches {
                    subcommand: Some(box SubcommandMatches { name, matches, .. }),
                    ..
                }) => match name.as_str() {
                    "fetch" => {
                        let source_arg = matches.args.get("source").expect("validated by tauri");
                        let source = source_arg.value.as_str().expect("validated by tauri");

                        let res = block_on(fetch_config(
                            source,
                            &app.state::<SSHTunnelState>(),
                            config_file,
                        ));
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
                _ => {}
            };

            let win = app.get_window("main").unwrap();

            #[cfg(all(feature = "cocoa", target_os = "macos"))]
            {
                use macos::WindowExt;
                win.set_transparent_titlebar(true);
            }

            Ok(())
        })
        .manage(SSHTunnelState(Mutex::new(SSHTunnels(
            config
                .as_ref()
                .map(|c| c.tunnels.iter().cloned().map(Into::into).collect())
                .unwrap_or_default(),
        ))))
        .manage(ServiceHealthCheckState(Mutex::new(
            config.map(|c| c.services).unwrap_or_default(),
        )))
        .invoke_handler(tauri::generate_handler![
            containers_list,
            set_container,
            tunnels_list,
            tunnels_toggle,
            get_healthcheck,
            show
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn fetch_config(
    source: &str,
    config: &SSHTunnelState,
    config_file: Option<PathBuf>,
) -> Result<(), String> {
    println!("fetching config from {}", source);
    let body = reqwest::blocking::get(source).unwrap().text().unwrap();

    let config_new: Config =
        serde_json::from_str(&body).map_err(|e| format!("invalid config: {}", e))?;

    let config_file = config_file
        .and_then(|p| {
            println!("saving config to {:?}", p);
            std::fs::create_dir_all(&p.parent().expect("this is ok"));
            File::create(p).ok()
        })
        .ok_or_else(|| format!("could not locate config path"))?;

    serde_json::to_writer(config_file, &config_new);

    let mut lock = config.0.lock().await;
    lock.0
        .extend(config_new.tunnels.into_iter().map(Into::into));

    Ok(())
}
