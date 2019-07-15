use crate::{comment_id, timestamp, author};

#[derive(Clone)]
pub struct Comment<'a> {
    pub id: comment_id::CommentId,
    pub body: String,
    pub created_at: timestamp::Timestamp,
    pub updated_at: timestamp::Timestamp,
    pub author: author::Author<'a>
}