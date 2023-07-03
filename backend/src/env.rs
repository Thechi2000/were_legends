use std::sync::OnceLock;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "LOL_API_KEY")]
    pub lol_api_key: String,
    #[envconfig(from = "LOL_API_REGION", default = "europe")]
    pub lol_api_region: String,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn env_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().unwrap())
}