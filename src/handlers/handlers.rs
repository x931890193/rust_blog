use actix_web::{HttpResponse, };
use serde::{Deserialize, Serialize};
use chrono::{Local};

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