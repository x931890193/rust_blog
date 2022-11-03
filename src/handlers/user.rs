use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder as _};
use actix_web::{web, HttpResponse, Result};
use chrono::Local;

use crate::handlers::e;
use crate::proto::pb;
use crate::utils::cache::{self, redis::Commands};
use crate::utils::captcha;

pub async fn get_captcha() -> Result<HttpResponse> {
    let cap = captcha::generate();
    let mut redis_client = cache::REDIS_POOL.get().unwrap();
    let id: i64 = Local::now().timestamp_millis();
    redis_client
        .set_ex::<i64, String, String>(id, cap.chars_as_string(), 60)
        .unwrap();
    let resp = pb::CaptchaResp {
        code: 0,
        msg: "".to_string(),
        id: id.to_string(),
        img: String::from("data:image/png;base64,") + &cap.as_base64().unwrap(),
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn admin_login(req: ProtoBuf<pb::LoginAdminRequest>) -> Result<HttpResponse> {
    let mut resp = pb::LoginAdminResp {
        code: 0,
        msg: String::new(),
        token: String::new(),
    };
    let flag = captcha::verify(req.id.parse::<i64>().unwrap(), req.code.clone());
    if !flag {
        resp.code = e::AUTH_ERROR.clone() as u32;
        resp.msg = e::ERROR_MSG[&e::AUTH_ERROR].clone()
    } else {
        resp.token = "111111".to_string();
    }
    HttpResponse::Ok().protobuf(resp)
}
