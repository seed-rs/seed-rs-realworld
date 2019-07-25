use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    comment: CommentBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentBody {
    body: String,
}

impl Comment {
    pub const fn new(text: String) -> Self {
        Self {
            comment: CommentBody { body: text },
        }
    }
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{self, json};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn encode_comment_test() {
        // ====== ARRANGE ======
        let comment = Comment::new("text".into());
        let expected_json = json!({
            "comment": {
                "body": "text"
            }
        });

        // ====== ACT ======
        let json = serde_json::to_string(&comment).expect("serialize Comment failed");

        // ====== ASSERT ======
        assert_eq!(json, expected_json.to_string());
    }
}
