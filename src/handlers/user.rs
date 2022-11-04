use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder as _};
use actix_web::{web, HttpResponse, Result};
use chrono::Local;
use std::error::Error;

use crate::handlers::e;
use crate::models::wrapper;
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
        resp.msg = format!(
            "{} {}",
            e::ERROR_MSG[&e::AUTH_ERROR].clone(),
            "验证码错误".to_string()
        )
    } else {
        let user = wrapper::WrapperUser::new(&req.username, &req.password);
        let token_res = user.auth_user().await;
        match token_res {
            Ok(token) => resp.token = token.to_string(),
            Err(error) => {
                resp.code = e::AUTH_ERROR.clone() as u32;
                resp.msg = format!(
                    "{} {}",
                    e::ERROR_MSG[&e::AUTH_ERROR].clone(),
                    &error.to_string()
                );
            }
        }
    }
    HttpResponse::Ok().protobuf(resp)
}

pub async fn admin_info(req: ProtoBuf<pb::AdminInfoResp>) -> Result<HttpResponse>{
    let resp = pb::AdminInfoResp{
        name: "".to_string(),
        avatar: "".to_string(),
        job: "".to_string(),
        organization: "".to_string(),
        location: "".to_string(),
        email: "".to_string(),
        introduction: "".to_string(),
        personal_website: "".to_string(),
        job_name: "".to_string(),
        organization_name: "".to_string(),
        location_name: "".to_string(),
        phone: "".to_string(),
        registration_date: "".to_string(),
        account_id: "".to_string(),
        certification: "".to_string(),
        role: "".to_string(),
        code: 0,
        msg: "".to_string()
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn routers() -> Result<HttpResponse> {
    let mut resp = pb::AdminRouterResp{
        code: 0,
        msg: "".to_string(),
        data: Vec::new()
    };
    resp.data.push(pb::Component {
        component: "Layout".to_string(),
        path: "/article".to_string(),
        name: "".to_string(),
        meta: Some(pb::ComponentMeta {
            title: "文章管理".to_string(),
            name: "".to_string(),
            icon: "documentation".to_string(),
            no_cache: false,
            affix: false,
            active_menu: "".to_string()
        }),
        hidden: false,
        children: vec!(
            pb::Component {
                component: "blog/blog/index".to_string(),
                path: "".to_string(),
                name: "index".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "".to_string(),
                    name: "".to_string(),
                    icon: "documentation".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string()
                }),
                hidden: true,
                children: vec![]
            },
            pb::Component{
                component: "blog/blog/index".to_string(),
                path: "".to_string(),
                name: "AddBlog".to_string(),
                meta: Some(pb::ComponentMeta{
                    title: "文章管理".to_string(),
                    name: "".to_string(),
                    icon: "documentation".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "/article/index".to_string()
                }),
                hidden: true,
                children: vec![]},
            pb::Component{
                component: "blog/blog/index".to_string(),
                path: "".to_string(),
                name: "AddBlog".to_string(),
                meta: Some(pb::ComponentMeta{
                    title: "文章管理".to_string(),
                    name: "".to_string(),
                    icon: "documentation".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "/article/index".to_string()
                }),
                hidden: true,
                children: vec![]},
            pb::Component{
                component: "blog/blog/index".to_string(),
                path: "".to_string(),
                name: "AddBlog".to_string(),
                meta: Some(pb::ComponentMeta{
                    title: "文章管理".to_string(),
                    name: "".to_string(),
                    icon: "documentation".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "/article/index".to_string()
                }),
                hidden: true,
                children: vec![]},
            pb::Component{
                component: "Layout".to_string(),
                path: "/tool".to_string(),
                name: "".to_string(),
                meta: Some(pb::ComponentMeta{
                    title: "系统工具".to_string(),
                    name: "".to_string(),
                    icon: "tool".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string()
                }),
                hidden: true,
                children: vec![]},
        )
    });
    HttpResponse::Ok().protobuf(resp)
}

pub async fn login_out() -> Result<HttpResponse> {
    let resp = pb::BaseResp{ code: 0, msg: "".to_string() };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn github_oauth() -> HttpResponse {
    // let resp = pb::
    HttpResponse::MovedPermanently().append_header(("Location", "/")).finish()
}

pub async fn user_info() -> Result<HttpResponse> {
    let resp = pb::UserInfoResp{
        code: 0,
        msg: "".to_string(),
        user_id: 0,
        username: "".to_string(),
        status: 0,
        avatar: "".to_string(),
        linkname: "".to_string(),
        link_url: "".to_string(),
        link_desc: "".to_string(),
        logo_url: "".to_string(),
        state: false,
        label: 0,
        receive_update: false,
        token: "".to_string(),
        verify_status: "".to_string()
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn edit() -> Result<HttpResponse> {
    let resp = pb::EditUserInfoRequest{
        user_id: 0,
        label: 0,
        state: false,
        link_url: "".to_string(),
        linkname: "".to_string(),
        link_desc: "".to_string(),
        receive_update: false,
        logo_url: "".to_string()
    };
    HttpResponse::Ok().protobuf(resp)
}