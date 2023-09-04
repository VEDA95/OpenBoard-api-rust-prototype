use std::any::Any;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder, Error};
use serde::Serialize;


#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub error: Any
}

#[derive(Serialize)]
pub struct ErrorArrayResponse {
    pub code: i16,
    pub count: usize,
    pub errors: Vec<Any>
}

impl ErrorResponse {
    pub fn error(data: Any) -> ErrorResponse {
        return ErrorResponse {
            code: 400,
            error: data
        };
    }

    pub fn unauthenticated(data: Any) -> ErrorResponse {
        return ErrorResponse {
            code: 401,
            error: data
        };
    }

    pub fn unauthorized(data: Any) -> ErrorResponse {
        return ErrorResponse {
            code: 403,
            error: data
        };
    }

    pub fn not_found(data: Any) -> ErrorResponse {
        return ErrorResponse {
            code: 404,
            error: data
        };
    }

    pub fn unsupported_media_type(data: Any) -> ErrorResponse {
        return ErrorResponse {
            code: 415,
            error: data
        };
    }
}

impl ErrorArrayResponse {
    pub fn error(data: Vec<Any>) -> ErrorResponse {
        return ErrorArrayResponse{
            code: 400,
            count: &data.len(),
            errors: data
        };
    }

    pub fn unauthenticated(data: Vec<Any>) -> ErrorResponse {
        return ErrorArrayResponse{
            code: 401,
            count: &data.len(),
            errors: data
        };
    }

    pub fn unauthorized(data: Vec<Any>) -> ErrorResponse {
        return ErrorArrayResponse{
            code: 403,
            count: &data.len(),
            errors: data
        };
    }

    pub fn not_found(data: Vec<Any>) -> ErrorResponse {
        return ErrorArrayResponse{
            code: 404,
            count: &data.len(),
            errors: data
        };
    }

    pub fn unsupported_media_type(data: Vec<Any>) -> ErrorResponse {
        return ErrorArrayResponse{
            code: 415,
            count: &data.len(),
            errors: data
        };
    }
}

impl Responder for ErrorResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

impl Responder for ErrorArrayResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}