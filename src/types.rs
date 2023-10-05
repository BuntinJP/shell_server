use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OnlyMessageBody {
    pub(crate) message: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub(crate) key_name: String,
    pub(crate) _key_value: String,
}

/*
type ExecRequest = {
    user: string,
    command: string,
    args?: string[],
}
*/
#[derive(Debug, Deserialize)]
pub struct ExecRequest {
    pub(crate) user: String,
    pub(crate) command: String,
    pub(crate) args: Option<Vec<String>>,
}

// Response(/exec/user)

/*
type Status = 'Success' | 'Error';

type ShellResponse = {
    status: Status;
    data: CommandOutput | null;
    error: CommandError | null;
}

type CommandOutput = {
    stdout: string;
    stderr: string;
    exit_code: number;
}

type CommandError = {
    message: string;
    code: string;
}

*/

#[derive(Serialize, Deserialize)]
pub enum Status {
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct ShellResponse {
    pub status: Status,
    pub data: Option<CommandOutput>,
    pub error: Option<CommandError>,
}

#[derive(Serialize, Deserialize)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CommandError {
    pub message: String,
    pub code: String,
}
