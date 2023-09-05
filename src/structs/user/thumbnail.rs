use serde_json::Value;
use serde::Serialize;
use chrono::{DateTime, FixedOffset};

#[derive(Serialize)]
pub struct UserThumbnail {
    pub id: String,
    pub name: String,
    pub date_created: DateTime<FixedOffset>,
    pub date_updated: DateTime<FixedOffset>,
    pub size: usize,
    pub additional_details: Value
}

impl UserThumbnail {
    pub fn new(
        id: String,
        name: String,
        date_created: String,
        date_updated: String,
        size: usize,
        additional_details: Value
    ) -> UserThumbnail {
        const DATETIME_FORMAT: &str = "%b %d %Y %H:%M %z";
        return UserThumbnail {
            id: id,
            name: name,
            date_created: DateTime::parse_from_str(date_created.as_str(), &DATETIME_FORMAT).unwrap(),
            date_updated: DateTime::parse_from_str(date_updated.as_str(), &DATETIME_FORMAT).unwrap(),
            size: size,
            additional_details: additional_details
        };
    }
}