use serde_aux::field_attributes::deserialize_number_from_string;

use super::{MailerSettings, SubscribedSettings};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    #[serde(default)]
    pub subscribed: SubscribedSettings,
    pub mailer: Option<MailerSettings>,
}

#[cfg(test)]
mod tests {
    use crate::config::{MailerProvider, MailerSettings};

    use super::ApplicationSettings;

    #[test]
    fn deserialize() {
        let serialized = r#"
            port: 100
            host: localhost
            subscribed:
                redirect: null
            mailer:
                provider: SendGrid
                token: sendgrid_token
        "#;

        let application_settings: ApplicationSettings = serde_yaml::from_str(serialized).unwrap();
        let mailer_settings = application_settings.mailer.unwrap();
        assert_eq!(
            MailerSettings {
                token: "sendgrid_token".to_string(),
                provider: MailerProvider::SendGrid
            },
            mailer_settings
        );
    }
}
