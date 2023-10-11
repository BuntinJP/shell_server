use crate::types::{CommandError, CommandOutput, ExecRequest, ShellResponse, Status};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::process::Command;
pub struct Users;

impl Users {
    pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::resource(Self::path()).route(web::post().to(exec_user)));
    }
    fn path() -> &'static str {
        "/exec/user"
    }
}

async fn exec_user(_req: HttpRequest, info: web::Json<ExecRequest>) -> impl Responder {
    if info.user == "root" {
        return HttpResponse::Forbidden().body("You can't run commands as root");
    }
    let output = Command::new("sudo")
        .arg("-u")
        .arg(&info.user)
        .arg(&info.command)
        .args(&*info.args.as_ref().unwrap_or(&vec![]))
        .output();
    match output {
        Ok(output) => {
            let result = CommandOutput {
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                exit_code: output.status.code().unwrap_or(-1),
            };
            if output.status.success() {
                HttpResponse::Ok().json(ShellResponse {
                    status: Status::Success,
                    data: Some(result),
                    error: None,
                })
            } else {
                HttpResponse::InternalServerError().json(ShellResponse {
                    status: Status::Error,
                    data: Some(result),
                    error: Some(CommandError {
                        message: "Command execution failed".to_string(),
                        code: "COMMAND_ERROR".to_string(),
                    }),
                })
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ShellResponse {
            status: Status::Error,
            data: None,
            error: Some(CommandError {
                message: format!("Error: {}", e),
                code: "COMMAND_EXECUTION_ERROR".to_string(),
            }),
        }),
    }
}
