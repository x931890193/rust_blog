use actix_protobuf::{ProtoBufResponseBuilder as _};
use actix_web::{web, HttpResponse, Result};

use crate::proto::pb;
use crate::utils::captcha;


pub async fn get_captcha() -> Result<HttpResponse> {
    let cap = captcha::generate();
    let resp = pb::CaptchaResp{
        code: 0,
        msg: "".to_string(),
        id: "".to_string(),
        img: cap.as_base64().unwrap(),
    };
    HttpResponse::Ok().protobuf(resp)
}