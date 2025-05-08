mod api;
mod db;
mod services;

use crate::api::routes::accounts::accounts::{get_account, get_account_list};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_account).service(get_account_list))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
