use rbatis::rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use crate::config::CONFIGURATION;

use lazy_static::lazy_static;

pub fn create_db_pool_rbatis(db_url: &str) -> Rbatis {
    let rb = Rbatis::new();
    rb.init(PgDriver {}, db_url).unwrap();
    rb
}

lazy_static! {
    pub static ref DB_POOL: Rbatis = {
        let db_url = format!("postgres://{}:{}@{}/{}", CONFIGURATION.db.pg_user, CONFIGURATION.db.password, CONFIGURATION.db.host, CONFIGURATION.db.db);
        let pool = create_db_pool_rbatis(&db_url);
        pool
    };
}
