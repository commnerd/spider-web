use containerd_client::connect;
use config::Config;
use containerd_client::tonic::transport::channel::Channel as ContainerdChannel;

pub struct Server {
    config: Config,
    channel: Option<ContainerdChannel>,
}

impl Server {
    pub async fn new(config: Config) -> Self {
        let channel_result = connect("/run/containerd/containerd.sock").await;
        Server{
            config: config,
            channel: match channel_result {
                Ok(channel) => Some(channel),
                Err(_) => None
            }
        }   
    }
}