use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SctlRequest {
    command: String,
}

#[derive(Serialize)]
pub struct SctlResponse {
    message: String,
}

pub struct SctlSv;

impl SctlSv {
    pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::resource(Self::path()).route(web::post().to(sctl))); // HTTPメソッドをGETからPOSTに変更
    }

    fn path() -> &'static str {
        "/systemctl"
    }
}

async fn sctl(req_body: web::Json<SctlRequest>) -> impl Responder {
    let command = &req_body.command;

    let response = SctlResponse {
        message: format!("Received command: {}", command),
    };

    HttpResponse::Ok().json(response)
}
