use crate::entity::{Timestamp, Author};

#[derive(Clone)]
pub struct Comment<'a> {
    pub id: CommentId,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub author: Author<'a>
}

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