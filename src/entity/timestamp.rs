use chrono::prelude::*;
use seed::prelude::*;
use std::{convert::TryFrom, fmt};

#[derive(Clone)]
pub struct Timestamp(DateTime<Local>);

impl Timestamp {
    pub fn format(&self, format: &str) -> String {
        self.0.format(format).to_string()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Timestamp {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        DateTime::parse_from_rfc3339(string.as_str())
            .map(|date_time| Timestamp(date_time.into()))
            .map_err(|parse_error| parse_error.to_string())
    }
}

pub fn view<Ms>(timestamp: &Timestamp) -> Node<Ms> {
    span![
        class!["date"],
        // "February 14, 2018"
        timestamp.format("%B %-d, %-Y")
    ]
}
