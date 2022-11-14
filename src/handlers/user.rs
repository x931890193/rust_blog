use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder as _};
use actix_web::{http, web, HttpResponse, Result};
use chrono::Local;
use std::error::Error;
use actix_web::web::ReqData;
use serde::de::Unexpected::Str;
use crate::models::entity::User;

use crate::utils::e;
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

pub async fn admin_info(user: Option<ReqData<Option<User>>>) -> Result<HttpResponse> {
    let mut resp = pb::AdminInfoResp {
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
        msg: "".to_string(),
    };
    if let Some(req_data) = user {
        let user = req_data.as_ref().unwrap();
        resp.name = String::from(user.user_name.as_ref().unwrap());
        resp.avatar = String::from(user.avatar.as_ref().unwrap())
    }
    HttpResponse::Ok().protobuf(resp)
}

pub async fn routers() -> Result<HttpResponse> {
    let mut resp = pb::AdminRouterResp {
        code: 0,
        msg: "".to_string(),
        data: vec![
            pb::Component {
                component: "Layout".to_string(),
                path: "/article".to_string(),
                name: "".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "文章管理".to_string(),
                    name: "".to_string(),
                    icon: "documentation".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string(),
                }),
                hidden: false,
                children: vec![
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
                            active_menu: "".to_string(),
                        }),
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/blog/index".to_string(),
                        path: "".to_string(),
                        name: "AddBlog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "文章管理".to_string(),
                            name: "".to_string(),
                            icon: "documentation".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/article/index".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/blog/add".to_string(),
                        path: "add".to_string(),
                        name: "BlogAdd".to_string(),
                        meta: None,
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/blog/edit".to_string(),
                        path: "edit/*".to_string(),
                        name: "BlogEdit".to_string(),
                        meta: None,
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/category/index".to_string(),
                        path: "categories".to_string(),
                        name: "categories".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "分类管理".to_string(),
                            name: "".to_string(),
                            icon: "component".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/article/categories".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/comment/index".to_string(),
                        path: "Comments".to_string(),
                        name: "Comments".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "评论管理".to_string(),
                            name: "".to_string(),
                            icon: "message".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/article/comments".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "blog/tag/index".to_string(),
                        path: "tags".to_string(),
                        name: "Tags".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "标签管理".to_string(),
                            name: "".to_string(),
                            icon: "code".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/article/tags".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                ],
            },
            pb::Component {
                component: "Layout".to_string(),
                path: "".to_string(),
                name: "/log".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "日志管理".to_string(),
                    name: "".to_string(),
                    icon: "log".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string(),
                }),
                hidden: false,
                children: vec![
                    pb::Component {
                        component: "log/loginLog/index".to_string(),
                        path: "".to_string(),
                        name: "".to_string(),
                        meta: None,
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "log/loginLog/index".to_string(),
                        path: "loginLog".to_string(),
                        name: "loginLog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "登陆日志".to_string(),
                            name: "".to_string(),
                            icon: "logininfor".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/log/loginLog".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "log/operateLog/index".to_string(),
                        path: "operateLog".to_string(),
                        name: "operateLog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "操作日志".to_string(),
                            name: "".to_string(),
                            icon: "form".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/log/operateLog".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "log/quartzLog/index".to_string(),
                        path: "quartzLog".to_string(),
                        name: "quartzLog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "任务日志".to_string(),
                            name: "".to_string(),
                            icon: "guide".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/log/quartzLog".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "log/RealTimeLog/index".to_string(),
                        path: "RealTimeLog".to_string(),
                        name: "RealTimeLog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "实时日志".to_string(),
                            name: "".to_string(),
                            icon: "online".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/log/realTimeLog".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "log/visitLog/index".to_string(),
                        path: "visitLog".to_string(),
                        name: "visitLog".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "访问日志".to_string(),
                            name: "".to_string(),
                            icon: "people".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/log/visitLog".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                ],
            },
            pb::Component {
                component: "Layout".to_string(),
                path: "/monitor".to_string(),
                name: "".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "系统监控".to_string(),
                    name: "".to_string(),
                    icon: "monitor".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string(),
                }),
                hidden: false,
                children: vec![
                    pb::Component {
                        component: "monitor/blacklist/index".to_string(),
                        path: "".to_string(),
                        name: "".to_string(),
                        meta: None,
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "monitor/blacklist/index".to_string(),
                        path: "blacklist".to_string(),
                        name: "blacklist".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "黑名单管理".to_string(),
                            name: "".to_string(),
                            icon: "password".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/monitor/blacklist".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "monitor/online/index".to_string(),
                        path: "online".to_string(),
                        name: "online".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "在线用户".to_string(),
                            name: "".to_string(),
                            icon: "online".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/monitor/online".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "monitor/redis/index".to_string(),
                        path: "redis".to_string(),
                        name: "redis".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "redis状态".to_string(),
                            name: "".to_string(),
                            icon: "password".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/monitor/redis".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "monitor/server/index".to_string(),
                        path: "server".to_string(),
                        name: "server".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "server状态".to_string(),
                            name: "".to_string(),
                            icon: "password".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/monitor/server".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                ],
            },
            pb::Component {
                component: "Layout".to_string(),
                path: "/system".to_string(),
                name: "".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "网站管理".to_string(),
                    name: "".to_string(),
                    icon: "system".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string(),
                }),
                hidden: false,
                children: vec![
                    pb::Component {
                        component: "system/carousel/index".to_string(),
                        name: "".to_string(),
                        meta: None,
                        hidden: false,
                        path: "".to_string(),
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/carousel/index".to_string(),
                        name: "carousel".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "轮播图".to_string(),
                            name: "".to_string(),
                            icon: "example".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/carousel".to_string(),
                        }),
                        hidden: false,
                        path: "carousel".to_string(),
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/link/index".to_string(),
                        path: "link".to_string(),
                        name: "link".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "友链".to_string(),
                            name: "".to_string(),
                            icon: "people".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/link".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/notice/index".to_string(),
                        path: "notice".to_string(),
                        name: "notice".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "公告".to_string(),
                            name: "".to_string(),
                            icon: "online".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/notice".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/role/index".to_string(),
                        path: "role".to_string(),
                        name: "role".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "角色管理".to_string(),
                            name: "".to_string(),
                            icon: "user".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/role".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/setting/index".to_string(),
                        path: "setting".to_string(),
                        name: "setting".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "网站设置".to_string(),
                            name: "".to_string(),
                            icon: "system".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/setting".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "system/user/index".to_string(),
                        path: "user".to_string(),
                        name: "user".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "用户管理".to_string(),
                            name: "".to_string(),
                            icon: "user".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/system/user".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                ],
            },
            pb::Component {
                component: "Layout".to_string(),
                path: "/tool".to_string(),
                name: "".to_string(),
                meta: Some(pb::ComponentMeta {
                    title: "系统工具".to_string(),
                    name: "".to_string(),
                    icon: "tool".to_string(),
                    no_cache: false,
                    affix: false,
                    active_menu: "".to_string(),
                }),
                hidden: false,
                children: vec![
                    pb::Component {
                        component: "tool/quartz/index".to_string(),
                        path: "".to_string(),
                        name: "".to_string(),
                        meta: None,
                        hidden: true,
                        children: vec![],
                    },
                    pb::Component {
                        component: "tool/quartz/index".to_string(),
                        path: "quartz".to_string(),
                        name: "quartz".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "quartz状态".to_string(),
                            name: "".to_string(),
                            icon: "".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/tool/quartz".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                    pb::Component {
                        component: "tool/storage/index".to_string(),
                        path: "storage".to_string(),
                        name: "storage".to_string(),
                        meta: Some(pb::ComponentMeta {
                            title: "存储状态".to_string(),
                            name: "".to_string(),
                            icon: "".to_string(),
                            no_cache: false,
                            affix: false,
                            active_menu: "/tool/storage".to_string(),
                        }),
                        hidden: false,
                        children: vec![],
                    },
                ],
            },
        ],
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn login_out() -> Result<HttpResponse> {
    let resp = pb::BaseResp {
        code: 0,
        msg: "".to_string(),
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn github_oauth() -> HttpResponse {
    // let resp = pb::
    HttpResponse::MovedPermanently()
        .append_header((http::header::LOCATION, "/"))
        .finish()
}

pub async fn user_info() -> Result<HttpResponse> {
    let resp = pb::UserInfoResp {
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
        verify_status: "".to_string(),
    };
    HttpResponse::Ok().protobuf(resp)
}

pub async fn edit() -> Result<HttpResponse> {
    let resp = pb::EditUserInfoRequest {
        user_id: 0,
        label: 0,
        state: false,
        link_url: "".to_string(),
        linkname: "".to_string(),
        link_desc: "".to_string(),
        receive_update: false,
        logo_url: "".to_string(),
    };
    HttpResponse::Ok().protobuf(resp)
}
