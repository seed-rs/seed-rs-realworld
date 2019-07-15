
#[derive(Clone, PartialEq, Eq)]
pub struct CommentId(String);

impl CommentId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for CommentId {
    fn from(id: String) -> CommentId {
        CommentId(id)
    }
}