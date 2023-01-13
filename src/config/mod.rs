mod admin_settings;
mod application_settings;
mod database_settings;
mod environment;
mod settings;
mod subscribed_settings;

use config::ConfigError;

pub use admin_settings::AdminSettings;
pub use application_settings::ApplicationSettings;
pub use database_settings::DatabaseSettings;
use environment::Environment;
pub use settings::Settings;
pub use subscribed_settings::SubscribedSettings;

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let environment: Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(config::Environment::default().separator("_"))
        .build()?;

    settings.try_deserialize()
}
