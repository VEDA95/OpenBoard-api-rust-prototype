use actix_web::{HttpRequest, Responder, web};
use crate::AppState;
use crate::db::user::get_users;
use crate::structs::response::success::{SuccessDataResponse, SuccessDataArrayResponse};
use crate::structs::user::user::User;


pub async fn list_users(request: HttpRequest) -> impl Responder {
    let app_state: &web::Data<AppState> = request.app_data().expect("App state is missing!");
    let users: Vec<User> = get_users(&app_state.db).await;

    return SuccessDataArrayResponse::ok(users);
}
