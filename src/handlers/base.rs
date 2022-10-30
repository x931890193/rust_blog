use actix_protobuf::ProtoBufResponseBuilder as _;
use actix_web::{web, HttpResponse, Result};

use chrono::Local;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::models::entity;
use crate::proto::pb;
use crate::utils::cache;
use crate::{db, schema};

use self::schema::article::dsl::*;

#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    msg: String,
    server_time: String,
}

/// This is index handler  db redis get demo
pub async fn index(
    db_pool: web::Data<db::PoolDiesel>,
    redis_pool: web::Data<cache::RedisPool>,
) -> HttpResponse {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let mut pool = db_pool.get();
    let resp = Greet {
        msg: String::from("rust_blog"),
        server_time: Local::now().format(fmt).to_string().to_owned(),
    };
    let mut pool = redis_pool.get().unwrap();
    let res = pool.set::<&str, i32, String>("11", 2222);
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
