use crate::entity::ErrorMessage;
use chrono::prelude::*;
use newtype::NewType;
use seed::prelude::*;
use std::convert::TryFrom;

// ------ Timestamp ------

#[derive(NewType, Clone)]
pub struct Timestamp(DateTime<Local>);

impl Timestamp {
    pub fn format(&self, format: &str) -> String {
        self.0.format(format).to_string()
    }
}

impl TryFrom<String> for Timestamp {
    type Error = ErrorMessage;

    fn try_from(string: String) -> Result<Self, Self::Error> {
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
        timestamp.format("%B %-d, %-Y")
    ]
}
