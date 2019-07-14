use crate::{comment_id, timestamp, author};

pub struct Comment<'a> {
    id: comment_id::CommentId,
    body: String,
    created_at: timestamp::Timestamp,
    author: author::Author<'a>
}