use std::any::Any;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;


#[derive(Serialize)]
pub struct SuccessMessageResponse {
    pub code: i16,
    pub message: String
}

#[derive(Serialize)]
pub struct SuccessDataResponse {
    pub code: i16,
    pub data: Any
}

#[derive(Serialize)]
pub struct SuccessDataArrayResponse {
    pub code: i16,
    pub count: usize,
    pub data: Vec<Any>
}

impl SuccessMessageResponse {
    pub fn ok(message: &str) -> SuccessMessageResponse {
        return SuccessMessageResponse {
            code: 200,
            message: message.to_string()
        };
    }

    pub fn created(message: &str) -> SuccessMessageResponse {
        return SuccessMessageResponse {
            code: 201,
            message: message.to_string()
        };
    }
}

impl SuccessDataResponse {
    pub fn ok(data: Any) -> SuccessDataResponse {
        return SuccessDataResponse {
            code: 200,
            data: data
        };
    }

    pub fn created(data: Any) -> SuccessDataResponse {
        return SuccessDataResponse {
            code: 201,
            data: data
        };
    }
}

impl SuccessDataArrayResponse {
    pub fn ok(data: Vec<Any>) -> SuccessDataArrayResponse {
        return SuccessDataArrayResponse {
            code: 200,
            count: &data.len(),
            data: data
        };
    }
}

impl Responder for SucessMessageResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

impl Responder for SuccessDataResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

impl Responder for SuccessDataArrayResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}
