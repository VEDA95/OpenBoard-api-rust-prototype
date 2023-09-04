use actix_web::{body::BoxBody, http::header::ContentType, Responder, HttpResponse, HttpRequest};
use serde::Serialize;
use chrono::{DateTime, FixedOffset};
use crate::endpoints::auth::user::structs::roles::{Permission, Role};
use crate::endpoints::auth::user::structs::external_provider::UserExternalProvider;


#[derive(Serialize)]
enum ExternalProviderValueTypes {
    UserExternalProvider(UserExternalProvider),
    Nil
}

#[derive(Serialize)]
enum UserThumbnailValueTypes {
    Nil
}

#[derive(Serialize)]
pub struct User {
    id: String,
    date_created: DateTime<FixedOffset>,
    date_updated: DateTime<FixedOffset>,
    last_login: DateTime<FixedOffset>,
    enabled: bool,
    thumbnail: UserThumbnailValueTypes,
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    roles: Vec<Role>,
    external_provider: ExternalProviderValueTypes
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

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}