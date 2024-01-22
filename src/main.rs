use actix_web::{middleware, App, HttpServer};
use actix_rt::{spawn, time};
use std::time::Duration;
use rustc_version_runtime::version;

use rust_blog::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db_url = "mysql://root:flzx3qc@127.0.0.1:3306/blog";
    let _ = db::create_db_pool_rbatis(&db_url);

    let address = format!("{}:{}", config::CONFIGURATION.server.host, config::CONFIGURATION.server.port);
    log::info!("{}", format!("starting HTTP server at http://{}", address));
    // crontab task
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            println!("crontab task running！")
        }
    });

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
    .bind(&address)?
    .run()
    .await
}
