use secrecy::SecretString;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub rabbitmq: RabbitMqSettings,
    pub postgres: PostgresSettings,
}

#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct RabbitMqSettings {
    pub uri: SecretString,
}

#[derive(Clone, Deserialize)]
pub struct PostgresSettings {
    pub host: String,
    pub port: u16,
    pub username: SecretString,
    pub password: SecretString,
    pub database_name: String,
    pub schema_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("TX")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local`, `development` or `production`.",
                other
            )),
        }
    }
}
