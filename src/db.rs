use rbatis::rbatis::Rbatis;
use rbatis::rbatis::RbatisOption;
use rbdc_pg::driver::PgDriver;

use lazy_static::lazy_static;

pub fn create_db_pool_rbatis(db_url: &str) -> Rbatis {
    let rb = Rbatis::new();
    rb.init(PgDriver{}, db_url).unwrap();
    rb
}

lazy_static! {
    pub static ref DB_POOL: Rbatis = {
        let db_url = "postgres://postgres:flzx3qc@localhost/blog";
        let pool = create_db_pool_rbatis(db_url);
        pool
    };
}
