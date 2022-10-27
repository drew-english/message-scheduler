use config::Config;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub env: String,
}

pub fn load() -> Result<AppConfig, config::ConfigError> {
    let builder = Config::builder()
        .set_default("host", "0.0.0.0")?
        .set_default("port", "8000")?
        .set_default("env", "local")?
        .add_source(config::Environment::default());

    let mut cfg = builder.build_cloned()?;

    if cfg.get_string("env")? == "local" {
        info!("Loading environment variables from .env file");
        dotenv::dotenv().unwrap();
        cfg = builder.build()?;
    }

    cfg.try_deserialize()
}
