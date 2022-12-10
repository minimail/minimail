#[derive(Debug, Clone, PartialEq)]
pub struct Email(pub String);

impl From<String> for Email {
    fn from(email: String) -> Self {
        Email(email)
    }
}

impl From<&str> for Email {
    fn from(email: &str) -> Self {
        Email(email.to_string())
    }
}
