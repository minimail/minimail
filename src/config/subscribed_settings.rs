use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SubscribedSettings {
    pub redirect: Option<String>,
}
