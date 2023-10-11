use crate::endpoints::ep_utils::parse_output;
use crate::middleware::AuthMiddleware;
use crate::types::ExecRequest;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::process::Command;

pub struct Sudo;

impl Sudo {
    pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::resource(Self::path())
                .wrap(AuthMiddleware)
                .route(web::post().to(exec_user)),
        );
    }
    fn path() -> &'static str {
        "/exec/sudo"
    }
}

async fn exec_user(_req: HttpRequest, info: web::Json<ExecRequest>) -> impl Responder {
    if info.user != "root" {
        return HttpResponse::Forbidden().body("You post to the wrong endpoint");
    }
    let output = Command::new("sudo")
        .arg(&info.command)
        .args(&*info.args.as_ref().unwrap_or(&vec![]))
        .output();
    parse_output(output)
}
