use actix_web::web::{get, post, resource as r, scope, ServiceConfig};

use crate::handlers::base;
use crate::handlers::user;
use crate::handlers::websocket;

// dispatch router
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(r("/").route(get().to(base::index)))
        .service(
            scope("/admin")
                .service(r("/generate").route(get().to(base::base_resp)))
                .service(r("/captcha").route(get().to(user::get_captcha)))
                .service(r("/login").route(post().to(user::admin_login)))
                .service(r("/info").route(get().to(base::base_resp)))
                .service(r("/routers").route(get().to(base::base_resp)))
                .service(r("/logout").route(get().to(base::base_resp)))
                .service(r("/article/add").route(get().to(base::base_resp)))
                .service(r("/article/:id").route(get().to(base::base_resp)))
                .service(r("/article/:id").route(get().to(base::base_resp)))
                .service(r("/article/list").route(get().to(base::base_resp)))
                .service(r("/article/category/add").route(get().to(base::base_resp)))
                .service(r("/article/category/edit").route(get().to(base::base_resp)))
                .service(r("/article/category/list").route(get().to(base::base_resp)))
                .service(r("/link/add").route(get().to(base::base_resp)))
                .service(r("/link/edit").route(get().to(base::base_resp)))
                .service(r("/link/list").route(get().to(base::base_resp)))
                .service(r("/about/get").route(get().to(base::base_resp)))
                .service(r("/about/edit").route(get().to(base::base_resp)))
                .service(r("/dashboard/panelGroup").route(get().to(base::base_resp)))
                .service(r("/dashboard/lineChartData/:type").route(get().to(base::base_resp)))
                .service(r("/dashboard/access").route(get().to(base::base_resp)))
                .service(r("/dashboard/spiderData").route(get().to(base::base_resp)))
                .service(r("/dashboard/log/:LogType").route(get().to(base::base_resp)))
                .service(r("/system/setting/siteSetting").route(get().to(base::base_resp)))
                .service(r("/system/setting/siteSetting").route(get().to(base::base_resp)))
                .service(r("/tool/qiNiu/upload").route(get().to(base::base_resp))),
        )
        .service(scope("/article").service(r("/user/{user_id}").route(get().to(base::index))))
        .service(scope("/comment").service(r("/list").route(get().to(base::base_resp))))
        .service(scope("/resource").service(r("/list").route(get().to(base::base_resp))))
        .service(scope("/user").service(r("")))
        .service(scope("/reward").service(r("")))
        .service(scope("link").service(r("")))
        .service(scope("likeOrCollect").service(r("")))
        .service(r("/ws").route(get().to(websocket::calculate_online)))
        .service(r("/qrcode"));
}
