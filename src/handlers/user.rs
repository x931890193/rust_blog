use actix_protobuf::{ProtoBufResponseBuilder as _};
use actix_web::{HttpResponse, Result};
use chrono::Local;

use crate::proto::pb;
use crate::utils::captcha;
use crate::utils::cache::{self, redis::Commands};

pub async fn get_captcha() -> Result<HttpResponse> {
    let cap = captcha::generate();
    let mut redis_client = cache::REDIS_POOL.get().unwrap();
    let id: i64 = Local::now().timestamp_millis();
    redis_client.set_ex::<i64, String, String>(id, cap.chars_as_string(), 60).unwrap();
    let resp = pb::CaptchaResp{
        code: 0,
        msg: "".to_string(),
        id: id.to_string(),
        img: String::from("data:image/png;base64,") + &cap.as_base64().unwrap(),
    };
    HttpResponse::Ok().protobuf(resp)
}