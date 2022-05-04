use containerd_client::connect as containerd_connect;
use std::sync::mpsc::{Sender, Receiver};
use config::Config;

/*
pub enum Status {
    Unknown,
    Created,
    Running,
    Stopped,
    Paused,
    Pausing,
}
*/

async fn is_connected() -> bool {
    let channel = containerd_connect("/run/containerd/containerd.sock").await;

    return match channel {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    };
    
    // let resp = channel.version(()).await?;

    // println!("Response: {:?}", resp.get_ref());

    return true;
}

pub async fn connect(config: Config) {
    if is_connected().await {
        println!("YAY");
        return;
    }
    println!("BOO!");
}