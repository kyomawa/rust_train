mod db;
mod models;
mod handlers;

use actix_web::{ web, App, HttpServer };
use handlers::users::create_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_client = db::init_db().await.expect("Failed to initialize db.");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(db_client.clone())).service(create_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
