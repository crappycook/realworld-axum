use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SystemTimeResp {
    pub t: i64,
}

#[derive(Deserialize)]
pub struct EchoReq {
    pub msg: String,
}

#[derive(Serialize)]
pub struct EchoResp {
    pub msg: String,
}

// pub type AppResponse = (StatusCode, Json)
