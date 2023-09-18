use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;


#[derive(Serialize)]
pub struct ErrorResponse<T> {
    pub code: i16,
    pub error: T
}

#[derive(Serialize)]
pub struct ErrorArrayResponse<T> {
    pub code: i16,
    pub count: usize,
    pub errors: Vec<T>
}

impl <T> ErrorResponse<T> {
    pub fn error(data: T) -> Self {
        return ErrorResponse {
            code: 400,
            error: data
        };
    }

    pub fn unauthenticated(data: T) -> Self {
        return ErrorResponse {
            code: 401,
            error: data
        };
    }

    pub fn unauthorized(data: T) -> Self {
        return ErrorResponse {
            code: 403,
            error: data
        };
    }

    pub fn not_found(data: T) -> Self {
        return ErrorResponse {
            code: 404,
            error: data
        };
    }

    pub fn unsupported_media_type(data: T) -> Self {
        return ErrorResponse {
            code: 415,
            error: data
        };
    }
}

impl <T> ErrorArrayResponse<T> {
    pub fn error(data: Vec<T>) -> Self {
        return ErrorArrayResponse{
            code: 400,
            count: data.len(),
            errors: data
        };
    }

    pub fn unauthenticated(data: Vec<T>) -> Self {
        return ErrorArrayResponse{
            code: 401,
            count: data.len(),
            errors: data
        };
    }

    pub fn unauthorized(data: Vec<T>) -> Self {
        return ErrorArrayResponse{
            code: 403,
            count: data.len(),
            errors: data
        };
    }

    pub fn not_found(data: Vec<T>) -> Self {
        return ErrorArrayResponse{
            code: 404,
            count: data.len(),
            errors: data
        };
    }

    pub fn unsupported_media_type(data: Vec<T>) -> Self {
        return ErrorArrayResponse{
            code: 415,
            count: data.len(),
            errors: data
        };
    }
}

impl <T> Responder for ErrorResponse<T> where T: Serialize {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        if &self.code == &mut 401 {
            return HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 403 {
            return HttpResponse::Forbidden()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 404 {
            return HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 415 {
            return HttpResponse::UnsupportedMediaType()
                .content_type(ContentType::json())
                .body(body);
        }

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body);
    }
}

impl <T> Responder for ErrorArrayResponse<T> where T: Serialize {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        if &self.code == &mut 401 {
            return HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 403 {
            return HttpResponse::Forbidden()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 404 {
            return HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(body);
        }

        if &self.code == &mut 415 {
            return HttpResponse::UnsupportedMediaType()
                .content_type(ContentType::json())
                .body(body);
        }

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body);
    }
}