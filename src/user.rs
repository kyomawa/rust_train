use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    #[serde(rename = "id")]
    _id: u8,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
}

impl User {
    pub fn new(first_name: &str, last_name: &str, username: &str, email: &str, age: u8) -> User {
        User {
            _id: 5,
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            username: String::from(username),
            email: String::from(email),
            age,
        }
    }
}
