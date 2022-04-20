use config::Config as ImpConfig;
use std::collections::HashMap;

pub struct Config {
    config: Box<ImpConfig>
}

impl Default for Config {
    fn default() -> Self {
        Config{
            config: Box::new(ImpConfig::default()),
        }
    }
}

impl Config {
    pub fn init() -> Config {
        let imp_config = ImpConfig::builder()
            // Add in `./Settings.toml`
            .add_source(config::File::with_name("examples/simple/Settings"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();
    
        Config{
            config: Box::new(imp_config),
        }
    }
}