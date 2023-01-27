use serde::{Deserialize, Serialize};

use crate::mail::{send_grid::SendGridMail, SendMail};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MailerSettings {
    SendGrid(SendGridSettings),
}

impl MailerSettings {
    pub fn get_send_mail(&self) -> impl SendMail {
        match self {
            MailerSettings::SendGrid(it) => it.get_send_mail(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendGridSettings {
    pub token: String,
}

impl SendGridSettings {
    pub fn get_send_mail(&self) -> impl SendMail {
        SendGridMail {
            token: self.token.to_string(),
        }
    }
}
