use rbatis::rbatis::{Rbatis,};
use rbdc_mysql::driver::MysqlDriver;

use lazy_static::lazy_static;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type PoolDiesel = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// let db_url = "postgres://username:password@localhost/rust_db"
// diesel
pub fn create_db_pool_diesel(db_url: &str) -> PoolDiesel {
    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    // let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = PoolDiesel::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

// batis keep for test
pub fn create_db_pool_rbatis(db_url: &str) -> Rbatis  {
    let rb = Rbatis::new();
    rb.init(MysqlDriver{}, db_url).unwrap();
    rb
}


lazy_static! {
    pub static ref DB_POOL: PoolDiesel = {
        // let db_url = "mysql://root:flzx3qc@127.0.0.1:3306/blog";
        let db_url = "postgres://postgres:flzx3qc@localhost/blog";
        let pool = create_db_pool_diesel(&db_url);
        pool
    };
}
