use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use futures::future::{Ready, ready};
use jsonwebtoken::{DecodingKey, Validation, decode};
/* use log::{info, warn}; */
use sea_orm::DbConn;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/* use crate::auth::{extract_foster_id_from_token, extract_shelter_id_from_token}; */
use crate::auth::jwt::{Claims, JWT_SECRET, decode_jwt};

pub struct AuthMiddleware {
    pub db: Arc<DbConn>,
}

impl AuthMiddleware {
    pub fn new(db: DbConn) -> Self {
        Self { db: Arc::new(db) }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Arc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return Box::pin(async {
                    Err(ErrorUnauthorized("Authorization header not found"))
                });
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async {
                    Err(ErrorUnauthorized("Invalid authorization header format"))
                });
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return Box::pin(async {
                Err(ErrorUnauthorized("Invalid authorization header format"))
            });
        }

        let token = auth_str.trim_start_matches("Bearer ").trim().to_string();
        let service = self.service.clone();

        Box::pin(async move {
            match decode_jwt(&token) {
                Ok(token_data) => {
                    let user_id = extract_user_id_from_token(&token).unwrap();
                    /* let shelter_id = extract_shelter_id_from_token(&token).unwrap();
                    req.extensions_mut().insert(shelter_id);
                    let foster_id =extract_foster_id_from_token(&token).unwrap();
                    req.extensions_mut().insert(foster_id); */
                    req.extensions_mut().insert(user_id);
                    req.extensions_mut().insert(token_data);
                    service.call(req).await
                }
                Err(err) => {
                    log::debug!("Token validation failed: {:?}", err);
                    Err(ErrorUnauthorized("Invalid token format. Please log in again.",))
                }
            }
        })
    }
}

fn extract_user_id_from_token(token: &str) -> Option<i32> {
    let validation = Validation::default();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => Some(token_data.claims.user_id),
        Err(e) => {
            log::error!("Failed to decode token: {:?}", e);
            None
        }
    }
}