use std::collections::HashMap;
use sqlx::{postgres::PgRow, Row, Postgres, Transaction, PgPool, query_builder::{QueryBuilder, Separated}, Execute};
use crate::structs::user::{user::User, roles::{Permission, Role}, thumbnail::UserThumbnail, external_provider::UserExternalProvider};

const SELECT_USER_QUERY: &str = "SELECT (
    usr.id,
    usr.username,
    usr.email,
    usr.first_name,
    usr.last_name,
    usr.enabled,
    usr.date_created,
    usr.date_updated,
    usr.last_login,
    file_upload.id,
    file_upload.name,
    file_upload.date_created,
    file_upload.date_updated,
    file_upload.file_size,
    file_upload.additional_details,
    external_auth.id,
    external_auth.name,
    external_auth.default_login_method
) FROM open_board_user usr
  JOIN open_board_file_upload file_upload ON file_upload.id = usr.thumbnail
  JOIN open_board_external_auth_provider external_auth ON external_auth.id = usr.external_provider_id;";

const SELECT_USER_ROLES_QUERY: &str = "SELECT (
    user_role.user_id,
    role.*
) FROM open_board_user_roles user_role
  JOIN open_board_role role ON role.id = user_role.role_id";

const SELECT_ROLE_PERMISSIONS_QUERY: &str = "SELECT (
    role_permission.role_id,
    permission.*
) FROM open_board_role_permissions role_permission
  JOIN open_board_role_permission permission ON permission.id = role_permission.permission_id";

enum QueryHashMapValue {
    Role(Role),
    Vec(Vec<String>)
}

pub async fn get_users(connection: &PgPool) -> Vec<User> {
    let mut transaction: Transaction<'_, Postgres> = connection.begin().await.unwrap();
    let mut users: Vec<User> = Vec::new();
    let mut role_filter: Vec<String> = Vec::new();
    let mut roles: Vec<HashMap<&str, QueryHashMapValue>> = Vec::new();
    let mut select_roles_permissions_query_builder: QueryBuilder<Postgres> = QueryBuilder::new(SELECT_ROLE_PERMISSIONS_QUERY);
    let mut user_query: Vec<User> = sqlx::query(&SELECT_USER_QUERY)
        .map(|row: PgRow| {
            let thumbnail_id: String = row.get("file_upload.id");
            let external_auth_id: String = row.get("external_auth.id");
            let mut new_user: User = User::new(
                row.get("usr.id"),
                row.get("usr.date_created"),
                row.get("usr.date_updated"),
                row.get("usr.last_login"),
                row.get("usr.enabled"),
                row.get("usr.username"),
                row.get("usr.email"),
                row.get("usr.first_name"),
                row.get("usr.last_name")
            );

            if !thumbnail_id.is_empty() {
                let user_thumnail: UserThumbnail = UserThumbnail::new(
                    thumbnail_id,
                    row.get("file_upload.name"),
                    row.get("file_upload.date_created"),
                    row.get("file_upload.date_updated"),
                    row.get("file_upload.file_size"),
                    row.get("file_upload.additional_details")
                );

                new_user.add_thumbnail(user_thumnail);
            }

            if !external_auth_id.is_empty() {
                let external_auth_provider: UserExternalProvider = UserExternalProvider {
                    id: external_auth_id,
                    name: row.get("external_auth.name"),
                    default_login_method: row.get("external_auth.default_login_method")
                };

                new_user.add_external_provider(external_auth_provider);
            }

            return new_user.clone();
        })
        .fetch_all(&mut *transaction)
        .await
        .unwrap();

    let user_roles_query: Vec<HashMap<&str, String>> = sqlx::query(&SELECT_USER_ROLES_QUERY)
        .map(|row: PgRow| {
            let mut output: HashMap<&str, String> = HashMap::new();
            let role_id: String = row.get("role.id");

            role_filter.push(role_id.clone());
            output.insert("id", row.get("user_role.user_id"));
            output.insert("role_id", role_id);
            output.insert("role_name", row.get("role.name"));

            return output;
        })
        .fetch_all(&mut *transaction)
        .await
        .unwrap();

    if role_filter.len() != 0 {
        select_roles_permissions_query_builder.push(" WHERE role_permission.role_id IN (");

        let mut separated_roles_permissions: Separated<'_, '_, Postgres, &str> = select_roles_permissions_query_builder.separated(", ");

        for filter_value in role_filter.iter() {
            println!("{}", filter_value);
            separated_roles_permissions.push_bind(filter_value);
        }

        separated_roles_permissions.push_unseparated(");");
    } else {
        select_roles_permissions_query_builder.push(";");
    }

    let user_roles_permissions_query: Vec<HashMap<&str, String>> = sqlx::query(select_roles_permissions_query_builder.build().sql().into())
        .map(|row: PgRow| {
            let mut output: HashMap<&str, String> = HashMap::new();

            output.insert("id", row.get("role_permission.role_id"));
            output.insert("permission_id", row.get("permission.id"));
            output.insert("permission_path", row.get("permission.path"));

            return output;
        })
        .fetch_all(&mut *transaction)
        .await
        .unwrap();

    transaction.commit().await;

    for role_query in user_roles_query.iter() {
        let user_id: &String = role_query.get("id").unwrap();
        let role_id: &String = role_query.get("role_id").unwrap();
        let mut pre_existing_role: Vec<&mut HashMap<&str, QueryHashMapValue>> = roles.iter_mut().filter(|item: &&mut HashMap<&str, QueryHashMapValue>| -> bool {
            let existing_role: &Role = match item.get("role").unwrap() {
                QueryHashMapValue::Role(role) => role,
                QueryHashMapValue::Vec(_vec) => panic!("wrong value!")
            };

            if role_id == &existing_role.id {
                return true;
            }

            return false;
        }).collect();

        if pre_existing_role.len() == 0 {
            let mut new_entry: HashMap<&str, QueryHashMapValue> = HashMap::new();
            let mut new_role: Role = Role::new(role_id.clone(), role_query.get("role_name").unwrap().clone());

            new_entry.insert("id_list", QueryHashMapValue::Vec(Vec::from([user_id.clone()])));

            let role_permissions: Vec<&HashMap<&str, String>> = user_roles_permissions_query.iter().filter(|item: &&HashMap<&str, String>| -> bool {
                let role_permission_id: &String = item.get("id").unwrap();

                if role_permission_id == role_id {
                    return true;
                }

                return false;
            }).collect();

            for role_permission in role_permissions {
                let permission_id: String = role_permission.get("permission_id").unwrap().to_string();
                let permission_name: String = role_permission.get("permission_path").unwrap().to_string();
                let new_permission: Permission = Permission { id: permission_id, path: permission_name };

                new_role.add_permission(new_permission);
            }

            new_entry.insert("role", QueryHashMapValue::Role(new_role));

            roles.push(new_entry);
            continue;
        }

        let pre_existing_role_item: &mut &mut HashMap<&str, QueryHashMapValue> = pre_existing_role.get_mut(0).unwrap();
        let mut user_ids: Vec<String> = match pre_existing_role_item.get("id_list").unwrap() {
            QueryHashMapValue::Vec(list) => list.clone(),
            QueryHashMapValue::Role(_role) => panic!("wrong value!")
        };

        if user_ids.contains(user_id) {
            continue;
        }

        user_ids.push(user_id.clone());
        pre_existing_role_item.insert("id_list", QueryHashMapValue::Vec(user_ids));
    }

    for user in user_query.iter_mut() {
        for role in roles.iter() {
            let id_list = match role.get("id_list").unwrap() {
                QueryHashMapValue::Role(_role) => panic!("wrong value!"),
                QueryHashMapValue::Vec(vect) => vect.clone()
            };

            if id_list.contains(&user.id) {
                continue;
            }

            let role_entry: Role = match role.get("role").unwrap() {
                QueryHashMapValue::Role(role) => role.clone(),
                QueryHashMapValue::Vec(_vect) => panic!("wrong value!")
            };

            user.add_role(role_entry);
        }

        users.push(user.clone());
    }

    return users;
}