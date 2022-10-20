mod handlers;
mod routers;
mod db;

use actix_web::{middleware, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let db_url = "postgres://postgres:flzx3qc@127.0.0.1/blog";
    let pool = db::create_db_pool(&db_url);
    let bind = "127.0.0.1:8080";
    log::info!("{}", format!("starting HTTP server at http://{}", bind));
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(routers::config)
    })
        .bind(&bind)?
        .run()
        .await
}
