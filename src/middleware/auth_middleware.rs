use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderMap,
    http::StatusCode,
    Error,
};
use futures::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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

impl<S, B> Service<ServiceRequest> for AuthMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if auth(&req.headers()) {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // Create a new ServiceResponse with a Forbidden status
            let res: ServiceResponse<B> = req.error_response(StatusCode::FORBIDDEN);
            Box::pin(async move { Ok(res) })
        }
    }
}

// Authentication function
fn auth(headers: &HeaderMap) -> bool {
    // For now, always return true. In reality, inspect headers to decide.
    true
}
