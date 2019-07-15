use std::fmt;

#[derive(Clone)]
pub struct Markdown(String);

impl Markdown {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Markdown {
    fn from(id: String) -> Markdown {
        Markdown(id)
    }
}

impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}