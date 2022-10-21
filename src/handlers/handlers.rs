use actix_protobuf::{ProtoBufResponseBuilder as _};
use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};
use chrono::{Local};
use crate::proto::pb;


#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    msg: String,
    server_time: String,
}

/// This is index handler
pub async fn index() -> HttpResponse {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let resp = Greet{
        msg : String::from("rust_blog"),
        server_time: Local::now().format(fmt).to_string().to_owned()
    };
    HttpResponse::Ok().json(resp) // <- send response
}

pub async fn base_resp() -> Result<HttpResponse> {
    let base = pb::BaseResp{ code: 111111, msg: "111111111".to_string() };
    HttpResponse::Ok().protobuf(base)
}