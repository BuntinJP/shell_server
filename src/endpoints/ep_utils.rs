use crate::types::{CommandError, CommandOutput, ShellResponse, Status};
use actix_web::HttpResponse;
use std::{io::Error, process::Output};

pub fn parse_output(output: Result<Output, Error>) -> HttpResponse {
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
