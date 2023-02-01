use serde::{Deserialize, Serialize};

use crate::mail::{send_grid::SendGridMail, SendMail};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MailerProvider {
    SendGrid,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MailerSettings {
    pub provider: MailerProvider,
    pub token: String,
}

impl MailerSettings {
    pub fn get_send_mail(&self) -> impl SendMail {
        match self.provider {
            MailerProvider::SendGrid => SendGridMail {
                token: self.token.to_string(),
            },
        }
    }
}
