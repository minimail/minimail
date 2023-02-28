use anyhow::Result;

use crate::model::Email;

pub trait SendMail {
    async fn send(&mut self, emails: Vec<Email>, body: &str) -> Result<()>;
}
