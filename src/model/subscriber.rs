use crate::model::Email;

#[derive(Debug, Clone)]
pub struct NewSubscriber {
    pub email: Email,
}

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub id: i64,
    pub email: Email,
}
