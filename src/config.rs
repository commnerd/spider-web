use config::Config;

pub fn load() -> Config {
    Config::builder()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("SW"))
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("/etc/spiderweb/config").required(false))
        .build()
        .unwrap()
}
