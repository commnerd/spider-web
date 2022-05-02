use containerd_client::connect as containerd_connect;
use std::sync::mpsc::{Sender, Receiver};

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

async fn test_connect() -> bool {
    let channel = containerd_connect("/run/containerd/containerd.sock").await;

    match channel {
        Ok(ch) => {
            println!("YAY")
        },
        Err(ch) => {
            println!("Boo")
        }
    }
    
    // let resp = channel.version(()).await?;

    // println!("Response: {:?}", resp.get_ref());

    return true;
}

pub async fn connect() {
    test_connect();
}