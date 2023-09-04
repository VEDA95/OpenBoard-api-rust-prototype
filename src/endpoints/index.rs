use actix_web::{web, HttpRequest, Responder};
use serde::Serialize;
use sqlx::{postgres::PgRow, Row};
use crate::AppState;


#[derive(Serialize)]
pub struct HelloWorld {
    code: i32,
    message: String,
    db_tables: Vec<String>
}

pub async fn index(request: HttpRequest) -> impl Responder {
    let app_state: &web::Data<AppState> = request.app_data().expect("App state is missing!");
    let query_result = sqlx::query("SELECT (table_name) FROM information_schema.tables;")
        .map(|item: PgRow| item.get("table_name"))
        .fetch_all(app_state.db)
        .await
        .unwrap();

    return web::Json(HelloWorld { code: 200, message: "Hello, World!".to_string(), db_tables: query_result});
}