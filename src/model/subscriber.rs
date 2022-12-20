use serde::{Deserialize, Serialize};

use crate::model::Email;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSubscriber {
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: i64,
    pub email: Email,
}
