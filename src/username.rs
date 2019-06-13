#[derive(Clone)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Username(username)
    }
}