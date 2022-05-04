extern crate tonic;

use config::Config;

pub mod channel;

pub async fn serve(config: Config) {
    let channel = channel::Channel::new(config).await;

    channel.listen();
}