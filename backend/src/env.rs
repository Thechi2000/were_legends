use std::sync::OnceLock;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    /* #[envconfig(from = "LOL_API_KEY")]
    pub lol_api_key: String,
    #[envconfig(from = "LOL_API_REGION", default = "europe")]
    pub lol_api_region: String,
    #[envconfig(from = "LOL_API_SERVER", default = "euw1")]
    pub lol_api_server: String, */

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,

    #[envconfig(from = "BASE_URI", default = "/api")]
    pub uri: String,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn env_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().unwrap())
}
