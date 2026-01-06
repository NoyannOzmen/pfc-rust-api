pub mod api;
pub mod config;

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
        App::new()
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