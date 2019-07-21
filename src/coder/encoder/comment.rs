use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    comment: CommentBody
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentBody {
    body: String
}

impl Comment {
    pub fn new(text: String) -> Self {
        Self {
            comment: CommentBody {
                body: text
            }
        }
    }
}