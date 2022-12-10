#[derive(Debug, Clone)]
pub struct NewSubscriber {
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub id: i32,
    pub email: String,
}
