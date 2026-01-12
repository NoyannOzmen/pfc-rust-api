use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, web};
use sea_orm::DbConn;

pub mod auth;
mod animal;
mod association;
mod demande;
mod espece;
mod famille;
mod media;
mod tag;
mod utilisateur;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "UP",
        "message": "Hello World !"
    }))
}

pub fn configure_routes(cfg: &mut ServiceConfig, db: DbConn) {
    let db_data = web::Data::new(db.clone());

    cfg.app_data(db_data.clone())
        .route("/", web::get().to(hello))
        .service(web::scope("/connexion").configure(|c| auth::configure(c)))
        .service(web::scope("/animaux").configure(|c| animal::configure_public(c)))
        .service(web::scope("/associations").configure(|c| association::configure_public(c)))
        .service(web::scope("/demandes").configure(|c| demande::configure_public(c)))
        .service(web::scope("/especes").configure(|c| espece::configure_public(c)))
        .service(web::scope("/famille").configure(|c| famille::configure_public(c)))
        .service(web::scope("/media").configure(|c| media::configure_public(c)))
        .service(web::scope("/tags").configure(|c| tag::configure_public(c)))
        .service(web::scope("/users").configure(|c| utilisateur::configure(c)))
        .service(web::scope("/register").configure(|c| utilisateur::configure_public(c)));
}