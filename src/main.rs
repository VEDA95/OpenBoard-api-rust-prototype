use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;
use std::env;
mod endpoints;
mod db;
mod structs;

pub struct AppState {
    db: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host: String = env::var("HOST").expect("Host IP to run api on is required!");
    let port: i32 = env::var("PORT").expect("Port number to run api on is requires!").parse().unwrap();
    let db_uri: String = env::var("DB_URI").expect("A database URI is required!");
    let pool = match PgPoolOptions::new()
       .max_connections(5)
       .connect(&db_uri)
       .await
       {
        Ok(pool) => {
            println!("Connected to the database successfully!");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the database: {:?}", error);
            std::process::exit(1);
        }
       };

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(AppState{ db: pool.clone() }))
            .configure(endpoints::endpoints_factory);

        return app;
    })
    .bind(format!("{url_host}:{url_port}", url_host = &host, url_port = &port))?
    .run()
    .await
}
