pub mod api;
pub mod auth;
pub mod config;
pub mod database;
pub mod validators;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use sea_orm::{Database, DbConn};

use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = AppConfig::from_env();

    log::info!(
        "Starting server at {}:{}",
        app_config.server.host,
        app_config.server.port
    );

    let db: DbConn = Database::connect(&app_config.database.url)
        .await
        .expect("Error connecting to the database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3001")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:4173")
            .allowed_origin("http://localhost:4200")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .configure(|config| api::configure_routes(config, db.clone()))
            .wrap(Logger::default())
    }).bind(format!(
        "{}:{}",
        app_config.server.host, app_config.server.port
    ))?
    .run()
    .await
}