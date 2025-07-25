use crate::settings::Settings;
use config::{Config, Environment, File};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name(".castnowrc").required(false))
        .add_source(Environment::with_prefix("CASTNOW").separator("__"))
        .build()?;

    settings.try_deserialize()
}
