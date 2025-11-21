use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use actix_web::http::header;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    task::{Context, Poll},
};
use log;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareMiddleware { service }))
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token_header = req.headers().get(header::AUTHORIZATION);
        let mut is_authorized = false;
        let mut error_message = "Token faltante o inválido. Requiere Authorization: Bearer [JWT]".to_string();

        if let Some(auth_val) = token_header {
            if let Ok(auth_str) = auth_val.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Extraer el JWT

                    match crate::utils::jwt_utils::validate_token(token) {
                        Ok(claims) => {
                            is_authorized = true;
                            log::debug!("Petición autorizada para: {}", claims.sub);
                        }
                        Err(e) => {
                            error_message = format!("Error JWT: {}", e);
                        }
                    }
                }
            }
        }

        if !is_authorized {
            return Box::pin(async move {
                // Retorna 401 Unauthorized
                Err(actix_web::error::ErrorUnauthorized(error_message))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}