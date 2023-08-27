extern crate dotenv;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Port must be number");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/page2doc")
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(handlers::create_files)
                .service(handlers::index),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
