pub struct Username(String);

// @TODO do we need new (check everything)?
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