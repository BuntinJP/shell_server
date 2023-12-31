use actix_service::Service;
use actix_web::body::BoxBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;
use std::sync::Arc;
use std::task::{Context, Poll}; // 追加

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareTransform<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareTransform { service })
    }
}

pub struct AuthMiddlewareTransform<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match auth(&req) {
            Ok(true) => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Ok(false) | Err(_) => {
                let error_response = AuthErrorResponse {
                    error: "Unauthorized".to_string(),
                };
                let response: HttpResponse = HttpResponse::Unauthorized()
                    .body(serde_json::to_string(&error_response).unwrap());
                let res: ServiceResponse<BoxBody> =
                    ServiceResponse::new(req.into_parts().0, response);
                Box::pin(futures::future::ok(res))
            }
        }
    }
}

fn auth(req: &ServiceRequest) -> Result<bool, &'static str> {
    if let Some(passwords) = req.app_data::<Data<Arc<Vec<String>>>>() {
        let headers = req.headers();
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                return Ok(passwords.iter().any(|pw| pw == auth_str));
            }
        }
        Ok(false)
    } else {
        Err("Passwords not found")
    }
}

#[derive(Serialize)]
struct AuthErrorResponse {
    error: String,
}
