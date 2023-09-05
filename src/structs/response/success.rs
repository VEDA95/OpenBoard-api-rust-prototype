use serde_json::Value;
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
    pub data: Value
}

#[derive(Serialize)]
pub struct SuccessDataArrayResponse {
    pub code: i16,
    pub count: usize,
    pub data: Vec<Value>
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
    pub fn ok(data: Value) -> SuccessDataResponse {
        return SuccessDataResponse {
            code: 200,
            data: data
        };
    }

    pub fn created(data: Value) -> SuccessDataResponse {
        return SuccessDataResponse {
            code: 201,
            data: data
        };
    }
}

impl SuccessDataArrayResponse {
    pub fn ok(data: Vec<Value>) -> SuccessDataArrayResponse {
        return SuccessDataArrayResponse {
            code: 200,
            count: data.len(),
            data: data
        };
    }
}

impl Responder for SuccessMessageResponse {
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

        if &self.code == &mut 201 {
            return HttpResponse::Created()
                .content_type(ContentType::json())
                .body(body);
        }

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

impl Responder for SuccessDataArrayResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();

        if &self.code == &mut 201 {
            return HttpResponse::Created()
                .content_type(ContentType::json())
                .body(body);
        }

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}
