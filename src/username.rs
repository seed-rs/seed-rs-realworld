pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Username(username)
    }
}

impl From<String> for Username {
    fn from(string: String) -> Username {
        Username(string)
    }
}