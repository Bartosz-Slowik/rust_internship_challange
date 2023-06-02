#![allow(dead_code)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate random_number;

use crate::db::users::{create_user, User};
use crate::db_connection::{create_db_pool, DbPool};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use uuid::Uuid;

mod db;
mod db_connection;
mod errors;
mod mocks;
mod routes;
mod schema;
mod search;

// Wrapper around user id
pub struct ActingUserId(pub Uuid);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let pool = create_db_pool();
    let mock_user_id = create_mock_user(&pool)
        .expect("Failed to create mock user!")
        .id;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(ActingUserId(mock_user_id))
            .data(mocks::api::PartSearchAPI {})
            .service(routes::healthcheck)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Just helper to prepopulate a user in DB that so that we can pretend that we can use real user id
fn create_mock_user(pool: &DbPool) -> Result<User, diesel::result::Error> {
    let conn = pool.get().expect("Failed to get connection");
    let mock_user_email = "mock@user.com";
    if let Ok(user) = User::find_by_email(&conn, mock_user_email) {
        return Ok(user);
    }
    create_user(&conn, mock_user_email, "SECRETAPIKEY")
}
