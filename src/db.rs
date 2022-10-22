use rbatis::rbatis::{Rbatis,};
use rbdc_mysql::driver::MysqlDriver;



use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type PoolDiesel = r2d2::Pool<ConnectionManager<MysqlConnection>>;
// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// let db_url = "postgres://username:password@localhost/rust_db"
// diesel
pub fn create_db_pool_diesel(db_url: &str) -> PoolDiesel {
    // set up database connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    // let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = PoolDiesel::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

// rbatis
pub fn create_db_pool_rbatis(db_url: &str) -> Rbatis  {
    let rb = Rbatis::new();
    rb.init(MysqlDriver{}, db_url).unwrap();
    rb
}