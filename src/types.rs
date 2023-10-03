use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OnlyMessageResponse {
    pub(crate) message: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub(crate) key_name: String,
    pub(crate) key_value: String,
}
