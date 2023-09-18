use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SuccessMessageResponse {
    pub code: i16,
    pub message: String
}

#[derive(Serialize, Debug)]
pub struct SuccessDataResponse<T> {
    pub code: i16,
    pub data: T
}

#[derive(Serialize, Debug)]
pub struct SuccessDataArrayResponse<T> {
    pub code: i16,
    pub count: usize,
    pub data: Vec<T>
}

impl SuccessMessageResponse {
    pub fn ok(message: &str) -> Self {
        return SuccessMessageResponse {
            code: 200,
            message: message.to_string()
        };
    }

    pub fn created(message: &str) -> Self {
        return SuccessMessageResponse {
            code: 201,
            message: message.to_string()
        };
    }
}

impl<T> SuccessDataResponse<T> {
    pub fn ok(data: T) -> Self {
        return SuccessDataResponse {
            code: 200,
            data: data
        };
    }

    pub fn created(data: T) -> Self {
        return SuccessDataResponse {
            code: 201,
            data: data
        };
    }
}

impl<T> SuccessDataArrayResponse<T> {
    pub fn ok(data: Vec<T>) -> Self {
        return SuccessDataArrayResponse {
            code: 200,
            count: data.len(),
            data: data
        };
    }

    pub fn created(data: Vec<T>) -> Self {
        return SuccessDataArrayResponse {
            code: 201,
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

impl <T> Responder for SuccessDataResponse<T> where T: Serialize {
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

impl <T> Responder for SuccessDataArrayResponse<T> where T: Serialize {
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
