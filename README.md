# ShellServer

### WORNING

**This program is not secure enough. Please use it at your own risk.**

Rust と Web の相性を確認したかったのと、スレッドセーフなキャッシュ`Arc<Mutex<T>>`を使ってみたかった。

## Endpoints

### [POST] /keys/register

#### key register

##### Request

method: `POST`
body: `RegisterRequest`

> header: `Content-Type: application/json`
> header: `Auth: ${MASTER_PASSWORD}`

MASTER_PASSWORD is shoud be set in env. But, if you don't set it, it will be generated automatically. Please check logs.

##### Response

body: `OnlyMessageBody`

> header: `Content-Type: application/json`

message include the result of authentication.

### [POST] /exec/user

#### Request

method: `POST`
body: `ExecRequest`

#### Response

body: `ShellResponse`

## types

### RegisterRequest

```rs
pub struct RegisterRequest {
    pub(crate) key_name: String,
    pub(crate) key_value: String,
}
```

### OnlyMessageBody

```rs
#[derive(Serialize)]
pub struct OnlyMessageBody {
    pub(crate) message: String,
}
```

### ExecRequest

```ts
type ExecRequest = {
  user: string;
  command: string;
  args?: string[];
};
```

### ShellResponse(/exec/user, /exec/root)

```ts
type Status = 'Success' | 'Error';

type ShellResponse = {
  status: Status;
  data: CommandOutput | null;
  error: CommandError | null;
};

type CommandOutput = {
  stdout: string;
  stderr: string;
  exit_code: number;
};

type CommandError = {
  message: string;
  code: string;
};
```

```rs
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
```
