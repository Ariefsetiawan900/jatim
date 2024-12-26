use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(handlers::register))
            .route("/login", web::post().to(handlers::login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
