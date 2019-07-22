use crate::entity::{Author, Timestamp};
use shrinkwraprs::Shrinkwrap;

// ------ Comment ------

#[derive(Clone)]
pub struct Comment {
    pub id: CommentId,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub author: Author,
}

// ------ CommentId ------

#[derive(Shrinkwrap, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct CommentId(String);

impl From<usize> for CommentId {
    fn from(id: usize) -> Self {
        Self(id.to_string())
    }
}
