pub type Uuid = u128; // TODO : change u128 to uuid

#[derive(Debug, Hash)]
pub struct User {
    pub username: Option<String>,
    pub uuid: Option<Uuid>,
}

impl User {
    pub fn new() -> Self {
        User { username: None, uuid: None }
    }
}