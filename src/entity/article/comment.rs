use crate::entity::{timestamp, author};

#[derive(Clone)]
pub struct Comment<'a> {
    pub id: CommentId,
    pub body: String,
    pub created_at: timestamp::Timestamp,
    pub updated_at: timestamp::Timestamp,
    pub author: author::Author<'a>
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