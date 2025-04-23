use config::Config;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    Development,
    Stage,
    Production,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
    pub env: AppEnv,
}

#[derive(Debug, Deserialize)]
struct BootstrapSettings {
    pub env: AppEnv,
}

impl Settings {
    fn get_env_file_name(env: AppEnv) -> String {
        match env {
            AppEnv::Development => "develop.toml",
            AppEnv::Stage => "stage.toml",
            AppEnv::Production => "production.toml",
        }
        .to_string()
    }

    fn get_config(environment: Option<AppEnv>) -> Result<Config, config::ConfigError> {
        let base_path = Path::new(file!())
            .parent()
            .ok_or(config::ConfigError::NotFound(
                "could not get base path".to_string(),
            ))?;

        let mut builder =
            Config::builder().add_source(config::File::from(base_path.join("default.toml")));

        if let Some(env) = environment {
            builder = builder.add_source(config::File::from(
                base_path.join(Self::get_env_file_name(env)),
            ));
        }

        builder = builder
            .add_source(config::File::from(base_path.join("local.toml")).required(false))
            .add_source(config::Environment::with_prefix("APP"));

        builder.build()
    }

    fn get_runtime_env() -> Result<AppEnv, config::ConfigError> {
        let config = Self::get_config(None)?;
        let bootstrap_settings: BootstrapSettings = config.try_deserialize()?;
        Ok(bootstrap_settings.env)
    }

    pub fn new() -> Result<Self, config::ConfigError> {
        let env: AppEnv = Self::get_runtime_env()?;

        let config = Self::get_config(Some(env))?;
        config.try_deserialize::<Settings>()
    }
}
