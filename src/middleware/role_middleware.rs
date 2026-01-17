use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use futures::future::{Ready, ready};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::auth::jwt::Claims;

pub struct RoleGuard {
    pub role: String,
}

impl RoleGuard {
    pub fn new(role: String) -> Self {
        Self { role }
    }

    pub fn foster() -> Self {
        Self {
            role: "foster".to_string(),
        }
    }

    pub fn shelter() -> Self {
        Self {
            role: "shelter".to_string(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RoleGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleGuardMiddleware {
            service: Arc::new(service),
            role: self.role.clone(),
        }))
    }
}

pub struct RoleGuardMiddleware<S> {
    service: Arc<S>,
    role: String,
}

impl<S, B> Service<ServiceRequest> for RoleGuardMiddleware<S>
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
        let service = self.service.clone();
        let required_role = self.role.clone();

        Box::pin(async move {
            let has_permission = if let Some(claims) = req.extensions_mut().get::<Claims>() {
                let user_role_str = claims.role.to_lowercase();
                user_role_str == required_role.as_str()
            } else {
                return Err(ErrorUnauthorized("User not authenticated"));
            };

            if has_permission {
                service.call(req).await
            } else {
                Err(ErrorUnauthorized(format!("Insufficient permissions.")))
            }
        })
    }
}