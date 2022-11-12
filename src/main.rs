use std::fmt::format;
use actix_web::{middleware, App, HttpServer};
use rustc_version_runtime::version;

use rust_blog::*;
use std::os;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db_url = "mysql://root:flzx3qc@127.0.0.1:3306/blog";
    let _ = db::create_db_pool_rbatis(&db_url);

    let bind = format!("{}:{}", config::CONFIGURATION.server.host, config::CONFIGURATION.server.port);
    log::info!("{}", format!("starting HTTP server at http://{}", bind));
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db::DB_POOL.clone()))
            .app_data(actix_web::web::Data::new(utils::cache::REDIS_POOL.clone()))
            .wrap(middleware::DefaultHeaders::new().add(("Rust-Version", version().to_string())))
            .wrap(middleware::Logger::default())
            .wrap(middlewares::Request)
            .wrap(middlewares::BaseAuth)
            .configure(routers::config)
    })
    .bind(&bind)?
    .run()
    .await
}
