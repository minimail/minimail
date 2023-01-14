use super::{AdminSettings, ApplicationSettings, DatabaseSettings};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub admin: AdminSettings,
}
