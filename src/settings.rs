use config::{Config, ConfigError, Environment, File};

enum AppEnv {
    Development,
    Test,
    Production,
}

impl std::fmt::Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnv::Development => write!(f, "development"),
            AppEnv::Test => write!(f, "test"),
            AppEnv::Production => write!(f, "production"),
        }
    }
}

impl TryFrom<String> for AppEnv {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "test" => Ok(Self::Test),
            "production" => Ok(Self::Production),
            other => Err(format!("Invalid environment: {other}")),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = std::env::current_dir().unwrap().join("config");
        let environment: AppEnv = std::env::var("APP_ENV")
            .unwrap_or(String::from("development"))
            .try_into()
            .unwrap();
        let settings = Config::builder()
            .add_source(File::from(config_dir.join("default.yaml")))
            .add_source(File::from(config_dir.join(format!("{environment}.yaml"))).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .unwrap();
        settings.try_deserialize()
    }
}
