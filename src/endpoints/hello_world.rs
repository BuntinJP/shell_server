use crate::middleware::AuthMiddleware;
use actix_web::{web, Responder};
pub struct HelloWorld;

impl HelloWorld {
    pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::resource(Self::path())
                .route(web::get().to(hello_world))
                .wrap(AuthMiddleware),
        );
    }
    fn path() -> &'static str {
        "/hello"
    }
}

async fn hello_world() -> impl Responder {
    "Hello, world!"
}
