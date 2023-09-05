use serde::Serialize;
use chrono::{DateTime, FixedOffset};
use crate::structs::user::roles::{Permission, Role};
use crate::structs::user::external_provider::UserExternalProvider;
use crate::structs::user::thumbnail::UserThumbnail;


#[derive(Serialize)]
enum ExternalProviderValueTypes {
    UserExternalProvider(UserExternalProvider),
    Nil
}

#[derive(Serialize)]
enum UserThumbnailValueTypes {
    UserThumbnail(UserThumbnail),
    Nil
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub date_created: DateTime<FixedOffset>,
    pub date_updated: DateTime<FixedOffset>,
    pub last_login: DateTime<FixedOffset>,
    pub enabled: bool,
    pub thumbnail: UserThumbnailValueTypes,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<Role>,
    pub external_provider: ExternalProviderValueTypes
}

impl User {
    pub fn new(
    id: String,
    date_created: String,
    date_updated: String,
    last_login: String,
    enabled: bool,
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    thumnail: UserThumbnailValueTypes,
    external_provider: ExternalProviderValueTypes,
    roles: Vec<Role>,
    permissions: Vec<Permission>) -> User {
        const DATETIME_FORMAT: &str = "%b %d %Y %H:%M %z";
        return User {
            id: id,
            date_created: DateTime::parse_from_str(&date_created.as_str(), &DATETIME_FORMAT).unwrap(),
            date_updated: DateTime::parse_from_str(&date_updated.as_str(), &DATETIME_FORMAT).unwrap(),
            last_login: DateTime::parse_from_str(&last_login.as_str(), DATETIME_FORMAT).unwrap(),
            enabled: enabled,
            username: username,
            email: email,
            first_name: first_name,
            last_name: last_name,
            thumbnail: UserThumbnailValueTypes::Nil,
            roles: Vec::new(),
            external_provider: ExternalProviderValueTypes::Nil
        };
    }
}