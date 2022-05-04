use containerd_client::connect;
use std::sync::mpsc::{Sender, Receiver};
use config::Config;
use containerd_client::tonic::transport::{Error, channel::Channel as ContainerdChannel};

pub struct Channel {
    config: Config,
    channel: Option<ContainerdChannel>,
}

impl Channel {
    pub async fn new(config: Config) -> Self {
        let channel_result = connect("/run/containerd/containerd.sock").await;
        Channel{
            config: config,
            channel: match channel_result {
                Ok(channel) => Some(channel),
                Err(_) => None
            }
        }   
    }

    pub fn listen(&self) {
        match &self.channel {
            Some(channel) => {
                println!("We got a channel!");
            },
            None => println!("No channel to listen to"),
        }
    }
}