use actix_web::{web, HttpRequest, Responder};
use sqlx::{postgres::PgRow, Row, Transaction, Postgres};
use crate::AppState;
use crate::endpoints::auth::user::structs::user;

const SELECT_USER_QUERY: &str = "SELECT (
    user.id,
    user.username,
    user.email,
    user.first_name,
    user.last_name,
    user.enabled,
    user.date_created,
    user.date_updated,
    user.last_login
    file_upload.id,
    file_upload.name
    file_upload.date_created
    file_upload.date_updated
    file_upload.file_size
    file_upload.additional_details
    external_auth.id
    external_auth.name
    external_auth.default_login_method
) FROM open_board_user user
  JOIN open_board_file_upload file_upload ON file_upload.id = user.thumbnail
  JOIN open_board_external_auth_provider external_auth ON external_auth.id = user.external_provider_id;";

const SELECT_USER_ROLES_QUERY: &str = "SELECT (
    role.id,
    role.name
) FROM open_board_user_roles user_role
  JOIN open_board_user user ON user.id = user_role.user_id
  JOIN open_board_role role ON role.id = user_role.role_id";

const SELECT_ROLE_PERMISSIONS_QUERY: &str = "SELECT (
    role.id
    permission.*
) FROM open_board_role_permissions role_permission
  JOIN open_board_role role ON role.id = role_permission.role_id
  JOIN open_board_role_permission permission ON permission.id = role_permission.permission_id
  WHERE role.id in ?";

pub async fn list_users(request: HttpRequest) -> impl Responder {
    let app_state: &AppState<'_> = *request.app_data::<&AppState>().unwrap();
    let mut transaction: &Transaction<'_, Postgres> = &app_state.db.begin().await?;
    let user_query = sqlx::query(&SELECT_USER_QUERY)
        .fetch_all(&mut *transaction)
        .await?;
    let user_roles_query = sqlx::query(&SELECT_USER_ROLES_QUERY)
        .fetch_all(&mut *transaction)
        .await?;
    let user_roles_permissions_query = sqlx::query(&SELECT_ROLE_PERMISSIONS_QUERY)
        .fetch_all(&mut *transaction)
        .await?;

    transaction.commit().await?;

    return web::Json(HelloWorld { code: 200, message: "Hello, World!".to_string(), db_tables: query_result});
}
