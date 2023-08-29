extern crate dotenv;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Port must be number");

    HttpServer::new(|| {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let static_dir = current_dir.join("static");
        App::new().service(
            web::scope("/page2doc")
                .service(handlers::create_files)
                .service(handlers::generate_token)
                .service(handlers::index)
                .service(fs::Files::new("/static", static_dir).show_files_listing())
                .service(handlers::get_pdf),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
