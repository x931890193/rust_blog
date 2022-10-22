use actix_protobuf::{ProtoBufResponseBuilder as _};
use actix_web::{web, HttpResponse, Result};
use rbatis::rbatis::Rbatis;

use serde::{Deserialize, Serialize};
use chrono::{Local};
use rbatis::executor::RbatisRef;
use crate::proto::pb;
use crate::models::article;


#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    msg: String,
    server_time: String,
}

/// This is index handler
pub async fn index(db_pool: web::Data<Rbatis>,) -> HttpResponse {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let mut rb = db_pool.get_rbatis();
    let resp = Greet{
        msg : String::from("rust_blog"),
        server_time: Local::now().format(fmt).to_string().to_owned()
    };
    let data = article::Article::select_all(&mut rb).await;
    println!("select_all = {:?}", data);
    HttpResponse::Ok().json(resp) // <- send response
}

pub async fn base_resp() -> Result<HttpResponse> {
    let base = pb::BaseResp{ code: 0, msg: "Hello World!".to_string() };
    HttpResponse::Ok().protobuf(base)
}