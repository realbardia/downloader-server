extern crate env_logger;

use actix_files::Files;
use handlebars::Handlebars;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use std::env;
use dotenv::dotenv;

mod handlers; 
mod structs;

use structs::general::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let listen_url: String = env::var("LISTEN_ADDRESS").unwrap_or("127.0.0.1:7686".to_string());
    let workers_count: usize = env::var("WORKERS_COUNT").unwrap_or("4".to_string()).parse::<usize>().unwrap_or(4);
    let static_url: String = env::var("STATIC_ROOT").unwrap_or("./static".to_string());
    let storage_url: String = env::var("STORAGE_ROOT").unwrap_or("./storage".to_string());

    println!("Listening on \"{}\" with {} workers", listen_url, workers_count);
    println!("To change configurations please edit .env file.");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", static_url)
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // let store = RedisStore::connect(&rate_limit_redis_address);
    let app = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("res: %s, ip: %a, time: %Dms, size: %b, \"%r\""))
            .data(StaticData {
                website_url: env::var("WEBSITE_URL").unwrap_or("https://tricks.aseman.io".to_string()),
                storage_url: env::var("STORAGE_ROOT").unwrap_or("./storage".to_string()),
            })

            .app_data(handlebars_ref.clone())
            .service(Files::new("/storage", storage_url.to_string()))
            .service(Files::new("/files/static", "static/website"))
            .route("/files/{file_id}", web::get().to(handlers::htmls::open))
    })
    .workers(workers_count);

    app.bind(listen_url)?
    .run()
    .await
}
