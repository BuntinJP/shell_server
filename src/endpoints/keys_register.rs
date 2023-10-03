use crate::types::{OnlyMessageResponse, RegisterRequest};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::env;

pub struct KeysRegister;

impl KeysRegister {
    pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::resource(Self::path()).route(web::post().to(register_key)));
    }

    fn path() -> &'static str {
        "/keys/register"
    }
}

async fn register_key(req: HttpRequest, info: web::Json<RegisterRequest>) -> impl Responder {
    let master_password = env::var("MASTER_PASSWORD").expect("MASTER_PASSWORD must be set");
    if let Some(password) = req.headers().get("Auth") {
        match password.to_str() {
            Ok(password_str) if password_str == master_password => {
                HttpResponse::Ok().json(OnlyMessageResponse {
                    message: format!("Key {} registered", info.key_name),
                })
            }
            _ => HttpResponse::Forbidden().body("Incorrect password"),
        }
    } else {
        HttpResponse::BadRequest().body("Password missing")
    }
}
