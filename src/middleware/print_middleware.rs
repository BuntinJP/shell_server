/* // middleware/print_middleware.rs
use actix_service::Service;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

pub struct PrintMiddleware;

impl<S, B> Transform<S, ServiceRequest> for PrintMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PrintMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PrintMiddlewareMiddleware { service }))
    }
}

pub struct PrintMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PrintMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Request: {:?}", req);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            println!("Response: {:?}", res);
            Ok(res)
        })
    }
}
 */
