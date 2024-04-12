use crate::dto::common::{EchoReq, EchoResp, SystemTimeResp};
use axum::{http::StatusCode, Json};
use chrono::Local;

pub async fn system_time() -> (StatusCode, Json<SystemTimeResp>) {
    let now = Local::now();
    let rsp = SystemTimeResp { t: now.timestamp() };
    tracing::info!("now: {:?}", now.to_rfc3339());
    (StatusCode::OK, Json(rsp))
}

pub async fn echo(Json(req): Json<EchoReq>) -> (StatusCode, Json<EchoResp>) {
    let rsp = EchoResp {
        msg: "server recv your msg: ".to_string() + &req.msg,
    };
    (StatusCode::OK, Json(rsp))
}
