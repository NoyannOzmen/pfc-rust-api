use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, web};
use sea_orm::DbConn;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "UP",
        "message": "Hello World !"
    }))
}

pub fn configure_routes(cfg: &mut ServiceConfig, db: DbConn) {
    let db_data = web::Data::new(db.clone());

    cfg.app_data(db_data.clone())
        .route("/", web::get().to(hello));
}