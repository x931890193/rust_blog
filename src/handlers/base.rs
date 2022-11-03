use actix_protobuf::ProtoBufResponseBuilder as _;
use actix_web::{web, HttpResponse, Result};

use chrono::Local;
use rbatis::executor::RbatisRef;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::models::entity;
use crate::proto::pb;
use crate::utils::cache;

use rbatis::rbatis::Rbatis;

#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    msg: String,
    server_time: String,
}

/// This is index handler  db redis get demo
pub async fn index(
    db_pool: web::Data<Rbatis>,
    redis_pool: web::Data<cache::RedisPool>,
) -> HttpResponse {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let mut db = db_pool.acquire().await.unwrap();
    let resp = Greet {
        msg: String::from("rust_blog"),
        server_time: Local::now().format(fmt).to_string().to_owned(),
    };
    let mut redis_client = redis_pool.get().unwrap();
    let res = redis_client.set::<&str, i32, String>("11", 2222);
    match res {
        Ok(_) => {}
        Err(error) => {
            log::error!("{}", error)
        }
    }
    HttpResponse::Ok().json(resp) // <- send response
}
// return protobuf without req
pub async fn base_resp() -> Result<HttpResponse> {
    let base = pb::BaseResp {
        code: 0,
        msg: "Hello World!".to_string(),
    };
    HttpResponse::Ok().protobuf(base)
}
