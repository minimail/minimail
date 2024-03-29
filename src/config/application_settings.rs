use serde_aux::field_attributes::deserialize_number_from_string;

use super::SubscribedSettings;

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    #[serde(default)]
    pub subscribed: SubscribedSettings,
}
