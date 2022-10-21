use actix_web::web::{ServiceConfig, resource as r, get, scope, post};

use crate::handlers::handlers;

// dispatch router
pub fn config(cfg: &mut ServiceConfig) {
    cfg
        .service(r("/").route(get().to(handlers::index)))
        .service(
            scope("/api/v1")
                .service(r("/user").route(get().to(handlers::base_resp)))
        )
        .service(scope("/api/v2")
            .service(r("/user/{user_id}").route(get().to(handlers::index)))
        );
}

