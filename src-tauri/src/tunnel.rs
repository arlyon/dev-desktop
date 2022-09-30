use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_util::sync::CancellationToken;

/// Configuation for an SSH tunnel to a remote host
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Tunnel {
    /// The name of the tunnel
    pub name: String,
    /// The port to open on the local machine
    pub local_port: u32,
    /// The port to tunnel to through the ssh connection
    pub away_port: u32,
    /// The host to tunnel to through the ssh connection
    pub away_host: String,
    /// The ssh target
    pub target: String,
    /// An AWS profile to use
    pub aws_profile: Option<String>,
    /// An AWS region to use
    pub aws_region: Option<String>,
}

impl Tunnel {
    /// opens a tunnel, and awaits until it closes
    pub async fn open(&self, cancel: Option<CancellationToken>) -> () {
        let mut command = tokio::process::Command::new("ssh");
        let mut builder = command
            .arg("-L")
            .arg(format!(
                "{}:{}:{}",
                self.local_port, self.away_host, self.away_port
            ))
            .arg(&self.target)
            .kill_on_drop(true);

        if let Some(region) = &self.aws_region {
            builder = builder.env("AWS_REGION", region);
        };

        if let Some(profile) = &self.aws_profile {
            builder = builder.env("AWS_PROFILE", profile);
        };

        let mut child = builder.spawn().expect("must have libssh");
        let fut = child.wait();

        match cancel {
            Some(cancel) => {
                select! {
                    _ = fut => { cancel.cancel() },
                    _ = cancel.cancelled() => {},
                }
            }
            None => {
                fut.await;
            }
        };

        println!("closed!");
    }
}
