pub mod server;

use config::Config;
use self::server::Server;

pub async fn serve(config: Config) -> Server {
    server::Server::new(config).await
}