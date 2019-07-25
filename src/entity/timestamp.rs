use crate::entity::ErrorMessage;
use chrono::prelude::*;
use newtype::NewType;
use seed::prelude::*;
use std::convert::TryFrom;

// ------ Timestamp ------

#[derive(NewType, Clone)]
pub struct Timestamp(DateTime<Local>);

impl TryFrom<String> for Timestamp {
    type Error = ErrorMessage;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // "2019-07-24T08:21:36.453Z"
        DateTime::parse_from_rfc3339(string.as_str())
            .map(|date_time| Self(date_time.into()))
            .map_err(|parse_error| parse_error.to_string().into())
    }
}

// ------ view timestamp ------

pub fn view<Ms>(timestamp: &Timestamp) -> Node<Ms> {
    span![
        class!["date"],
        // "February 14, 2018"
        timestamp.format("%B %-d, %-Y").to_string()
    ]
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::convert::TryInto;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn parse_and_view_timestamp_test() {
        // ====== ARRANGE ======
        let timestamp: Timestamp = "2019-07-24T08:21:36.453Z"
            .to_string()
            .try_into()
            .expect("cannot parse given timestamp");

        // ====== ACT ======
        let node: Node<()> = view(&timestamp);

        // ====== ASSERT ======
        assert_eq!(node.get_text(), "July 24, 2019");
    }
}
