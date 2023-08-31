extern crate dotenv;

// use actix_files as fs;
use actix_web::{
    web::{self, JsonConfig},
    App, HttpServer,
};
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
        let default_size = env::var("DEFAULT_REQUEST_SIZE")
            .unwrap_or_else(|_| "2097152".to_string())
            .parse::<usize>()
            .unwrap_or(2097152);
        App::new().service(
            web::scope("/page2doc")
                .app_data(JsonConfig::default().limit(default_size))
                .service(handlers::create_report)
                .service(handlers::generate_token)
                .service(handlers::index)
                // .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(handlers::get_pdf),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
