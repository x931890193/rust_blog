use actix_protobuf::{ProtoBufResponseBuilder as _};
use actix_web::{web, HttpResponse, Result};

use redis::Commands;
use serde::{Deserialize, Serialize};
use chrono::{Local};

use crate::proto::pb;
use crate::models::entity;
use crate::{db, schema};
use crate::utils::cache;

use self::schema::article::dsl::*;

#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    msg: String,
    server_time: String,
}

/// This is index handler  db redis get demo
pub async fn index(db_pool: web::Data<db::DB_POOL>, redis_pool: web::Data<cache::REDIS_POOL>) -> HttpResponse {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let mut pool = db_pool.get();
    let resp = Greet{
        msg : String::from("rust_blog"),
        server_time: Local::now().format(fmt).to_string().to_owned()
    };
    // let results = article
    //     .filter(support.eq(true))
    //     .limit(5)
    //     .load::<article::Article>(pool)
    //     .expect("Error loading posts");

    // let data = article::Article::select_all(&mut rb).await;
    // println!("select_all = {:?}", data);
    let mut pool = redis_pool.get().unwrap();
    let res = pool.set::<&str, i32, String>("11", 2222);
    match res {
        Ok(_) => {},
        Err(error) => {
            log::error!("{}", error)
        }
    }
    HttpResponse::Ok().json(resp) // <- send response
}
// return protobuf without req
pub async fn base_resp() -> Result<HttpResponse> {
    let base = pb::BaseResp{ code: 0, msg: "Hello World!".to_string() };
    HttpResponse::Ok().protobuf(base)
}

