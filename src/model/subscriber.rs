use serde::{Deserialize, Serialize};

use crate::model::Email;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSubscriber {
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: i32,
    pub email: Email,
}
