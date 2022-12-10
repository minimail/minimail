use crate::model::Email;

#[derive(Debug, Clone)]
pub struct NewSubscriber {
    pub email: Email,
}

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub id: i32,
    pub email: Email,
}
