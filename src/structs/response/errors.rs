use serde_json::Value;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;


#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub error: Value
}

#[derive(Serialize)]
pub struct ErrorArrayResponse {
    pub code: i16,
    pub count: usize,
    pub errors: Vec<Value>
}

impl ErrorResponse {
    pub fn error(data: Value) -> ErrorResponse {
        return ErrorResponse {
            code: 400,
            error: data
        };
    }

    pub fn unauthenticated(data: Value) -> ErrorResponse {
        return ErrorResponse {
            code: 401,
            error: data
        };
    }

    pub fn unauthorized(data: Value) -> ErrorResponse {
        return ErrorResponse {
            code: 403,
            error: data
        };
    }

    pub fn not_found(data: Value) -> ErrorResponse {
        return ErrorResponse {
            code: 404,
            error: data
        };
    }

    pub fn unsupported_media_type(data: Value) -> ErrorResponse {
        return ErrorResponse {
            code: 415,
            error: data
        };
    }
}

impl ErrorArrayResponse {
    pub fn error(data: Vec<Value>) -> ErrorArrayResponse {
        return ErrorArrayResponse{
            code: 400,
            count: data.len(),
            errors: data
        };
    }

    pub fn unauthenticated(data: Vec<Value>) -> ErrorArrayResponse {
        return ErrorArrayResponse{
            code: 401,
            count: data.len(),
            errors: data
        };
    }

    pub fn unauthorized(data: Vec<Value>) -> ErrorArrayResponse {
        return ErrorArrayResponse{
            code: 403,
            count: data.len(),
            errors: data
        };
    }

    pub fn not_found(data: Vec<Value>) -> ErrorArrayResponse {
        return ErrorArrayResponse{
            code: 404,
            count: data.len(),
            errors: data
        };
    }

    pub fn unsupported_media_type(data: Vec<Value>) -> ErrorArrayResponse {
        return ErrorArrayResponse{
            code: 415,
            count: data.len(),
            errors: data
        };
    }
}

impl Responder for ErrorResponse {
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

impl Responder for ErrorArrayResponse {
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