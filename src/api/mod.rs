use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, web};
use sea_orm::DbConn;

use crate::middleware::AuthMiddleware;

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
        .service(
            web::scope("/connexion")
            .configure(|c| auth::configure(c))
        )
        .service(web::scope("/animaux/nouveau-profil")
            .wrap(AuthMiddleware::new(db.clone()))
            .configure(|c| animal::configure_create(c))
        )
        .service(
            web::scope("/animaux")
            .configure(|c| animal::configure_public(c))
            .service(
                web::scope("/{id}/requests")
                .wrap(AuthMiddleware::new(db.clone()))
                .configure(|c| animal::configure_protected_req(c))
            )
            .service(
                web::scope("/{id}/faire-une-demande")
                .wrap(AuthMiddleware::new(db.clone()))
                .configure(|c| animal::configure_protected_foster(c))
            )
        )
        .service(
            web::scope("/associations/inscription")
            .configure(|c| association::configure_register(c))
        )
        .service(
            web::scope("/associations/profil")
            .wrap(AuthMiddleware::new(db.clone()))
            .configure(|c| association::configure(c))
        )
        .service(
            web::scope("/associations")
            .configure(|c| association::configure_public(c))
            .service(
                web::scope("/{id}/fostered")
                .wrap(AuthMiddleware::new(db.clone()))
                .configure(|c| association::configure_protected_fostered(c))
            )
            .service(
                web::scope("/{id}/requested")
                .wrap(AuthMiddleware::new(db.clone()))
                .configure(|c| association::configure_protected_requested(c))
            )
        )
        .service(
            web::scope("/demandes")
            .wrap(AuthMiddleware::new(db.clone()))
            .configure(|c| demande::configure_protected(c))
        )
        .service(
            web::scope("/especes")
            .configure(|c| espece::configure_public(c))
        )
        .service(
            web::scope("/famille/inscription")
            .configure(|c| famille::configure_register(c))
        )       
        .service(
            web::scope("/famille/profil")
            .wrap(AuthMiddleware::new(db.clone()))
            .configure(|c| famille::configure_protected(c))
        )
        .service(
            web::scope("/media")
            .configure(|c| media::configure_public(c))
        )
        .service(web::scope("/tags/create")
            .wrap(AuthMiddleware::new(db.clone()))
            .configure(|c| tag::configure_protected(c))
        )
        .service(
            web::scope("/tags")
            .configure(|c| tag::configure_public(c))
        )
        .service(
            web::scope("/users")
            .configure(|c| utilisateur::configure_protected(c))
        );
}