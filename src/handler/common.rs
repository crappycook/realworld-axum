use crate::dto::common::{EchoReq, EchoResp, SystemTimeResp};
use axum::{http::StatusCode, Json};
use chrono::Utc;

pub async fn system_time() -> (StatusCode, Json<SystemTimeResp>) {
    let now = Utc::now();
    let rsp = SystemTimeResp { t: now.timestamp() };
    (StatusCode::OK, Json(rsp))
}

pub async fn echo(Json(req): Json<EchoReq>) -> (StatusCode, Json<EchoResp>) {
    let rsp = EchoResp {
        msg: "server recv your msg: ".to_string() + &req.msg,
    };
    (StatusCode::OK, Json(rsp))
}
