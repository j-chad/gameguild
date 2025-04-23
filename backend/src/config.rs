pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub env: AppEnv,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Production,
    Test,
}

pub fn load_config() -> anyhow::Result<Config> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    
    config.try_deserialize::<Config>()
}