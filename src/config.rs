use config::Config;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct AppConfig {
    pub api_host: String,
    pub api_port: String,
    pub database_url: String,
    pub rust_env: String,
}

pub fn load() -> Result<AppConfig, config::ConfigError> {
    let builder = Config::builder()
        .set_default("api_host", "0.0.0.0")?
        .set_default("api_port", "8000")?
        .set_default("rust_env", "local")?
        .add_source(config::Environment::default());

    let mut cfg = builder.build_cloned()?;

    if cfg.get_string("rust_env")? == "local" {
        info!("Loading environment variables from .env file");
        dotenv::dotenv().unwrap();
        cfg = builder.build()?;
    }

    cfg.try_deserialize()
}
