#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct DatabaseSettings {
    pub url: String,
}
