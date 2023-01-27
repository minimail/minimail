use anyhow::Result;

use crate::{mail::SendMail, model::Email};

pub struct SendGridMail {
    pub token: String,
}

impl SendMail for SendGridMail {
    async fn send(&mut self, _emails: Vec<Email>, _body: &str) -> Result<()> {
        Ok(())
    }
}
